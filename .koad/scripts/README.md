# Koad Scripts

Utility scripts for common Koad OS workflows.

## Entry Point

Use:

```bash
.koad/scripts/koad <command> [options]
```

## Commands

### `lane-start`

Create a lane branch + dedicated worktree and print onboarding evidence.

Example:

```bash
.koad/scripts/koad lane-start \
  --role Platform \
  --slug s2-p1-scene-queue-protocol \
  --base v1
```

### `pr-open`

Create a PR with a governance-template body populated from flags and local git metadata.

Example:

```bash
.koad/scripts/koad pr-open \
  --title "S2-P1: deterministic queue + scene protocol" \
  --role Platform \
  --task-packet S2-P1 \
  --backlog "BL-004, BL-002" \
  --summary-what "Implemented scene queue and protocol message types" \
  --summary-why "Enable deterministic scene runtime updates" \
  --in-scope "Queue/runtime/protocol integration for S2-P1" \
  --out-of-scope "CLI rendering" \
  --ac1 "PASS - deterministic replay test added" \
  --ac2 "PASS - snapshot/delta protocol types compile" \
  --impl-doc "`docs/implementation/s2-p1-scene-queue-protocol.md`" \
  --coverage-evidence "Changed-line coverage: 84%" \
  --automated-test-updates "Added queue replay regression tests and protocol serialization tests" \
  --regression-tests "queue_replay_regression.rs" \
  --negative-path-tests "invalid-command ordering rejection tests"
```

### `pr-gate`

Run PM merge-gate checks against a PR (`validate-pr-governance`, `validate-koad-os-scope`, `validate-implementation-doc`, `validate-test-evidence`, mergeability, scope policy, and required evidence markers).

Example:

```bash
.koad/scripts/koad pr-gate --pr 25
```

Wait for checks to settle (up to 10 minutes) before final verdict:

```bash
.koad/scripts/koad pr-gate --pr 25 --watch
```

If all gates pass, auto-check `Koad git review approved` in PR body and add a Koad PM approval comment:

```bash
.koad/scripts/koad pr-gate --pr 25 --watch --apply-koad-approved --comment
```

### `pr-finish`

Merge a PR only when merge gates and review checkboxes are ready.

Example:

```bash
.koad/scripts/koad pr-finish --pr 25 --watch
```

Dry-run merge readiness and selected strategy:

```bash
.koad/scripts/koad pr-finish --pr 25 --watch --dry-run
```

### `saveup`

Append a role-aware saveup record.

Mode behavior:
- Global ledger mode: writes `.koad/.agent-core/sessions/SAVEUP_CALLS.md` + `.koad/.agent-core/sessions/LOG.md`
- Lane-isolated mode: writes `.koad/.agent-core/sessions/lane-saveups/<context-ref>.md`
- Team-role lane contexts (`--role Gameplay|Platform|Experience` + `--context-ref lane/...`) default to lane-isolated mode to reduce merge conflicts.
- Lane journals live in `.koad/.agent-core/sessions/lane-saveups/` and are gitignored local artifacts (not for feature-lane PR inclusion).

Example:

```bash
.koad/scripts/koad saveup \
  --role "Koad (PM)" \
  --context-ref koad-os \
  --scope koad-os-governance-sync \
  --notes "continuity checkpoint completed" \
  --objective "Capture durable governance updates" \
  --action "ran duplicate pre-check" \
  --action "updated standards + ops logs" \
  --artifact ".koad/.agent-core/sessions/SAVEUP_CALLS.md" \
  --artifact ".koad/.agent-core/sessions/LOG.md"
```

By default, `saveup` skips `PROJECT_PROGRESS.md` refresh to avoid status-only churn.
Use `--sync-progress` to refresh in global-ledger mode, or `--sync-progress-in-lane` in lane-isolated mode.
Use `--no-progress-sync` to force-disable refresh even when sync flags are set.

Force global ledger for a lane context:

```bash
.koad/scripts/koad saveup ... --global-ledger
```

### `context`

Dump a consolidated boot context for a specific role to stdout.
Reads Identity, Mission, Memory, Role definitions, Backlog, and Risks.

Example:

```bash
.koad/scripts/koad context --role "Koad (PM)"
```

### `saveup-reconcile`

Merge lane-isolated saveup journals into the global `SAVEUP_CALLS.md` and `LOG.md` ledgers.
Moves processed journals to `.koad/.agent-core/sessions/lane-saveups/archive/`.
Must be run on `koad-os` branch unless `--force` is used.

Example:

```bash
.koad/scripts/koad saveup-reconcile
```

Dry-run to see what would be merged:

```bash
.koad/scripts/koad saveup-reconcile --dry-run
```

### `standards-check`

Verify standards freshness and required source presence.
Replaces the legacy `standards_sync_status.py` script.

Example:

```bash
.koad/scripts/koad standards-check --max-age-hours 24
```

### `progress-sync`

Generate/update a root project progress dashboard against backlog + roadmap + sprint references.

Example:

```bash
.koad/scripts/koad progress-sync
```

Operational note:
- `.github/workflows/update-v1-dashboard.yml` runs this command after merges to `v1` and publishes the dashboard to the managed `V1 Project Dashboard` issue.

### `status`

Print a compact progress summary from `PROJECT_PROGRESS.md`.

Example:

```bash
.koad/scripts/koad status
```

Refresh dashboard first:

```bash
.koad/scripts/koad status --refresh
```

## Notes

- `pr-open` uses `gh pr create`; ensure GitHub CLI is authenticated.
- `pr-gate` reads GitHub PR metadata and can patch PR body/comments via `gh api`/`gh pr comment`.
- `pr-finish` uses `gh pr merge` and auto-selects an allowed strategy when `--strategy auto` is used.
- GitHub MCP operating guidance is defined in `.koad/.agent-ops/runbooks/github-mcp-workflow.md`.
- Preferred policy for agent-led GitHub actions is MCP first, with `gh` CLI as fallback.
- Current `koad` script commands remain `gh`-backed; use them as the standard fallback/operator path.
- Use `--dry-run` on each command to preview actions without writing.
