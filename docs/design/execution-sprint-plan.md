# Execution Sprint Plan (M2-M3)

Status: Active (living)  
Last updated: 2026-02-22  
Owner: Project Manager (Koad)

## Purpose

Turn roadmap direction into immediate execution slices with:

- Clear ownership
- Concrete file/module targets
- Validation gates
- Handoff-ready acceptance criteria

Primary references:

- `docs/design/game-system-roadmap.md`
- `.agents/backlog.md`
- `.agents/risk-register.md`
- `CODEX_ROLE_PROMPTS.md`

## Current Standards and Risk Posture

- Applicable standards: `STD-001`, `STD-002`, `STD-003`, `STD-004`, `STD-005`, `STD-006`, `STD-007`, `STD-008`, `STD-009`, `STD-010`, `STD-011`, `STD-012`, `STD-013`, `STD-014`
- Planning risk level: `Medium` (cross-system architecture and persistence decisions)

## Focus Horizon

1. Sprint S1: Core data contracts + campaign boot pipeline
2. Sprint S2: Scene runtime and deterministic queue + protocol expansion
3. Sprint S3: Party-based encounter skeleton + turn flow
4. Sprint S4: Reliability hardening + DM-agent advisory boundary scaffolding

## Codex Multi-Instance Execution Model

Koad (PM) owns sprint orchestration and publishes starter prompts in `CODEX_ROLE_PROMPTS.md`.

- Antigravity usage is paused for now.
- Three additional Codex instances represent team roles: Gameplay, Platform, Experience.
- One active task lane maps to one dedicated git worktree and one lane branch (`lane/<ROLE>/<task-slug>`).
- Active release base branch is `v1`; lane branches are cut from `v1` and PRs target `v1`.
- Lanes in the same sprint can run in parallel when file overlap is low.
- Shared-file hotspots (protocol core, main server runtime) should be sequenced or split by explicit boundaries.
- Every lane flows through a PR and remains open until both review gates pass.
- Default PR shape is one PR per lane; docs/chore lanes may be batched only when still reviewable as one PR.
- Cross-lane wiring is merged via an integration PR after dependency lanes complete.
- Every lane must return:
  - changed files
  - tests/verification evidence
  - known risks/deferred items
  - role instance + task packet id
  - branch/worktree evidence and PR metadata (URL, base/head, latest commit, dependency order)
  - review state for Koad git review and Ian review

### Suggested parallel role lanes by sprint

- Sprint S1: Platform + Gameplay
- Sprint S2: Platform + Experience
- Sprint S3: Gameplay + Platform + Experience
- Sprint S4: Platform + Gameplay

## Sprint S1: Contracts and Campaign Runtime

Backlog alignment: `BL-011`, `BL-012`, `BL-019`, `BL-001`

### Platform Team

- Deliver campaign manifest and active campaign selection.
- Introduce persistence model for accounts, characters, account-authenticated login/session flow, and campaign-lock metadata.

Target files/modules:

- `crates/ttrpg-server/src/main.rs` (temporary integration points)
- `crates/ttrpg-server/src/campaign/mod.rs` (new)
- `crates/ttrpg-server/src/persistence/mod.rs` (new)
- `crates/ttrpg-server/src/account/mod.rs` (new)

### Gameplay Team

- Define scene data contract and occupancy primitives.
- Publish SRD compliance checklist artifact for V1 mechanics implementation.

Target files/modules:

- `crates/ttrpg-server/src/scene/mod.rs` (new)
- `docs/design/mechanics-decision-matrix-v1.md` (SRD source mapping pass)
- `docs/design/srd-compliance-checklist-v1.md` (new)

### Validation gates

- Server boots only with explicit campaign selection.
- Account-authenticated login is required before character selection/join, and join enforces campaign lock policy.
- Scene model compiles with occupancy rule tests.
- SRD checklist exists with mapped mechanics rows.

## Sprint S2: Scene Runtime and Protocol

