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
  - Require `validate-pr-governance` status check to pass before merge.
  - Require Koad git-side review disposition and user GitHub review disposition before approval.
  - If either review requests changes, keep lane state `in_progress`.
  - Mark lane/backlog task complete only after PR merge confirmation.
