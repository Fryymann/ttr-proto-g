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
