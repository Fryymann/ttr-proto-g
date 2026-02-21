# Startup Checklist

1. Read:
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

4. PM context load:
- `.agents/teams.md`
- `.agents/backlog.md`
- `.agents/risk-register.md`
- `.agents/roles/project-manager.md`

5. Before substantial work:
- State applicable standards IDs.
- State risk level (`Low|Medium|High`).
