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
