# Agent Bootstrap

Any coding agent instance started in this workspace must initialize context before substantial work.

## Required Startup Order
1. Read `.koad/.agent-core/ops/STARTUP_CHECKLIST.md`.
2. Read `.koad/.agent-core/IDENTITY.md`.
3. Read `.koad/.agent-core/memory/WORKING_MEMORY.md`.
4. Read `.koad/.agent-core/memory/LEARNINGS.md` and `.koad/.agent-core/memory/USER_PREFERENCES.md`.
5. Run `python3 .koad/.agent-core/scripts/standards_sync_status.py --manifest .koad/.standards/sync_manifest.json --required-sources .koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md --max-age-hours 24`.
6. Read `.koad/.agent-ops/STANDARDS_REGISTRY.md`.
7. Read `.koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md`.
8. Run role routing in `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md` before substantial work.

## Operational Rules
- The persona in `.koad/.agent-core/IDENTITY.md` is the active execution posture.
- Use `.koad/.agent-core` for identity and memory.
- Use `.koad/.agent-ops` for operational standards and logs.
- Never store secrets in `.koad/.agent-core` or `.koad/.agent-ops`.
- Sprint execution guardrail: do not perform sprint implementation work unless the user explicitly asks for sprint execution in the current thread.
- Antigravity execution is paused unless the user explicitly re-enables it.
- For Codex team-role lanes, enforce onboarding evidence before edits and acceptance-evidence handoff before lane closure.
- For parallel team-role lanes, enforce one lane per dedicated git worktree/branch and capture PR metadata in handoffs.
- Enforce PR completion gate: Koad git review + Ian review + merged PR before marking lane task complete.
- Enforce branch-scope separation: Koad/agent support artifacts (`.koad/**`, `.agents/**`, `AGENTS.md`, `CODEX_ROLE_PROMPTS.md`, `PROJECT_PROGRESS.md`, PR-governance workflows/templates, and PM workflow runbooks) are committed via `koad-os`, then promoted through `koad-os` -> release-line PRs; feature/runtime development changes stay on release-line lane branches.
- Keep `koad-os` synchronized from `v1` via `.github/workflows/sync-koad-os-from-v1.yml` to reduce manual sync overhead.

## Saveup Rule
- When user says `saveup`, execute `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`.
- For team-role lane contexts (`context_ref` starts with `lane/`), prefer lane-isolated saveup journals in `.koad/.agent-core/sessions/lane-saveups/` to avoid merge conflicts.
- Treat lane saveup journals as local continuity artifacts; do not include them in feature-lane PRs.
- Any saveup execution that writes tracked `.koad/**` artifacts or `PROJECT_PROGRESS.md` must be committed on `koad-os` (not feature lane branches).
- Use `.koad/.agent-core/sessions/SAVEUP_CALLS.md` + `.koad/.agent-core/sessions/LOG.md` for global/PM saveup entries and reconciled summaries.
- Refresh root `PROJECT_PROGRESS.md` during `saveup` (default behavior in `.koad/scripts/koad saveup`) or via explicit `progress-sync`.

## Scope Rule
- This workspace may contain multiple projects.
- Confirm target repo/path before destructive or deployment actions.
