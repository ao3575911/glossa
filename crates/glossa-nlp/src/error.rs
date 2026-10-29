use thiserror::Error;

pub type NlpResult<T> = Result<T, NlpError>;

#[derive(Debug, Error)]
pub enum NlpError {
    #[error("{0}")]
    Message(String),
}
