# SAVEUP Calls Ledger

Tracks every explicit `saveup` invocation for auditability and duplicate-control.

## Fields
- `call_id`: `SAVEUP-YYYYMMDD-HHMMSSZ`
- `scope`: short label of session scope
- `result`: `completed` | `partial` | `blocked`
- `new_learnings`: count of net-new learning entries
- `duplicates_skipped`: count of skipped duplicate candidates
- `notes`: blockers/risks

## Entries
| call_id | scope | result | new_learnings | duplicates_skipped | notes |
|---|---|---|---:|---:|---|
| SAVEUP-20260221-043542Z | koad-pm-os-adaptation | completed | 2 | 0 | continuity checkpoint completed; unresolved gameplay defaults remain |
| SAVEUP-20260221-073905Z | worktree-policy-rollout | completed | 1 | 0 | continuity checkpoint completed; S2 queue/runtime live wiring remains deferred until post-S2-E1 |
