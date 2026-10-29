use crate::error::{IndexError, IndexResult};
use crate::postings::write_postings;
use byteorder::{LittleEndian, WriteBytesExt};
use fst::MapBuilder;
use glossa_io::{AnnotatedToken, Document, ImportBatch};
use glossa_nlp::{split_sentences, tokenize};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

const ATTRS: &[&str] = &["word", "lower", "lemma", "upos"];

#[derive(Debug, Clone, Serialize)]
pub struct IndexStats {
    pub token_count: u64,
    pub doc_count: u64,
    pub sentence_count: u64,
    pub corpus_hash: String,
    pub vocab_sizes: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Serialize)]
struct Manifest {
    version: u32,
    corpus_hash: String,
    token_count: u64,
    doc_count: u64,
    sentence_count: u64,
    attributes: Vec<String>,
    documents: Vec<DocMeta>,
}

#[derive(Debug, Clone, Serialize)]
struct DocMeta {
    filename: String,
    token_start: u32,
    token_count: u32,
    sentence_start: u32,
    sentence_count: u32,
}

struct AttrBuilder {
    /// provisional id assignment (insertion order)
    vocab: HashMap<String, u32>,
    /// reverse: provisional id → term
    terms: Vec<String>,
    /// column of provisional ids
    column: Vec<u32>,
}

impl AttrBuilder {
    fn new() -> Self {
        Self {
            vocab: HashMap::new(),
            terms: Vec::new(),
            column: Vec::new(),
        }
    }

    fn push(&mut self, value: &str) {
        let id = if let Some(&id) = self.vocab.get(value) {
            id
        } else {
            let id = self.terms.len() as u32;
            self.terms.push(value.to_string());
            self.vocab.insert(value.to_string(), id);
            id
        };
        self.column.push(id);
    }
}

pub struct IndexBuilder {
    attrs: BTreeMap<&'static str, AttrBuilder>,
    sentence_starts: Vec<u32>,
    document_starts: Vec<u32>,
    documents: Vec<DocMeta>,
    token_count: u32,
    hasher: Sha256,
}

impl Default for IndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexBuilder {
    pub fn new() -> Self {
        let mut attrs = BTreeMap::new();
        for a in ATTRS {
            attrs.insert(*a, AttrBuilder::new());
        }
        Self {
            attrs,
            sentence_starts: Vec::new(),
            document_starts: Vec::new(),
            documents: Vec::new(),
            token_count: 0,
            hasher: Sha256::new(),
        }
    }

    pub fn add_document(&mut self, doc: &Document) -> IndexResult<()> {
        let token_start = self.token_count;
        let sentence_start = self.sentence_starts.len() as u32;
        self.document_starts.push(token_start);

        self.hasher.update(doc.filename.as_bytes());
        self.hasher.update([0]);
        self.hasher.update(doc.text.as_bytes());

        if let Some(ann) = &doc.annotations {
            for sent in ann {
                self.sentence_starts.push(self.token_count);
                for tok in sent {
                    self.push_annotated(tok);
                }
            }
        } else {
            let sentences = split_sentences(&doc.text);
            if sentences.is_empty() {
                let toks = tokenize(&doc.text);
                if !toks.is_empty() {
                    self.sentence_starts.push(self.token_count);
                    for t in toks {
                        self.push_surface(&t.text, None, None);
                    }
                }
            } else {
                for sent in sentences {
                    self.sentence_starts.push(self.token_count);
                    for t in sent.tokens {
                        self.push_surface(&t.text, None, None);
                    }
                }
            }
        }

        let token_count = self.token_count - token_start;
        let sentence_count = self.sentence_starts.len() as u32 - sentence_start;
        self.documents.push(DocMeta {
            filename: doc.filename.clone(),
            token_start,
            token_count,
            sentence_start,
            sentence_count,
        });
        Ok(())
    }

    fn push_annotated(&mut self, tok: &AnnotatedToken) {
        self.push_surface(&tok.form, tok.lemma.as_deref(), tok.upos.as_deref());
    }

    fn push_surface(&mut self, form: &str, lemma: Option<&str>, upos: Option<&str>) {
        let lower = form.to_lowercase();
        let lemma = lemma.unwrap_or(form);
        let upos = upos.unwrap_or("_");
        self.record("word", form);
        self.record("lower", &lower);
        self.record("lemma", lemma);
        self.record("upos", upos);
        self.token_count += 1;
    }

    fn record(&mut self, attr: &'static str, value: &str) {
        if let Some(a) = self.attrs.get_mut(attr) {
            a.push(value);
        }
    }

