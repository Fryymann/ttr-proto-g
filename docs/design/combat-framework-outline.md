# Combat Framework Outline

Status: Draft (living document)
Last updated: 2026-02-21
Owners: You + Codex

## Purpose

Define a medium-scale architecture for scene-based multiplayer gameplay with turn-based combat encounters in an open world.

This document is intentionally iterative. Add decisions, revisions, and unresolved questions as we discover constraints.

## Product Direction (Agreed So Far)

- Replace classic MUD "room only" navigation with configurable `scenes`.
- Each scene uses a 5-foot square grid (D&D-compatible spatial model).
- Players move freely in scenes during exploration.
- Scene maps are rendered as text graphics (walls, obstacles, NPCs, players).
- Combat is turn-based and table-top inspired:
  - Initiative rolls
  - Turn tracker
  - Action economy per turn
- When combat starts in a scene, only the triggering actor's party is captured in V1 (plus relevant NPCs).

## Core Concepts

## Scene

A `Scene` is the authoritative simulation space for exploration and encounters.

- Size: configurable width/height in squares.
- Tile layers:
  - Terrain (floor type)
  - Collision (wall/blocked)
  - Objects (cover, tables, doors)
  - Occupants (players, NPCs)
- Zones (optional): named sub-areas for triggers (bar, stairs, doorway).
- Visibility: initial design uses full scene visibility; fog-of-war is a future extension.

## Encounter

An `Encounter` is a temporary turn-based rules envelope within a scene.

- Bound to one scene (`scene_id`).
- Has participants (players + NPCs).
- Has initiative order and current turn pointer.
- Has round counter and timers.
- Ends when victory, defeat, escape, surrender, or scripted condition is met.

## Actor

Any entity that can take turns.

- Player-controlled character
- AI-controlled NPC/monster

## Modes

- `ExplorationMode`: free movement and non-turn interactions.
- `EncounterMode`: strict turn order and action limits.
- Scene can only host one active combat encounter at a time in V1.

## Campaign Runtime Model (Draft)

- A server instance loads exactly one active campaign at a time.
- Campaigns come from a prebuilt campaign catalog/manifest.
- Server startup should present/select which campaign to load.
- Each campaign uses a single save slot in V1:
  - Progress is continued over time.
  - No parallel save branches for the same campaign on one server.

## Account and Character Model (Draft)

- Players authenticate into server accounts.
- Accounts own persistent character records.
- Players can create and maintain multiple characters per account.
- When a character joins the active campaign, that character becomes campaign-locked.
- Character export is supported as standardized JSON snapshots (copy/export, not detached overwrite).

## Scene Model (V1)

```text
Scene
  id
  name
  width, height
  tiles[y][x]
  objects[]
  occupants[]
  active_encounter_id?
```

### Tile and Occupancy Rules

- Coordinate system: `(x, y)` where each tile == 5 ft.
- Occupancy: one medium creature per tile in V1.
- Movement blocked by:
  - Walls/solid objects
  - Occupied tiles (except special actions later)
- Difficult terrain support planned in V2.

## Text-Graphics Rendering (V1)

Server sends a scene snapshot; client renders with symbols.

Suggested symbol set (changeable):

- `#` wall
- `.` walkable floor
- `+` door/interactable
- `~` difficult terrain (reserved)
- `P` current player
- `@` other players
- `N` neutral NPC
- `E` enemy NPC

Example tavern floor:

```text
####################
#....+......T......#
#.####......T...N..#
#..............@...#
#..P.........B.....#
####################
```

## Client Surface Direction (Draft)

- Keep terminal-first game feel as the baseline client.
- Add optional richer client surface that can separate feeds/panels:
  - map stream
  - combat/event feed
  - communication channels
- Protocol design should support split-view clients without requiring them.

## Encounter Lifecycle

1. Trigger
- Hostile action, scripted event, or aggression AI in a scene.

2. Participant Capture
- Collect triggering party members + relevant NPCs.
- Freeze entry into free actions in that scene.

