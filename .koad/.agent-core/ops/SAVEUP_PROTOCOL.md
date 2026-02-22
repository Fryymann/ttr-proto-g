# SAVEUP Protocol

## Trigger
When user says `saveup`, run this protocol.

## Objective
Preserve continuity with two outputs:
1. Work record
2. Learning record
3. Role-scoped operating record
4. Conflict-resistant lane saveup record for team-role execution lanes

## Preconditions
- Role must already be resolved via `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.
- If role is unresolved, run role routing before continuing.
- Record one role label for this call: `Koad (PM)` | `Gameplay` | `Platform` | `Experience`.
- Capture a `context_ref` for the call (`task packet id`, `lane branch`, or `n/a`).
- Determine saveup mode before writes:
  - `global-ledger` (default for `Koad (PM)` and non-lane contexts)
  - `lane-isolated` (default for team-role calls where `context_ref` starts with `lane/`)
- Branch-scope guardrail:
  - Any saveup mode that writes tracked support artifacts (`.koad/.agent-core/sessions/SAVEUP_CALLS.md`, `.koad/.agent-core/sessions/LOG.md`, `PROJECT_PROGRESS.md`) must run on `koad-os`.
  - Team-role developer lanes on feature branches should use `lane-isolated` mode without progress sync.

## Steps
1. Call Registration
- Create call id: `SAVEUP-YYYYMMDD-HHMMSSZ` (UTC).
- If mode is `global-ledger` (on `koad-os`), append row to `.koad/.agent-core/sessions/SAVEUP_CALLS.md` with:
  - `role`
  - `context_ref`
  - `result=partial`
- If mode is `lane-isolated`, append entry to lane journal:
  - `.koad/.agent-core/sessions/lane-saveups/<context-ref>.md`

2. Duplicate Pre-Check
- Review:
  - `.koad/.agent-core/sessions/SAVEUP_CALLS.md`
  - `.koad/.agent-core/memory/LEARNINGS.md`
  - `.koad/.agent-core/memory/PATTERNS.md`
  - `.koad/.agent-core/memory/FACTS_LEDGER.md`
- Skip duplicate learnings/facts.
- Reuse existing role-tagged learnings/patterns when the same lesson already exists.

3. Session Summary
- If mode is `global-ledger`, append concise entry to `.koad/.agent-core/sessions/LOG.md` with:
  - role
  - context reference
  - objective, actions, artifacts, and risks
- If mode is `lane-isolated`, append the same fields to lane journal entry in:
  - `.koad/.agent-core/sessions/lane-saveups/<context-ref>.md`

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
- Merge-conflict guardrail:
  - Team-role developer lanes should prefer `lane-isolated` saveup mode to avoid touching shared saveup ledgers on feature branches.
  - Team-role developer lanes must not check in tracked `.koad/**` saveup artifacts from non-`koad-os` branches.
  - Lane journals under `.koad/.agent-core/sessions/lane-saveups/` are local continuity artifacts and should not be included in feature-lane PRs.
  - Reconcile lane saveup records into PM/global artifacts on `koad-os` during support sync.

7. Finalize Call
- If mode is `global-ledger`, update saveup row with:
  - `role`
  - `context_ref`
  - `result` (`completed|partial|blocked`)
  - `new_learnings`
  - `duplicates_skipped`
  - notes/blockers
- If mode is `lane-isolated`, finalize lane journal entry with:
  - `result`
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
- Role + context metadata must be present in the saveup record (`global ledger` or `lane journal`).
