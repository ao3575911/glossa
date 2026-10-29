//! Rule-based NLP utilities used before neural models (Stage 3).

mod abbrev;
mod error;
mod sentence;
mod tokenize;

pub use abbrev::ENGLISH_ABBREVIATIONS;
pub use error::{NlpError, NlpResult};
pub use sentence::{split_sentences, Sentence};
pub use tokenize::{normalize_nfc, tokenize, Token};
