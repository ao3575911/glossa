use anyhow::{bail, Result};
use glossa_index::{search_kwic, CorpusIndex, KwicHit, KwicOptions, KwicSort};
use glossa_workspace::Workspace;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct KwicRequest {
    pub query: String,
    pub corpus: Option<String>,
    pub regex: bool,
    pub sort: String,
    pub context: usize,
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct KwicResult {
    pub query: String,
    pub corpus: String,
    pub hits: Vec<KwicHit>,
    pub hit_count: usize,
    pub corpus_hash: String,
}

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

pub fn execute(ws: &Workspace, args: KwicRequest) -> Result<KwicResult> {
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
            .ok_or_else(|| anyhow::anyhow!("invalid sort (use L1, R1, or R2)"))?
    };

    let opts = KwicOptions {
        regex: args.regex,
        sort: sort_key,
        context: args.context,
        limit: args.limit,
    };
    let hits = search_kwic(&index, &args.query, &opts)?;
    let hit_count = hits.len();

    Ok(KwicResult {
        query: args.query,
        corpus: corpus_name,
        hits,
        hit_count,
        corpus_hash: index.corpus_hash().to_string(),
    })
}
