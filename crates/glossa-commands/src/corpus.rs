use anyhow::{bail, Result};
use chrono::Utc;
use glossa_index::build_corpus_index;
use glossa_io::{apply_metadata, import_path, load_metadata_csv};
use glossa_workspace::{CorpusRecord, Workspace};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct AddResult {
    pub name: String,
    pub path: String,
    pub token_count: u64,
    pub doc_count: u64,
    pub sentence_count: u64,
    pub corpus_hash: String,
    pub documents: Vec<String>,
}

pub fn add(
    ws: &Workspace,
    path: PathBuf,
    name: Option<String>,
    meta: Option<PathBuf>,
) -> Result<AddResult> {
    let corpus_name = name.unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("corpus")
            .to_string()
    });

    if ws.get_corpus(&corpus_name).is_ok() {
        bail!("corpus already exists: {corpus_name} (remove it first with `glossa corpus rm`)");
    }

    let mut batch = import_path(&path)?;
    if let Some(meta_path) = &meta {
        let meta_map = load_metadata_csv(meta_path)?;
        apply_metadata(&mut batch, &meta_map);
    }
    if batch.documents.is_empty() {
        bail!("no supported documents found at {}", path.display());
    }

    let corpus_dir = ws.corpora_dir().join(&corpus_name);
    std::fs::create_dir_all(&corpus_dir)?;
    let stats = build_corpus_index(&batch, &corpus_dir)?;

    let record = CorpusRecord {
        name: corpus_name.clone(),
        path: corpus_dir.display().to_string(),
        token_count: stats.token_count,
        doc_count: stats.doc_count,
        sentence_count: stats.sentence_count,
        corpus_hash: stats.corpus_hash.clone(),
        created_at: Utc::now(),
    };
    ws.upsert_corpus(&record)?;

    for doc in &batch.documents {
        let meta_map: serde_json::Map<String, serde_json::Value> = doc
            .metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        ws.insert_document(&corpus_name, &doc.filename, &meta_map, 0)?;
    }

    Ok(AddResult {
        name: corpus_name,
        path: corpus_dir.display().to_string(),
        token_count: stats.token_count,
        doc_count: stats.doc_count,
        sentence_count: stats.sentence_count,
        corpus_hash: stats.corpus_hash,
        documents: batch.documents.iter().map(|d| d.filename.clone()).collect(),
    })
}

pub fn list(ws: &Workspace) -> Result<Vec<CorpusRecord>> {
    ws.list_corpora().map_err(Into::into)
}

pub fn info(ws: &Workspace, name: &str) -> Result<CorpusRecord> {
    ws.get_corpus(name).map_err(Into::into)
}

pub fn remove(ws: &Workspace, name: &str) -> Result<CorpusRecord> {
    ws.remove_corpus(name).map_err(Into::into)
}
