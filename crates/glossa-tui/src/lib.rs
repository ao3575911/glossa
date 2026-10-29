//! Glossa full-screen TUI (ratatui + crossterm).

mod app;
mod theme;

pub use app::{run_terminal, App, RunConfig, Tab};

/// Run the TUI (alternate screen, raw mode).
pub fn run(config: RunConfig) -> anyhow::Result<()> {
    run_terminal(config)
}

#[cfg(test)]
mod tests;
