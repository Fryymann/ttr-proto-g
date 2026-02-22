## Scope

- Task packet: `S4-P2`
- Backlog: `BL-018`
- Milestone/Sprint: `S4`
- Goal: implement admin campaign-unlock guardrails with explicit reason + permission checks and immutable audit entries for success/denied outcomes.

## Behavior Changes

### User-visible

- Added admin command path: `admin unlock <character> <campaign> --token <token> --reason <text>`.
- Non-admin actors now receive deterministic rejection for unlock attempts.
- Successful admin unlock returns confirmation with updated campaign binding.

### Internal

- Added a dedicated admin command parser/authorization module.
- Replaced prefix-based auth with policy-based authorization requiring both:
  - explicit account allowlist membership (`TTRPG_ADMIN_ACCOUNTS`)
  - matching admin unlock token (`TTRPG_ADMIN_UNLOCK_TOKEN`)
- Expanded persistence API to record immutable unlock-attempt audit events for denied and success outcomes.
- Wired unlock command execution to:
  - enforce permission checks before mutation
  - require admin token in command parsing
  - require reason text in command parsing
  - audit denied unauthorized attempts
  - audit denied character-not-found attempts
  - fail closed when denied-path audit persistence fails
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
- Verify authorization requires both allowlisted account and matching admin token.
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
- Evidence basis: new paths in token + allowlist authorization and unlock audit outcomes are covered by focused unit/integration tests in `main.rs`, `admin/mod.rs`, and `persistence/mod.rs`.

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

- Risk: authorization now depends on correctly configured env vars (`TTRPG_ADMIN_ACCOUNTS`, `TTRPG_ADMIN_UNLOCK_TOKEN`); missing config blocks all unlocks.
- Mitigation: fail-closed behavior is intentional for security; deployment docs should include required admin env setup.
- Risk: malformed unlock commands return usage errors without persisted invalid-request audit rows.
- Mitigation: authorization failures and character-not-found failures are audited and denied-path audit persistence failures now surface explicit errors.

## Deferred Follow-ups

- Introduce durable role/permission model instead of env allowlist + shared token.
- Add dedicated admin command observability counters for accepted/denied unlock events.
- Add integration test path for env-configured authorization matrix and startup config validation.

## PM Extraction Notes

- Acceptance checklist:
  - `PASS` - Unlock requires explicit token + reason + actor identity via `admin unlock <character> <campaign> --token <token> --reason <text>` and authenticated actor context (`crates/ttrpg-server/src/admin/mod.rs`, `crates/ttrpg-server/src/main.rs`).
  - `PASS` - Unauthorized attempts are deterministically rejected and audited (`tests::admin_unlock_denied_for_non_admin_and_attempt_is_audited`, audit outcome `denied_unauthorized` in `crates/ttrpg-server/src/persistence/mod.rs`).
  - `PASS` - Successful unlock emits immutable audit record with actor/reason metadata and binds `unlock_audit_ref` (`persistence::tests::admin_override_rebinds_lock_and_appends_audit_event`).
- Sample audit payload (success): `{"character_name_key":"target","target_campaign_id":"ashfall","reason":"support-ticket-91","outcome":"success",...}`
- Sample audit payload (denied): `{"character_name_key":"target","target_campaign_id":"ashfall","reason":"ticket-77","outcome":"denied_unauthorized",...}`
- Deferred-risk notes: authorization model is currently convention/env-list based pending dedicated admin role store.
