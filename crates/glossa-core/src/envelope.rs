use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::Duration;

/// Crate version embedded in every JSON envelope.
pub const GLOSSA_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Provenance-bearing result wrapper used by every `--json` command.
#[derive(Debug, Clone, Serialize)]
pub struct ResultEnvelope<T: Serialize> {
    pub glossa_version: String,
    pub command: String,
    pub params: BTreeMap<String, Value>,
    pub corpus_hash: Option<String>,
    pub model_hashes: BTreeMap<String, String>,
    pub started_at: DateTime<Utc>,
    pub duration_ms: u64,
    pub data: T,
}

impl<T: Serialize> ResultEnvelope<T> {
    pub fn new(
        command: impl Into<String>,
        params: BTreeMap<String, Value>,
        started_at: DateTime<Utc>,
        duration: Duration,
        data: T,
    ) -> Self {
        Self {
            glossa_version: GLOSSA_VERSION.to_string(),
            command: command.into(),
            params,
            corpus_hash: None,
            model_hashes: BTreeMap::new(),
            started_at,
            duration_ms: u64::try_from(duration.as_millis()).unwrap_or(u64::MAX),
            data,
        }
    }

    pub fn with_corpus_hash(mut self, hash: impl Into<String>) -> Self {
        self.corpus_hash = Some(hash.into());
        self
    }

    pub fn with_model_hash(mut self, name: impl Into<String>, hash: impl Into<String>) -> Self {
        self.model_hashes.insert(name.into(), hash.into());
        self
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
