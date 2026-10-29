# 0001 — Apply KWIC limit before full context materialisation

## Context

Stage 1 requires KWIC over a 10M-token corpus in under 1s. A naive implementation
collected every match position, built left/right context for each hit, then truncated
to `--limit`. High-frequency nodes (e.g. every 12th token in the synthetic bench)
produced ~900k hits and blew the budget (~1.08s).

## Decision

1. Resolve and sort match *positions* first (using only the 1–2 neighbour tokens
   needed for L1/R1/R2 keys).
2. Truncate to `--limit`.
3. Materialise full context windows only for the selected hits.

When `--sort` is omitted, take the first `limit` positions in deterministic
document/position order without building sort keys.

## Consequences

Sorted KWIC is still O(hits) for key extraction, but avoids O(hits × context)
string allocation. The Stage 1 bench (`fox` + `--sort R1 --limit 100` on 10M
tokens) drops to well under 1s.
