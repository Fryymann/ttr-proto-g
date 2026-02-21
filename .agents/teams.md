# Team Model (Consolidated)

Status: Active
Last updated: 2026-02-21

## Objective

Run development with 3 core teams plus Project Manager oversight, minimizing handoff overhead while preserving clear ownership and safe parallelism via lane-isolated git worktrees across Codex role instances and PR-mediated merge gates.

## Team Structure

## 1) Gameplay Team

Owns game mechanics and content behavior.

- Rules engine (dice, actions, conditions, turn logic)
- Scene simulation behavior (movement legality, occupancy, encounter capture rules)
- Combat and social encounter systems
- NPC behavior policy design (scripted and AI-intent contracts)
- Encounter/content balance and tuning

Primary role file:
- `roles/gameplay-lead.md`

## 2) Platform Team

Owns reliability and authoritative server infrastructure.

- Networking/session model and protocol reliability
- Server simulation orchestration and deterministic processing
- Persistence/data schemas/migrations
- Performance, observability, and production-readiness
- Deployment and runtime operations

Primary role file:
- `roles/platform-lead.md`

## 3) Experience Team

Owns client experience and quality verification.

- Terminal client UX and rendering
- Command ergonomics and onboarding flows
- Combat/readability presentation in client
- Test automation for play loops and regressions
- Manual playtest process and bug triage

Primary role file:
- `roles/experience-lead.md`

## PM Oversight

Koad (Project Manager) coordinates all three teams and owns roadmap, priorities, and release gates.

PM role file:
- `roles/project-manager.md`

## Ownership Boundaries

- Gameplay decides rules behavior.
- Platform decides server reliability and data safety constraints.
- Experience decides client UX and test quality strategy.
- PM resolves conflicts and sequencing across boundaries.

## Cross-Team Handoff Contract

Every handoff must include:

1. Scope and acceptance criteria
2. Changed files/modules
3. Test evidence or verification notes
4. Open risks or deferred work
5. Codex role instance + task packet reference (if agent-executed)
6. Lane worktree path + branch name
7. PR URL/id + base/head + latest commit + merge dependency notes
8. Review state for both gates:
   - Koad git review status
   - User GitHub review status
   - Merge status

## Task Closure Gate

- A lane task is not complete until its PR is merged.
- Required approvals: Koad git review + user GitHub review.
- If either review requests changes, task state returns to `in_progress` until updated and re-approved.

## Current Near-Term Focus

1. Gameplay: scene + encounter core mechanics
2. Platform: deterministic scene loop and protocol expansion
3. Experience: scene renderer and multiplayer playtest harness
