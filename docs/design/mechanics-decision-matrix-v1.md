# Mechanics Decision Matrix v1

Status: Draft (living)
Last updated: 2026-02-21
Scope: SRD-aligned mechanics for a scene-based, turn-based multiplayer RPG

## Purpose

Capture what we will keep, adapt, or drop from D&D-style rules so implementation stays consistent across server, client, and content design.

## Decision Legend

- `KEEP`: implement close to 5.5e behavior with minimal changes.
- `ADAPT`: preserve spirit, but modify for multiplayer/server determinism.
- `DROP`: exclude from V1.
- `DEFER`: postpone beyond V1.

## Foundation Assumptions

- Grid combat uses 5-foot squares.
- Server is fully authoritative.
- Dice rolls remain core to resolution.
- Combat and social scenes both use turn order when in encounter mode.
- V1 uses SRD-only mechanics content plus original project-authored additions.

## Decision Matrix

| System | Baseline Intent | Decision | Source | V1 Implementation Direction | Why |
| --- | --- | --- | --- | --- | --- |
| d20 tests (checks/attacks/saves) | Core resolution mechanic | KEEP | SRD | Server rolls and validates all outcomes | Familiar and deterministic |
| Ability scores + modifiers | Character math base | KEEP | SRD | Standard modifiers and derived bonuses | Required for class/skill identity |
| Proficiency bonus scaling | Competency progression | KEEP | SRD | Global progression table in rules module | Clean and compact |
| Advantage/disadvantage | Binary roll modifier | KEEP | SRD | Single source of truth for stacking rules | Fast to compute and explain |
| Initiative | Turn order entry point | ADAPT | SRD | Roll once per encounter, stable tie-breakers, turn timer | Needed for online pacing |
| Surprise | Opening advantage/disruption | ADAPT | SRD | Apply initiative penalties/bonuses; avoid skip-turn feel-bad | Better multiplayer UX |
| Action/bonus/reaction economy | Tactical turn limits | KEEP | SRD | Hard-enforced action slots each turn/round | Strong tactical clarity |
| Movement on 5-ft grid | Tactical positioning | KEEP | SRD | Tile movement with speed budget per turn | Aligns with scene model |
| Diagonal movement | Grid distance edge case | ADAPT | Original | Use 5/5 diagonals in V1 | Simpler for terminal play |
| Reach and range | Melee/ranged constraints | KEEP | SRD | Manhattan/Chebyshev policy fixed in rules core | Required for weapon/spell identity |
| Opportunity attacks | Threat zones | KEEP | SRD | Trigger on hostile reach exit unless exempt | Core tactical depth |
| Cover | Positional defense | ADAPT | SRD | Tile/object tags grant half/three-quarters cover | Works with text maps |
| Conditions | Status effects | KEEP | SRD | Implement common combat conditions first | Reusable across systems |
| Exhaustion | Long-term fatigue | ADAPT | SRD | Use simplified track in V1 | Full detail is heavy early |
| HP/temp HP/death saves | Survival model | KEEP | SRD | Standard damage pipeline and death saves | Critical to D&D feel |
| Grapple/shove | Battlefield control | ADAPT | SRD | Keep with constrained deterministic checks | Useful but rules-heavy |
| Ready action | Tactical planning | KEEP | SRD | Queue trigger + stored action intent | Important for turn play |
| Dodge/disengage/dash/help/use object | Action set staples | KEEP | SRD | Already aligned with combat framework outline | Low risk, high value |
| Spell slots + spellcasting | Class identity | ADAPT | SRD | Start with curated spell list + strict validators | Complexity control |
| Concentration | Ongoing spell balance | KEEP | SRD | Single concentration effect per actor | Essential for spell balance |
| Components/material tracking | Spell friction realism | ADAPT | SRD | Ignore most material tracking except costly components | Reduce bookkeeping |
| Ammunition tracking | Resource realism | ADAPT | SRD | Optional toggle; default light tracking in V1 | Reduce friction |
| Encumbrance | Inventory realism | DROP | - | No weight simulation in V1 | High overhead, low fun early |
| Short/long rests | Recovery cadence | ADAPT | SRD | Server-timed, interruption-aware rest states | Persistent world needs explicit timing |
| Passive perception/insight | Background awareness | ADAPT | SRD | Derived passive values used by server checks | Reduces hidden GM logic |
| Stealth/hiding | Visibility gameplay | ADAPT | SRD | Deterministic visibility + contested checks | Must work without GM ad hoc |
| Social influence | Structured social play | ADAPT | Original | Turn-based social actions + NPC attitude states | Supports party social gameplay goal |
| Exploration travel pace | Overland procedure | DEFER | - | Scene-level movement first; world travel later | Not needed for early vertical slice |
| Mounted combat | Specialized subsystem | DROP | - | Excluded from V1 | Too large for early scope |
| Legendary/lair actions | Boss complexity | DEFER | - | Add after core encounter loop stabilizes | Advanced content layer |
| Multiattack/monster actions | NPC combat variety | KEEP | SRD | Per-NPC action profiles in data | Needed for encounter variety |

