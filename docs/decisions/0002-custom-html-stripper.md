# 0002 — Minimal HTML stripper instead of a full HTML crate

## Context

Stage 1 needs HTML → plain text. Full parsers (`html5ever`, `scraper`) add
compile time and binary size for MVP import quality that only requires tag
stripping and basic entity decoding.

## Decision

Ship a small tag/script/style stripper in `glossa-io` with `&`-entity handling
and block-level newlines. Revisit if TEI/CHAT (Stage 3) needs a real HTML/XML
DOM.

## Consequences

Faster builds and a smaller dependency graph; imperfect handling of malformed
HTML and exotic entities is acceptable for MVP corpus import.
