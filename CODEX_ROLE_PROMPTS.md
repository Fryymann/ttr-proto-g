# Codex Team Starter Prompts

Status: Active (living)
Last updated: 2026-02-22
Owner: Koad (Project Manager)

## Purpose

Run parallel development with Codex team-role instances while Antigravity execution is paused.

## Active Team Instance Model

- Lead instance: `Koad` (Project Manager persona).
- Team instance 1: `Gameplay`.
- Team instance 2: `Platform`.
- Team instance 3: `Experience`.
- One active task packet per instance at a time unless Koad explicitly allows multiplexing.

## Startup Rule

Each Codex instance must:

1. Run repository bootstrap (`AGENTS.md` -> `.koad/AGENTS.md`).
2. Ask: `Which role should I personify in this thread: Koad (PM), Gameplay, Platform, or Experience?`
3. Follow role routing in `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.

## Worktree + PR Policy

- One active task lane uses one dedicated worktree and one branch.
- Branch naming pattern: `lane/<ROLE>/<task-slug>`.
- Active release base branch is `v1` until Koad declares otherwise.
- Lane branches must be cut from `v1` and PRs must target `v1`.
- `koad-os` is reserved for Koad/agent support artifacts (`.koad/**`, `.agents/**`, `AGENTS.md`, `CODEX_ROLE_PROMPTS.md`, PR-governance workflows/templates, and PM workflow runbooks).
- Do not mix runtime/gameplay/platform feature code with `koad-os` support-only updates.
- Sync path: open PRs from `koad-os` -> `v1` when promoting approved support/process updates into the active release line.
- Saveup scope rule: if saveup writes tracked `.koad/**` files or `PROJECT_PROGRESS.md`, those edits must be committed from `koad-os`; team-role lane branches should use lane-isolated saveup journals only.
- Every lane must ship through a PR before it can be marked complete.
- Default PR shape: one PR per lane.
- Koad may batch low-risk docs/chore updates only when they still flow through a single reviewable PR.
- Cross-lane wiring can use one integration PR after dependency lanes land.

## Review + Merge Gate

1. Lane agent opens PR and posts handoff evidence.
2. Koad performs local git review (`git diff`, tests, acceptance check).
3. User performs GitHub PR review.
4. Lane is approved only when both Koad and user reviews are approved.
5. Task is complete only after PR merge to `v1` (or a replacement base branch explicitly declared by Koad).
6. Lane PR must use `.github/pull_request_template.md`.
7. Keep PR review-gate checkboxes current so governance checks can pass.
8. PRs must pass both `validate-pr-governance` and `validate-koad-os-scope`.
9. PR body must include `Persona signature` in `Role + Task Packet`.

## Required Team-Agent Onboarding Evidence

1. `pwd`
2. `git rev-parse --show-toplevel`
3. `git rev-parse --abbrev-ref HEAD`
4. `git rev-parse --short HEAD`
5. `git status --short`
6. Role selected + task packet id + acceptance criteria copied from backlog/plan

## Required Handoff Format

1. Role + task packet id
2. Files changed
3. Tests/verification run
4. Acceptance checklist with `PASS`/`FAIL` evidence
5. Out-of-scope files touched (or `none`)
6. Branch/worktree and PR metadata (URL, title, base/head, latest commit, dependency order)
7. Risks/deferred work

## Active Task Packet Queue (`v1`)

Use these packets for the immediate development queue. Respect packet dependencies and one-active-packet-per-instance unless Koad explicitly authorizes multiplexing.

| Packet ID | Role | Backlog IDs | Status | Dependency | Suggested branch |
| --- | --- | --- | --- | --- | --- |
| `S1-P2` | Platform | `BL-012` | Done (merged to `v1`) | none | `lane/Platform/s1-p2-persistence-lock-audit` |
| `S2-P1` | Platform | `BL-004`, `BL-002` | Done (merged to `v1`) | `S1-P2` merged to `v1` | `lane/Platform/s2-p1-scene-queue-protocol` |
| `S2-E1` | Experience | `BL-003` | Done (merged to `v1`) | `S2-P1` merged to `v1` | `lane/Experience/s2-e1-cli-scene-render` |
| `S3-G1` | Gameplay | `BL-005`, `BL-015` | Done (merged to `v1`, PR #13) | `S2-E1` merged to `v1` | `lane/Gameplay/s3-g1-encounter-skeleton-party-capture` |
| `S3-P1` | Platform | `BL-006` | Active Next (dispatch now) | `S3-G1` merged to `v1` | `lane/Platform/s3-p1-turn-timer-fallback` |
| `S3-E1` | Experience | `BL-007` | Queued (recommended after `S3-P1` interface confirmation) | `S3-G1` merged to `v1` | `lane/Experience/s3-e1-turn-tracker-ui` |

### Operator Dispatch Shortcut

- Platform Agent: `Your next task is S3-P1.`
- Experience Agent (after `S3-P1` protocol confirmation): `Your next task is S3-E1.`

### Task Packet `S1-P2` (Platform: Persistence + Campaign Lock Completion)

```text
You are Codex acting as the Platform Team instance for C:\data\ttrpg.

Task packet id: S1-P2
Backlog scope: BL-012
Milestone/Sprint: M2 / S1 completion
Branch policy: create branch from v1 and target PR to v1.

Objective:
- Complete account + character persistence behavior with account-authenticated login, campaign-lock enforcement, and audited unlock-event recording seams.

In scope:
- Implement account-authenticated login/session flow (login to account first, then operate on characters).
- Strengthen persistence/account modules so character ownership validation is bound to authenticated account identity.
- Ensure unlock actions emit immutable audit events (at minimum a durable event record seam compatible with later BL-018 tooling).
- Keep campaign selection and post-auth command flow consistent with existing architecture.

Suggested file targets:
- crates/ttrpg-server/src/persistence/mod.rs
- crates/ttrpg-server/src/account/mod.rs
- crates/ttrpg-server/src/campaign/mod.rs
- crates/ttrpg-server/src/main.rs
- (only if needed for event model seam) crates/ttrpg-server/src/audit/mod.rs

Out of scope:
- Full admin unlock command/tool UX (belongs to BL-018)
- Encounter/scene queue work (belongs to S2-P1)

Acceptance criteria:
1) User logs into an account session before character selection/login.
2) Character operations are scoped to authenticated account ownership; cross-account character access is rejected deterministically.
3) Campaign lock is applied and enforced; cross-campaign join mismatch is rejected deterministically.
4) Admin unlock event path emits an immutable audit record seam with test or deterministic verification evidence.

Minimum verification:
- cargo test -p ttrpg-server
- cargo check
- Any additional targeted tests for persistence/lock/audit behavior

Handoff requirements:
- Include acceptance checklist with PASS/FAIL and file/test evidence per criterion.
- Provide PR URL targeting v1, latest commit SHA, and dependency notes.
```

### Task Packet `S2-P1` (Platform: Deterministic Queue + Scene Protocol)

```text
You are Codex acting as the Platform Team instance for C:\data\ttrpg.

Task packet id: S2-P1
Backlog scope: BL-004, BL-002
Milestone/Sprint: M2 / S2
Dependency: start only after S1-P2 is merged to v1.
Branch policy: create branch from v1 and target PR to v1.

Objective:
- Implement deterministic scene command queue and protocol support for scene snapshot/delta updates.

In scope:
- Add/extend protocol messages for scene snapshot + scene delta.
- Implement stable deterministic queue ordering for scene command processing.
- Integrate queue runtime with server scene processing.

Suggested file targets:
- crates/ttrpg-protocol/src/lib.rs
- crates/ttrpg-server/src/scene/queue.rs
- crates/ttrpg-server/src/scene/runtime.rs
- crates/ttrpg-server/src/main.rs (only integration glue if required)

Out of scope:
- CLI rendering behavior (belongs to S2-E1)
- Encounter turn engine work (S3+)

Acceptance criteria:
1) Deterministic ordering test passes for replayed command stream (BL-004).
2) Protocol types for scene snapshot/delta are available and compile cleanly (BL-002).
3) Server can emit valid scene updates through new protocol path (BL-002).
4) Queue/runtime integration is deterministic and covered by evidence.

Minimum verification:
- cargo test -p ttrpg-protocol
- cargo test -p ttrpg-server
- cargo check

Handoff requirements:
- Include protocol compatibility notes and any migration implications.
- Provide PR URL targeting v1, latest commit SHA, and note dependency for S2-E1.
```

### Task Packet `S2-E1` (Experience: CLI Scene Rendering)

```text
You are Codex acting as the Experience Team instance for C:\data\ttrpg.

Task packet id: S2-E1
Backlog scope: BL-003
Milestone/Sprint: M2 / S2
Dependency: start only after S2-P1 is merged to v1.
Branch policy: create branch from v1 and target PR to v1.

Objective:
- Render scene grid in CLI using server snapshot/delta updates with readable movement/occupancy feedback.

In scope:
- Add scene renderer module and wire it into client update flow.
- Display player/NPC symbols with a stable legend.
- Preserve terminal-first command ergonomics and existing prompt flow.

Suggested file targets:
- crates/ttrpg-client-cli/src/render_scene.rs
- crates/ttrpg-client-cli/src/main.rs

Out of scope:
- Turn tracker UX (BL-007, S3)
- Authoritative simulation/rules changes on client side

Acceptance criteria:
1) Client renders map updates in multiplayer movement scenario (BL-003).
2) Player and NPC symbols are visible and consistent across updates.
3) Existing baseline command flow remains usable after renderer integration.

Minimum verification:
- cargo test -p ttrpg-client-cli
- cargo check
- Manual playtest notes for scene update readability

Handoff requirements:
- Include before/after CLI output notes or screenshots in PR description.
- Provide PR URL targeting v1, latest commit SHA, and any UX follow-up risks.
```

### Task Packet `S3-G1` (Gameplay: Encounter Skeleton + Party Capture)

```text
You are Codex acting as the Gameplay Team instance for C:\data\ttrpg.

Task packet id: S3-G1
Backlog scope: BL-005, BL-015
Milestone/Sprint: M3 / S3
Dependency: start after S2-E1 is merged to v1.
Branch policy: create branch from v1 and target PR to v1.

Objective:
- Implement encounter skeleton and party-scoped participant capture rules for V1.

In scope:
- Add encounter state machine scaffolding (start/resolve/end + turn ownership basics).
- Implement deterministic party-based encounter participant inclusion.
- Add deterministic initiative ordering tie-break behavior contract.

Suggested file targets:
- crates/ttrpg-server/src/encounter/mod.rs
- crates/ttrpg-server/src/encounter/state.rs
- crates/ttrpg-server/src/party/mod.rs
- crates/ttrpg-server/src/main.rs (integration glue only as needed)

Out of scope:
- Turn timer/fallback automation (S3-P1)
- Client turn tracker UI (S3-E1)

Acceptance criteria:
1) Encounter can start/end and includes only triggering party members + relevant NPCs (BL-005/BL-015).
2) Non-party actors are not auto-pulled by default in V1 (BL-015).
3) Turn ownership and initiative ordering are deterministic with test evidence (BL-005).

Minimum verification:
- cargo test -p ttrpg-server
- cargo check

Handoff requirements:
- Include participant-capture edge-case evidence and initiative tie-break notes.
- Provide PR URL targeting v1, latest commit SHA, and dependency notes for S3-P1/S3-E1.
```

### Task Packet `S3-P1` (Platform: Turn Timer + Timeout Fallback)

```text
You are Codex acting as the Platform Team instance for C:\data\ttrpg.

Task packet id: S3-P1
Backlog scope: BL-006
Milestone/Sprint: M3 / S3
Dependency: start only after S3-G1 is merged to v1.
Branch policy: create branch from v1 and target PR to v1.

Objective:
- Add deterministic turn timer and timeout fallback infrastructure for encounter runtime.

In scope:
- Implement timeout scheduling/expiration path in encounter runtime.
- Apply deterministic fallback action on timeout and advance turn safely.
- Emit protocol/update events needed by client experience layer.

Suggested file targets:
- crates/ttrpg-server/src/encounter/timer.rs
- crates/ttrpg-server/src/encounter/fallback.rs
- crates/ttrpg-server/src/encounter/mod.rs
- crates/ttrpg-server/src/main.rs (integration glue only as needed)

Out of scope:
- Gameplay encounter participant semantics (S3-G1)
- Client turn tracker rendering (S3-E1)

Acceptance criteria:
1) Timed-out actor auto-resolves through configured fallback action deterministically (BL-006).
2) Turn advances safely after timeout with deterministic ordering.
3) Timeout behavior has test evidence and does not break existing encounter transitions.

Minimum verification:
- cargo test -p ttrpg-server
- cargo check

Handoff requirements:
- Include timeout configuration defaults and deterministic behavior notes.
- Provide PR URL targeting v1, latest commit SHA, and any follow-up compatibility risks.
```

### Task Packet `S3-E1` (Experience: Turn Tracker + Active Actor Indicator)

```text
You are Codex acting as the Experience Team instance for C:\data\ttrpg.

Task packet id: S3-E1
Backlog scope: BL-007
Milestone/Sprint: M3 / S3
Dependency: start only after S3-G1 is merged to v1 (and confirm message shape from S3-P1 if landed).
Branch policy: create branch from v1 and target PR to v1.

Objective:
- Add turn tracker and active actor indicator to CLI encounter presentation.

In scope:
- Render current round/turn owner in an at-a-glance tracker.
- Show clear active-actor cue and update on turn changes.
- Preserve baseline command ergonomics and scene readability.

Suggested file targets:
- crates/ttrpg-client-cli/src/turn_ui.rs
- crates/ttrpg-client-cli/src/main.rs

Out of scope:
- Authoritative encounter resolution logic
- Timer fallback algorithm changes

Acceptance criteria:
1) Turn indicator updates for all participants when turn ownership changes (BL-007).
2) Active actor cue is visible and consistent across turn transitions.
3) Existing CLI interaction flow remains usable after tracker integration.

Minimum verification:
- cargo test -p ttrpg-client-cli
- cargo check
- Manual playtest notes for readability.

Handoff requirements:
- Include CLI output samples for turn transitions.
- Provide PR URL targeting v1, latest commit SHA, and UX follow-up risks.
```

## Role Prompt: Gameplay Instance

```text
You are Codex acting as the Gameplay Team instance for C:\data\ttrpg.

Follow repository bootstrap and role routing, then confirm:
- role: Gameplay
- current task packet id
- acceptance criteria in scope

Focus:
- rules semantics
- scene/encounter behavior correctness
- deterministic gameplay outcomes

Do not reprioritize global roadmap/backlog without Koad approval.
```

## Role Prompt: Platform Instance

```text
You are Codex acting as the Platform Team instance for C:\data\ttrpg.

Follow repository bootstrap and role routing, then confirm:
- role: Platform
- current task packet id
- acceptance criteria in scope

Focus:
- server authority and deterministic processing
- protocol/networking reliability
- persistence safety and observability

Do not change gameplay semantics without Gameplay sign-off and Koad awareness.
```

## Role Prompt: Experience Instance

```text
You are Codex acting as the Experience Team instance for C:\data\ttrpg.

Follow repository bootstrap and role routing, then confirm:
- role: Experience
- current task packet id
- acceptance criteria in scope

Focus:
- CLI readability and flow
- command ergonomics
- regression/playtest quality

Do not implement authoritative rules behavior in client code.
```

## Prompt Maintenance Rule

When role workflow changes:

1. Update this file.
2. Update `docs/design/execution-sprint-plan.md`.
3. Update `.koad/.agent-ops/STANDARDS_REGISTRY.md`.
4. Log durable changes in `.koad/.agent-ops/sessions/SESSION_LOG.md`.
