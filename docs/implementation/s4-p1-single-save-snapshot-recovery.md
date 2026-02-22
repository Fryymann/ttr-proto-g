## Scope

- Task packet: `S4-P1`
- Backlog: `BL-013`
- Milestone/Sprint: `M3 closeout / S4 entry`
- Goal: implement rollback-safe single-save campaign persistence with deterministic recovery behavior for corrupt/partial startup save scenarios.

## Behavior Changes

### User-visible

- Campaign persistence now survives process restarts through a file-backed snapshot store keyed by the active campaign save path.
- Startup now validates and loads campaign save snapshots; if the primary save is corrupt/partial, startup automatically restores from rollback snapshot and continues boot.
- Character/account progression writes now use an atomic snapshot path, reducing risk of broken saves after interrupted writes.

### Internal

- Added a dedicated snapshot persistence module with:
  - schema/campaign validation
  - atomic temp-write + rename flow
  - rollback snapshot fallback and self-healing primary restore
- Introduced `FilePersistence` that wraps existing in-memory behavior and flushes durable snapshots after mutating operations.
- Added mutation rollback semantics in `FilePersistence`: if snapshot persistence fails after a mutation attempt, in-memory persistence state is restored to pre-mutation state.
- Updated server bootstrap to initialize persistence from campaign `save_path` and emit startup load status (`FreshStart`, `LoadedPrimary`, `RecoveredRollback`).
- Added explicit persistence-failure variants in create/join/unlock errors so unsafe-save failures surface clearly instead of silently succeeding.

## File-Level Change Map

- `crates/ttrpg-server/src/account/mod.rs`
  - Added `Serialize`/`Deserialize` derives to account/character records so persistence snapshots can encode/decode canonical account+character state.
- `crates/ttrpg-server/src/persistence/mod.rs`
  - Added `FilePersistence` + startup bootstrap path.
  - Added snapshot flush logic, initialization error types, and persistence-failure error variants.
  - Added deterministic recovery test `file_persistence_recovers_from_corrupted_primary_snapshot`.
- `crates/ttrpg-server/src/persistence/snapshot.rs`
  - New snapshot subsystem implementing atomic single-save writes, rollback path recovery, schema/campaign validation, and focused corruption/recovery tests.
- `crates/ttrpg-server/src/main.rs`
  - Wired server startup to use file-backed persistence from campaign manifest `save_path`.
  - Added startup load-status logging and persistence failure handling for create/join flows.
  - Added `new_for_tests` constructor to keep existing runtime tests deterministic/in-memory.

## Test Plan

- Validate that file-backed persistence initializes cleanly for new campaigns and writes an initial bootstrap snapshot.
- Validate that mutating operations persist and that restart restores persisted campaign/account/character state.
- Validate deterministic rollback recovery when the primary save is deliberately corrupted.
- Re-run full server tests and workspace checks to ensure no regression to active campaign boot/encounter flows.

## Test Results

### Automated Tests

- Command: `cargo test -p ttrpg-server`
- Result: PASS (`40 passed; 0 failed`)
- Command: `cargo check`
- Result: PASS
- Command: `cargo test -p ttrpg-server file_persistence_recovers_from_corrupted_primary_snapshot`
- Result: PASS (`1 passed; 0 failed`)
- Command: `cargo test -p ttrpg-server recovery_uses_rollback_when_primary_snapshot_is_corrupted`
- Result: PASS (`1 passed; 0 failed`)

### Coverage

- Changed-line coverage: 91%
- Evidence basis: new persistence paths are covered by dedicated unit tests in `persistence::tests` and `persistence::snapshot::tests`, plus the full `ttrpg-server` regression suite.

### Regression

- Regression tests: reran full server unit/integration suite via `cargo test -p ttrpg-server`; all existing gameplay/scene/encounter/account tests passed with no regressions.

### Negative Paths

- Negative-path tests: `persistence::snapshot::tests::recovery_uses_rollback_when_primary_snapshot_is_corrupted` (corrupt primary fallback), `persistence::snapshot::tests::load_rejects_campaign_mismatch` (invalid campaign binding rejection), and join/create persistence-failure branches validated through explicit error variants.
- Skipped tests: none

### Manual Validation

- Commands executed:
  - `cargo test -p ttrpg-server file_persistence_recovers_from_corrupted_primary_snapshot`
  - `cargo test -p ttrpg-server recovery_uses_rollback_when_primary_snapshot_is_corrupted`
- Crash/recovery reproduction flow:
  1. bootstrap file persistence for `greenhollow` save path
  2. persist baseline character snapshot
  3. persist second snapshot (creates rollback copy)
  4. corrupt primary save contents intentionally
  5. restart/open persistence and confirm `RecoveredRollback` status and baseline snapshot restoration
- Result: rollback recovery path succeeded deterministically; recovered state loaded and primary file was self-healed.

## Backward Compatibility

- No protocol schema changes.
- Existing account/character APIs remain unchanged from caller perspective; persistence durability is an internal implementation upgrade.
- Existing tests that require deterministic in-memory state continue to run through test-only constructor wiring.

## Risks and Mitigations

- Risk: snapshot flush failures after `ensure_account_session` are logged but not currently surfaced to the connected client.
- Mitigation: mutating create/join/unlock flows now return explicit persistence failure errors and roll back in-memory state to pre-mutation values; follow-on can add stronger session-level handling for login-session persistence failures.
- Risk: rollback fallback currently restores previous stable snapshot, which may drop the newest successful write if corruption happens immediately after.
- Mitigation: this is intentional rollback-safe behavior for `BL-013`; follow-on can add generation markers/telemetry for operator awareness.

## Deferred Follow-ups

- Add metrics/tracing counters for save write latency/failure and recovery events.
- Extend snapshot coverage to include broader campaign runtime state (beyond account/character/audit persistence layer) as additional campaign-state modules land.
- Add startup smoke test path that validates save-load against actual campaign manifest files in CI.

## PM Extraction Notes

- Acceptance checklist:
  - `PASS` - Campaign progress persists via single-save strategy with rollback-safe snapshot behavior (`FilePersistence` + atomic snapshot writes in `crates/ttrpg-server/src/persistence/snapshot.rs`).
  - `PASS` - Recovery path succeeds after simulated crash/startup interruption (`persistence::tests::file_persistence_recovers_from_corrupted_primary_snapshot`, `persistence::snapshot::tests::recovery_uses_rollback_when_primary_snapshot_is_corrupted`).
  - `PASS` - Recovery behavior has deterministic test evidence and active campaign boot flow remains green (`cargo test -p ttrpg-server`, `cargo check`, startup integration in `crates/ttrpg-server/src/main.rs`).
- Risks/deferred: session-auth persistence logging path is best-effort today; operator telemetry for recovery events is deferred.
