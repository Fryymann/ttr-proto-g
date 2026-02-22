# Patterns

Track recurring workflows and known effective approaches.

## Pattern: IAN Idea Integration
- Trigger:
  - User says `review latest IAN entry`.
- Steps:
  - Read newest section in `IAN.md`.
  - Extract decisions, implementation impacts, open questions.
  - Update `.agents/backlog.md` + `.agents/risk-register.md` on `koad-os`.
  - If design/spec docs are required under `docs/design/**`, ship them via a dedicated `v1` docs lane PR (not `koad-os` promotion).
  - Return concise summary with unresolved choices.
- Validation:
  - Changes present in backlog/risk continuity artifacts and the appropriate design-doc lane PR when spec updates are needed.

## Pattern: PM Koad Sync Pass
- Trigger:
  - Significant planning/operational updates or OS-configuration edits.
- Steps:
  - Run freshness gate.
  - Confirm required-source paths resolve.
  - Mirror durable updates to `.agents` artifacts and `.koad/.agent-ops` logs.
  - Capture final session summary in `.koad/.agent-core/sessions/LOG.md`.
- Validation:
  - `status: FRESH` from standards sync check and aligned entries in both planning + ops logs.

## Pattern: Codex Team Sprint Packeting
- Trigger:
  - New sprint starts or sprint scope changes.
- Steps:
  - Update `docs/design/execution-sprint-plan.md` with role-lane map and validation gates.
  - Publish/update active prompts in `CODEX_ROLE_PROMPTS.md`.
  - Keep `ANTIGRAVITY_SPRINT_PROMPTS.md` marked paused unless user explicitly re-enables Antigravity.
  - Ensure `.agents/backlog.md` active focus matches sprint lane priorities.
  - Log process changes in Koad session/decision logs.
- Validation:
  - Codex team-role agents can be launched directly with clear onboarding/handoff requirements.

## Pattern: Lane-Isolated Worktree Launch
- Trigger:
  - User asks for parallel coder-agent execution or lane kickoff.
- Steps:
  - Define PM integration base ref for the sprint.
  - Create one worktree/branch per lane using `lane/<ROLE>/<task-slug>`.
  - Assign one agent to each worktree and require onboarding evidence (`pwd`, toplevel, branch, commit, status).
  - Require handoff PR metadata (branch, title, dependency order) before lane closure.
- Validation:
  - No active coder lanes share a worktree.
  - Each lane handoff includes branch/worktree evidence plus PR dependency notes.

## Pattern: Role-Routed Codex Bootstrap
- Trigger:
  - New Codex thread starts in repository.
- Steps:
  - Run `.koad` bootstrap and standards freshness gate.
  - Ask role question from `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.
  - Route to `Koad (PM)` or team role and load matching project context.
  - Declare selected role, standards, risk level, and scope before substantial work.
- Validation:
  - Substantial work never starts with unresolved role.

## Pattern: Koad-OS Support Sync
- Trigger:
  - Koad/agent-support artifacts are updated on `koad-os` and need to flow into active release line.
- Steps:
  - Commit support-scope updates on `koad-os` (manual or agent-authored).
  - Open PR from `koad-os` to release line (`v1` while active).
  - Use PR template with `Persona signature`, and ensure both governance checks pass.
  - Merge PR after review to keep release line current with support workflow updates.
- Validation:
  - PR head is `koad-os`, files are support-scope only, and `validate-pr-governance` + `validate-koad-os-scope` are green.

## Pattern: Lane-Isolated Saveup
- Trigger:
  - Team-role lane agent needs a continuity checkpoint while feature lane is active.
- Steps:
  - Run `.koad/scripts/koad saveup` with lane `context_ref` (e.g., `lane/Platform/...`).
  - Let default lane-isolated mode write to `.koad/.agent-core/sessions/lane-saveups/<context-ref>.md`.
  - Avoid shared global saveup ledgers in feature lanes unless explicitly required.
  - Reconcile PM/global saveup summaries on `koad-os` during support sync.
- Validation:
  - Feature-lane PRs avoid touching shared `SAVEUP_CALLS.md`/`LOG.md` while still preserving role/context continuity records.

## Pattern: Post-Edit Worktree Verification
- Trigger:
  - After applying patches or scripted edits while multiple worktrees are active.
- Steps:
  - Run `git status --short --branch` in the intended worktree.
  - Run `git status --short --branch` in `/mnt/c/data/ttrpg` primary workspace.
  - If support/doc edits appear on `v1`, copy updated files into `koad-os` worktree and restore `v1` immediately.
- Validation:
  - Intended worktree shows expected edits; primary workspace remains free of accidental support/doc changes.