Backlog alignment: `BL-002`, `BL-003`, `BL-004`

### Platform Team

- Implement deterministic scene command queue.
- Add scene snapshot/delta protocol messages.

Target files/modules:

- `crates/ttrpg-protocol/src/lib.rs`
- `crates/ttrpg-server/src/scene/runtime.rs` (new)
- `crates/ttrpg-server/src/scene/queue.rs` (new)

### Experience Team

- Render scene grid in CLI.
- Surface movement and occupancy feedback clearly.

Target files/modules:

- `crates/ttrpg-client-cli/src/main.rs`
- `crates/ttrpg-client-cli/src/render_scene.rs` (new)

### Validation gates

- Two clients receive identical scene deltas for same command stream.
- Deterministic ordering test passes for replayed command set.
- CLI map renders player + NPC symbols with update consistency.

## Sprint S3: Encounter Skeleton and Turn Loop

Backlog alignment: `BL-005`, `BL-006`, `BL-007`, `BL-015`

### Gameplay Team

- Implement party-based participant capture and encounter state machine.
- Define initiative tie-breakers and encounter lifecycle transitions.

Target files/modules:

- `crates/ttrpg-server/src/encounter/mod.rs` (new)
- `crates/ttrpg-server/src/encounter/state.rs` (new)
- `crates/ttrpg-server/src/party/mod.rs` (new)

### Platform Team

- Add turn timer and timeout fallback infrastructure.

Target files/modules:

- `crates/ttrpg-server/src/encounter/timer.rs` (new)
- `crates/ttrpg-server/src/encounter/fallback.rs` (new)

### Experience Team

- Add turn tracker and active actor indicator.

Target files/modules:

- `crates/ttrpg-client-cli/src/turn_ui.rs` (new)

### Validation gates

- Encounter includes only triggering party members + relevant NPCs in V1.
- Turn ownership rejects out-of-turn actions deterministically.
- Timeout applies configured fallback and advances turn safely.

## Sprint S4: Reliability and Advisory Boundaries

Backlog alignment: `BL-013`, `BL-014`, `BL-018`, `BL-020`

### Platform Team

- Implement rollback-safe snapshots and restore drills.
- Add admin campaign unlock tooling with immutable audit events.

Target files/modules:

- `crates/ttrpg-server/src/persistence/snapshot.rs` (new)
- `crates/ttrpg-server/src/admin/mod.rs` (new)
- `crates/ttrpg-server/src/audit/mod.rs` (new)

### Gameplay Team

- Implement staged disconnect fallback policy.
- Define and enforce DM-agent advisory acceptance boundaries.

Target files/modules:

- `crates/ttrpg-server/src/encounter/disconnect.rs` (new)
- `crates/ttrpg-server/src/npc/advisory.rs` (new)
- `crates/ttrpg-server/src/npc/policy_scripted.rs` (new)

### Validation gates

- Recovery test passes after simulated crash.
- Disconnect policy behaves deterministically over defensive rounds + AI-limited phase.
- Advisory output cannot mutate authoritative state without script/rules acceptance.

## V2 Concept Intake Parking Lot (Not Scheduled in M2-M3)

Source intake: `IAN.md` latest entry (`2026-02-22 - 11:42pm`).

- Keep these items out of active `S3`/`S4` lane scope unless Koad explicitly re-prioritizes.
- Treat these as V2 spec starters and backlog-anchored design tasks.

| Backlog ID | Theme | Initial spec direction |
| --- | --- | --- |
| `BL-021` | Chained movement commands | Support vector-segment movement (`go e 3, s 3`) with deterministic segment-by-segment cost and encounter budget gating. |
| `BL-022` | Terrain movement costs | Add terrain tags + temporary spell-driven terrain modifiers with explicit revert rules when effects expire. |
| `BL-023` | Auto-walk | Provide long-route travel queueing, interruption handling, and safe cancel/resume behavior. |
| `BL-024` | Spell targeting grammar | Support actor-target and coordinate-target spell syntax with shared legality and AOE center resolution. |
| `BL-025` | NPC promotion lifecycle | Add tiered NPC detail/promotion path so low-interaction NPCs can be upgraded into story-critical actors. |
| `BL-026` | Relationship/reputation progression | Track PC relationship/faction standings and expose gating hooks for quests, shops, and social outcomes. |

