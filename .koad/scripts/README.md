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
  --ac2 "PASS - snapshot/delta protocol types compile"
```

### `pr-gate`

Run PM merge-gate checks against a PR (`validate-pr-governance`, `validate-koad-os-scope`, mergeability, scope policy, and required evidence markers).

Example:

```bash
.koad/scripts/koad pr-gate --pr 25
```

If all gates pass, auto-check `Koad git review approved` in PR body and add a Koad PM approval comment:

```bash
.koad/scripts/koad pr-gate --pr 25 --apply-koad-approved --comment
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
- Use `--dry-run` on each command to preview actions without writing.
