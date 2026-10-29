---
title: "chore: adopt canonical label taxonomy"
labels: ["type:chore", "area:ci", "status:accepted", "priority:high", "size:S"]
---

## Goal

Confirm that `.github/labels.yml` is synced on `main` and that issue/PR templates apply the new `type:*`, `area:*`, `priority:*`, and `status:*` labels.

## Acceptance

- [ ] Labels workflow completed successfully on `main`
- [ ] Default GitHub labels that conflict were replaced or retired
- [ ] New issues from templates receive triage labels automatically

## Notes

Run **Actions → Labels → Run workflow** if labels are missing after merge.
