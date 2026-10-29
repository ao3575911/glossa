use crate::error::{IndexError, IndexResult};
use crate::postings::PostingsReader;
use fst::Map;
use memmap2::Mmap;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct DocMeta {
    pub filename: String,
    pub token_start: u32,
    pub token_count: u32,
    pub sentence_start: u32,
    pub sentence_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub corpus_hash: String,
    pub token_count: u64,
    pub doc_count: u64,
    pub sentence_count: u64,
    pub attributes: Vec<String>,
    pub documents: Vec<DocMeta>,
}

pub struct AttrColumn {
    pub map: Map<Mmap>,
    pub ids: Mmap,
    pub postings: PostingsReader,
}

impl AttrColumn {
    pub fn token_id(&self, pos: u32) -> IndexResult<u32> {
        let start = pos as usize * 4;
        let end = start + 4;
        if end > self.ids.len() {
            return Err(IndexError::Corrupt(format!(
                "token position {pos} out of range"
            )));
        }
        let bytes: [u8; 4] = self.ids[start..end]
            .try_into()
            .map_err(|_| IndexError::Corrupt("id slice".into()))?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn token_count(&self) -> u32 {
        (self.ids.len() / 4) as u32
    }

    pub fn lookup_term(&self, term: &str) -> Option<u32> {
        self.map.get(term).map(|v| v as u32)
    }

    pub fn term_at_pos(&self, pos: u32) -> IndexResult<String> {
        let id = self.token_id(pos)?;
        // Stream map keys — FST Map doesn't give id→key directly; keep a reverse cache externally.
        // For KWIC we resolve via streaming once into a vec.
        Err(IndexError::Message(format!(
            "use CorpusIndex::surface_at for term resolution (id={id})"
        )))
    }
}

pub struct CorpusIndex {
    pub root: PathBuf,
    pub manifest: Manifest,
    pub attrs: BTreeMap<String, AttrColumn>,
    /// word id → surface form (built from FST)
    pub word_forms: Vec<String>,
    pub lower_forms: Vec<String>,
    sentences: Mmap,
    documents: Mmap,
}

impl CorpusIndex {
    pub fn open(corpus_dir: &Path) -> IndexResult<Self> {
        let manifest_text = std::fs::read_to_string(corpus_dir.join("manifest.json"))?;
        let manifest: Manifest = serde_json::from_str(&manifest_text)?;
        let index_dir = corpus_dir.join("index");

        let mut attrs = BTreeMap::new();
        for attr in &manifest.attributes {
            let map_file = File::open(index_dir.join(format!("{attr}.fst")))?;
            let map_mmap = unsafe { Mmap::map(&map_file)? };
            let map = Map::new(map_mmap)?;

            let ids_file = File::open(index_dir.join(format!("{attr}.col")))?;
            let ids = unsafe { Mmap::map(&ids_file)? };

            let postings = PostingsReader::open(&index_dir.join(format!("{attr}.postings")))?;
            attrs.insert(attr.clone(), AttrColumn { map, ids, postings });
        }

        let sentences = unsafe { Mmap::map(&File::open(index_dir.join("sentences.col"))?)? };
        let documents = unsafe { Mmap::map(&File::open(index_dir.join("documents.col"))?)? };

        let word_forms = invert_fst(
            attrs
                .get("word")
                .ok_or_else(|| IndexError::MissingAttr("word".into()))?,
        )?;
        let lower_forms = invert_fst(
            attrs
                .get("lower")
                .ok_or_else(|| IndexError::MissingAttr("lower".into()))?,
        )?;

        Ok(Self {
            root: corpus_dir.to_path_buf(),
            manifest,
            attrs,
            word_forms,
            lower_forms,
            sentences,
            documents,
        })
    }

    pub fn corpus_hash(&self) -> &str {
        &self.manifest.corpus_hash
    }

    pub fn token_count(&self) -> u64 {
        self.manifest.token_count
    }

    pub fn word_at(&self, pos: u32) -> IndexResult<&str> {
        let col = self
            .attrs
            .get("word")
            .ok_or_else(|| IndexError::MissingAttr("word".into()))?;
        let id = col.token_id(pos)? as usize;
        self.word_forms
            .get(id)
            .map(String::as_str)
            .ok_or_else(|| IndexError::Corrupt(format!("word id {id} missing")))
    }

    pub fn lower_at(&self, pos: u32) -> IndexResult<&str> {
        let col = self
            .attrs
            .get("lower")
            .ok_or_else(|| IndexError::MissingAttr("lower".into()))?;
        let id = col.token_id(pos)? as usize;
        self.lower_forms
            .get(id)
            .map(String::as_str)
            .ok_or_else(|| IndexError::Corrupt(format!("lower id {id} missing")))
    }

    pub fn read_u32_col(mmap: &Mmap, idx: usize) -> IndexResult<u32> {
        let start = idx * 4;
        let end = start + 4;
        if end > mmap.len() {
            return Err(IndexError::Corrupt("boundary column OOB".into()));
        }
        let bytes: [u8; 4] = mmap[start..end]
            .try_into()
            .map_err(|_| IndexError::Corrupt("boundary slice".into()))?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn sentence_count(&self) -> usize {
        self.sentences.len() / 4
    }

    pub fn document_boundary_count(&self) -> usize {
        self.documents.len() / 4
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn document_for_token(&self, pos: u32) -> IndexResult<&DocMeta> {
        for doc in &self.manifest.documents {
            let end = doc.token_start + doc.token_count;
            if pos >= doc.token_start && pos < end {
                return Ok(doc);
            }
        }
        Err(IndexError::Message(format!(
            "no document for token position {pos}"
        )))
    }

    pub fn attr(&self, name: &str) -> IndexResult<&AttrColumn> {
        self.attrs
            .get(name)
            .ok_or_else(|| IndexError::MissingAttr(name.to_string()))
    }
}

fn invert_fst(col: &AttrColumn) -> IndexResult<Vec<String>> {
    use fst::Streamer;
    let mut stream = col.map.stream();
    let mut max_id = 0u64;
    let mut pairs: Vec<(u64, String)> = Vec::new();
    while let Some((k, v)) = stream.next() {
        max_id = max_id.max(v);
        let term = String::from_utf8(k.to_vec())
            .map_err(|e| IndexError::Corrupt(format!("non-utf8 term: {e}")))?;
        pairs.push((v, term));
    }
    let mut out = vec![String::new(); (max_id as usize).saturating_add(1)];
    for (id, term) in pairs {
        if let Some(slot) = out.get_mut(id as usize) {
            *slot = term;
        }
    }
    Ok(out)
}
