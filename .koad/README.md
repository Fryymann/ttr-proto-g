# Standalone Agent Memory + Saveup Kit

This kit is self-contained and does not depend on any external memory framework.

## Use
1. Copy this entire folder into the root of a new workspace.
2. Keep the internal structure unchanged.
3. Ensure your agent runtime reads `.koad/AGENTS.md` at startup.
4. Run:
   - `python3 .koad/.agent-core/scripts/standards_sync_status.py --manifest .koad/.standards/sync_manifest.json --required-sources .koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md --max-age-hours 24`
5. Run `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md` and resolve role (`Koad (PM)|Gameplay|Platform|Experience`) before substantial work.
6. Use `saveup` to persist session continuity.

## Purpose
- Persistent agent memory
- Duplicate-aware, role-aware session checkpointing (`saveup`)
- Standards freshness gating before standards-dependent work
- Sprint-execution authorization gating (execution only when explicitly requested by user)
- Role-routed multi-instance Codex workflow (`Koad PM` + team-role agents)
- Worktree-isolated coder-lane execution policy with lightweight PR governance

## Utility Scripts

Use `.koad/scripts/koad` for common workflow actions:

- `lane-start`: create lane branch + worktree and print onboarding evidence.
- `pr-open`: open a governance-template PR with lane metadata.
- `saveup`: append role-aware saveup records (lane-isolated journaling for team-role lane contexts by default).
- `progress-sync`: generate a local `PROJECT_PROGRESS.md` snapshot from current backlog/roadmap/sprint sources.
- `status`: print compact current progress summary in terminal (`--refresh` to regenerate first).

`v1` merges also refresh the managed `V1 Project Dashboard` issue automatically via `.github/workflows/update-v1-dashboard.yml`.
