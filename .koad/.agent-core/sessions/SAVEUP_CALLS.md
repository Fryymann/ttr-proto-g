# SAVEUP Calls Ledger

Tracks every explicit `saveup` invocation for auditability and duplicate-control.

Note: this ledger is for global/PM saveup mode. Team-role lane-isolated saveups are recorded in local gitignored journals under `.koad/.agent-core/sessions/lane-saveups/`.

## Fields
- `call_id`: `SAVEUP-YYYYMMDD-HHMMSSZ`
- `role`: `Koad (PM)` | `Gameplay` | `Platform` | `Experience`
- `context_ref`: task packet id, lane branch, or `n/a`
- `scope`: short label of session scope
- `result`: `completed` | `partial` | `blocked`
- `new_learnings`: count of net-new learning entries
- `duplicates_skipped`: count of skipped duplicate candidates
- `notes`: blockers/risks

## Entries
| call_id | role | context_ref | scope | result | new_learnings | duplicates_skipped | notes |
|---|---|---|---|---|---:|---:|---|
| SAVEUP-20260221-043542Z | Koad (PM) | n/a | koad-pm-os-adaptation | completed | 2 | 0 | continuity checkpoint completed; unresolved gameplay defaults remain |
| SAVEUP-20260221-073905Z | Koad (PM) | n/a | worktree-policy-rollout | completed | 1 | 0 | continuity checkpoint completed; S2 queue/runtime live wiring remains deferred until post-S2-E1 |
| SAVEUP-20260221-213545Z | Koad (PM) | koad-os | koad-os-governance-sync | completed | 4 | 0 | continuity checkpoint completed; PR #3 open with passing governance checks; apply branch protection runbook in GitHub UI |
| SAVEUP-20260222-002034Z | Koad (PM) | S2-E1 | post-merge docs+saveup-prep | completed | 2 | 0 | Synced post-PR#11 roadmap docs/status and shipped lane-isolated saveup journaling defaults to reduce cross-lane merge conflicts. |
