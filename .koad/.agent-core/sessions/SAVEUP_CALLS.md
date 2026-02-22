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
| SAVEUP-20260222-020839Z | Koad (PM) | S3-G1 | post-merge status + S3-P1 prep | partial | 1 | 0 | S3-G1 merged and S3-P1 lane prepared; koad-os status docs committed locally but push blocked by intermittent github DNS resolution. |
| SAVEUP-20260222-024209Z | Experience | S2-E1/pr-11 | s2-e1-cli-scene-render | completed | 1 | 0 | S2-E1 delivered; PR #11 opened; sandbox blocked socket-based manual multiplayer run. |
| SAVEUP-20260222-042201Z | Koad (PM) | koad-os | post-review-flow-main-savepoint | completed | 0 | 0 | main savepoint after PR #18/#19 merge and branch/worktree cleanup |
| SAVEUP-20260222-044326Z | Koad (PM) | koad-os | post-pr20-automation-baseline | completed | 0 | 1 | savepoint after PR #20 merge; automation baseline active |
| SAVEUP-20260222-071006Z | Koad (PM) | koad-os | post-pr28-merge-automation-offload | completed | 0 | 3 | automation-offload checkpoint complete after PR #27/#28 merges; no open blockers |
| SAVEUP-20260222-091019Z | Koad (PM) | koad-os | post-pr36-ian-intake-scope-split | partial | 1 | 1 | PR #37 is Koad-approved; dedicated v1 docs lane branch is prepared but push is blocked by intermittent DNS resolution to github.com. |
| SAVEUP-20260222-225121Z | Koad (PM) | gemini | koad-os-v2-upgrade | completed | 0 | 0 | KoadOS v2 core automation and memory improvements completed and committed to gemini branch. |
| SAVEUP-20260222-230026Z | Koad (PM) | gemini | pm-workflow-automation | completed | 0 | 0 | Implemented dispatch and task-complete commands for dynamic prompt generation and multi-artifact synchronization. |