    pub fn finish(mut self, out_dir: &Path) -> IndexResult<IndexStats> {
        fs::create_dir_all(out_dir)?;
        let index_dir = out_dir.join("index");
        fs::create_dir_all(&index_dir)?;

        self.sentence_starts.push(self.token_count);
        self.document_starts.push(self.token_count);

        let mut vocab_sizes = BTreeMap::new();

        for attr in ATTRS {
            let builder = self
                .attrs
                .remove(attr)
                .ok_or_else(|| IndexError::MissingAttr((*attr).into()))?;

            // Deterministic final ids: sort terms lexicographically.
            let mut sorted_terms = builder.terms.clone();
            sorted_terms.sort();
            sorted_terms.dedup(); // shouldn't need, but keep deterministic

            let mut final_id: BTreeMap<&str, u32> = BTreeMap::new();
            for (i, term) in sorted_terms.iter().enumerate() {
                final_id.insert(term.as_str(), i as u32);
            }

            // Remap provisional → final
            let mut remap = vec![0u32; builder.terms.len()];
            for (prov, term) in builder.terms.iter().enumerate() {
                remap[prov] = *final_id.get(term.as_str()).ok_or_else(|| {
                    IndexError::Message(format!("missing final id for {attr}:{term}"))
                })?;
            }

            let fst_path = index_dir.join(format!("{attr}.fst"));
            let wtr = BufWriter::new(File::create(&fst_path)?);
            let mut map_builder = MapBuilder::new(wtr)?;
            for term in &sorted_terms {
                let id = final_id[term.as_str()];
                map_builder.insert(term, u64::from(id))?;
            }
            map_builder.finish()?;
            vocab_sizes.insert((*attr).to_string(), sorted_terms.len() as u64);

            let mut ids: Vec<u32> = Vec::with_capacity(builder.column.len());
            let mut postings: Vec<Vec<u32>> = vec![Vec::new(); sorted_terms.len()];
            for (pos, &prov) in builder.column.iter().enumerate() {
                let tid = remap[prov as usize];
                ids.push(tid);
                postings[tid as usize].push(pos as u32);
            }
            write_u32_column(&index_dir.join(format!("{attr}.col")), &ids)?;
            write_postings(&index_dir.join(format!("{attr}.postings")), &postings)?;
        }

        write_u32_column(&index_dir.join("sentences.col"), &self.sentence_starts)?;
        write_u32_column(&index_dir.join("documents.col"), &self.document_starts)?;

        let digest = self.hasher.finalize();
        let corpus_hash = hex::encode(digest);

        let manifest = Manifest {
            version: 1,
            corpus_hash: corpus_hash.clone(),
            token_count: u64::from(self.token_count),
            doc_count: self.documents.len() as u64,
            sentence_count: self.sentence_starts.len().saturating_sub(1) as u64,
            attributes: ATTRS.iter().map(|s| (*s).to_string()).collect(),
            documents: self.documents,
        };
        fs::write(
            out_dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)?,
        )?;
        fs::write(index_dir.join("corpus.sha256"), format!("{corpus_hash}\n"))?;

        Ok(IndexStats {
            token_count: manifest.token_count,
            doc_count: manifest.doc_count,
            sentence_count: manifest.sentence_count,
            corpus_hash,
            vocab_sizes,
        })
    }
}

fn write_u32_column(path: &Path, values: &[u32]) -> IndexResult<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    for v in values {
        w.write_u32::<LittleEndian>(*v)?;
    }
    w.flush()?;
    Ok(())
}

/// Build an on-disk index for an import batch under `corpus_dir`.
pub fn build_corpus_index(batch: &ImportBatch, corpus_dir: &Path) -> IndexResult<IndexStats> {
    let mut builder = IndexBuilder::new();
    for doc in &batch.documents {
        builder.add_document(doc)?;
    }
    builder.finish(corpus_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glossa_io::{Document, DocumentKind};
    use indexmap::IndexMap;
    use tempfile::tempdir;

    #[test]
    fn builds_nonempty_index() {
        let dir = tempdir().unwrap();
        let mut batch = ImportBatch::default();
        batch.documents.push(Document {
            filename: "a.txt".into(),
            source_path: "a.txt".into(),
            kind: DocumentKind::Txt,
            text: "Hello world. Hello again.".into(),
            metadata: IndexMap::new(),
            annotations: None,
        });
        let stats = build_corpus_index(&batch, dir.path()).unwrap();
        assert!(stats.token_count > 0);
        assert_eq!(stats.doc_count, 1);
        assert!(dir.path().join("index/word.fst").exists());
        assert!(dir.path().join("manifest.json").exists());
    }

    #[test]
    fn index_roundtrip_positions() {
        use crate::kwic::{search_kwic, KwicOptions};
        use crate::reader::CorpusIndex;

        let dir = tempdir().unwrap();
        let mut batch = ImportBatch::default();
        batch.documents.push(Document {
            filename: "b.txt".into(),
            source_path: "b.txt".into(),
            kind: DocumentKind::Txt,
            text: "alpha beta alpha gamma".into(),
            metadata: IndexMap::new(),
            annotations: None,
        });
        build_corpus_index(&batch, dir.path()).unwrap();
        let index = CorpusIndex::open(dir.path()).unwrap();
        let hits = search_kwic(&index, "alpha", &KwicOptions::default()).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(index.word_at(hits[0].position).unwrap(), "alpha");
    }
}
