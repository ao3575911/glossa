use crate::output::{self, param_str, param_u64, params_from, EmitArgs};
use anyhow::Result;
use chrono::Utc;
use glossa_commands::corpus as corpus_cmd;
use glossa_core::OutputMode;
use glossa_workspace::CorpusRecord;
use serde::Serialize;
use std::path::PathBuf;
use std::time::Instant;

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
    ws: &glossa_workspace::Workspace,
    path: PathBuf,
    name: Option<String>,
    meta: Option<PathBuf>,
    mode: OutputMode,
) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let path_display = path.display().to_string();
    let data = corpus_cmd::add(ws, path, name, meta)?;

    let params = params_from([
        param_str("path", path_display),
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
            corpus_hash: Some(data.corpus_hash.clone()),
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

pub fn ls(ws: &glossa_workspace::Workspace, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let corpora = corpus_cmd::list(ws)?;
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

pub fn info(ws: &glossa_workspace::Workspace, name: &str, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let corpus = corpus_cmd::info(ws, name)?;
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

pub fn rm(ws: &glossa_workspace::Workspace, name: &str, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let removed = corpus_cmd::remove(ws, name)?;
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
