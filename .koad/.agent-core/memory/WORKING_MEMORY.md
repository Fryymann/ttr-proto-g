# Working Memory

Last updated: 2026-02-21

## Active Context
- Project is a Rust multiplayer terminal RPG moving from M1 scaffold to scene-based combat architecture.
- PM operations now use `.agents/` artifacts (teams/backlog/risk) as source of planning truth.
- User uses `IAN.md` for raw brainstorm entries and asks for integration reviews.
- Koad OS is adapted for PM workflow with `.koad`-scoped freshness gating and artifact sync rules.

## Open Unknowns
- Default `defensive_rounds_before_ai` for disconnect fallback.
- AI action whitelist after defensive rounds.
- Pull-radius default profiles for scene size classes.

## Risk Watch
- Single-save campaign data safety and rollback behavior.
- Campaign-lock admin unlock guardrails and audit requirements.
- Pull-radius misconfiguration causing wrong encounter participants.
