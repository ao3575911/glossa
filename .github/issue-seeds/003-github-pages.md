---
title: "docs: publish GitHub Pages from docs/pages"
labels: ["type:docs", "area:docs", "area:ci", "status:accepted", "priority:medium", "size:S"]
---

## Goal

Turn on GitHub Pages (Actions source) so the **Pages** workflow deploys `docs/pages` and mirrored wiki content.

## Acceptance

- [ ] Pages source set to GitHub Actions
- [ ] `Pages` workflow deploys from `main`
- [ ] Site index loads with product one-liner and links to wiki/docs

## References

- `.github/workflows/pages.yml`
- `docs/pages/index.html`
