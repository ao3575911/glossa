use crate::output::{self, param_str, param_u64, params_from, EmitArgs};
use anyhow::{bail, Result};
use chrono::Utc;
use glossa_core::OutputMode;
use glossa_index::build_corpus_index;
use glossa_io::{apply_metadata, import_path, load_metadata_csv};
use glossa_workspace::{CorpusRecord, Workspace};
use serde::Serialize;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Serialize)]
struct AddData {
    name: String,
    path: String,
    token_count: u64,
    doc_count: u64,
    sentence_count: u64,
    corpus_hash: String,
    documents: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LsData {
    corpora: Vec<CorpusRecord>,
}

#[derive(Debug, Serialize)]
struct InfoData {
    corpus: CorpusRecord,
}

#[derive(Debug, Serialize)]
struct RmData {
    removed: CorpusRecord,
}

pub fn add(
    ws: &Workspace,
    path: PathBuf,
    name: Option<String>,
    meta: Option<PathBuf>,
    mode: OutputMode,
) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();

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

    let data = AddData {
        name: corpus_name,
        path: corpus_dir.display().to_string(),
        token_count: stats.token_count,
        doc_count: stats.doc_count,
        sentence_count: stats.sentence_count,
        corpus_hash: stats.corpus_hash.clone(),
        documents: batch.documents.iter().map(|d| d.filename.clone()).collect(),
    };

    let params = params_from([
        param_str("path", path.display().to_string()),
        param_str("name", data.name.clone()),
        param_u64("token_count", data.token_count),
    ]);

    output::emit(
        EmitArgs {
            mode,
            command: "corpus add",
            params,
            started_at,
            duration: started.elapsed(),
            corpus_hash: Some(stats.corpus_hash),
            data: &data,
        },
        |d| {
            println!(
                "Added corpus '{}' ({} docs, {} tokens, hash {})",
                d.name, d.doc_count, d.token_count, d.corpus_hash
            );
            for name in &d.documents {
                println!("  - {name}");
            }
        },
    )?;
    Ok(())
}

pub fn ls(ws: &Workspace, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let corpora = ws.list_corpora()?;
    let data = LsData { corpora };

    output::emit(
        EmitArgs {
            mode,
            command: "corpus ls",
            params: Default::default(),
            started_at,
            duration: started.elapsed(),
            corpus_hash: None,
            data: &data,
        },
        |d| {
            if d.corpora.is_empty() {
                println!("(no corpora)");
                return;
            }
            for c in &d.corpora {
                println!(
                    "{}\t{}\ttokens={}\tdocs={}\thash={}",
                    c.name, c.path, c.token_count, c.doc_count, c.corpus_hash
                );
            }
        },
    )?;
    Ok(())
}

pub fn info(ws: &Workspace, name: &str, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let corpus = ws.get_corpus(name)?;
    let hash = corpus.corpus_hash.clone();
    let data = InfoData { corpus };

    output::emit(
        EmitArgs {
            mode,
            command: "corpus info",
            params: params_from([param_str("name", name)]),
            started_at,
            duration: started.elapsed(),
            corpus_hash: Some(hash),
            data: &data,
        },
        |d| {
            let c = &d.corpus;
            println!("name: {}", c.name);
            println!("path: {}", c.path);
            println!("tokens: {}", c.token_count);
            println!("docs: {}", c.doc_count);
            println!("sentences: {}", c.sentence_count);
            println!("hash: {}", c.corpus_hash);
            println!("created_at: {}", c.created_at.to_rfc3339());
        },
    )?;
    Ok(())
}

pub fn rm(ws: &Workspace, name: &str, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let removed = ws.remove_corpus(name)?;
    let hash = removed.corpus_hash.clone();
    let data = RmData { removed };

    output::emit(
        EmitArgs {
            mode,
            command: "corpus rm",
            params: params_from([param_str("name", name)]),
            started_at,
            duration: started.elapsed(),
            corpus_hash: Some(hash),
            data: &data,
        },
        |d| {
            println!("Removed corpus '{}'", d.removed.name);
        },
    )?;
    Ok(())
}
