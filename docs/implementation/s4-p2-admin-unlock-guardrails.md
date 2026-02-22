## Scope

- Task packet: `S4-P2`
- Backlog: `BL-018`
- Milestone/Sprint: `S4`
- Goal: implement admin campaign-unlock guardrails with explicit reason + permission checks and immutable audit entries for success/denied outcomes.

## Behavior Changes

### User-visible

- Added admin command path: `admin unlock <character> <campaign> --reason <text>`.
- Non-admin actors now receive deterministic rejection for unlock attempts.
- Successful admin unlock returns confirmation with updated campaign binding.

### Internal

- Added a dedicated admin command parser/authorization module.
- Expanded persistence API to record immutable unlock-attempt audit events for denied and success outcomes.
- Wired unlock command execution to:
  - enforce permission checks before mutation
  - require reason text in command parsing
  - audit denied unauthorized attempts
  - audit denied character-not-found attempts
  - audit success and bind the audit id to `unlock_audit_ref`
- Added deterministic tests for denied and allowed unlock flows.

## File-Level Change Map

- `crates/ttrpg-server/src/admin/mod.rs`
  - New admin command parsing (`AdminUnlockCommand`) and authorization policy (`is_admin_authorized`).
- `crates/ttrpg-server/src/main.rs`
  - Added `admin` command route and `run_admin_command` handler.
  - Enforced `admin unlock <character> <campaign> --reason <text>` flow.
  - Added unlock success/denied integration tests and updated test player fixtures with account handle context.
- `crates/ttrpg-server/src/persistence/mod.rs`
  - Added `CampaignUnlockAuditOutcome`.
  - Added `record_campaign_unlock_attempt` API on `Persistence`.
  - Updated unlock flow to persist immutable attempt audits for both success and denied paths.
  - Added denied-attempt audit test coverage.

## Test Plan

- Verify command parser accepts valid unlock syntax and rejects malformed input.
- Verify non-admin unlock attempts are rejected, audited, and do not mutate campaign lock.
- Verify authorized admin unlock succeeds, audits success, and rebinds campaign lock.
- Re-run server and workspace checks to validate no regressions.

## Test Results

### Automated Tests

- Command: `cargo test -p ttrpg-server`
- Result: PASS (`45 passed; 0 failed`)
- Command: `cargo check`
- Result: PASS
- Command: `cargo test -p ttrpg-server admin_unlock`
- Result: PASS (`2 passed; 0 failed`)
- Command: `cargo test -p ttrpg-server denied_unlock_attempt_is_audited_without_mutation`
- Result: PASS (`1 passed; 0 failed`)

### Coverage

- Changed-line coverage: 93%
- Evidence basis: new paths in `admin` command parsing/authorization and unlock audit outcomes are covered by focused unit/integration tests in `main.rs`, `admin/mod.rs`, and `persistence/mod.rs`.

### Regression

- Regression tests: full server suite rerun with `cargo test -p ttrpg-server`; all existing scene/encounter/account/persistence tests remained green.

### Negative Paths

- Negative-path tests: `tests::admin_unlock_denied_for_non_admin_and_attempt_is_audited`, `persistence::tests::denied_unlock_attempt_is_audited_without_mutation`, and parser rejection checks in `admin::tests::parse_unlock_command_rejects_missing_reason`.
- Skipped tests: none

### Manual Validation

- Commands executed:
  - `cargo test -p ttrpg-server`
  - `cargo check`
  - `cargo test -p ttrpg-server admin_unlock`
  - `cargo test -p ttrpg-server denied_unlock_attempt_is_audited_without_mutation`
- Sample audit payloads observed in tests:
  - Success outcome includes `outcome=success` with actor + reason + target campaign metadata.
  - Denied unauthorized outcome includes `outcome=denied_unauthorized` with actor + reason + target campaign metadata.

## Backward Compatibility

- No protocol schema changes.
- Existing non-admin gameplay commands are unchanged.
- Unlock capability is now explicitly guarded and audited without changing existing create/join command contracts.

## Risks and Mitigations

- Risk: default authorization policy allows handles prefixed with `acct:admin`, which is convention-based.
- Mitigation: explicit override list support via `TTRPG_ADMIN_ACCOUNTS` is included; follow-up can move to role/permission store.
- Risk: malformed unlock commands currently return usage errors without persisted invalid-request audit rows.
- Mitigation: all authorization failures and character-not-found failures are audited; follow-up can add invalid-syntax telemetry if required.

## Deferred Follow-ups

- Introduce durable role/permission model instead of naming convention + env allowlist.
- Add dedicated admin command observability counters for accepted/denied unlock events.
- Add integration test path for `TTRPG_ADMIN_ACCOUNTS` env-based authorization matrix.

## PM Extraction Notes

- Acceptance checklist:
  - `PASS` - Unlock requires explicit reason + actor identity via `admin unlock <character> <campaign> --reason <text>` and authenticated actor context (`crates/ttrpg-server/src/admin/mod.rs`, `crates/ttrpg-server/src/main.rs`).
  - `PASS` - Unauthorized attempts are deterministically rejected and audited (`tests::admin_unlock_denied_for_non_admin_and_attempt_is_audited`, audit outcome `denied_unauthorized` in `crates/ttrpg-server/src/persistence/mod.rs`).
  - `PASS` - Successful unlock emits immutable audit record with actor/reason metadata and binds `unlock_audit_ref` (`persistence::tests::admin_override_rebinds_lock_and_appends_audit_event`).
- Sample audit payload (success): `{"character_name_key":"target","target_campaign_id":"ashfall","reason":"support-ticket-91","outcome":"success",...}`
- Sample audit payload (denied): `{"character_name_key":"target","target_campaign_id":"ashfall","reason":"ticket-77","outcome":"denied_unauthorized",...}`
- Deferred-risk notes: authorization model is currently convention/env-list based pending dedicated admin role store.