### Encounter Participation Policy (V1)

- V1 uses party-based participation rules.
- Non-party actors in the same scene are not auto-pulled into combat.
- Scene scripts can explicitly add/remove participants when required by content.

3. Initiative
- Roll initiative for each participant.
- Tie-breakers: higher DEX mod, then stable actor id.

4. Turn Loop
- Active actor takes turn under action economy.
- Validate and resolve action server-side.
- Advance to next actor.
- Increment round when cycle completes.

5. End Conditions
- One faction eliminated.
- All enemies flee/surrender.
- Objective complete.
- Scene script terminates encounter.

6. Cleanup
- Apply persistent effects (HP/state/loot).
- Return scene to exploration mode.

## Turn System (Combat V1)

Per turn:

- 1 Action
- 1 Bonus Action (if available)
- Movement up to speed in squares
- 1 Reaction per round (resolved off-turn)

Supported V1 action set:

- `move`
- `attack`
- `dash`
- `disengage`
- `dodge`
- `help`
- `use_object`
- `end_turn`

## Multiplayer Concurrency Model

Server-authoritative and scene-local deterministic processing.

- Each scene has a command queue.
- Commands are stamped with server tick + actor id.
- Resolver processes queue in deterministic order.
- During encounter mode:
  - Only active actor's turn actions are accepted.
  - Invalid/out-of-turn commands return structured errors.

### Late Join/Leave Policy (Initial)

- Players entering a scene with active encounter spawn as observers in V1.
- Reconnect during encounter:
  - If participant, restore control on their turn.
  - If turn times out, default action (`dodge` + end turn) is applied.

### Quit/Disconnect in Combat Policy (Draft)

- Outside encounters: player can quit immediately.
- During active encounter:
  - Manual quit is treated as combat disconnect.
  - Actor remains in encounter under staged fallback behavior:
    - Phase 1: strictly defensive actions only for a configured number of rounds.
    - Phase 2: limited AI behavior after defensive rounds are exhausted.
- Reconnect should restore control of that actor when legal in turn flow.
- Safe removal and penalties are open for balancing decisions.

## AI-Driven NPC Framework (Draft)

Goal: make NPC behavior feel dynamic while keeping gameplay fair, deterministic, and affordable.

### Design Principles

- Server remains authoritative for all game rules.
- AI proposes `intent`; server validates and resolves.
- DM-agent guidance is advisory; scripted rules and narrative policy remain authoritative.
- AI calls are event-driven, not tick-driven.
- Every AI decision has a deterministic fallback.

### Control Loop

1. Build `NpcContext` from current game state.
2. Call AI policy for intent.
3. Validate intent against legal actions for that NPC/state.
4. Resolve action in rules engine.
5. Broadcast structured result to scene clients.
6. Log decision + fallback status for debugging.

### NPC Intelligence Tiers

- Tier 0: fully scripted/template NPCs (no model call).
- Tier 1: lightweight model for common NPC dialogue/combat choices.
- Tier 2: richer model for named/story-critical NPCs.

### Intent Contract

AI output should be compact and machine-checkable.

Combat intent shape:

```json
{
  "intent_type": "combat",
  "action": "attack",
  "target_actor_id": 42,
  "move_path": [[11, 7], [12, 7]],
  "bonus_action": null,
  "reasoning_tags": ["focus_low_hp_enemy", "maintain_cover"]
}
```

Dialogue intent shape:

```json
{
  "intent_type": "dialogue",
  "tone": "wary",
  "goal": "deflect_question",
  "line": "Rumors spread fast in this town. What exactly are you asking?",
  "skill_hooks": ["insight", "persuasion"],
  "state_delta": {
    "trust_delta": -1,
    "suspicion_delta": 1
  }
}
```

### Prompt Input Envelope

Each call should include only minimal state:

