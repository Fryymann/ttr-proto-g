# Standards Registry

Purpose: codable representation of standards for this workspace.

## How To Use
- Each standard has a stable ID.
- Source points to required local artifact.
- Local interpretation defines execution rules.

## Standards

### STD-001 - Change Safety
- Status: Active
- Source: `.koad/.standards/standards_registry.md`
- Intent: Avoid unsafe or hard-to-rollback changes.
- Local interpretation:
  - Prefer incremental edits with clear impact.
  - Surface risky assumptions before broad refactors.

### STD-002 - Documentation Minimum
- Status: Active
- Source: `.koad/.standards/standards_registry.md`
- Intent: Maintain continuity.
- Local interpretation:
  - Update `.koad/.agent-ops/sessions/SESSION_LOG.md` for substantial tasks.
  - Update `.koad/.agent-ops/decisions/DECISION_LOG.md` for durable decisions.

### STD-003 - Canonical Source Freshness Gate
- Status: Active
- Source: `.koad/.standards/standards_registry.md`
- Intent: Verify standards source freshness before standards-dependent work.
- Local interpretation:
  - Run `python3 .koad/.agent-core/scripts/standards_sync_status.py --manifest .koad/.standards/sync_manifest.json --required-sources .koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md --max-age-hours 24`.
  - If stale or missing required sources, stop standards-dependent implementation and log blocker details.

### STD-004 - PM Artifact Sync
- Status: Active
- Source: `.agents/roles/project-manager.md`
- Intent: Keep PM system artifacts aligned after substantial planning or scope changes.
- Local interpretation:
  - Update `.agents/backlog.md` when new work is identified or reprioritized.
  - Update `.agents/risk-register.md` when new risks or mitigations are identified.
  - Update `.agents/teams.md` when ownership boundaries change.

### STD-005 - Sprint Execution Authorization Gate
- Status: Active
- Source: `.koad/AGENTS.md`
- Intent: Prevent PM agent from self-initiating sprint execution work intended for development lanes.
- Local interpretation:
  - Treat sprint implementation/execution as blocked by default.
  - Only run sprint implementation when the user explicitly requests sprint execution in the current thread.
  - Without explicit user authorization, limit sprint activity to planning, delegation packets, review, and coordination artifacts.

### STD-006 - Role Selection Boot Gate
- Status: Active
- Source: `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`
- Intent: Ensure every Codex instance explicitly resolves its role before substantial work.
- Local interpretation:
  - Ask role selection question (`Koad (PM)|Gameplay|Platform|Experience`) before substantial work.
  - Route `Koad|PM` variants to Koad PM posture.
  - Route other selected roles to team-role posture and load role-specific context.
  - Do not begin substantial work with unresolved or ambiguous role selection.

### STD-007 - Worktree-Isolated Team Lane Execution + PR Policy
- Status: Active
- Source: `CODEX_ROLE_PROMPTS.md`
- Intent: Prevent cross-lane interference and reduce merge risk while preserving parallel throughput across Codex team instances.
- Local interpretation:
  - Active release base branch is `v1`; coder lanes branch from `v1` and target PRs to `v1` unless PM declares a replacement base branch.
  - Require one coding lane per dedicated git worktree and lane branch (`lane/<ROLE>/<task-slug>`).
  - Prohibit team-role development edits on `v1` and `koad-os`; treat those branches as integration/support-only contexts.
  - If a team-role instance boots on `v1`/`koad-os` or any non-lane branch, stop and relocate to the assigned lane worktree before editing files.
  - Require onboarding evidence to include worktree path, branch, and base commit before edits.
  - Require lane handoff to include PR metadata (URL, base/head, title, latest commit, merge dependency order).
  - Default to one PR per lane; allow docs/chore batching and post-lane integration PR only when scope risk is low.

### STD-008 - Team-Agent Familiarization Gate
- Status: Active
- Source: `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`
- Intent: Ensure non-PM team-role instances align with current roadmap and project state before implementation.
- Local interpretation:
  - For Gameplay/Platform/Experience roles, load `.agents/teams.md`, `docs/design/game-system-roadmap.md`, `docs/design/execution-sprint-plan.md`, `.agents/backlog.md`, and `.agents/risk-register.md` before substantial work.
  - Require role declaration and planned scope statement before edits.

### STD-009 - Dual Review + Merge Completion Gate
- Status: Active
- Source: `CODEX_ROLE_PROMPTS.md`
- Intent: Ensure every lane is reviewed and merged through PR workflow before being treated as complete.
- Local interpretation:
  - Require one PR for every execution lane (code/docs/chore), with clear base/head and latest commit metadata.
  - Require `.github/pull_request_template.md` usage for lane PRs.
  - Require `validate-pr-governance`, `validate-koad-os-scope`, `validate-implementation-doc`, and `validate-test-evidence` checks to pass before merge.
  - Require PR review-gate lines to exist in PR body; checkbox states are informational audit metadata, not merge authority.
  - Treat GitHub approval + required checks as authoritative merge gates.
  - Require Koad git-side review disposition and Ian review disposition before approval.
  - If either review requests changes, keep lane state `in_progress`.
  - Mark lane/backlog task complete only after PR merge confirmation.

