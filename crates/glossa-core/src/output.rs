/// How the CLI should render command output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputMode {
    #[default]
    Human,
    Json,
    Plain,
}

/// Resolve output mode from mutually exclusive flags. `--json` wins over `--plain`.
pub fn resolve_output_mode(json: bool, plain: bool) -> OutputMode {
    if json {
        OutputMode::Json
    } else if plain {
        OutputMode::Plain
    } else {
        OutputMode::Human
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_wins_over_plain() {
        assert_eq!(resolve_output_mode(true, true), OutputMode::Json);
    }
}
