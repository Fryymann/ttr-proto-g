# Working Memory

Last updated: 2026-02-21

## Active Context
- Project is a Rust multiplayer terminal RPG moving from M1 scaffold to scene-based combat architecture.
- PM operations now use `.agents/` artifacts (teams/backlog/risk) as source of planning truth.
- User uses `IAN.md` for raw brainstorm entries and asks for integration reviews.
- Koad OS is adapted for PM workflow with `.koad`-scoped freshness gating and artifact sync rules.
- `docs/design/game-system-roadmap.md` is now the primary living focus reference for sequencing and system design order.
- `docs/design/execution-sprint-plan.md` is the primary near-term execution sequencing reference for M2-M3.
- Parallel implementation execution is delegated to Antigravity coder agents via root prompt pack `ANTIGRAVITY_SPRINT_PROMPTS.md`.
- Antigravity coder-lane workflow now defaults to one lane per dedicated git worktree/branch with lightweight PR policy.

## Open Unknowns
- Default `defensive_rounds_before_ai` for disconnect fallback.
- AI action whitelist after defensive rounds.
- Party-membership edge rules for encounter entry (temporary allies, summons, cross-party assist actions).

## Risk Watch
- Single-save campaign data safety and rollback behavior.
- Campaign-lock admin unlock guardrails and audit requirements.
- SRD-only compliance drift in V1 content implementation.
- Advisory DM-agent boundary leakage into authoritative NPC actions.
