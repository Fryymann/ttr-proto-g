# Working Memory

Last updated: 2026-02-22

## Active Context
- Project is a Rust multiplayer terminal RPG moving from M1 scaffold to scene-based combat architecture.
- PM operations now use `.agents/` artifacts (teams/backlog/risk) as source of planning truth.
- User uses `IAN.md` for raw brainstorm entries and asks for integration reviews.
- Koad OS is adapted for PM workflow with `.koad`-scoped freshness gating and artifact sync rules.
- `docs/design/game-system-roadmap.md` is now the primary living focus reference for sequencing and system design order.
- `docs/design/execution-sprint-plan.md` is the primary near-term execution sequencing reference for M2-M3.
- Active execution model is Codex multi-instance: Koad PM lead + Gameplay/Platform/Experience role agents.
- Antigravity execution is paused; `ANTIGRAVITY_SPRINT_PROMPTS.md` is archival unless explicitly re-enabled.
- Codex team-role workflow defaults to one lane per dedicated git worktree/branch with lightweight PR policy.
- Startup now requires role selection routing via `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.
- Codex agents execute inside WSL2 (Ubuntu), so lane prompt workspace paths should use `/mnt/c/...` instead of `C:\...`.
- `koad-os` branch sync from `v1` is now automated via `.github/workflows/sync-koad-os-from-v1.yml` to reduce manual maintenance.
- `koad-os` -> `v1` promotion PR lifecycle is now automated via `.github/workflows/promote-koad-os-to-v1.yml`.
- Promotion workflow now seeds required PR checks for auto-created promotion PRs to avoid missing-check deadlocks.
- PM PR gate review preflight is now scriptable via `.koad/scripts/koad pr-gate` (including `--watch` check-settle mode and optional Koad checkbox apply).
- PR merge-finalization is now scriptable via `.koad/scripts/koad pr-finish` (gate validation + strategy auto-selection + merge).
- V1 status dashboard publishing is merge-driven via `.github/workflows/update-v1-dashboard.yml` (managed issue `V1 Project Dashboard`) to avoid per-change status commit overhead.

## Open Unknowns
- Default `defensive_rounds_before_ai` for disconnect fallback.
- AI action whitelist after defensive rounds.
- Party-membership edge rules for encounter entry (temporary allies, summons, cross-party assist actions).

## Risk Watch
- Single-save campaign data safety and rollback behavior.
- Campaign-lock admin unlock guardrails and audit requirements.
- SRD-only compliance drift in V1 content implementation.
- Advisory DM-agent boundary leakage into authoritative NPC actions.
