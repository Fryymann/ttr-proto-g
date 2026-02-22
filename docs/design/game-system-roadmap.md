# Game System Roadmap and Execution Outline

Status: Active (living document)  
Last updated: 2026-02-21  
Owner: Project Manager (Koad) + team leads

## Purpose

Provide one durable reference for:

- What we are building first
- Which foundations are mandatory
- How each major system works
- What players should experience at each stage
- Where team focus should go next

Update this document whenever product decisions, sequencing, or system boundaries change.

## Product Direction (Locked for V1)

- Rules content scope is SRD-only plus original project-authored content.
- One active campaign per server instance, selected at startup.
- Campaign progress uses a single save slot with rollback-safe snapshots.
- Characters are account-owned and campaign-locked on join (audited admin unlock path).
- Encounters are party-based in V1 (not whole-scene auto-pull).
- DM-agent is advisory by default; scripted story and server rules stay authoritative.
- Terminal-first client experience remains the baseline; richer views are optional.

## Player Experience and Expectations

### Core player promise

- Players can join a persistent story world, keep a character over time, and participate in tactical encounters without losing the terminal RPG feel.

### Expected experience loop

1. Player connects and authenticates.
2. Player creates/selects a character tied to their account.
3. Player enters active campaign and joins or forms a party.
4. Party explores scenes in real time.
5. Encounter starts for the acting party when a trigger occurs.
6. Turn-based actions resolve with clear feedback and deterministic outcomes.
7. Disconnects recover safely without breaking encounter integrity.
8. Campaign progress persists and resumes cleanly next session.

### UX quality expectations

- Commands must be discoverable and forgiving.
- Combat state must be legible (turn, actor, options, consequences).
- Errors should be explicit and actionable.
- Reconnect should feel reliable, not punishing.

## Foundational Data Model (Design First)

| Entity | Why it exists | Minimum V1 fields |
| --- | --- | --- |
| CampaignManifest | Defines selectable campaigns at server boot | `campaign_id`, `name`, `content_version`, `entry_scene_id`, `save_path` |
| CampaignState | Persistent world and progression state | `campaign_id`, `world_flags`, `scene_states`, `quest_states`, `updated_at` |
| Account | Persistent player identity container | `account_id`, `handle`, `created_at`, `status` |
| Character | Playable actor owned by account | `character_id`, `account_id`, `name`, `species`, `class`, `stats`, `inventory_ref` |
| CharacterCampaignLock | Prevents cross-campaign leakage | `character_id`, `campaign_id`, `locked_at`, `unlock_audit_ref?` |
| Party | Encounter participation unit in V1 | `party_id`, `campaign_id`, `member_character_ids`, `leader_character_id` |
| Scene | Authoritative spatial simulation unit | `scene_id`, `width`, `height`, `tiles`, `occupants`, `scripts_ref` |
| Encounter | Turn-based rules envelope | `encounter_id`, `scene_id`, `party_id`, `participants`, `initiative`, `turn_state` |
| ActorState | Runtime combat/exploration state | `actor_id`, `hp`, `conditions`, `position`, `resources` |
| NpcPolicyProfile | Scripted behavior + advisory AI profile | `npc_id`, `script_profile`, `ai_enabled`, `allowed_advice_types` |
| AuditEvent | Immutable ops and moderation trail | `event_id`, `event_type`, `actor`, `payload`, `timestamp` |

## Foundational Functionality (Must Exist)

### Platform foundations

- Campaign selection and loading pipeline
- Deterministic command queue per simulation scope
- Persistence with snapshot and recovery flow
- Session management and reconnect handling
- Structured audit logging for sensitive operations

### Gameplay foundations

- Scene movement and occupancy validation
- Party membership and lifecycle rules
- Encounter state machine (start, turns, resolve, end)
- Rules resolver (checks, attacks, conditions, resources)
- Deterministic timeout/disconnect fallback behavior

### Experience foundations

- Stable command grammar and help system
- Scene rendering and event feed clarity
- Turn tracker and active actor cues
- Error and warning surfaces that explain next action

### Narrative and AI foundations

- Scripted NPC behavior as default authority
- Advisory DM-agent interface with explicit approval gates
- Content script hooks for quest and scene transitions

## Conventions and Operational Flows

### Runtime conventions

- Server is authoritative for all state transitions.
- All game actions are validated before state mutation.
- Any nondeterministic input must resolve to deterministic state changes.

### Protocol conventions

- Messages are explicit, versionable, and additive when possible.
- Errors include machine-readable code plus player-readable guidance.
- Client should never assume success before server acknowledgement.

### Identity and progression conventions

- Accounts own characters.
- Characters lock to campaign on first join.
- Unlock requires explicit admin reason and immutable audit trail.

### Encounter conventions

