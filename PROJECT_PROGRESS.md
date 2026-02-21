# Project Progress Dashboard

Auto-generated snapshot aligned to roadmap and backlog.

- Generated (UTC): 2026-02-21 23:24:04Z
- Source files:
  - `.agents/backlog.md`
  - `docs/design/game-system-roadmap.md`
  - `docs/design/execution-sprint-plan.md`
  - `CODEX_ROLE_PROMPTS.md`

## Snapshot

- Active release branch: `v1`
- Total backlog items: `20`
- Done: `5`
- In progress: `1`
- Blocked: `0`
- Todo: `14`
- Open items remaining: `15`

## Roadmap Alignment

| Milestone | Backlog Items | Done | In Progress | Todo | Blocked | Completion | Status |
|---|---:|---:|---:|---:|---:|---:|---|
| M2 | 7 | 5 | 1 | 1 | 0 | 71% | In Progress |
| M3 | 9 | 0 | 0 | 9 | 0 | 0% | Queued |
| M4 | 4 | 0 | 0 | 4 | 0 | 0% | Queued |

## Active Focus Window

- Now: `BL-012`, `BL-003`
- Next: `BL-005`, `BL-006`, `BL-015`, `BL-007`

## Active Task Packet Queue

| Packet ID | Role | Backlog IDs | Status | Dependency | Suggested Branch |
|---|---|---|---|---|---|
| S1-P2 | Platform | `BL-012` | Done (merged to `v1`) | none | lane/Platform/s1-p2-persistence-lock-audit |
| S2-P1 | Platform | `BL-004`, `BL-002` | Done (merged to `v1`) | `S1-P2` merged to `v1` | lane/Platform/s2-p1-scene-queue-protocol |
| S2-E1 | Experience | `BL-003` | Active Next (dispatch now) | `S2-P1` merged to `v1` | lane/Experience/s2-e1-cli-scene-render |

## In-Progress / Blocked Items

| Backlog ID | Team | State | Milestone | Task |
|---|---|---|---|---|
| BL-012 | Platform | in_progress | M2 | Implement account + character persistence model with account-authenticated login and campaign lock metadata |

## Recent Sprint Status Notes

- 2026-02-21: Active release branch updated to `v1`; coder lanes now branch from `v1` and PR back into `v1`.
- 2026-02-21: Active Codex task packets published in `CODEX_ROLE_PROMPTS.md` for immediate queue (`S1-P2`, `S2-P1`, `S2-E1`) with dependency order on `v1`.
- 2026-02-21: `BL-012` scope clarified: login must authenticate account first; character operations are account-scoped rather than name-derived.
- 2026-02-21: Added `koad-os` scope separation gate; Koad/agent support files now route through `koad-os` and are blocked on non-`koad-os` PRs by `validate-koad-os-scope`.
- 2026-02-21: Saveup protocol updated for multi-role operation with role/context ledger metadata and role-boundary mirror rules (`STD-011`).
- 2026-02-21: Added root `PROJECT_PROGRESS.md` dashboard sync workflow with `koad progress-sync` and default `saveup` refresh path (`STD-012`).
- 2026-02-21: Post-merge queue advance set `S2-P1` as active next dispatch (Platform), with `S2-E1` held until `S2-P1` merges to `v1`.
- 2026-02-21: `S2-P1` merged to `v1` with deterministic queue + scene protocol evidence; queue advanced to `S2-E1` as active next dispatch (Experience).

## Update Command

```bash
bash .koad/scripts/koad progress-sync
```
