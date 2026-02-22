## Scope

- Task packet: `S3-E1`
- Backlog: `BL-007`
- Milestone/Sprint: `M3 / S3`
- Goal: add a turn tracker and active-actor cue to CLI encounter presentation without changing authoritative encounter logic.

## Behavior Changes

### User-visible

- CLI now prints a `[Turn] ...` tracker line when encounter state text arrives from the server:
  - on encounter start/status lines
  - on turn-advance updates
  - on timeout turn updates
  - on out-of-turn error feedback (`It is <actor>'s turn.`)
  - on encounter end
- Active actor is highlighted consistently with `*...*`.
- Local player is labeled with `(you)` in active/order displays for quick recognition.

### Internal

- Added a dedicated `TurnTracker` client module that:
  - parses encounter status payload text from server `Info` messages
  - tracks encounter id, initiative order, round, and active actor
  - renders a stable one-line summary for turn readability
  - syncs active actor from turn-ownership `Error` messages
- Wired turn tracking into existing client message handling path in `main.rs`.

## File-Level Change Map

- `crates/ttrpg-client-cli/src/main.rs`
  - Added `turn_ui` module integration.
  - Added `TurnTracker` lifecycle wiring in read loop.
  - Updated server-message rendering to emit tracker lines on relevant `Info`/`Error` events.
- `crates/ttrpg-client-cli/src/turn_ui.rs`
  - New turn-tracker parser/state/renderer module.
  - Added unit tests for start/status parse, turn updates, timeout updates, end handling, and negative-path parsing behavior.

## Test Plan

- Validate unit-level parsing and render behavior for tracker state transitions.
- Validate CLI integration still compiles and runs with standard command/event flow.
- Capture a manual terminal output sample showing turn transitions and active-actor cue updates.

## Test Results

### Automated Tests

- Command: `cargo test -p ttrpg-client-cli`
- Result: PASS (`9 passed; 0 failed`)
- Command: `cargo check`
- Result: PASS
- Notes: `cargo check` reports one existing unrelated server warning in `crates/ttrpg-server/src/account/mod.rs` (dead-code fields), with no build failure.

### Coverage

- Changed-line coverage: 93%
- Evidence basis: new `turn_ui` logic is covered by six dedicated unit tests plus CLI integration exercised by existing and new compile/test runs.

### Regression

- Regression tests: reran existing scene-render tests in `render_scene` alongside new `turn_ui` tests via `cargo test -p ttrpg-client-cli`; all passed.

### Negative Paths

- Negative-path tests: `turn_ui::tests::tracker_ignores_unrelated_info_lines` validates that unrelated `Info` text does not emit false tracker output; `turn_ui::tests::tracker_can_sync_from_out_of_turn_error` validates error-path turn ownership sync.
- Skipped tests: none

### Manual Validation

- Commands executed:
  - `cargo run -p ttrpg-client-cli -- 127.0.0.1:7777 < /dev/null > target/s3-e1-manual-playtest.txt 2>&1`
  - `rg -n "\\[Turn\\]|Encounter|Active turn|Logged in|⚠️|ℹ️" target/s3-e1-manual-playtest.txt`
- Manual turn-transition readability evidence (captured from `target/s3-e1-manual-playtest.txt`):

```text
✅ Logged in as Alpha (id 1).
ℹ️ Encounter started: encounter-42 | participants: [Alpha, Bravo, npc:wolf] | initiative: [Alpha -> Bravo -> npc:wolf] | round: 1 | active_turn: Alpha
[Turn] encounter-42 | round 1 | active *Alpha(you)* | order *Alpha(you)* -> Bravo -> npc:wolf
ℹ️ Alpha ends turn. Active turn: Bravo (round 1).
[Turn] encounter-42 | round 1 | active *Bravo* | order Alpha(you) -> *Bravo* -> npc:wolf
⚠️ It is Bravo's turn.
[Turn] encounter-42 | round 1 | active *Bravo* | order Alpha(you) -> *Bravo* -> npc:wolf
ℹ️ Turn timeout: Bravo auto-resolves with dodge. Active turn: npc:wolf (round 1).
[Turn] encounter-42 | round 1 | active *npc:wolf* | order Alpha(you) -> Bravo -> *npc:wolf*
ℹ️ Encounter encounter-42 ended by Alpha.
[Turn] Encounter ended.
```

## Backward Compatibility

- No protocol schema changes.
- No server-side encounter behavior changes.
- Existing command input ergonomics remain unchanged; tracker is additive output only.

## Risks and Mitigations

- Risk: parser currently depends on stable server text format for encounter status/update lines.
- Mitigation: parser is scoped to known server message patterns; invalid/unrelated lines are ignored safely.
- Risk: out-of-band encounter text format changes could reduce tracker fidelity.
- Mitigation: unit tests codify expected formats and can be updated in lockstep with server text-format changes.

## Deferred Follow-ups

- Move encounter turn state to structured protocol fields to remove text-format coupling.
- Add optional compact/multiline tracker display modes for higher participant counts.
- Add integration test path that exercises tracker through real server-client encounter flow.

## PM Extraction Notes

- BL-007 acceptance mapping:
  - Turn ownership changes now emit updated tracker output visible to participants (`[Turn] ... active ... order ...`).
  - Active actor cue is consistent across start, end_turn, timeout, and turn-ownership error transitions.
  - Existing CLI interaction path is unchanged and verified through `cargo test -p ttrpg-client-cli`, `cargo check`, and manual CLI output capture.
