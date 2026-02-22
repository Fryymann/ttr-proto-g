# Project Progress Dashboard

Auto-generated snapshot aligned to roadmap and backlog.

- Generated (UTC): 2026-02-22 08:52:13Z
- Source files:
  - `.agents/backlog.md`
  - `docs/design/game-system-roadmap.md`
  - `docs/design/execution-sprint-plan.md`
  - `CODEX_ROLE_PROMPTS.md`

## Snapshot

- Active release branch: `v1`
- Total backlog items: `26`
- Done: `11`
- In progress: `1`
- Blocked: `0`
- Todo: `14`
- Open items remaining: `15`

## Roadmap Alignment

| Milestone | Backlog Items | Done | In Progress | Todo | Blocked | Completion | Status |
|---|---:|---:|---:|---:|---:|---:|---|
| M2 | 7 | 6 | 1 | 0 | 0 | 86% | In Progress |
| M3 | 9 | 5 | 0 | 4 | 0 | 56% | Queued |
| M4 | 4 | 0 | 0 | 4 | 0 | 0% | Queued |

## Active Focus Window

- Now: `BL-012`, `BL-018`, `BL-014`
- Next: `BL-008`, `BL-020`, `BL-010`

## Active Task Packet Queue

| Packet ID | Role | Backlog IDs | Status | Dependency | Suggested Branch |
|---|---|---|---|---|---|
| S1-P2 | Platform | `BL-012` | Done (merged to `v1`) | none | lane/Platform/s1-p2-persistence-lock-audit |
| S2-P1 | Platform | `BL-004`, `BL-002` | Done (merged to `v1`) | `S1-P2` merged to `v1` | lane/Platform/s2-p1-scene-queue-protocol |
| S2-E1 | Experience | `BL-003` | Done (merged to `v1`) | `S2-P1` merged to `v1` | lane/Experience/s2-e1-cli-scene-render |
| S3-G1 | Gameplay | `BL-005`, `BL-015` | Done (merged to `v1`, PR #13) | `S2-E1` merged to `v1` | lane/Gameplay/s3-g1-encounter-skeleton-party-capture |
| S3-P1 | Platform | `BL-006` | Done (merged to `v1`, PR #17) | `S3-G1` merged to `v1` | lane/Platform/s3-p1-turn-timer-fallback |
| S3-E1 | Experience | `BL-007` | Done (merged to `v1`, PR #32) | `S3-P1` merged to `v1` | lane/Experience/s3-e1-turn-tracker-ui |
| S4-P1 | Platform | `BL-013` | Done (merged to `v1`, PR #34) | `S3-E1` merged to `v1` | lane/Platform/s4-p1-single-save-snapshot-recovery |
| S4-P2 | Platform | `BL-018` | Active Next (dispatch now) | `S4-P1` merged to `v1` | lane/Platform/s4-p2-admin-unlock-guardrails |

## In-Progress / Blocked Items

| Backlog ID | Team | State | Milestone | Task |
|---|---|---|---|---|
| BL-012 | Platform | in_progress | M2 | Implement account + character persistence model with account-authenticated login and campaign lock metadata |

## Recent Sprint Status Notes

- 2026-02-21: Post-merge queue advance set `S2-P1` as active next dispatch (Platform), with `S2-E1` held until `S2-P1` merges to `v1`.
- 2026-02-21: `S2-P1` merged to `v1` with deterministic queue + scene protocol evidence; queue advanced to `S2-E1` as active next dispatch (Experience).
- 2026-02-22: `S2-E1` merged to `v1`; M2 scene-render baseline landed and queue advanced to S3 entry (`S3-G1` active next).
- 2026-02-22: Saveup process updated for lane-isolated developer journaling to reduce cross-lane merge conflicts (`STD-013`).
- 2026-02-22: `S3-G1` merged to `v1` (PR #13); `BL-005` + `BL-015` marked done and queue advanced to `S3-P1` as active next dispatch (Platform).
- 2026-02-22: `S3-P1` merged to `v1` (PR #17); `BL-006` marked done and queue advanced to `S3-E1` as active next dispatch (Experience).
- 2026-02-22: `S3-E1` merged to `v1` (PR #32); `BL-007` marked done and queue advanced to `S4-P1` as active next dispatch (Platform, `BL-013`).
- 2026-02-22: `S4-P1` merged to `v1` (PR #34); `BL-013` marked done and queue advanced to `S4-P2` as active next dispatch (Platform, `BL-018`).

## Update Command

```bash
bash .koad/scripts/koad progress-sync
```
