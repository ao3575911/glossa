# Projects

Recommended GitHub Project (v2) for Glossa delivery tracking.

## Board: Glossa delivery

| Column | Meaning | Suggested automation |
| --- | --- | --- |
| **Triage** | New issues (`status:triage`) | Auto-add issues on open |
| **Accepted** | Scheduled work (`status:accepted`) | Move when label applied |
| **In progress** | Active (`status:in-progress`) | Move when assignee set / label applied |
| **Review** | Open pull requests | Auto-add PRs; require CI |
| **Done** | Closed / merged | Auto-archive on close |

## Views

1. **Board** — columns above.
2. **Table** — fields: Title, `type`, `area`, `priority`, `size`, Assignees, Linked PRs.
3. **Milestone roadmap** — group by GitHub Milestone (Stage 0, Stage 1, …).

## Label conventions

Planning labels (`priority:*`, `status:*`, `size:*`) are defined in
[`.github/labels.yml`](../.github/labels.yml). Prefer labels over free-text project
fields so Issues, Discussions, and the board stay aligned.

## Setup checklist

1. Create a Project named **Glossa delivery** and link this repository.
2. Add the columns in the table above.
3. Optional: workflow “Item added to project” → set Status = Triage.
4. After merge, run **Seed labelled issues** (workflow_dispatch, dry-run off) so
   starter chores appear on the board.
5. Track enablement of Discussions and Pages via the seeded issues.

## Related

- [`.github/settings.yml`](../.github/settings.yml) — desired repository settings
- [`SUPPORT.md`](../SUPPORT.md) — how humans get help
- [`AGENTS.md`](../AGENTS.md) — how agents should work
