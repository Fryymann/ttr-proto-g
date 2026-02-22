# GitHub Branch Protection (PR Workflow)

Status: Active  
Last updated: 2026-02-21  
Owner: Project Manager (Koad)

## Goal

Enforce lane completion through PR review + merge:

1. Koad local git review approved.
2. Ian review approved.
3. PR merged to target branch.

## Target Branches

Use split governance:
- `v1` (active release integration branch): strict PR protection.
- `koad-os` (Koad/agent-support branch): lightweight, allows direct maintainer pushes.

## Required Repository Files

- `.github/pull_request_template.md`
- `.github/workflows/pr-template-gate.yml`
- `.github/workflows/koad-os-scope-gate.yml`

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
2. Lane agent performs self-review and checks `Coding agent self-review completed` in PR body.
3. Koad performs local git review; once approved, checks `Koad git review approved` and notifies Ian.
4. Ian performs final review in GitHub and checks `Ian review approved` in PR body.
5. `validate-pr-governance` and `validate-koad-os-scope` pass and GitHub approval requirement is satisfied.
6. PR is merged.
7. Only then mark task/backlog item complete.

## Notes

- The workflow enforces that all three review-gate checkboxes are checked in the PR body.
- Scope gate behavior:
  - PRs targeting `koad-os` may only modify Koad/agent support files.
  - PRs sourced from `koad-os` (e.g., `koad-os` -> `v1`) may only modify Koad/agent support files.
  - Other PRs targeting non-`koad-os` branches must not modify Koad/agent support files.
- `v1` branch protection enforces PR approval and required status checks before merge.
- If a different release line becomes active later, copy the `v1` rule to that branch pattern and retain `koad-os` as lightweight support branch.
