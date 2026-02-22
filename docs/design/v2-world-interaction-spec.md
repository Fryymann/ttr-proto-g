# V2 World Interaction Spec (IAN Intake 2026-02-22)

Status: Draft (parking-lot, not scheduled in active S4 lanes)  
Last updated: 2026-02-22  
Owner: Project Manager (Koad) + Gameplay/Platform/Experience leads

## Purpose

Convert the latest `IAN.md` concept intake into implementation-ready V2 specs with clear backlog mapping, constraints, and dependencies.

Source intake reference:
- `IAN.md` entry dated `2026-02-22 - 11:42pm`.

Backlog mapping:
- `BL-021`, `BL-022`, `BL-023`, `BL-024`, `BL-025`, `BL-026`, `BL-027`

## Scope Posture

- These concepts are V2+ planning scope and are not authorized for active S4 implementation lanes unless Koad re-prioritizes.
- V1 guardrails remain unchanged:
  - server-authoritative simulation
  - deterministic turn and command resolution
  - SRD-only + original project-authored content

## 1) Chained Movement Grammar (`BL-021`)

Goal:
- Support movement chains such as `go e 3` and `go e 3, s 3` with deterministic segment-by-segment execution.

Command contract:
- Grammar: `go <dir> <tiles>(, <dir> <tiles>)*`
- Allowed directions for V2 baseline: `n|s|e|w`.
- `tiles` is a positive integer.

Resolution contract:
- Parse and validate the full chain before mutating state.
- Execute movement segment-by-segment in stable order.
- Stop at first illegal segment (collision, occupancy, budget exceeded) and return partial-progress result with explicit reason.

Mode behavior:
- Free-roam: one command can spend up to the actor's current movement budget.
- Encounter: command can include chained segments, but total legal movement cannot exceed per-turn budget.

## 2) Terrain Movement Costs + Reversion (`BL-022`)

Goal:
- Add terrain-aware movement costs and temporary spell-driven terrain overrides that cleanly revert.

Terrain contract:
- Tiles have a base movement cost and terrain tags.
- Runtime overlays can modify terrain tags/cost for a bounded duration.

Reversion contract:
- Every temporary terrain change has a source effect id and deterministic expiry condition.
- On effect expiry/removal, tile state reverts to previous baseline without orphaned modifiers.

Determinism requirement:
- Terrain cost and revert results must be replay-stable across clients for the same event stream.

## 3) Auto-Walk for Long Travel (`BL-023`)

Goal:
- Provide route queueing for long movement while preserving server authority and interruption safety.

Behavior contract:
- Auto-walk queue is explicit and cancelable.
- Route execution halts on blocking events (encounter start, collision, scene script gate, disconnect).
- Client receives clear state transitions: started, advanced, interrupted, canceled, completed.

Safety constraints:
- No silent background movement after a blocking condition.
- Reconnect resumes only with explicit server-confirmed queue state.

## 4) Spell Targeting Grammar (`BL-024`)

Goal:
- Support actor-target and location-target spell syntax with one normalized targeting model.

Command forms:
- Actor target: `cast <spell_id> target <actor_id>`
- Location target: `cast <spell_id> location <dx><dir>,<dy><dir>` (example: `cast web location 10e,2s`)

Resolution contract:
- Parser normalizes both forms into one target-intent structure.
- Location target resolves an authoritative center coordinate before area rules apply.
- Actor target resolves to authoritative actor position if spell requires location centering.

Validation contract:
- Reject illegal target forms with machine-readable error code and clear user guidance.

## 5) NPC Promotion Lifecycle (`BL-025`)

Goal:
- Allow low-detail NPCs to be promoted into story-critical actors without unsafe state mutation.

Tier model (initial):
- Tier 0: ambient/minimal NPC
- Tier 1: interactive NPC with richer local behavior
- Tier 2: story-critical NPC with expanded state/policy hooks

Promotion contract:
- Promotions are explicit events with old tier, new tier, and reason metadata.
- Promotion applies schema-safe attribute expansion and preserves identity continuity.
- Promotion and demotion policies are audit-visible.

## 6) Relationship and Faction Standing (`BL-026`)

Goal:
- Track character relationships with NPCs/factions to support non-combat progression paths.

State model:
- Per-character NPC relationship scores/tags.
- Per-character faction standing scores/tags.

Gameplay hooks:
- Quest/dialog/shop access gates can reference relationship/faction thresholds.
- Outcomes from choices and skills can update standing in deterministic, logged steps.

## 7) Spell Capability Staging (`BL-027`)

Goal:
- Stage broad spell support safely by defining what is enabled now versus blocked behind missing systems.

Catalog contract:
- Each spell declares:
  - targeting mode(s)
  - allowed context(s): free-roam, encounter, social
  - dependency tags (for example: requires relationship system, requires promoted-NPC attributes)

Runtime contract:
- Spells with unmet dependencies fail fast with explicit, user-readable gating reasons.
- Spell behavior remains deterministic even when partially enabled by feature flags.

## Dependency Order (Planning)

1. `BL-021` (movement grammar baseline)
2. `BL-022` (terrain cost + modifier/revert model)
3. `BL-023` (auto-walk queueing on top of movement + terrain legality)
4. `BL-024` (spell targeting grammar and target-intent normalization)
5. `BL-025` + `BL-026` (NPC promotion + relationship/faction state)
6. `BL-027` (spell capability staging and dependency gating across the above)

## Open Questions

- Should diagonal movement syntax be deferred until after cardinal-direction baseline stability?
- Should location targeting support absolute coordinates, relative vectors, or both in first pass?
- How should standing decay over time, if at all, for factions and NPC relationships?
- Which promotion events are reversible versus permanent for story-critical NPCs?
