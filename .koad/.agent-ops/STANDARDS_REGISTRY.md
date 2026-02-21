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