- NPC identity: archetype, personality tags, current goals.
- Scene summary: local map slice, nearby actors, threats, cover.
- Encounter summary (if active): initiative index, HP bands, conditions.
- Relationship summary: party reputation/trust/fear flags.
- Allowed actions list for this decision point.
- Hard constraints (no illegal movement, no impossible knowledge).

### Safety and Rule Guardrails

- Never execute raw AI text directly as commands.
- Validate all coordinates, targets, and action economy limits.
- Reject illegal intents with one automatic retry.
- If retry fails or times out, use fallback policy:
  - Combat fallback: `dodge` or nearest legal basic attack.
  - Dialogue fallback: short in-character neutral response.

### Cost and Throughput Model

Use event-triggered invocation with budgets.

- Call on:
  - NPC turn start in encounter
  - player-initiated conversation turns
  - key world events (rare)
- Do not call during passive idle ticks.
- Keep per-call context compact (summaries, not full logs).
- Cache reusable dialogue snippets per NPC mood/state.

Budget controls:

- `max_ai_calls_per_scene_per_minute`
- `max_ai_calls_per_npc_per_minute`
- `max_tokens_in` / `max_tokens_out`
- Queue with timeout; fallback on timeout

### Determinism and Replay

- Store:
  - prompt hash
  - model config
  - response payload
  - final validated action
- For tests, allow seedable non-AI policies to reproduce outcomes.

### Suggested Rust Interfaces (Sketch)

```rust
enum NpcDecisionKind {
    CombatTurn,
    DialogueTurn,
    EventReaction,
}

struct NpcDecisionRequest {
    npc_id: ActorId,
    scene_id: SceneId,
    kind: NpcDecisionKind,
    allowed_actions: Vec<AllowedAction>,
    context_summary: String,
}

enum NpcIntent {
    Combat(CombatIntent),
    Dialogue(DialogueIntent),
}

trait NpcPolicy {
    fn decide(&self, req: NpcDecisionRequest) -> Result<NpcIntent, NpcPolicyError>;
}
```

### Rollout Plan

1. Phase A: implement deterministic scripted policy (`NpcPolicyScripted`).
2. Phase B: add model-backed policy behind feature flag (`NpcPolicyModel`).
3. Phase C: tiering, budgets, retry/fallback, and metrics.
4. Phase D: richer social reasoning hooks for party skill interactions.

## Data Structures (Rust Sketch)

```rust
struct SceneId(String);
struct EncounterId(String);
struct ActorId(u64);

struct Scene {
    id: SceneId,
    name: String,
    width: u16,
    height: u16,
    tiles: Vec<Tile>,
    occupants: HashMap<ActorId, Position>,
    active_encounter: Option<EncounterId>,
}

struct Encounter {
    id: EncounterId,
    scene_id: SceneId,
    phase: EncounterPhase,
    round: u32,
    initiative: Vec<ActorId>,
    active_index: usize,
    participants: HashMap<ActorId, ParticipantState>,
}
```

## Protocol/Event Shape (Additive Plan)

Client -> Server:

- `MoveIntent { dx, dy }` (exploration)
- `InteractIntent { target_id }`
- `EncounterAction { action_type, targets, path }`
- `EndTurn`

Server -> Client:

- `SceneSnapshot`
- `SceneDelta`
- `EncounterStarted`
- `InitiativeOrder`
- `TurnStarted`
- `ActionResolved`
- `EncounterEnded`

## Phased Implementation Plan

## Phase 1: Scene Foundation

- Add scene grid model in server.
- Move from room links to `(x,y)` movement.
- Render ASCII scene in CLI.
- Keep current chat + who intact.

Exit criteria:
- Multiple players can move in one scene and see position updates.

## Phase 2: Encounter Skeleton

- Add encounter state machine.
- Add participant capture from scene.
- Add initiative + turn tracker + timeout.
- Implement `end_turn` and no-op fallback.

Exit criteria:
- Deterministic turn order visible to all clients.

## Phase 3: Combat Actions V1