## Cross-Sprint Definition of Done

For each backlog item completed:

1. Acceptance criteria in `.agents/backlog.md` are met.
2. Verification evidence (test or manual) is recorded in PR/handoff notes.
3. Koad git review and Ian review are both approved on the lane PR.
4. Lane PR is merged to the target branch.
5. Relevant design docs are updated if behavior/contract changed.
6. `.agents/risk-register.md` is updated for new or retired risks.

## Weekly PM Update Protocol

1. Re-rank `Now`/`Next` focus in `.agents/backlog.md`.
2. Update this file's `Last updated` and sprint status notes.
3. Log durable changes in `.koad/.agent-ops/sessions/SESSION_LOG.md`.
4. If scope/authority decisions changed, update `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## Sprint Status Notes

- 2026-02-21: Initial execution plan created. S1 planning ready to begin.
- 2026-02-21: `S1-P1` completed. Campaign manifest selection flow and account/character campaign-lock scaffolding landed (`BL-011` done; `BL-012` remains in progress pending durable persistence and audited admin unlock integration).
- 2026-02-21: `S1-G1` completed. Scene contract/occupancy module and SRD compliance checklist + source mapping pass landed (`BL-001` and `BL-019` done).
- 2026-02-21: Execution model pivoted from Antigravity to Codex multi-instance role lanes (`Koad PM` + Gameplay/Platform/Experience agents). Antigravity prompts paused.
- 2026-02-21: Workflow updated to PR-required merge gating with dual approval lanes (Koad git review + Ian review) before task closure.
- 2026-02-21: Active release branch updated to `v1`; coder lanes now branch from `v1` and PR back into `v1`.
- 2026-02-21: Active Codex task packets published in `CODEX_ROLE_PROMPTS.md` for immediate queue (`S1-P2`, `S2-P1`, `S2-E1`) with dependency order on `v1`.
- 2026-02-21: `BL-012` scope clarified: login must authenticate account first; character operations are account-scoped rather than name-derived.
- 2026-02-21: Added `koad-os` scope separation gate; Koad/agent support files now route through `koad-os` and are blocked on non-`koad-os` PRs by `validate-koad-os-scope`.
- 2026-02-21: Saveup protocol updated for multi-role operation with role/context ledger metadata and role-boundary mirror rules (`STD-011`).
- 2026-02-21: Added root `PROJECT_PROGRESS.md` dashboard sync workflow with `koad progress-sync` and default `saveup` refresh path (`STD-012`).
- 2026-02-21: Post-merge queue advance set `S2-P1` as active next dispatch (Platform), with `S2-E1` held until `S2-P1` merges to `v1`.
- 2026-02-21: `S2-P1` merged to `v1` with deterministic queue + scene protocol evidence; queue advanced to `S2-E1` as active next dispatch (Experience).
- 2026-02-22: `S2-E1` merged to `v1`; M2 scene-render baseline landed and queue advanced to S3 entry (`S3-G1` active next).
- 2026-02-22: Saveup process updated for lane-isolated developer journaling to reduce cross-lane merge conflicts (`STD-013`).
- 2026-02-22: `S3-G1` merged to `v1` (PR #13); `BL-005` + `BL-015` marked done and queue advanced to `S3-P1` as active next dispatch (Platform).
- 2026-02-22: `S3-P1` merged to `v1` (PR #17); `BL-006` marked done and queue advanced to `S3-E1` as active next dispatch (Experience).
- 2026-02-22: `S3-E1` merged to `v1` (PR #32); `BL-007` marked done and queue advanced to `S4-P1` as active next dispatch (Platform, `BL-013`).
