# Backlog

Status: Active
Last updated: 2026-02-22
Owner: Project Manager

## Priority Legend

- `P0`: critical path for next playable milestone
- `P1`: important, near-term
- `P2`: valuable, can follow core delivery

## Workflow States

- `todo`
- `in_progress`
- `blocked`
- `done`

## Active Focus Window

- `Now` (M2 closeout + M3 execution): `BL-012`, `BL-007`, `BL-013`
- `Next` (M3 follow-on): `BL-018`, `BL-008`, `BL-014`, `BL-020`
- `V2 intake parking lot`: `BL-021`, `BL-022`, `BL-023`, `BL-024`, `BL-025`, `BL-026`
- `Reference`: detailed sequencing in `docs/design/execution-sprint-plan.md`

## Backlog Items

| ID | Priority | Team | State | Milestone | Task | Acceptance Criteria |
| --- | --- | --- | --- | --- | --- | --- |
| BL-001 | P0 | Gameplay | done | M2 | Define `Scene` data model and tile occupancy rules in code | Scene structs compile, occupancy checks unit-tested |
| BL-002 | P0 | Platform | done | M2 | Add protocol messages for scene snapshot/delta | Protocol types added, server sends valid scene updates |
| BL-003 | P0 | Experience | done | M2 | Render scene grid in CLI with player/NPC symbols | Client shows map updates for movement in multiplayer test |
| BL-004 | P0 | Platform | done | M2 | Implement deterministic scene command queue | Stable ordering test passes with same input stream |
| BL-005 | P1 | Gameplay | done | M3 | Implement encounter skeleton (start, party-scoped participants, initiative, turns) | Encounter can start/end, includes only the initiating actor's party in V1, and cycles turns correctly |
| BL-006 | P1 | Platform | done | M3 | Add turn timer + timeout fallback action | Timed-out actor auto-resolves with configured fallback |
| BL-007 | P1 | Experience | todo | M3 | Show turn tracker and active actor in client | Turn indicator updates for all participants |
| BL-008 | P1 | Gameplay | todo | M3 | Implement core combat actions (`move`, `attack`, `dodge`, `help`) | Action legality and resolution tests pass |
| BL-009 | P2 | Gameplay | todo | M4 | Implement scripted NPC policy interface (`NpcPolicyScripted`) | NPCs take valid turns without model calls |
| BL-010 | P2 | Platform | todo | M4 | Add combat replay event logging | Replay log reconstructs encounter timeline |
| BL-011 | P0 | Platform | done | M2 | Implement campaign manifest and startup campaign selection flow | Server boots with explicit campaign selection and loads active campaign config |
| BL-012 | P0 | Platform | in_progress | M2 | Implement account + character persistence model with account-authenticated login and campaign lock metadata | User logs into an account session first, character create/login is scoped to authenticated account ownership, campaign lock is enforced on join, and admin unlock events are audited |
| BL-013 | P1 | Platform | todo | M3 | Implement single-save campaign storage with rollback-safe snapshot strategy | Campaign progress persists and recovery test passes after simulated crash |
| BL-014 | P1 | Gameplay | todo | M3 | Define and implement staged quit/disconnect in encounter rules | Defensive-only fallback for configured rounds then limited AI behavior is deterministic and covered by integration tests |
| BL-015 | P1 | Gameplay | done | M3 | Implement V1 party-based encounter participation rules | Non-party players in the scene are not auto-pulled, party members are included deterministically, and tests cover join edge cases |
| BL-016 | P2 | Experience | todo | M4 | Design split-feed client mode (map/combat/chat panels) while preserving terminal-first mode | Optional multi-panel view works without breaking baseline CLI flow |
| BL-017 | P2 | Platform | todo | M4 | Add character export endpoint/command with standardized JSON schema | Exported JSON validates against schema and round-trip import validation passes |
| BL-018 | P1 | Platform | todo | M3 | Implement admin campaign-unlock tooling with guardrails | Unlock requires explicit reason, is permission-gated, and emits immutable audit log entries |
| BL-019 | P0 | Gameplay | done | M2 | Define and enforce SRD-only mechanics/content scope for V1 | V1 rules/content references map to SRD or original content only, with a committed compliance checklist |
| BL-020 | P1 | Gameplay | todo | M3 | Implement DM-agent advisory channel for semi-scripted NPC behavior | AI output is suggestions-only, scripted policy remains authoritative, and acceptance/override behavior is covered by tests |
| BL-021 | P2 | Gameplay | todo | V2 | Add chained movement grammar for free-roam and encounter-aware movement budgets (`go e 3`, `go e 3, s 3`) | Parser supports chained vectors, movement cost is validated per segment, and encounter turns enforce legal movement budget |
| BL-022 | P2 | Gameplay | todo | V2 | Implement terrain movement-cost model with reversible spell-driven terrain overrides | Difficult terrain and temporary terrain effects modify movement cost deterministically and revert correctly when effects end |
| BL-023 | P2 | Experience | todo | V2 | Add auto-walk UX for long-route exploration | Player can queue multi-step travel safely, receives clear interruption feedback, and can cancel or resume without desync |
| BL-024 | P2 | Gameplay | todo | V2 | Define spell-targeting syntax and resolver for location and actor targets | Commands like `cast web location 10e,2s` and `cast web target bandit_1` resolve to valid target intents with legality checks |
| BL-025 | P2 | Platform | todo | V2 | Implement NPC promotion tiers to upgrade low-detail NPCs into story-critical actors | NPC lifecycle supports tier promotion with schema-safe attribute expansion and migration/audit coverage |
| BL-026 | P2 | Gameplay | todo | V2 | Add PC relationship and faction reputation systems for non-combat progression | Character state tracks NPC/faction standing changes and gameplay systems can gate quests/resources on those standings |

## Intake Template

Use this format for new items:

- ID:
- Priority:
- Team:
- Milestone:
- Task:
- Acceptance Criteria:
- Dependencies:
- Notes:
