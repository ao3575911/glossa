# Security policy

## Supported versions

| Version | Supported |
| --- | --- |
| `0.1.x` (main) | Yes |
| Pre-release / forks | Best effort |

## Reporting a vulnerability

**Do not open a public issue for security vulnerabilities.**

1. Prefer [GitHub private vulnerability reporting](https://github.com/ao3575911/glossa/security/advisories/new) on this repository.
2. If that is unavailable, email the maintainer listed on the repository profile with a clear subject line: `Glossa security report`.

Include:

- Affected version or commit SHA
- Impact (data exposure, crash, supply-chain, etc.)
- Reproduction steps or a minimal proof of concept
- Whether you are willing to be credited

We aim to acknowledge reports within **7 days** and to publish a fix or mitigation timeline once impact is confirmed.

## Scope

In scope:

- Memory safety issues in Glossa crates that lead to undefined behaviour or sandbox escape
- Path traversal or unexpected filesystem writes outside the workspace
- Deserialization / parser issues on untrusted corpus files that enable RCE
- Compromised release artifacts or CI supply-chain weaknesses

Out of scope (unless they escalate):

- Denial of service via pathological corpora without a clear fix path
- Issues only present with intentionally unsafe plugins or user-granted capabilities
- Social engineering against individual users

## Supply chain and quality

- Dependencies are audited with **cargo-audit** and policy-checked with **cargo-deny** (see `deny.toml` and `.github/workflows/security.yml`).
- **CodeQL** (`security-and-quality`) runs on `main` and pull requests.
- Releases are produced via **cargo-dist**; verify checksums from GitHub Releases when installing binaries.
- Never commit secrets, API tokens, or private corpus data.

## Safe handling of corpora

Glossa is offline-first and may ingest untrusted text, HTML, DOCX, PDF, and CoNLL-U. Treat imported files as untrusted input. Prefer dedicated workspaces for unknown corpora and keep personal data out of bug reports.
