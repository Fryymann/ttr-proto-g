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

### Parallel Delivery
- Observation: Parallel coder lanes are safer and easier to review when each lane runs in its own git worktree/branch with explicit PR dependency notes.
- Why it matters: Shared worktrees increase accidental overlap, hidden conflicts, and ambiguous merge order.
- Behavior update: Enforce one lane per dedicated worktree/branch and require PR metadata in lane handoffs.

### Policy Propagation
- Observation: Workflow policies drift when they are documented only in sprint prompts and not mirrored into standards/startup/role artifacts.
- Why it matters: Later agents may follow incomplete guidance and bypass intended guardrails.
- Behavior update: Treat policy rollout as complete only after synchronized updates across prompt pack, Koad standards/startup, and role handoff docs.

### Role Clarity
- Observation: Multi-instance Codex execution works best when role selection is explicit at boot, not inferred from prior thread context.
- Why it matters: Implicit role assumptions cause PM/team boundary drift and inconsistent onboarding quality.
- Behavior update: Require role question at startup and route to PM or team-role context before substantial work.

### Review Governance
- Observation: Lane closure quality improves when GitHub user review and local git review are both required before merge.
- Why it matters: Prevents handoff-only completion claims and keeps merge as the hard completion checkpoint.
- Behavior update: Enforce PR-required dual review gates and mark tasks complete only after merged PR confirmation.

### Policy Enforcement
- Observation: Review policy is more reliable when branch protection uses a concrete required status check tied to PR template governance.
- Why it matters: Process-only guidance can drift; required checks make merge gates auditable and consistent.
- Behavior update: Keep PR template + `validate-pr-governance` workflow aligned with branch protection settings.

### Release-Line Branching
- Observation: Parallel lane coordination stays clearer when one named release branch is the shared base for all lane branches and PR targets.
- Why it matters: Mixed base branches during active delivery windows increase merge ambiguity and review overhead.
- Behavior update: While release line `v1` is active, cut lane branches from `v1` and merge lane PRs back into `v1`.

### Technical
- Observation: A branch-scope gate that inspects only PR target branch rules can accidentally block valid support-branch promotion PRs.
- Why it matters: Governance checks can deadlock the intended `koad-os` -> release-line sync path.
- Behavior update: Scope-gate workflows must evaluate both base and head refs and explicitly allow support-only sync PRs from `koad-os`.

### Process
- Observation: Split governance works best when the release branch is strict but the support branch stays lightweight for maintainer direct commits.
- Why it matters: This preserves delivery controls without slowing operational/agent-support maintenance.
- Behavior update: Keep strict required checks on `v1`, keep `koad-os` lightweight, and promote support updates by PR.

### Operational
- Observation: Saveup quality improves when CI check names, template fields, and branch-protection docs are synchronized before finalizing continuity records.
- Why it matters: Unsynced governance artifacts create false assumptions for later agent sessions.
- Behavior update: Before finalizing saveup, verify workflow names, required check IDs, and runbook settings match exactly.

### Collaboration
- Observation: Human+agent shared workflows need explicit PR-level authorship labeling for unambiguous review context.
- Why it matters: Reviewers can otherwise lose attribution clarity on governance/meta updates.
- Behavior update: Require a `Persona signature` line in PR bodies and enforce it via governance validation.

## 2026-02-22
### Operational
- Observation: Shared saveup ledgers create avoidable merge conflicts when multiple developer lanes append continuity records in parallel.
- Why it matters: Saveup conflicts slow merge flow and can hide or drop continuity metadata during conflict resolution.
- Behavior update: Default team-role lane saveups to lane-isolated journals and reconcile global PM logs on `koad-os`.

### Experience
- Observation: CLI scene symbol clarity depends on combining `AuthOk`, `RoomState`, and `WhoList` identity hints instead of relying on scene occupant IDs alone.
- Why it matters: Without identity hydration, player actors are misclassified as generic NPCs and movement feedback becomes harder to interpret in multiplayer sessions.
- Behavior update: Keep a client-side identity cache hydrated from auth/room/who messages and drive map symbol mapping (`@`/`P`/`N`) from that cache.

### Execution Environment
- Observation: Codex team-role agents in this workspace run inside WSL2 (typically Ubuntu), not Windows-native shells.
- Why it matters: Prompt packets that direct Codex agents to `C:\...` paths can fail or target the wrong filesystem context.
- Behavior update: Use `/mnt/c/...` workspace paths in Codex task packets; keep Windows-native pathing only for Antigravity-specific workflows if re-enabled.

### Operational
- Observation: Requiring checked PR review-gate boxes in CI creates avoidable merge friction when GitHub approvals already enforce reviewer authority.
- Why it matters: Checkbox timing failures block merges without improving real review quality and slow development throughput.
- Behavior update: Validate review-gate line presence in CI, keep checkbox states informational, and rely on required GitHub approvals + status checks as merge authority.

### Process
- Observation: Manual `v1`/`koad-os` sync maintenance creates recurring branch drift and conflict overhead.
- Why it matters: Sync chores consume PM time and delay development lanes without adding product value.
- Behavior update: Auto-sync `koad-os` from `v1` on every `v1` push, with conflict fallback to explicit `v1` -> `koad-os` sync PR.

### Operational
- Observation: Status dashboards create recurring maintenance friction when they require separate status commits during active lane delivery.
- Why it matters: Extra status-only commits and PR noise slow development flow and distract from roadmap execution.
- Behavior update: Publish the V1 dashboard automatically after merges to `v1` via workflow-managed issue updates; keep manual dashboard generation on-demand.

### Process
- Observation: Even with branch sync automation, manually opening recurring `koad-os` promotion PRs still adds PM overhead and context switching.
- Why it matters: Repetitive promotion steps slow flow and increase risk of support updates lingering on `koad-os`.
- Behavior update: Auto-manage one `koad-os` -> `v1` promotion PR from a workflow keyed off effective branch deltas.

### Operational
- Observation: PM PR gate reviews repeatedly perform the same merge/check/scope/evidence inspection and manual checkbox update steps.
- Why it matters: Manual repetition increases review-cycle time and creates avoidable inconsistency in Koad approval handling.
- Behavior update: Use `koad pr-gate` as the default scripted PM review preflight and optional checkbox/comment apply path.
