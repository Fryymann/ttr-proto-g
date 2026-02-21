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
- Intent: Require PR-based dual review (Koad git review + user GitHub review) and merged PR confirmation before task closure.
- Enforcement: Use PR template and required `validate-pr-governance` status check under branch protection.

### STD-010 - Koad-OS Branch Scope Separation
- Intent: Keep Koad/agent support and workflow-governance artifacts isolated from gameplay/platform feature delivery lanes.
- Enforcement:
  - Koad/agent support artifacts are committed through `koad-os`.
  - Runtime/feature delivery lanes targeting `v1` (or replacement release branch) must not include Koad/agent support files.
  - Required status check `validate-koad-os-scope` enforces the boundary on PRs.
