use thiserror::Error;

pub type WorkspaceResult<T> = Result<T, WorkspaceError>;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("workspace not found (run `glossa init` first): looked from {0}")]
    NotFound(String),

    #[error("workspace already exists at {0}")]
    AlreadyExists(String),

    #[error("corpus not found: {0}")]
    CorpusNotFound(String),

    #[error("corpus already exists: {0}")]
    CorpusExists(String),

    #[error("{0}")]
    Message(String),
}
