use crate::error::WorkspaceResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub const SESSION_LOG: &str = "session.jsonl";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub ts: DateTime<Utc>,
    pub command: String,
    pub args: Value,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

pub struct SessionLog {
    path: PathBuf,
}

impl SessionLog {
    pub fn open(dot_dir: &Path) -> Self {
        Self {
            path: dot_dir.join(SESSION_LOG),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append(&self, event: &SessionEvent) -> WorkspaceResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        serde_json::to_writer(&mut file, event)?;
        file.write_all(b"\n")?;
        file.flush()?;
        Ok(())
    }

    /// Ensure the log file exists (empty is fine).
    pub fn ensure_exists(&self) -> WorkspaceResult<()> {
        if !self.path.exists() {
            File::create(&self.path)?;
        }
        Ok(())
    }
}