## Campaign and Session Operations

| System | Baseline Intent | Decision | Source | V1 Implementation Direction | Why |
| --- | --- | --- | --- | --- | --- |
| Server campaign selection | Choose world/campaign at startup | KEEP | Original | One active campaign chosen from prebuilt manifest at server boot | Clear operational model |
| Campaign save model | Persist ongoing progress | ADAPT | Original | Single save slot per campaign on server (no branches) | Simpler state management |
| Account ownership | Persistent player identity | KEEP | Original | Server accounts own character roster | Supports long-term progression |
| Character-to-campaign binding | Prevent cross-campaign leakage | KEEP | Original | Character becomes campaign-locked at join time; audited admin unlock path is allowed by policy | Preserves campaign continuity with operational recovery |
| Character export | Portability/backups | KEEP | Original | Export character snapshots as standardized JSON | User control and tooling support |
| Quit/disconnect in encounter | Session resilience | ADAPT | Original | Treat quit as disconnect; strictly defensive actions for N rounds, then limited AI behavior until reconnect/safe resolution | Prevent exploit and broken turns while preserving fairness |
| Encounter participation scope | Bring participants into combat | ADAPT | Original | V1 party-based participation; non-party actors are not auto-pulled by location | Supports distraction/sneak play and deterministic scope |
| DM-agent authority | Add dynamic NPC support without losing control | ADAPT | Original | DM-agent output is advisory only; scripted/rules layers decide authoritative outcomes | Preserves deterministic rule authority |
| Client presentation | Keep terminal feel with richer UX | ADAPT | Original | Terminal-first baseline with optional split-feed client surfaces | Better usability without forcing GUI |

## What Will Not Work Unchanged

- GM-only discretionary rulings with no deterministic tie-breaker.
- Purely manual bookkeeping for all edge rules in live multiplayer.
- Freeform NPC behavior that bypasses legal action validation.
- Unlimited-turn pacing without timeout or fallback actions.

## Multiplayer-First Adaptations We Intend To Add

- Turn timer with default fallback (`dodge` + `end_turn`) on timeout.
- Scene encounter capture via explicit party-based participation rules.
- Observer mode for late joiners during active encounter.
- Deterministic command queue per scene.
- Replay/event log for debugging and eventual combat recap.

## New Ideas Beyond Baseline D&D

- Unified encounter engine for both combat and social rounds.
- AI-driven NPC intent layer with strict server validation.
- Text-map overlays for turn order, threat, and cover cues.
- Rules profiles (`strict`, `fast`, `cinematic`) as future server config.

## V1 Mechanics Package (Recommended)

- d20 tests, advantage/disadvantage, initiative, action economy.
- Movement/range/reach/opportunity attacks on 5-ft squares.
- Core conditions, HP/death saves, concentration.
- Basic weapon attacks and curated low-complexity spell list.
- Social turn actions with party participation.

## V1 Exclusions (Hard Scope Cut)

- Full encumbrance simulation.
- Mounted combat subsystem.
- Full spell component economy.
- Large optional tactical variants (flanking variants, optional grids, etc.).

## Open Questions

- Should diagonal movement remain 5/5 permanently or become configurable?
- What party edge rules should ship for temporary allies, summons, and cross-party assist?
- How strict should turn timers be in public scenes versus party instances?
- Which exact conditions are mandatory for V1 launch versus V1.1?
- How many spells per class can we support before validation overhead spikes?
- What guardrails/audit requirements should admin campaign-unlock enforce?
- What is the default defensive-round count before AI takeover on disconnect?
- What exact post-defensive AI action whitelist should be allowed?
- What minimum backup/snapshot cadence is required for single-save campaign safety?

## Next Implementation Steps

1. Convert this matrix into a typed `RuleConfig` schema in server code.
2. Define exact formulas and tie-breakers for every `ADAPT` row.
3. Add SRD source mapping for each V1 mechanics row.
4. Add tests that pin each chosen mechanic behavior.
5. Mark each row as `implemented`, `in_progress`, or `not_started`.
