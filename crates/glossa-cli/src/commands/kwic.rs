use crate::output::{self, param_bool, param_str, param_u64, params_from, EmitArgs};
use anyhow::{bail, Result};
use chrono::Utc;
use glossa_core::OutputMode;
use glossa_index::{format_kwic_plain, search_kwic, CorpusIndex, KwicHit, KwicOptions, KwicSort};
use glossa_workspace::Workspace;
use serde::Serialize;
use std::time::Instant;

/// Resolve a corpus directory from its registered name only (never from a stored path).
fn resolve_corpus_dir(ws: &Workspace, name: &str) -> Result<std::path::PathBuf> {
    if name.is_empty()
        || name.contains(['/', '\\', '\0'])
        || name == "."
        || name == ".."
        || name.contains("..")
    {
        bail!("invalid corpus name: {name}");
    }
    let dir = ws.corpora_dir().join(name);
    let corpora = ws
        .corpora_dir()
        .canonicalize()
        .map_err(|e| anyhow::anyhow!("corpora dir missing: {e}"))?;
    let canon = dir
        .canonicalize()
        .map_err(|e| anyhow::anyhow!("corpus '{name}' not on disk: {e}"))?;
    if !canon.starts_with(&corpora) {
        bail!("refusing to open corpus outside workspace corpora dir");
    }
    if !canon.join("manifest.json").is_file() {
        bail!("corpus index missing manifest.json for '{name}'");
    }
    Ok(canon)
}

#[derive(Debug, Serialize)]
struct KwicData {
    query: String,
    corpus: String,
    hits: Vec<KwicHit>,
    hit_count: usize,
}

pub struct KwicArgs {
    pub query: String,
    pub corpus: Option<String>,
    pub regex: bool,
    pub sort: String,
    pub context: usize,
    pub limit: usize,
    pub mode: OutputMode,
}

pub fn run(ws: &Workspace, args: KwicArgs) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();

    let corpus_name = match args.corpus {
        Some(n) => n,
        None => {
            let list = ws.list_corpora()?;
            match list.as_slice() {
                [only] => only.name.clone(),
                [] => bail!("no corpora in workspace; run `glossa corpus add` first"),
                _ => bail!("multiple corpora present; pass --corpus <name>"),
            }
        }
    };

    let _record = ws.get_corpus(&corpus_name)?;
    let corpus_path = resolve_corpus_dir(ws, &corpus_name)?;
    let index = CorpusIndex::open(&corpus_path)?;

    let sort_key = if args.sort.is_empty() {
        KwicSort::None
    } else {
        KwicSort::parse(&args.sort)
            .ok_or_else(|| anyhow::anyhow!("invalid --sort (use L1, R1, or R2)"))?
    };

    let opts = KwicOptions {
        regex: args.regex,
        sort: sort_key,
        context: args.context,
        limit: args.limit,
    };
    let hits = search_kwic(&index, &args.query, &opts)?;
    let hit_count = hits.len();
    let context = args.context;
    let data = KwicData {
        query: args.query.clone(),
        corpus: corpus_name.clone(),
        hits,
        hit_count,
    };

    let params = params_from([
        param_str("query", args.query),
        param_str("corpus", corpus_name),
        param_bool("regex", args.regex),
        param_str("sort", args.sort),
        param_u64("context", args.context as u64),
        param_u64("limit", args.limit as u64),
    ]);

    output::emit(
        EmitArgs {
            mode: args.mode,
            command: "kwic",
            params,
            started_at,
            duration: started.elapsed(),
            corpus_hash: Some(index.corpus_hash().to_string()),
            data: &data,
        },
        |d| {
            if d.hits.is_empty() {
                println!("(no hits)");
                return;
            }
            print!("{}", format_kwic_plain(&d.hits, context));
        },
    )?;
    Ok(())
}
