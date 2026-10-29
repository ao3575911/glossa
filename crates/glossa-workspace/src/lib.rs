//! Workspace layout, SQLite metadata, and append-only session log.

mod config;
mod error;
mod log;
mod store;

pub use config::{default_glossa_toml, find_workspace_root, WorkspaceConfig};
pub use error::{WorkspaceError, WorkspaceResult};
pub use log::{SessionEvent, SessionLog};
pub use store::{CorpusRecord, Workspace};
