use crate::error::{IndexError, IndexResult};
use crate::varint::{read_varint, write_varint};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

/// On-disk layout:
/// `[u32 num_ids][u64 offset_table[num_ids+1]][delta+varint posting bytes...]`
/// Positions for each id are delta-encoded then varint-compressed.
pub fn write_postings(path: &Path, postings: &[Vec<u32>]) -> IndexResult<()> {
    let mut body = Vec::new();
    let mut offsets = Vec::with_capacity(postings.len() + 1);
    offsets.push(0u64);
    for positions in postings {
        let mut prev = 0u32;
        for (i, &pos) in positions.iter().enumerate() {
            let delta = if i == 0 {
                pos
            } else {
                pos.checked_sub(prev)
                    .ok_or_else(|| IndexError::Message("postings must be strictly sorted".into()))?
            };
            write_varint(&mut body, u64::from(delta));
            prev = pos;
        }
        offsets.push(body.len() as u64);
    }

    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    w.write_u32::<LittleEndian>(postings.len() as u32)?;
    for off in &offsets {
        w.write_u64::<LittleEndian>(*off)?;
    }
    w.write_all(&body)?;
    w.flush()?;
    Ok(())
}

pub struct PostingsReader {
    data: Vec<u8>,
    num_ids: u32,
    /// offsets into the posting body; length = num_ids + 1
    offsets: Vec<u64>,
    body_start: usize,
}

impl PostingsReader {
    pub fn open(path: &Path) -> IndexResult<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Self::from_bytes(data)
    }

    pub fn from_bytes(data: Vec<u8>) -> IndexResult<Self> {
        if data.len() < 4 {
            return Err(IndexError::Corrupt("postings too short".into()));
        }
        let mut cursor = &data[..];
        let num_ids = cursor.read_u32::<LittleEndian>()?;
        let table_bytes = (num_ids as usize + 1) * 8;
        if data.len() < 4 + table_bytes {
            return Err(IndexError::Corrupt(
                "postings offset table truncated".into(),
            ));
        }
        let mut offsets = Vec::with_capacity(num_ids as usize + 1);
        for _ in 0..=num_ids {
            offsets.push(cursor.read_u64::<LittleEndian>()?);
        }
        let body_start = 4 + table_bytes;
        Ok(Self {
            data,
            num_ids,
            offsets,
            body_start,
        })
    }

    pub fn positions(&self, id: u32) -> IndexResult<Vec<u32>> {
        if id >= self.num_ids {
            return Ok(Vec::new());
        }
        let start = self.offsets[id as usize] as usize;
        let end = self.offsets[id as usize + 1] as usize;
        let slice = &self.data[self.body_start + start..self.body_start + end];
        let mut out = Vec::new();
        let mut offset = 0;
        let mut prev = 0u32;
        while offset < slice.len() {
            let delta = read_varint(slice, &mut offset)?;
            let pos = prev.saturating_add(delta as u32);
            out.push(pos);
            prev = pos;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn postings_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("p.bin");
        let lists = vec![vec![1u32, 5, 10], vec![], vec![0, 2, 100]];
        write_postings(&path, &lists).unwrap();
        let r = PostingsReader::open(&path).unwrap();
        assert_eq!(r.positions(0).unwrap(), vec![1, 5, 10]);
        assert_eq!(r.positions(1).unwrap(), Vec::<u32>::new());
        assert_eq!(r.positions(2).unwrap(), vec![0, 2, 100]);
    }
}
