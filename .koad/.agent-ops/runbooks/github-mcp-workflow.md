# GitHub MCP Workflow Runbook

Status: Active  
Last updated: 2026-02-22  
Owner: Koad (PM)

## Purpose

Define how Codex agents use GitHub MCP for repository-hosted workflow actions (PR review, checks, comments, merge) while keeping local git operations in shell.

## Scope

- Repository: `Fryymann/ttr-proto-g`
- Active release branch: `v1`
- Support branch: `koad-os`
- Applies to roles: `Koad (PM)`, `Gameplay`, `Platform`, `Experience`

## Non-Negotiables

1. Keep branch/worktree operations local (`git` in shell).
2. Use GitHub MCP for remote GitHub state/actions when available.
3. Preserve existing governance gates:
- `validate-pr-governance`
- `validate-koad-os-scope`
- `validate-implementation-doc`
- `validate-test-evidence`
4. Keep lane completion merge-gated (Koad review + Ian review + merged PR).

## Required Configuration

Workspace config (`config.toml`):

```toml
[mcp_servers.github]
url = "https://api.githubcopilot.com/mcp/"
bearer_token_env_var = "GITHUB_PAT"
```

Environment:

- `GITHUB_PAT` must be set in the active shell/session.

Token scope guidance:

- Public repo only: `public_repo`
- Private repo support: `repo`
- Optional org/team reads: `read:org` (only if team queries are required)

## Role Permissions Model

`Koad (PM)`:
- May read/update/merge PRs after gates pass.
- May write PM gate comments and review updates.

`Gameplay`, `Platform`, `Experience`:
- May read PR/check state and post handoff comments.
- Must not merge PRs unless explicitly directed by Koad.

## Standard Operating Flow

1. Local lane checks (shell):
- Verify branch/worktree isolation.
- Run tests/build/coverage evidence locally.

2. PR state read (MCP):
- `pull_request_read(method=get)` for base/head/state/mergeability.
- `pull_request_read(method=get_status)` for check suite status.
- `pull_request_read(method=get_files)` for scope inspection.

3. Gate evaluation (policy):
- Confirm required checks are passing.
- Confirm scope policy (`koad-os` vs non-`koad-os`) remains valid.
- Confirm review-gate evidence is present in PR body.

4. Review/annotation actions (MCP):
- Add PR/issue comment: `add_issue_comment`
- Update PR metadata/body/checklist state: `update_pull_request`
- Add line/thread replies when needed: `add_reply_to_pull_request_comment`

5. Merge (Koad PM only):
- Re-check mergeability and required checks.
- Merge using `merge_pull_request`.
- Confirm final merged state via `pull_request_read(method=get)`.

## Tool Mapping (Current Flow -> MCP)

| Current behavior | MCP tool(s) |
|---|---|
| `gh pr view --json ...` | `pull_request_read(method=get)` |
| `gh pr checks` / status rollup | `pull_request_read(method=get_status)` |
| changed files inspection | `pull_request_read(method=get_files)` |
| `gh pr comment` | `add_issue_comment` |
| `gh api PATCH pulls/{n}` body update | `update_pull_request` |
| `gh pr merge` | `merge_pull_request` |

## Fallback Policy

1. Preferred path: GitHub MCP.
2. Fallback path: `gh` CLI when MCP is unavailable or blocked.
3. If both fail: stop merge/write actions, log blocker, and request operator intervention.

## Failure Handling

- Auth failure:
  - Verify `GITHUB_PAT` exists and has required scopes.
  - Re-run a read action (`get_me` or PR read) before write actions.
- Permission failure:
  - Downgrade to read-only mode and escalate to Koad/operator.
- Transient API errors:
  - Retry read operations with short backoff.
  - Avoid repeated write retries without state re-check.

## Audit and Evidence Expectations

- Keep review comments explicit on failed gates and required fixes.
- Record merge decision basis (checks, scope, review approvals).
- Keep PR body review-gate evidence lines up to date for audit clarity.

## Example PM Gate Sequence (MCP)

1. Read PR details and checks.
2. Verify required checks passed and scope policy compliance.
3. Post PM gate outcome comment (`approve` or `changes requested`).
4. If approved and Ian gate is complete, merge PR.
5. Re-read PR and confirm `MERGED`.
