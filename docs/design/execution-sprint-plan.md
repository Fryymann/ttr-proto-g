# Execution Sprint Plan (M2-M3)

Status: Active (living)  
Last updated: 2026-02-21  
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

- Applicable standards: `STD-001`, `STD-002`, `STD-003`, `STD-004`, `STD-005`, `STD-006`, `STD-007`, `STD-008`, `STD-009`
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
  - review state for Koad git review and user GitHub review

### Suggested parallel role lanes by sprint

- Sprint S1: Platform + Gameplay
- Sprint S2: Platform + Experience
- Sprint S3: Gameplay + Platform + Experience
- Sprint S4: Platform + Gameplay

## Sprint S1: Contracts and Campaign Runtime

Backlog alignment: `BL-011`, `BL-012`, `BL-019`, `BL-001`

### Platform Team

- Deliver campaign manifest and active campaign selection.
- Introduce persistence model for accounts, characters, and campaign-lock metadata.

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
- Character join enforces campaign lock policy.
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

## Cross-Sprint Definition of Done

For each backlog item completed:

1. Acceptance criteria in `.agents/backlog.md` are met.
2. Verification evidence (test or manual) is recorded in PR/handoff notes.
3. Koad git review and user GitHub review are both approved on the lane PR.
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
- 2026-02-21: Workflow updated to PR-required merge gating with dual approval lanes (Koad git review + user GitHub review) before task closure.
- 2026-02-21: Active release branch updated to `v1`; coder lanes now branch from `v1` and PR back into `v1`.
