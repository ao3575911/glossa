//! Document importers for Glossa corpora.

mod conllu;
mod docx;
mod error;
mod html;
mod meta;
mod pdf;
mod txt;
mod types;

pub use conllu::import_conllu;
pub use docx::import_docx;
pub use error::{IoError, IoResult};
pub use html::import_html;
pub use meta::load_metadata_csv;
pub use pdf::import_pdf;
pub use txt::{decode_bytes, import_txt};
pub use types::{apply_metadata, import_path, AnnotatedToken, Document, DocumentKind, ImportBatch};
