//! Columnar positional index with FST vocabularies and KWIC search.

mod build;
mod error;
mod kwic;
mod postings;
mod reader;
mod varint;

pub use build::{build_corpus_index, IndexBuilder, IndexStats};
pub use error::{IndexError, IndexResult};
pub use kwic::{format_kwic_plain, search_kwic, KwicHit, KwicOptions, KwicSort};
pub use reader::CorpusIndex;
