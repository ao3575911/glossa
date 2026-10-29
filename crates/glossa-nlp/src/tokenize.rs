use crate::abbrev::is_abbreviation;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

/// A token with byte offsets into the (NFC-normalised) source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub start: usize,
    pub end: usize,
    /// True for alphabetic/numeric word-like tokens (UAX #29 word).
    pub is_word: bool,
}

/// NFC-normalise input text.
pub fn normalize_nfc(text: &str) -> String {
    text.nfc().collect()
}

/// Tokenise with UAX #29 word boundaries. Whitespace-only segments are dropped.
/// Punctuation tokens are retained (needed for KWIC context fidelity).
pub fn tokenize(text: &str) -> Vec<Token> {
    let nfc = normalize_nfc(text);
    let mut out = Vec::new();
    for (start, word) in nfc.split_word_bound_indices() {
        if word.chars().all(char::is_whitespace) {
            continue;
        }
        let end = start + word.len();
        let is_word = word.chars().any(|c| c.is_alphanumeric());
        out.push(Token {
            text: word.to_string(),
            start,
            end,
            is_word,
        });
    }
    out
}

/// Returns true if `token` looks like a sentence-ending punctuation mark,
/// after abbreviation filtering for periods.
pub fn is_sentence_ender(tokens: &[Token], idx: usize) -> bool {
    let Some(tok) = tokens.get(idx) else {
        return false;
    };
    let t = tok.text.as_str();
    if t == "!" || t == "?" || t == "…" {
        return true;
    }
    if t == "." || t.ends_with('.') {
        // Look back for abbreviation.
        if let Some(prev) = tokens[..idx].iter().rev().find(|t| t.is_word) {
            if is_abbreviation(&prev.text) {
                return false;
            }
            // Initials like "A." — single letter + period handled as separate tokens.
            if prev.text.chars().count() == 1 && prev.text.chars().all(|c| c.is_ascii_alphabetic())
            {
                return false;
            }
        }
        if t == "." {
            return true;
        }
        // Token itself is "Mr." style
        if is_abbreviation(t) {
            return false;
        }
        return t.ends_with('.');
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn simple_sentence() {
        let toks = tokenize("Hello, world!");
        let words: Vec<_> = toks.iter().map(|t| t.text.as_str()).collect();
        assert_eq!(words, ["Hello", ",", "world", "!"]);
    }

    #[test]
    fn nfc_combining() {
        // "é" as e + combining acute vs precomposed
        let a = tokenize("e\u{0301}");
        let b = tokenize("é");
        assert_eq!(a[0].text, b[0].text);
    }

    proptest! {
        #[test]
        fn tokenize_roundtrip_offsets(s in "\\PC{0,200}") {
            let nfc = normalize_nfc(&s);
            let toks = tokenize(&s);
            for t in &toks {
                assert!(t.start <= t.end);
                assert!(t.end <= nfc.len());
                assert_eq!(&nfc[t.start..t.end], t.text);
            }
        }

        #[test]
        fn deterministic(s in "\\PC{0,100}") {
            assert_eq!(tokenize(&s), tokenize(&s));
        }
    }
}
