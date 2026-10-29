use anyhow::Result;
use chrono::{DateTime, Utc};
use glossa_core::{OutputMode, ResultEnvelope};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::Duration;

pub struct EmitArgs<'a, T> {
    pub mode: OutputMode,
    pub command: &'a str,
    pub params: BTreeMap<String, Value>,
    pub started_at: DateTime<Utc>,
    pub duration: Duration,
    pub corpus_hash: Option<String>,
    pub data: &'a T,
}

pub fn emit<T, F>(args: EmitArgs<'_, T>, human: F) -> Result<()>
where
    T: Serialize,
    F: FnOnce(&T),
{
    match args.mode {
        OutputMode::Json => {
            let mut env = ResultEnvelope::new(
                args.command,
                args.params,
                args.started_at,
                args.duration,
                args.data,
            );
            if let Some(h) = args.corpus_hash {
                env = env.with_corpus_hash(h);
            }
            println!("{}", env.to_json_string()?);
        }
        OutputMode::Plain | OutputMode::Human => {
            human(args.data);
        }
    }
    Ok(())
}

pub fn param_str(key: &str, value: impl Into<String>) -> (String, Value) {
    (key.to_string(), Value::String(value.into()))
}

pub fn param_bool(key: &str, value: bool) -> (String, Value) {
    (key.to_string(), json!(value))
}

pub fn param_u64(key: &str, value: u64) -> (String, Value) {
    (key.to_string(), json!(value))
}

pub fn params_from(pairs: impl IntoIterator<Item = (String, Value)>) -> BTreeMap<String, Value> {
    pairs.into_iter().collect()
}
