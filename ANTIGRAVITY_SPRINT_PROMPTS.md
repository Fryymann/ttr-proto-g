# Antigravity Sprint Starter Prompts

Status: Active (living)  
Last updated: 2026-02-21  
Owner: Koad (Project Manager)

## How To Use

1. Pick the prompt id(s) for the sprint lanes you want to run.
2. Start one Antigravity agent per prompt id.
3. Paste the full prompt block into that agent.
4. Collect handoff output from each lane and merge in review order.

Global constraints for all prompts:

- Keep server-authoritative and deterministic behavior.
- V1 mechanics/rules scope is SRD-only + original project-authored content.
- Encounter participation is party-based in V1.
- DM-agent behavior is advisory only unless explicitly accepted by script/rules authority.

Standard handoff format expected from each agent:

1. Prompt id used
2. Files changed
3. Tests/verification run
4. Risks or deferred work
5. Recommended next lane dependency updates

---

## Sprint S1

### Prompt `S1-P1` (Platform: Campaign + Persistence Bootstrap)

```text
You are Antigravity coder agent lane S1-P1 for /mnt/c/data/ttrpg.

Objective:
- Implement campaign manifest selection and persistence scaffolding for account/character/campaign-lock metadata.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S1)
- .agents/backlog.md items BL-011 and BL-012

Scope:
- Add campaign manifest loader and explicit startup campaign selection flow.
- Add persistence module scaffolding for accounts/characters/campaign lock metadata.
- Keep implementation incremental and compatible with current M1 server.

Suggested file targets:
- crates/ttrpg-server/src/main.rs
- crates/ttrpg-server/src/campaign/mod.rs (new)
- crates/ttrpg-server/src/persistence/mod.rs (new)
- crates/ttrpg-server/src/account/mod.rs (new)

Constraints:
- No broad refactor beyond required seams.
- Do not implement full DB layer yet; scaffold with clear interfaces.
- Preserve existing commands/login behavior unless directly impacted.

Validation:
- Server boots with explicit campaign selection path.
- Basic compile/tests pass for touched modules.
- Character join path includes campaign-lock validation seam.

Deliverable handoff:
1) Prompt id S1-P1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Suggested follow-up for S2-P1
```

### Prompt `S1-G1` (Gameplay: Scene Contract + SRD Checklist)

```text
You are Antigravity coder agent lane S1-G1 for /mnt/c/data/ttrpg.

Objective:
- Define scene contract primitives and produce SRD compliance checklist artifact.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S1)
- .agents/backlog.md items BL-001 and BL-019
- docs/design/mechanics-decision-matrix-v1.md

Scope:
- Add scene data model primitives and occupancy rule validation scaffolding.
- Create SRD compliance checklist document and map V1 mechanics rows.

Suggested file targets:
- crates/ttrpg-server/src/scene/mod.rs (new)
- docs/design/srd-compliance-checklist-v1.md (new)
- docs/design/mechanics-decision-matrix-v1.md

Constraints:
- Keep scope to V1 foundation, not full encounter mechanics.
- Ensure terminology matches party-based encounter and advisory DM-agent policy.

Validation:
- Scene primitives compile with occupancy unit tests.
- SRD checklist exists and maps each V1 mechanic row to SRD/original source status.

Deliverable handoff:
1) Prompt id S1-G1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Suggested follow-up for S2-E1/S3-G1
```

---

## Sprint S2

### Prompt `S2-P1` (Platform: Deterministic Queue + Scene Protocol)

```text
You are Antigravity coder agent lane S2-P1 for /mnt/c/data/ttrpg.

Objective:
- Implement deterministic scene command queue and scene snapshot/delta protocol extensions.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S2)
- .agents/backlog.md items BL-002 and BL-004

Scope:
- Extend protocol for SceneSnapshot/SceneDelta.
- Add deterministic queue ordering by stable keys.
- Integrate queue processing into scene runtime.

Suggested file targets:
- crates/ttrpg-protocol/src/lib.rs
- crates/ttrpg-server/src/scene/runtime.rs (new)
- crates/ttrpg-server/src/scene/queue.rs (new)

Constraints:
- Message schema should be additive and backward-compatible where possible.
- Deterministic ordering must be explicit and test-covered.

Validation:
- Determinism test with replayed input stream passes.
- Two clients receive consistent scene updates from same command stream.

Deliverable handoff:
1) Prompt id S2-P1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Merge notes for S2-E1
```

### Prompt `S2-E1` (Experience: CLI Scene Rendering)

```text
You are Antigravity coder agent lane S2-E1 for /mnt/c/data/ttrpg.

Objective:
- Add CLI scene rendering and clear movement/occupancy feedback.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S2)
- .agents/backlog.md item BL-003

Scope:
- Consume SceneSnapshot/SceneDelta messages.
- Render grid with stable symbol legend.
- Keep terminal-first UX and preserve existing command ergonomics.

Suggested file targets:
- crates/ttrpg-client-cli/src/main.rs
- crates/ttrpg-client-cli/src/render_scene.rs (new)

Constraints:
- Do not block existing chat/prompt flow.
- Keep output readable for multiplayer updates.

Validation:
- CLI updates scene correctly after movement deltas.
- Existing baseline commands still work.

Deliverable handoff:
1) Prompt id S2-E1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Suggested UX follow-up for S3-E1
```

