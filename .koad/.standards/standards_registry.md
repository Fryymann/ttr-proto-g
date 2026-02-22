# Standards Registry Source

Status: Active
Last updated: 2026-02-21

## Core Standards

### STD-001 - Change Safety
- Intent: Prefer incremental, reversible changes and explicit risk surfacing.

### STD-002 - Documentation Continuity
- Intent: Keep durable session and decision records for multi-agent continuity.

### STD-003 - Source Freshness Gate
- Intent: Validate required standards sources before standards-dependent work.

### STD-004 - PM Artifact Synchronization
- Intent: Keep `.agents` planning artifacts aligned with decisions and scope changes.

### STD-005 - Sprint Execution Authorization Gate
- Intent: Keep PM sprint implementation blocked by default unless user explicitly authorizes sprint execution in-thread.

### STD-006 - Role Selection Boot Gate
- Intent: Require explicit role selection (`Koad (PM)|Gameplay|Platform|Experience`) before substantial work.

### STD-007 - Worktree-Isolated Team Lane Execution + PR Policy
- Intent: Run each coding lane in a dedicated worktree/branch and capture PR merge dependency metadata.
- Enforcement: Active release base branch is `v1`; lane branches and PR targets must use `v1` until PM declares otherwise.

### STD-008 - Team-Agent Familiarization Gate
- Intent: Require non-PM team-role agents to load roadmap + project state artifacts before implementation.

### STD-009 - Dual Review + Merge Completion Gate
- Intent: Require PR-based dual review (Koad git review + Ian GitHub review) and merged PR confirmation before task closure.
- Enforcement:
  - Use PR template and required `validate-pr-governance` status check under branch protection.
  - Governance validation requires review-gate lines to be present; checkbox states are informational audit metadata.
  - Merge authority remains GitHub review approvals plus required status checks.

### STD-010 - Koad-OS Branch Scope Separation
- Intent: Keep Koad/agent support and workflow-governance artifacts isolated from gameplay/platform feature delivery lanes.
- Enforcement:
  - Koad/agent support artifacts are committed through `koad-os` (including root `PROJECT_PROGRESS.md` dashboard).
  - Runtime/feature delivery lanes targeting `v1` (or replacement release branch) must not include Koad/agent support files unless the source branch is `koad-os` (support sync PR).
  - `koad-os` is auto-synced from `v1` via `.github/workflows/sync-koad-os-from-v1.yml` to reduce manual drift.
  - Scope-gate exception allows `v1` -> `koad-os` sync PRs when automatic sync hits merge conflicts.
  - Required status check `validate-koad-os-scope` enforces the boundary on PRs.

### STD-011 - Role-Aware Saveup Continuity
- Intent: Ensure continuity checkpoints remain attributable and usable across PM and team-role agents.
- Enforcement:
  - Every `saveup` entry must include `role` and `context_ref`.
  - Any saveup operation that writes tracked support artifacts (`.koad/**` ledgers/logs or `PROJECT_PROGRESS.md`) must execute on `koad-os`.
  - `saveup` operational mirror behavior must follow role boundaries (`Koad (PM)` may sync `.agents/*`; team roles log proposed PM deltas without reprioritizing directly).
  - `saveup` session summary must include role and context metadata.

### STD-012 - Project Progress Dashboard Continuity
- Intent: Keep a root-level, roadmap-aligned progress dashboard continuously available for fast status inspection.
- Enforcement:
  - Maintain `PROJECT_PROGRESS.md` in repo root.
  - Regenerate dashboard after backlog/sprint/queue/merge state changes using `.koad/scripts/koad progress-sync` (or default `saveup` sync path).
  - Dashboard content must be sourced from canonical planning artifacts (`.agents/backlog.md`, `docs/design/game-system-roadmap.md`, `docs/design/execution-sprint-plan.md`, `CODEX_ROLE_PROMPTS.md`).

### STD-013 - Lane-Isolated Saveup Journaling
- Intent: Prevent cross-lane merge conflicts from concurrent developer saveup activity.
- Enforcement:
  - Team-role saveups with lane context (`context_ref` starts with `lane/`) should write to lane journal files under `.koad/.agent-core/sessions/lane-saveups/`.
  - Lane journal files are local continuity artifacts and should be excluded from feature-lane PR scope.
  - Team-role lanes on non-`koad-os` branches should not use global-ledger saveup writes.
  - Shared global saveup ledgers (`SAVEUP_CALLS.md`, `LOG.md`) should be updated from PM/global mode or reconciliation on `koad-os`.
  - Lane-isolated saveup entries must still include `role` and `context_ref` metadata.
