# SAVEUP Protocol

## Trigger
When user says `saveup`, run this protocol.

## Objective
Preserve continuity with two outputs:
1. Work record
2. Learning record

## Steps
1. Call Registration
- Create call id: `SAVEUP-YYYYMMDD-HHMMSSZ` (UTC).
- Append row to `.koad/.agent-core/sessions/SAVEUP_CALLS.md` with `result=partial`.

2. Duplicate Pre-Check
- Review:
  - `.koad/.agent-core/sessions/SAVEUP_CALLS.md`
  - `.koad/.agent-core/memory/LEARNINGS.md`
  - `.koad/.agent-core/memory/PATTERNS.md`
  - `.koad/.agent-core/memory/FACTS_LEDGER.md`
- Skip duplicate learnings/facts.

3. Session Summary
- Append concise entry to `.koad/.agent-core/sessions/LOG.md` with objective, actions, artifacts, and risks.

4. Learning Extraction
- Add durable lessons to `.koad/.agent-core/memory/LEARNINGS.md` under:
  - Technical
  - Process
  - Operational
  - Collaboration
- Every lesson requires:
  - Observation
  - Why it matters
  - Behavior update

5. Update Recurring Knowledge
- Update `.koad/.agent-core/memory/PATTERNS.md` for repeated workflows/failure modes.
- Update `.koad/.agent-core/memory/USER_PREFERENCES.md` for clarified user style.
- Update `.koad/.agent-core/memory/FACTS_LEDGER.md` for confirmed durable facts.

6. Operational Mirror
- Mirror durable process/decision updates in `.koad/.agent-ops/sessions/SESSION_LOG.md` and `.koad/.agent-ops/decisions/DECISION_LOG.md` when relevant.
- For PM scope, also mirror applicable changes in:
  - `.agents/backlog.md`
  - `.agents/risk-register.md`
  - `.agents/teams.md` (if ownership changed)

7. Finalize Call
- Update saveup row with:
  - `result` (`completed|partial|blocked`)
  - `new_learnings`
  - `duplicates_skipped`
  - notes/blockers

8. Receipt
- Return:
  - call id
  - files updated
  - key lessons
  - duplicates skipped
  - open risks

## Quality Bar
- No vague lessons
- No duplicate memory entries
- Explicit uncertainty where needed
- Never store secrets
