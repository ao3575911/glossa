use thiserror::Error;

pub type IndexResult<T> = Result<T, IndexError>;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FST error: {0}")]
    Fst(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("corrupt index: {0}")]
    Corrupt(String),

    #[error("attribute not found: {0}")]
    MissingAttr(String),

    #[error("{0}")]
    Message(String),
}

impl From<fst::Error> for IndexError {
    fn from(value: fst::Error) -> Self {
        IndexError::Fst(value.to_string())
    }
}
