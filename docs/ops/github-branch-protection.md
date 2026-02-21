# GitHub Branch Protection (PR Workflow)

Status: Active  
Last updated: 2026-02-21  
Owner: Project Manager (Koad)

## Goal

Enforce lane completion through PR review + merge:

1. Koad local git review approved.
2. User GitHub review approved.
3. PR merged to target branch.

## Target Branches

Apply this rule set to both:
- `v1` (active release integration branch)
- `koad-os` (Koad/agent-support branch)

## Required Repository Files

- `.github/pull_request_template.md`
- `.github/workflows/pr-template-gate.yml`
- `.github/workflows/koad-os-scope-gate.yml`

## Branch Protection Settings (GitHub UI)

Open: `Settings` -> `Branches` -> `Add branch protection rule`.

Create one rule per branch name pattern:
- `v1`
- `koad-os`

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

## Operational Flow

1. Lane agent opens PR using the template.
2. Koad performs local git review and checks `Koad git review approved` in PR body.
3. User reviews in GitHub and checks `User GitHub review approved` in PR body.
4. `validate-pr-governance` and `validate-koad-os-scope` pass and GitHub approval requirement is satisfied.
5. PR is merged.
6. Only then mark task/backlog item complete.

## Notes

- The workflow enforces that both review-gate checkboxes are checked in the PR body.
- Scope gate behavior:
  - PRs targeting `koad-os` may only modify Koad/agent support files.
  - PRs targeting other branches must not modify Koad/agent support files.
- GitHub branch protection enforces PR approval and required status checks before merge.
- If a different release line becomes active later, copy the `v1` rule to that branch pattern and keep the `koad-os` rule in place.
