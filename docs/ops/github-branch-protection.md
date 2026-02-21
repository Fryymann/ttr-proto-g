# GitHub Branch Protection (PR Workflow)

Status: Active  
Last updated: 2026-02-21  
Owner: Project Manager (Koad)

## Goal

Enforce lane completion through PR review + merge:

1. Koad local git review approved.
2. User GitHub review approved.
3. PR merged to target branch.

## Target Branch

Apply this rule set to the default integration branch (currently `master`).

## Required Repository Files

- `.github/pull_request_template.md`
- `.github/workflows/pr-template-gate.yml`

## Branch Protection Settings (GitHub UI)

Open: `Settings` -> `Branches` -> `Add branch protection rule`.

Use branch name pattern:
- `master`

Enable:

- `Require a pull request before merging`
  - `Require approvals`: `1`
  - `Dismiss stale pull request approvals when new commits are pushed`: `ON`
  - `Require review from Code Owners`: `OFF` (enable later if CODEOWNERS is added)
- `Require status checks to pass before merging`: `ON`
  - `Require branches to be up to date before merging`: `ON`
  - Required checks:
    - `validate-pr-governance`
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
4. `validate-pr-governance` passes and GitHub approval requirement is satisfied.
5. PR is merged.
6. Only then mark task/backlog item complete.

## Notes

- The workflow enforces that both review-gate checkboxes are checked in the PR body.
- GitHub branch protection enforces PR approval and required status checks before merge.
