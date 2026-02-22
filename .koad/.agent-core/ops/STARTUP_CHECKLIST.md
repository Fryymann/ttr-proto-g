# Startup Checklist

1. Read core context:
- `.koad/.agent-core/IDENTITY.md`
- `.koad/.agent-core/MISSION.md`
- `.koad/.agent-core/memory/WORKING_MEMORY.md`
- `.koad/.agent-core/memory/LEARNINGS.md`
- `.koad/.agent-core/memory/USER_PREFERENCES.md`
- `.koad/.agent-core/sessions/SAVEUP_CALLS.md` (latest entries)

2. Verify standards freshness:
- `python3 .koad/.agent-core/scripts/standards_sync_status.py --manifest .koad/.standards/sync_manifest.json --required-sources .koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md --max-age-hours 24`

3. Read standards inputs:
- `.koad/.agent-ops/STANDARDS_REGISTRY.md`
- `.koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md`

4. Run role routing (required before substantial work):
- `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`
- Ask user: `Which role should I personify in this thread: Koad (PM), Gameplay, Platform, or Experience?`

5. Role context load:
- All roles:
  - `.agents/teams.md`
  - `docs/design/game-system-roadmap.md`
  - `docs/design/execution-sprint-plan.md`
  - `.agents/backlog.md`
  - `.agents/risk-register.md`
- Koad/PM route:
  - `.agents/roles/project-manager.md`
- Team-role routes:
  - `.agents/roles/gameplay-lead.md`
  - `.agents/roles/platform-lead.md`
  - `.agents/roles/experience-lead.md`

6. Before substantial work:
- State applicable standards IDs.
- State risk level (`Low|Medium|High`).
- State selected role and planned scope.
- Carry selected role label into any `saveup` call metadata (`role` + `context_ref` in `SAVEUP_CALLS.md`).
- For team-role lane contexts (`context_ref` starts with `lane/`), use lane-isolated saveup journaling (`.koad/.agent-core/sessions/lane-saveups/`) unless global ledger mode is explicitly required.
- For team-role execution, verify branch/worktree isolation before edits: active branch must match `lane/<ROLE>/<task-slug>` in a dedicated worktree; if on `v1` or `koad-os`, stop and relocate before file changes.
- If acting as Koad/PM and work resembles sprint implementation, verify explicit user authorization first; otherwise remain in planning/delegation mode.
- For parallel team-role execution, require one lane per dedicated worktree/branch and a defined PR strategy before launch.