- V1 participant scope is party-based.
- Non-party actors remain outside the encounter unless explicitly added by rule/script.
- Turn ownership is strict; out-of-turn actions are rejected with clear feedback.

### DM-agent conventions

- DM-agent produces advisory intent only.
- Scripts/rules engine decide whether to accept, modify, or reject advisory output.
- Every accepted advisory action is traceable in logs.

## Core Systems: How Each One Works

### 1) Campaign Runtime System

- At boot, server loads `CampaignManifest` catalog.
- Operator selects one campaign.
- Server loads current `CampaignState` (or initializes seed state).
- All runtime subsystems use this campaign context as root namespace.

### 2) Account and Character System

- Authenticate account session.
- Load account character roster.
- Create/edit character within validation rules.
- On campaign join, apply `CharacterCampaignLock`.
- Reject joins that violate lock policy unless authorized unlock exists.

### 3) Party System

- Party membership is explicit and persisted.
- Party leader or policy controls invites/removals.
- Encounter capture references active party membership at trigger time.

### 4) Scene Simulation System

- Scene tick processes movement and non-encounter interactions.
- Occupancy and collision checks gate legal moves.
- Scene events can emit narrative hooks or encounter triggers.

### 5) Encounter Engine

- Trigger creates encounter bound to `scene_id` and `party_id`.
- Build participant list from party + relevant NPCs.
- Roll initiative using deterministic tie-breakers.
- Process turn loop until end condition.
- Persist post-encounter effects back into campaign state.

### 6) Rules Resolution Engine

- Receives normalized action intent.
- Validates legality (range, resources, action economy, target state).
- Resolves dice and modifiers server-side.
- Emits structured result event for UI/logging/replay.

### 7) Narrative Script Engine

- Drives predetermined story beats and scene outcomes.
- Owns quest state updates and scripted NPC gates.
- Can request encounter starts, dialogue states, or world flag changes.

### 8) NPC and DM-Agent Advisory System

- Script policy chooses when advisory call is allowed.
- Advisory output returns suggested intent and tags.
- Rules/script layers accept or reject suggestion.
- Fallback defaults apply if advisory is unavailable or invalid.

### 9) Persistence and Recovery System

- Use atomic writes and periodic snapshots.
- Persist campaign, account, character, party, and encounter-critical state.
- Run restore checks on startup; fail safe with recovery guidance.

### 10) Client Experience System

- Terminal client renders scene, feed, prompts, and turn state.
- Command parser maps text input to protocol messages.
- Optional split-feed presentation can be layered without changing server authority.

## Design Sequence: What Must Be Designed First

1. Data contracts and IDs across campaign/account/character/party/scene/encounter.
2. Campaign load/save and snapshot recovery model.
3. Party lifecycle and encounter participation rules.
4. Scene movement legality and occupancy model.
5. Encounter state machine and turn-loop contracts.
6. Rules resolver formulas and deterministic tie-breakers.
7. Protocol message set for scene and encounter events.
8. Client rendering and command UX contract.
9. Script engine hooks and DM-agent advisory boundary.

## Delivery Roadmap (Milestone Focus)

## M2: Foundation Build

- Campaign manifest + selection flow
- Account/character persistence + campaign lock metadata
- Scene grid model + deterministic queue
- CLI scene rendering and baseline protocol expansion

Exit criteria:

- Multiplayer exploration works on scene grid with persistent identities and campaign context.

## M3: Playable Encounter Loop

- Party-based encounter capture rules
- Encounter lifecycle + initiative + turn tracking
- Combat action core and timeout/disconnect fallback
- Admin unlock guardrails + audit trail

Exit criteria:

- Party can complete an end-to-end encounter with reconnect-safe turn flow.

## M4: Narrative and Depth

- Scripted NPC policy framework
- Advisory DM-agent channel (non-authoritative)
- Replay/event diagnostics and richer client readability
- Character export/import validation path

Exit criteria:

- Predetermined story beat with semi-scripted NPC behavior is playable and observable.

## M5: Stabilization and Expansion Prep

- Performance and reliability hardening
- Additional encounter/content variety
- Social encounter scaffolding for V2 entry

Exit criteria:

- Vertical slice is stable enough for repeated multi-session playtests.

## Focus Board (Update Weekly)

- `Now`: M2 data contracts, campaign persistence, scene foundation.
- `Next`: M3 party encounter loop and deterministic turn flow.
- `Later`: M4 narrative depth, advisory AI refinement, UX expansion.

## Open Questions (Track and resolve)

- Default `defensive_rounds_before_ai` value.
- Post-defensive AI action whitelist.
- Party edge rules (temporary allies, summons, assist across parties).
- Minimum snapshot cadence and retention policy for single-save protection.
- Exact V1 SRD compliance checklist structure and review gate.

## Update Log

- 2026-02-21: Initial roadmap created as primary project focus reference.
