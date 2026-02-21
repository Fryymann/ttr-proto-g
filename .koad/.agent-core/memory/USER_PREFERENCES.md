# User Preferences

## Confirmed Preferences
- Use `IAN.md` as a raw idea dump and request review/integration from latest entry.
- Keep terminal-first game feel, but support richer optional client surfaces.
- Structure project for long-term development by agent teams.
- Consolidate complexity where possible (fewer teams, clearer ownership).
- Use Koad as PM lead with three additional Codex role agents (`Gameplay`, `Platform`, `Experience`) as parallel executors.
- Pause Antigravity usage for now; keep it disabled unless explicitly re-enabled.
- Keep a root-level role prompt pack for launching Codex role agents.
- Keep sprint execution reserved for development teams unless the user explicitly asks the PM agent to execute a sprint.
- For team-agent lane prompts, use Windows-native workspace paths (`C:\...`) instead of WSL `/mnt/c/...`.
- Use git worktrees for parallel coder-agent lanes (one lane per worktree/branch) with pragmatic PR count control.
- Use PR-gated lane closure: user reviews on GitHub, Koad reviews via local git, and tasks close only after merge.
- Use standardized PR bodies and enforce governance through required `validate-pr-governance` check.
- Use `v1` as the active base branch for agent lanes; cut lane branches from `v1` and merge back into `v1` until the release line is complete.
- Roll out workflow policy changes across all coder-agent operating docs, not only a single prompt or role file.
- Require Codex boot to run Koad OS first, then ask which role to personify.
- Keep Koad/agent support and workflow-governance edits on `koad-os`; keep feature/runtime development edits on release-lane branches.
- Keep `koad-os` lightweight enough for occasional manual maintainer commits, then sync those updates into release line by PR.
- Include explicit persona/role signature in governance PRs for Koad-OS updates.
- Keep `saveup` role-aware across all Koad OS roles with explicit role/context metadata.

## Working Style Notes
- User accepts rough/brain-dump inputs and expects conversion into actionable specs/tasks/risks.
- User prefers iterative design decisions captured in living docs.
- User expects explicit `saveup` execution when requested.
