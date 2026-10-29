//! Core types shared across Glossa crates: result envelopes and errors.

mod envelope;
mod error;
mod output;

pub use envelope::{ResultEnvelope, GLOSSA_VERSION};
pub use error::{CoreError, CoreResult};
pub use output::{resolve_output_mode, OutputMode};
