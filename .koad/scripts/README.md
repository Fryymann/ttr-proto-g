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

### `saveup`

Append a role-aware saveup row and session log entry.

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

By default, `saveup` also refreshes `PROJECT_PROGRESS.md`. Use `--no-progress-sync` to skip.

### `progress-sync`

Generate/update a root project progress dashboard against backlog + roadmap + sprint references.

Example:

```bash
.koad/scripts/koad progress-sync
```

## Notes

- `pr-open` uses `gh pr create`; ensure GitHub CLI is authenticated.
- Use `--dry-run` on each command to preview actions without writing.
