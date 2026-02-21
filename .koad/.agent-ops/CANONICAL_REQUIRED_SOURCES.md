# Canonical Required Sources

List required local standards sources for this workspace.

## Required by Scope

| Scope | Required local source | Notes |
|---|---|---|
| Global coding standards | `.koad/.standards/standards_registry.md` | Base standards source |
| Project profile | `.koad/.standards/project_profile.md` | Project constraints and conventions |
| Service features (if backend work) | `.koad/.standards/features_registry.json` | Service ownership/runtime/deploy policy |
| PM team model | `.agents/teams.md` | Ownership boundaries and handoff contracts |
| PM backlog | `.agents/backlog.md` | Prioritized work queue |
| PM risk register | `.agents/risk-register.md` | Active risk tracking and mitigations |

## Freshness Gate
- Run before standards-dependent work:
  - `python3 .koad/.agent-core/scripts/standards_sync_status.py --manifest .koad/.standards/sync_manifest.json --required-sources .koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md --max-age-hours 24`

## Blocker Contract
- If any required local source is missing, stop standards-dependent work.
- Log timestamp + missing path + error.
