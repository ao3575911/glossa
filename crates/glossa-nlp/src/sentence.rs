use crate::tokenize::{is_sentence_ender, tokenize, Token};

/// A sentence as a contiguous slice of tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub tokens: Vec<Token>,
    pub start: usize,
    pub end: usize,
}

/// Split text into sentences using UAX #29-ish tokenisation plus an English abbreviation list.
pub fn split_sentences(text: &str) -> Vec<Sentence> {
    let tokens = tokenize(text);
    if tokens.is_empty() {
        return Vec::new();
    }

    let mut sentences = Vec::new();
    let mut start_idx = 0usize;
    for i in 0..tokens.len() {
        if is_sentence_ender(&tokens, i) {
            // Include trailing closers: ")." '"!' etc.
            let mut end_idx = i;
            while end_idx + 1 < tokens.len() {
                let next = tokens[end_idx + 1].text.as_str();
                if matches!(next, "\"" | "'" | "”" | "’" | ")" | "]" | "}") {
                    end_idx += 1;
                } else {
                    break;
                }
            }
            let slice = &tokens[start_idx..=end_idx];
            if !slice.is_empty() {
                sentences.push(Sentence {
                    start: slice.first().map(|t| t.start).unwrap_or(0),
                    end: slice.last().map(|t| t.end).unwrap_or(0),
                    tokens: slice.to_vec(),
                });
            }
            start_idx = end_idx + 1;
        }
    }
    if start_idx < tokens.len() {
        let slice = &tokens[start_idx..];
        sentences.push(Sentence {
            start: slice.first().map(|t| t.start).unwrap_or(0),
            end: slice.last().map(|t| t.end).unwrap_or(0),
            tokens: slice.to_vec(),
        });
    }
    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_split_on_mr() {
        let sents = split_sentences("Mr. Smith went home. Then he slept.");
        assert_eq!(sents.len(), 2);
        assert!(sents[0].tokens.iter().any(|t| t.text == "Smith"));
    }

    #[test]
    fn splits_on_question() {
        let sents = split_sentences("Really? Yes.");
        assert_eq!(sents.len(), 2);
    }
}
