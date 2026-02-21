# SAVEUP Protocol

## Trigger
When user says `saveup`, run this protocol.

## Objective
Preserve continuity with two outputs:
1. Work record
2. Learning record
3. Role-scoped operating record

## Preconditions
- Role must already be resolved via `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.
- If role is unresolved, run role routing before continuing.
- Record one role label for this call: `Koad (PM)` | `Gameplay` | `Platform` | `Experience`.
- Capture a `context_ref` for the call (`task packet id`, `lane branch`, or `n/a`).

## Steps
1. Call Registration
- Create call id: `SAVEUP-YYYYMMDD-HHMMSSZ` (UTC).
- Append row to `.koad/.agent-core/sessions/SAVEUP_CALLS.md` with:
  - `role`
  - `context_ref`
  - `result=partial`

2. Duplicate Pre-Check
- Review:
  - `.koad/.agent-core/sessions/SAVEUP_CALLS.md`
  - `.koad/.agent-core/memory/LEARNINGS.md`
  - `.koad/.agent-core/memory/PATTERNS.md`
  - `.koad/.agent-core/memory/FACTS_LEDGER.md`
- Skip duplicate learnings/facts.
- Reuse existing role-tagged learnings/patterns when the same lesson already exists.

3. Session Summary
- Append concise entry to `.koad/.agent-core/sessions/LOG.md` with:
  - role
  - context reference
  - objective, actions, artifacts, and risks

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
- Role emphasis:
  - `Koad (PM)`: planning quality, delegation quality, governance quality.
  - `Gameplay`: rules clarity, deterministic behavior, content/system semantics.
  - `Platform`: reliability, determinism, persistence/network safety.
  - `Experience`: UX clarity, command ergonomics, playtest/test quality.

5. Update Recurring Knowledge
- Update `.koad/.agent-core/memory/PATTERNS.md` for repeated workflows/failure modes.
- Update `.koad/.agent-core/memory/USER_PREFERENCES.md` for clarified user style.
- Update `.koad/.agent-core/memory/FACTS_LEDGER.md` for confirmed durable facts.
- Only update `USER_PREFERENCES.md` when a new user preference is explicitly confirmed.

6. Operational Mirror
- Mirror durable process/decision updates in `.koad/.agent-ops/sessions/SESSION_LOG.md` and `.koad/.agent-ops/decisions/DECISION_LOG.md` when relevant.
- For PM scope, also mirror applicable changes in:
  - `.agents/backlog.md`
  - `.agents/risk-register.md`
  - `.agents/teams.md` (if ownership changed)
- For non-PM team-role scope:
  - Do not reprioritize `.agents/backlog.md` or `.agents/risk-register.md` directly unless user/Koad explicitly asks.
  - Record proposed backlog/risk updates in ops logs for PM review.

7. Finalize Call
- Update saveup row with:
  - `role`
  - `context_ref`
  - `result` (`completed|partial|blocked`)
  - `new_learnings`
  - `duplicates_skipped`
  - notes/blockers

8. Receipt
- Return:
  - call id
  - role
  - context_ref
  - files updated
  - key lessons
  - duplicates skipped
  - open risks

## Quality Bar
- No vague lessons
- No duplicate memory entries
- Explicit uncertainty where needed
- Never store secrets
- Role + context metadata must be present in the saveup ledger entry.
