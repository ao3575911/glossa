use crate::output::{self, param_bool, param_str, param_u64, params_from, EmitArgs};
use anyhow::Result;
use chrono::Utc;
use glossa_commands::kwic::{self, KwicRequest};
use glossa_core::OutputMode;
use glossa_index::{format_kwic_plain, KwicHit};
use glossa_workspace::Workspace;
use serde::Serialize;
use std::time::Instant;

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
    let context = args.context;

    let res = kwic::execute(
        ws,
        KwicRequest {
            query: args.query.clone(),
            corpus: args.corpus.clone(),
            regex: args.regex,
            sort: args.sort.clone(),
            context: args.context,
            limit: args.limit,
        },
    )?;
    let corpus_hash = res.corpus_hash.clone();
    let data = KwicData {
        query: res.query,
        corpus: res.corpus.clone(),
        hits: res.hits,
        hit_count: res.hit_count,
    };

    let params = params_from([
        param_str("query", args.query),
        param_str("corpus", data.corpus.clone()),
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
            corpus_hash: Some(corpus_hash),
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
