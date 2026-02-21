# Codex Team Starter Prompts

Status: Active (living)
Last updated: 2026-02-21
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
| `S1-P2` | Platform | `BL-012` | In review | none | `lane/Platform/s1-p2-persistence-lock-audit` |
| `S2-P1` | Platform | `BL-004`, `BL-002` | Queued | `S1-P2` merged to `v1` | `lane/Platform/s2-p1-scene-queue-protocol` |
| `S2-E1` | Experience | `BL-003` | Queued | `S2-P1` merged to `v1` | `lane/Experience/s2-e1-cli-scene-render` |

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
