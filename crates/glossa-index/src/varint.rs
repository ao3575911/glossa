//! Unsigned LEB128-style varints for posting lists.

use crate::error::{IndexError, IndexResult};

pub fn write_varint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn read_varint(input: &[u8], offset: &mut usize) -> IndexResult<u64> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        if *offset >= input.len() {
            return Err(IndexError::Corrupt("truncated varint".into()));
        }
        let byte = input[*offset];
        *offset += 1;
        result |= u64::from(byte & 0x7F) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(IndexError::Corrupt("varint overflow".into()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        for v in [0u64, 1, 127, 128, 255, 16_383, 1_000_000, u32::MAX as u64] {
            let mut buf = Vec::new();
            write_varint(&mut buf, v);
            let mut off = 0;
            assert_eq!(read_varint(&buf, &mut off).unwrap(), v);
            assert_eq!(off, buf.len());
        }
    }
}