- Implement move/attack/dodge/help/dash/disengage/use_object.
- Enforce movement/action limits.
- Resolve hits/damage and death/unconscious state.

Exit criteria:
- Full combat can start and end in one scene.

## Phase 4: AI + UX

- Basic NPC combat AI.
- Better map overlays (selected target, turn banner).
- Log panels for combat events.

Exit criteria:
- Tavern brawl scenario playable end-to-end.

## Testing Strategy

- Unit tests:
  - Initiative ordering
  - Movement validation
  - Action economy rules
  - End-condition detection
- Integration tests:
  - Multi-client encounter start
  - Turn enforcement
  - Disconnect/reconnect during turn
- Determinism tests:
  - Same command stream => same final encounter state

## Open Design Questions

- What party edge rules should we ship for temporary allies, summons, and cross-party assist?
- Do we support multiple simultaneous encounters in one scene after V1?
- How should stealth/surprise modify participant capture and initiative?
- How strict should turn timers be in public scenes vs private party instances?
- Should movement in exploration remain tile-by-tile or allow path commands?
- What is the default defensive-round count before AI takeover on disconnect?
- What AI behavior limits are allowed after defensive rounds (action whitelist)?
- How do we protect single-save campaigns from save corruption or operator mistakes?

## Decision Log

Use this section to append accepted decisions.

Format:

- Date: YYYY-MM-DD
- Decision:
- Why:
- Consequences:

- Date: 2026-02-21
- Decision: One active campaign per server instance; campaign selected at startup from prebuilt options.
- Why: Keep runtime simple and campaign progression coherent for multiplayer sessions.
- Consequences: Need campaign manifest loader and startup selection flow.

- Date: 2026-02-21
- Decision: Characters are account-owned and become locked to a campaign once joined.
- Why: Prevent cross-campaign contamination and preserve campaign continuity.
- Consequences: Need lock metadata, validation checks, and audited admin unlock policy.

- Date: 2026-02-21
- Decision: Campaign-lock is reversible by admins/tools under explicit policy.
- Why: Operational recovery and campaign lifecycle management require controlled exceptions.
- Consequences: Need admin unlock controls, audit logging, and guardrails.

- Date: 2026-02-21
- Decision: V1 encounter participation is party-based, not scene-wide pull by radius/zone.
- Why: Supports distraction/sneak play patterns and prevents over-capturing by location alone.
- Consequences: Need clear party membership contracts, participant-capture validation, and edge-case policies.

- Date: 2026-02-21
- Decision: Disconnect fallback is staged: strictly defensive for N rounds, then limited AI behavior.
- Why: Fairness and anti-exploit protection while keeping encounters moving.
- Consequences: Need configurable round count, AI action whitelist, and reconnect handoff rules.

- Date: 2026-02-21
- Decision: V1 mechanics and rules content are SRD-only plus original project-authored content.
- Why: Reduce licensing risk and keep implementation scope explicit for early milestones.
- Consequences: Need an SRD compliance checklist and review gating in design/code workflow.

- Date: 2026-02-21
- Decision: DM-agent is advisory by default for semi-scripted NPC support.
- Why: Preserve deterministic authority in scripts/rules while still adding dynamic behavior.
- Consequences: Need acceptance/rejection boundaries, logging, and non-authoritative AI interfaces.

## Next Editing Targets

- Define exact stat block schema for `ParticipantState`.
- Define action resolution formulas (hit, damage, cover).
- Define AI decision loop per NPC archetype.
- Define concrete wire protocol structs in `ttrpg-protocol`.
- Define `NpcDecisionRequest` and `NpcIntent` wire schema.
- Define per-scene/per-NPC AI budget defaults for dev/prod.
- Define campaign manifest/startup selection flow and save storage format.
- Define character campaign-lock lifecycle, export schema, and admin unlock policy.
- Define disconnect fallback defaults (`defensive_rounds_before_ai`, AI action whitelist).
- Define party-participation edge rules (temporary allies, summons, cross-party assist).
