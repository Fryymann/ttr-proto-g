# Patterns

Track recurring workflows and known effective approaches.

## Pattern: IAN Idea Integration
- Trigger:
  - User says `review latest IAN entry`.
- Steps:
  - Read newest section in `IAN.md`.
  - Extract decisions, implementation impacts, open questions.
  - Update design docs + `.agents/backlog.md` + `.agents/risk-register.md`.
  - Return concise summary with unresolved choices.
- Validation:
  - Changes present in all three layers: design, backlog, risk.

## Pattern: PM Koad Sync Pass
- Trigger:
  - Significant planning/operational updates or OS-configuration edits.
- Steps:
  - Run freshness gate.
  - Confirm required-source paths resolve.
  - Mirror durable updates to `.agents` artifacts and `.koad/.agent-ops` logs.
  - Capture final session summary in `.koad/.agent-core/sessions/LOG.md`.
- Validation:
  - `status: FRESH` from standards sync check and aligned entries in both planning + ops logs.

## Pattern: Antigravity Sprint Packeting
- Trigger:
  - New sprint starts or sprint scope changes.
- Steps:
  - Update `docs/design/execution-sprint-plan.md` with lane map and validation gates.
  - Publish/update lane prompts in `ANTIGRAVITY_SPRINT_PROMPTS.md`.
  - Ensure `.agents/backlog.md` active focus matches sprint lane priorities.
  - Log process changes in Koad session/decision logs.
- Validation:
  - Antigravity agents can be launched directly from prompt ids with clear handoff requirements.
