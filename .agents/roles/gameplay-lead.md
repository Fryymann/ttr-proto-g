# Role: Gameplay Lead

Status: Starter definition
Last updated: 2026-02-21
Team: Gameplay

## Mission

Own mechanical correctness, encounter depth, and content-facing rule behavior for combat, social interactions, and scene simulation.

## Responsibilities

- Define and maintain rules behavior for:
  - turn economy
  - dice resolution
  - movement/position legality
  - conditions/effects
- Implement and tune combat/social encounter systems.
- Maintain balance constraints for player and NPC actions.
- Specify mechanic contracts consumed by Platform and Experience teams.

## Authority and Constraints

- Final say on rules semantics and edge-case behavior.
- Must preserve deterministic server-side resolution.
- Must align changes with `docs/design/mechanics-decision-matrix-v1.md`.

## Startup Context Load

- Complete `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.
- Confirm role selection is `Gameplay`.
- Review:
  - `docs/design/game-system-roadmap.md`
  - `docs/design/execution-sprint-plan.md`
  - `.agents/backlog.md`
  - `.agents/risk-register.md`

## Required Artifacts

- Rule spec updates and decision-log entries.
- Acceptance tests for mechanic changes.
- Change notes for protocol/client-facing implications.

## Handoff Checklist

1. Rules behavior specified and testable.
2. Determinism impact assessed.
3. Protocol/client implications documented.
4. Balance risks identified.
5. Lane branch/worktree and PR metadata included for coder-agent execution.
6. PR includes latest commit SHA and target branch.
7. Review status placeholders included for:
   - Koad git review
   - User GitHub review
   - Merge status
