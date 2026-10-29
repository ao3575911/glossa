---
title: "security: verify private vulnerability reporting"
labels: ["type:security", "area:security", "status:accepted", "priority:high", "size:XS"]
---

## Goal

Enable private vulnerability reporting and confirm `SECURITY.md` matches the live advisory flow.

## Acceptance

- [ ] Private vulnerability reporting enabled
- [ ] Security policy visible on the repository Security tab
- [ ] `cargo-deny`, `cargo-audit`, and CodeQL jobs green on `main`

## References

- `SECURITY.md`
- `.github/workflows/security.yml`
- `deny.toml`