### STD-010 - Koad-OS Branch Scope Separation
- Status: Active
- Source: `CODEX_ROLE_PROMPTS.md`
- Intent: Keep Koad/agent support and workflow-governance artifacts isolated from development-lane feature code.
- Local interpretation:
  - Treat `koad-os` as support-only branch scope for `.koad/**`, `.agents/**`, `.codex/**`, `AGENTS.md`, `CODEX_ROLE_PROMPTS.md`, root `PROJECT_PROGRESS.md`, PR-governance workflows/templates, and PM workflow runbooks.
  - Keep `koad-os` synchronized from `v1` via `.github/workflows/sync-koad-os-from-v1.yml` to reduce manual branch maintenance.
  - Use `.github/workflows/promote-koad-os-to-v1.yml` to auto-create/update managed support-promotion PRs from `koad-os` to `v1` when effective deltas exist.
  - Allow `v1` -> `koad-os` sync PRs as explicit scope-gate exception when automatic sync encounters merge conflicts.
  - For PRs targeting `koad-os`, reject out-of-scope runtime/feature files.
  - For PRs sourced from `koad-os` (support sync PRs), allow only in-scope Koad/agent support files.
  - For other PRs targeting non-`koad-os` branches, reject in-scope Koad/agent support files and route them to `koad-os`.
  - Require `validate-koad-os-scope` status check before merge.

### STD-011 - Role-Aware Saveup Continuity
- Status: Active
- Source: `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`
- Intent: Ensure saveup continuity artifacts remain attributable and role-correct across PM and team-role instances.
- Local interpretation:
  - Require every saveup call row to include `role` (`Koad (PM)|Gameplay|Platform|Experience`) and `context_ref`.
  - Require saveup session summaries to include role + context metadata.
  - Any saveup mode that writes tracked support artifacts (`.koad/**` saveup ledgers/logs or `PROJECT_PROGRESS.md`) must run on `koad-os`.
  - For `Koad (PM)` saveup scope, allow `.agents/*` mirrors when relevant.
  - For non-PM saveup scope, do not reprioritize PM artifacts directly; log proposed PM deltas in ops logs for PM review.

### STD-012 - Project Progress Dashboard Continuity
- Status: Active
- Source: `.koad/scripts/koad_cli.py`
- Intent: Keep one easy-to-read progress dashboard aligned with roadmap + backlog state, refreshed after release-line merges without per-change commit overhead.
- Local interpretation:
  - Use `.github/workflows/update-v1-dashboard.yml` on pushes to `v1` to regenerate/publish the managed `V1 Project Dashboard` issue.
  - Keep `PROJECT_PROGRESS.md` as generated local artifact (`koad progress-sync`) for ad hoc terminal/reference usage.
  - `koad saveup` does not refresh `PROJECT_PROGRESS.md` by default; use explicit sync flags when needed.

### STD-013 - Lane-Isolated Saveup Journaling
- Status: Active
- Source: `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`
- Intent: Reduce merge conflicts caused by concurrent saveup writes from multiple developer lanes.
- Local interpretation:
  - For team-role saveups on lane contexts (`context_ref` starts with `lane/`), default to lane-isolated journaling under `.koad/.agent-core/sessions/lane-saveups/`.
  - Treat lane journals as local continuity artifacts; do not include them in feature-lane PR scope.
  - Team-role lanes on non-`koad-os` branches should not force global-ledger saveup writes.
  - Avoid writing shared global saveup ledgers (`SAVEUP_CALLS.md`, `LOG.md`) from feature-lane saveup calls unless explicitly forced.
  - Keep role/context metadata in lane journals and reconcile durable PM/global entries on `koad-os`.

### STD-014 - Developer Documentation + Test Evidence Gate
- Status: Active
- Source: `.github/workflows/implementation-doc-gate.yml`, `.github/workflows/test-evidence-gate.yml`
- Intent: Enforce high-quality implementation documentation and concrete automated test/coverage evidence for non-`koad-os` source-code PRs.
- Local interpretation:
  - For non-`koad-os` PRs with source changes, require an updated `docs/implementation/*.md` document with required headings and file-level mapping.
  - Require changed-line coverage evidence at or above 80%.
  - Require explicit automated test command/results plus negative-path and regression evidence in implementation documentation.
  - Reject source-code PRs that do not show test updates or detectable in-file test deltas.
