use thiserror::Error;

pub type IoResult<T> = Result<T, IoError>;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML error: {0}")]
    Xml(String),

    #[error("PDF error: {0}")]
    Pdf(String),

    #[error("unsupported format for {0}")]
    Unsupported(String),

    #[error("encoding error: {0}")]
    Encoding(String),

    #[error("{0}")]
    Message(String),
}
