# Backlog

Status: Active
Last updated: 2026-02-21
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

## Backlog Items

| ID | Priority | Team | State | Milestone | Task | Acceptance Criteria |
| --- | --- | --- | --- | --- | --- | --- |
| BL-001 | P0 | Gameplay | todo | M2 | Define `Scene` data model and tile occupancy rules in code | Scene structs compile, occupancy checks unit-tested |
| BL-002 | P0 | Platform | todo | M2 | Add protocol messages for scene snapshot/delta | Protocol types added, server sends valid scene updates |
| BL-003 | P0 | Experience | todo | M2 | Render scene grid in CLI with player/NPC symbols | Client shows map updates for movement in multiplayer test |
| BL-004 | P0 | Platform | todo | M2 | Implement deterministic scene command queue | Stable ordering test passes with same input stream |
| BL-005 | P1 | Gameplay | todo | M3 | Implement encounter skeleton (start, participants, initiative, turns) | Encounter can start/end and cycles turns correctly |
| BL-006 | P1 | Platform | todo | M3 | Add turn timer + timeout fallback action | Timed-out actor auto-resolves with configured fallback |
| BL-007 | P1 | Experience | todo | M3 | Show turn tracker and active actor in client | Turn indicator updates for all participants |
| BL-008 | P1 | Gameplay | todo | M3 | Implement core combat actions (`move`, `attack`, `dodge`, `help`) | Action legality and resolution tests pass |
| BL-009 | P2 | Gameplay | todo | M4 | Implement scripted NPC policy interface (`NpcPolicyScripted`) | NPCs take valid turns without model calls |
| BL-010 | P2 | Platform | todo | M4 | Add combat replay event logging | Replay log reconstructs encounter timeline |
| BL-011 | P0 | Platform | todo | M2 | Implement campaign manifest and startup campaign selection flow | Server boots with explicit campaign selection and loads active campaign config |
| BL-012 | P0 | Platform | todo | M2 | Implement account + character persistence model with campaign lock metadata | Characters are account-owned, lock is enforced on join, and admin unlock events are audited |
| BL-013 | P1 | Platform | todo | M3 | Implement single-save campaign storage with rollback-safe snapshot strategy | Campaign progress persists and recovery test passes after simulated crash |
| BL-014 | P1 | Gameplay | todo | M3 | Define and implement staged quit/disconnect in encounter rules | Defensive-only fallback for configured rounds then limited AI behavior is deterministic and covered by integration tests |
| BL-015 | P1 | Gameplay | todo | M3 | Implement encounter pull radius/zone config policy | Pull radius/zone is active in V1 scene config with validated defaults by scene profile |
| BL-016 | P2 | Experience | todo | M4 | Design split-feed client mode (map/combat/chat panels) while preserving terminal-first mode | Optional multi-panel view works without breaking baseline CLI flow |
| BL-017 | P2 | Platform | todo | M4 | Add character export endpoint/command with standardized JSON schema | Exported JSON validates against schema and round-trip import validation passes |
| BL-018 | P1 | Platform | todo | M3 | Implement admin campaign-unlock tooling with guardrails | Unlock requires explicit reason, is permission-gated, and emits immutable audit log entries |

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