---

## Sprint S3

### Prompt `S3-G1` (Gameplay: Party Encounter Skeleton)

```text
You are Antigravity coder agent lane S3-G1 for /mnt/c/data/ttrpg.

Objective:
- Implement party-based encounter participant capture and encounter state machine basics.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S3)
- .agents/backlog.md items BL-005 and BL-015

Scope:
- Add encounter model, lifecycle states, and party participant capture.
- Enforce V1 policy: non-party actors are not auto-pulled.

Suggested file targets:
- crates/ttrpg-server/src/encounter/mod.rs (new)
- crates/ttrpg-server/src/encounter/state.rs (new)
- crates/ttrpg-server/src/party/mod.rs (new)

Constraints:
- Keep deterministic tie-breaker rules explicit.
- Avoid implementing full action-resolution complexity in this lane.

Validation:
- Encounter start includes only triggering party + relevant NPCs.
- Lifecycle transitions are test-covered.

Deliverable handoff:
1) Prompt id S3-G1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Integration notes for S3-P1/S3-E1
```

### Prompt `S3-P1` (Platform: Turn Timer + Fallback)

```text
You are Antigravity coder agent lane S3-P1 for /mnt/c/data/ttrpg.

Objective:
- Add turn timing infrastructure and deterministic timeout fallback.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S3)
- .agents/backlog.md item BL-006

Scope:
- Implement timer hooks tied to encounter turn state.
- Apply deterministic fallback action on timeout.

Suggested file targets:
- crates/ttrpg-server/src/encounter/timer.rs (new)
- crates/ttrpg-server/src/encounter/fallback.rs (new)

Constraints:
- Keep fallback deterministic and traceable.
- Do not bypass active turn ownership rules.

Validation:
- Timeout advances turn safely and consistently.
- Out-of-turn actions remain rejected.

Deliverable handoff:
1) Prompt id S3-P1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Follow-up notes for S4-G1
```

### Prompt `S3-E1` (Experience: Turn Tracker UX)

```text
You are Antigravity coder agent lane S3-E1 for /mnt/c/data/ttrpg.

Objective:
- Render turn tracker and active actor indicators in CLI.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S3)
- .agents/backlog.md item BL-007

Scope:
- Add encounter-turn display components.
- Keep existing terminal command loop stable and readable.

Suggested file targets:
- crates/ttrpg-client-cli/src/turn_ui.rs (new)
- crates/ttrpg-client-cli/src/main.rs

Constraints:
- Avoid noisy redraw behavior that hides command prompts.

Validation:
- Active actor and turn progression are visible across clients.

Deliverable handoff:
1) Prompt id S3-E1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) UX refinements for later milestones
```

---

## Sprint S4

### Prompt `S4-P1` (Platform: Snapshot Safety + Admin Audit)

```text
You are Antigravity coder agent lane S4-P1 for /mnt/c/data/ttrpg.

Objective:
- Implement rollback-safe snapshot flow and audited admin unlock tooling.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S4)
- .agents/backlog.md items BL-013 and BL-018

Scope:
- Add snapshot module with recovery checks.
- Add admin unlock path with immutable audit events.

Suggested file targets:
- crates/ttrpg-server/src/persistence/snapshot.rs (new)
- crates/ttrpg-server/src/admin/mod.rs (new)
- crates/ttrpg-server/src/audit/mod.rs (new)

Constraints:
- Unlock actions must require explicit reason and identity attribution.

Validation:
- Recovery drill test passes after simulated crash.
- Admin unlock emits immutable audit record.

Deliverable handoff:
1) Prompt id S4-P1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Operational runbook gaps
```

### Prompt `S4-G1` (Gameplay: Disconnect Policy + DM Advisory Boundaries)

```text
You are Antigravity coder agent lane S4-G1 for /mnt/c/data/ttrpg.

Objective:
- Implement staged disconnect fallback and advisory-only DM-agent acceptance boundaries.

References:
- docs/design/game-system-roadmap.md
- docs/design/execution-sprint-plan.md (Sprint S4)
- .agents/backlog.md items BL-014 and BL-020

Scope:
- Add disconnect policy logic for defensive rounds then limited AI phase.
- Add advisory interface where DM-agent suggestions require script/rules acceptance.

Suggested file targets:
- crates/ttrpg-server/src/encounter/disconnect.rs (new)
- crates/ttrpg-server/src/npc/advisory.rs (new)
- crates/ttrpg-server/src/npc/policy_scripted.rs (new)

Constraints:
- DM-agent output cannot directly mutate authoritative state.
- Reconnect handoff must restore legal player control.

Validation:
- Disconnect transitions are deterministic and test-covered.
- Advisory output is logged and gated by authoritative acceptance path.

Deliverable handoff:
1) Prompt id S4-G1
2) Files changed
3) Verification evidence
4) Risks/deferred work
5) Remaining design decisions requiring PM input
```

---

## Prompt Maintenance Rule

When sprint scope changes:

1. Update affected prompt blocks in this file.
2. Update `docs/design/execution-sprint-plan.md` lane map if prompt ids change.
3. Record the process update in `.koad/.agent-ops/sessions/SESSION_LOG.md`.
