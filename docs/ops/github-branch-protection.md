# GitHub Branch Protection (PR Workflow)

Status: Active
Last updated: 2026-02-22
Owner: Project Manager (Koad)

## Goal

Enforce lane completion through PR review + merge:

1. Koad local git review completed.
2. Ian GitHub review approved.
3. PR merged to target branch.

## Target Branches

Use split governance:
- `v1` (active release integration branch): strict PR protection.
- `koad-os` (Koad/agent-support branch): lightweight, allows direct maintainer pushes.

## Required Repository Files

- `.github/pull_request_template.md`
- `.github/workflows/pr-template-gate.yml`
- `.github/workflows/koad-os-scope-gate.yml`
- `.github/workflows/sync-koad-os-from-v1.yml`
- `.github/workflows/promote-koad-os-to-v1.yml`
- `.github/workflows/update-v1-dashboard.yml`

## Branch Protection Settings (GitHub UI)

Open: `Settings` -> `Branches` -> `Add branch protection rule`.

Required rule for branch name pattern:
- `v1`

Enable:

- `Require a pull request before merging`
  - `Require approvals`: `1`
  - `Dismiss stale pull request approvals when new commits are pushed`: `ON`
  - `Require review from Code Owners`: `OFF` (enable later if CODEOWNERS is added)
- `Require status checks to pass before merging`: `ON`
  - `Require branches to be up to date before merging`: `ON`
  - Required checks:
    - `validate-pr-governance`
    - `validate-koad-os-scope`
- `Require conversation resolution before merging`: `ON`
- `Include administrators`: `ON`
- `Allow force pushes`: `OFF`
- `Allow deletions`: `OFF`

Optional but recommended:

- `Require linear history`: `ON`
- `Restrict who can push to matching branches`: maintainers only

For `koad-os`:
- Keep branch protection minimal or disabled to allow manual direct commits by maintainers.
- If you still protect `koad-os`, keep it lightweight (no required approval/check gate).

## Operational Flow

1. Lane agent opens PR using the template.
2. Lane agent performs self-review and keeps review-gate checkboxes accurate in PR body.
3. Koad performs local git review and records disposition.
4. Ian performs final review in GitHub and records approval.
5. `validate-pr-governance` and `validate-koad-os-scope` pass and GitHub approval requirement is satisfied.
6. PR is merged.
7. Only then mark task/backlog item complete.

## koad-os Sync Automation

- Workflow: `.github/workflows/sync-koad-os-from-v1.yml`
- Trigger: every push to `v1` (and manual `workflow_dispatch`).
- Behavior:
  - Attempts to merge `origin/v1` into `koad-os` and push the updated `koad-os` head.
  - If merge conflicts occur, opens (or reuses) a `v1` -> `koad-os` sync PR for manual resolution.
- This keeps `koad-os` near-current with `v1` and reduces manual sync maintenance between development merges.

## koad-os Promotion PR Automation

- Workflow: `.github/workflows/promote-koad-os-to-v1.yml`
- Trigger: every push to `koad-os` (and manual `workflow_dispatch`).
- Behavior:
  - Compares effective support/process file deltas between `koad-os` and `v1`.
  - Auto-creates or updates one managed PR from `koad-os` -> `v1` when deltas exist.
  - Auto-closes managed promotion PR when no effective delta remains.
- Outcome:
  - PM no longer needs to manually open routine support-promotion PRs for `koad-os` updates.

## V1 Dashboard Automation

- Workflow: `.github/workflows/update-v1-dashboard.yml`
- Trigger: every push to `v1` (and manual `workflow_dispatch`).
- Behavior:
  - Regenerates dashboard markdown from canonical planning artifacts using `bash .koad/scripts/koad progress-sync`.
  - Upserts a managed issue titled `V1 Project Dashboard` with the latest status snapshot.
- Outcome:
  - Dashboard visibility stays current after merges without requiring separate status-only commits in development lanes.

## Notes

- `validate-pr-governance` enforces required PR sections, persona signature, and review-gate line presence.
- Review-gate checkbox states are informational for human handoff tracking; merge authority comes from GitHub reviews + required checks.
- Scope gate behavior:
  - PRs from `v1` to `koad-os` are allowed as an explicit sync exception.
  - PRs targeting `koad-os` may only modify Koad/agent support files.
  - PRs sourced from `koad-os` (e.g., `koad-os` -> `v1`) may only modify Koad/agent support files.
  - Other PRs targeting non-`koad-os` branches must not modify Koad/agent support files.
- `v1` branch protection enforces PR approval and required status checks before merge.
- If a different release line becomes active later, copy the `v1` rule to that branch pattern and retain `koad-os` as lightweight support branch.
