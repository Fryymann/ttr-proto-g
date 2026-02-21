# Learnings

Append-only record of durable lessons.

## 2026-02-21
### Technical
- Observation: Koad path references break if the kit is nested under `.koad/` without explicit path normalization.
- Why it matters: Startup/freshness gates may silently point to missing manifests and reduce trust in standards checks.
- Behavior update: Use explicit `.koad/...` paths in startup and freshness commands.

### Process
- Observation: Converting raw `IAN.md` brainstorm entries directly into decisions/backlog/risks keeps momentum high.
- Why it matters: Reduces idea loss and shortens transition from concept to implementable work.
- Behavior update: Treat `review latest IAN entry` as a structured PM intake workflow.

### Operational
- Observation: The freshness gate is only trustworthy when script defaults and required-source paths match the real workspace layout.
- Why it matters: Misaligned defaults can silently report missing/stale state and derail standards-dependent work.
- Behavior update: Keep `standards_sync_status.py` defaults aligned with `.koad/` pathing and verify with a no-arg run.

### Collaboration
- Observation: Explicitly mirroring durable PM updates into both `.agents/` and Koad ops logs prevents split-brain project state.
- Why it matters: Future agents can recover both planning context and operational rationale without guessing.
- Behavior update: For substantial PM updates, always write to `.agents` artifacts and `.koad/.agent-ops` logs in the same session.

### Governance
- Observation: Without an explicit authorization gate, PM agents can drift from delegation into direct sprint execution.
- Why it matters: This blurs team boundaries and can violate intended sprint ownership.
- Behavior update: Treat sprint implementation as blocked by default unless user explicitly authorizes sprint execution in-thread.

### Integration
- Observation: Antigravity lane execution in this workspace resolves Windows paths, not WSL mount paths.
- Why it matters: Prompt packets using `/mnt/c/...` can target the wrong filesystem context.
- Behavior update: Emit `C:\...` workspace paths in Antigravity prompts and delegation docs.

### Execution Quality
- Observation: Shared-branch lane execution can diverge without explicit onboarding proof and acceptance mapping.
- Why it matters: Agents may report completion with missing artifacts or out-of-scope edits.
- Behavior update: Require pre-edit onboarding evidence (path/branch/commit/scope) and pass/fail acceptance evidence in lane handoffs.
