# Decision Log

## YYYY-MM-DD - Decision title
- Decision:
- Why:
- Impact:
- Revisit trigger:

## 2026-02-21 - Koad path normalization and PM integration
- Decision:
  - Treat `.koad/` as canonical base path for Koad startup, standards, and saveup docs in this repo.
  - Extend required sources to include `.agents` PM artifacts.
- Why:
  - Current repository stores Koad under `.koad`, not root-level `.agent-core` / `.standards`.
- Impact:
  - Freshness checks and startup instructions now resolve correctly.
  - PM workflow can enforce backlog/risk/team synchronization through Koad standards.
- Revisit trigger:
  - If Koad kit is relocated to repo root in a future migration.

## 2026-02-21 - Koad standards source activation
- Decision:
  - Seed `.koad/.standards/standards_registry.md` and `.koad/.standards/project_profile.md` with active local PM-operational standards content.
  - Set standards sync script defaults to `.koad`-scoped manifest and required sources.
- Why:
  - Placeholder-only standards sources and mismatched defaults weaken freshness gating and standards-dependent behavior.
- Impact:
  - Freshness gate now runs correctly without extra arguments and validates PM-required sources.
  - Startup/ops instructions are internally consistent for this repository layout.
- Revisit trigger:
  - If standards governance expands beyond current PM scope or repository structure changes.

## 2026-02-21 - V1 scope constraints for rules, DM authority, and encounter participation
- Decision:
  - V1 rules/mechanics content scope is SRD-only (plus original project-authored content).
  - DM-agent is advisory by default; story flow and semi-scripted NPC policy remain authoritative.
  - V1 encounter participation uses party-based rules; non-party actors are not auto-pulled by scene-wide proximity.
- Why:
  - Reduces licensing risk in early delivery, preserves deterministic control boundaries, and enables distraction/sneak scenarios without forced full-scene combat joins.
- Impact:
  - Backlog priorities now include SRD compliance gating and advisory DM-agent behavior contracts.
  - Encounter design shifts from pull-radius policy to party-membership policy and related validation tests.
- Revisit trigger:
  - After first full playable loop and initial social-encounter design pass.

## 2026-02-21 - Unified roadmap reference established
- Decision:
  - Use `docs/design/game-system-roadmap.md` as the primary living reference for cross-system sequencing, focus, and delivery scope.
  - Keep supporting design docs (`combat-framework-outline`, `mechanics-decision-matrix-v1`) aligned to this roadmap.
- Why:
  - Reduce planning fragmentation and provide one stable focus artifact for ongoing execution.
- Impact:
  - PM and team leads can prioritize work from one source while preserving specialized detail docs.
  - Future updates should first land in roadmap focus/sequence, then propagate to supporting docs and backlog/risk artifacts.
- Revisit trigger:
  - If planning scope grows beyond one roadmap and requires split domain roadmaps.

## 2026-02-21 - Execution sprint plan reference established
- Decision:
  - Use `docs/design/execution-sprint-plan.md` as the operational sequencing reference for M2-M3 delivery slices.
  - Keep `.agents/backlog.md` `Active Focus Window` aligned to the sprint plan `Now` and `Next` horizons.
- Why:
  - Convert high-level roadmap direction into file-targeted, test-gated execution focus.
- Impact:
  - Team handoffs can anchor to concrete module targets and validation gates per sprint.
  - PM updates now have a standard place to track near-term execution state.
- Revisit trigger:
  - If cadence shifts from sprint slices to release-train or kanban-only flow.

## 2026-02-21 - Antigravity execution model adoption
- Decision:
  - Koad (PM) remains planning/orchestration authority while Antigravity coder agents execute sprint lanes in parallel.
  - Maintain root prompt pack at `ANTIGRAVITY_SPRINT_PROMPTS.md` as canonical launcher input for each sprint lane.
- Why:
  - Parallel agent execution can increase throughput while preserving centralized scope/risk control.
- Impact:
  - PM role now includes publishing/maintaining sprint prompt packets.
  - Handoff artifacts must include Antigravity prompt id/lane for traceability.
- Revisit trigger:
  - If execution platform changes or parallel-lane merge overhead outweighs throughput gains.

## 2026-02-21 - Campaign startup and lock-validation contract for M2 bootstrap
- Decision:
  - Server startup now requires explicit active campaign selection from a manifest (`--campaign`, `TTRPG_CAMPAIGN_ID`, or interactive selection prompt).
  - Character create/login path is mediated by persistence-layer campaign-lock validation; first join applies lock metadata, mismatched campaign joins are rejected.
- Why:
  - Enforces one-active-campaign runtime policy and creates a clean seam for later durable persistence/admin unlock implementation.
- Impact:
  - Startup behavior is now campaign-context-aware and blocks ambiguous default campaign boot.
  - Account/character/campaign-lock scaffolding exists in code and can be replaced by a durable storage backend without rewriting auth command flow.
- Revisit trigger:
  - When implementing audited admin unlock tooling and non-volatile persistence in Sprint S4.

## 2026-02-21 - Sprint execution authorization guardrail
- Decision:
  - Add a hard operating guardrail: the PM agent must not execute sprint implementation work unless the user explicitly requests sprint execution in the current thread.
- Why:
  - Sprints are intended for development lanes/teams, and unrequested PM sprint execution can bypass the intended delegation model.
- Impact:
  - Sprint activity defaults to planning/delegation/review only.
  - Any sprint execution now requires explicit user authorization before implementation begins.
- Revisit trigger:
  - If user changes execution policy and explicitly allows autonomous PM sprint execution by default.

## 2026-02-21 - Antigravity path convention uses Windows workspace paths
- Decision:
  - Antigravity lane prompts must use Windows-native workspace paths (for this repo: `C:\data\ttrpg`) instead of WSL paths (`/mnt/c/data/ttrpg`).
- Why:
  - Antigravity executes at the Windows layer and WSL paths can misroute agent context.
- Impact:
  - Prompt packets and delegation docs should emit `C:\...` path form for Antigravity agents.
- Revisit trigger:
  - If Antigravity runtime changes to Linux/WSL-native path resolution.

## 2026-02-21 - Antigravity onboarding evidence gate for lane alignment
- Decision:
  - Add a mandatory onboarding evidence gate for every Antigravity lane before code edits.
  - Require lane handoffs to include acceptance criteria `PASS`/`FAIL` evidence and explicit out-of-scope modification disclosure.
- Why:
  - Shared branch/worktree execution can drift without explicit context verification and acceptance mapping.
- Impact:
  - Lane kickoff now must prove workspace path, branch, base commit, and backlog acceptance scope.
  - Lane closure quality is stricter; incomplete artifacts or unverified criteria cannot be treated as complete.
- Revisit trigger:
  - If Antigravity platform introduces native enforced lane templates/checklists.

## 2026-02-21 - S1 gameplay lane completion requires crate-integrated test evidence
- Decision:
  - Do not treat S1 gameplay lane work as complete unless `scene` primitives are integrated into crate compilation paths and occupancy tests run under standard server test commands.
- Why:
  - Unreferenced modules can contain passing local tests that never execute in CI/default lane verification.
- Impact:
  - Future lane reviews must verify artifact existence and execution-path integration, not only file presence.
- Revisit trigger:
  - If crate test architecture changes to auto-discover and run isolated module tests.

## 2026-02-21 - Worktree-isolated coder lane workflow with pragmatic PR policy
- Decision:
  - Adopt one lane per dedicated git worktree and branch (`lane/<PROMPT_ID>/<scope-slug>`) for Antigravity coder execution.
  - Keep current team/lane model; do not require team restructuring to adopt worktrees.
  - Use default one-PR-per-code-lane policy, with optional docs/chore batching and a sprint integration PR for cross-lane wiring.
- Why:
  - Improves parallel throughput while reducing cross-lane interference and merge ambiguity.
- Impact:
  - Coder prompts and PM operating rules now require worktree/branch assignment plus PR metadata in handoffs.
  - Review flow stays flexible without forcing PR inflation or organizational changes.
- Revisit trigger:
  - If merge queue or CI latency becomes the dominant delivery bottleneck and requires PR policy changes.

## 2026-02-21 - Pivot primary execution model to Codex team-role instances
- Decision:
  - Pause Antigravity usage as primary execution path.
  - Use Koad (PM) as lead agent and run three additional Codex instances as team roles: Gameplay, Platform, Experience.
  - Require all Codex instances to boot general Koad OS then explicitly resolve role before substantial work.
- Why:
  - Increase alignment and consistency by using a shared Codex boot/standards framework across all active development roles.
- Impact:
  - Startup flow now includes mandatory role-selection routing.
  - Active parallel execution prompts moved to `CODEX_ROLE_PROMPTS.md`; Antigravity prompts are archived/paused.
  - Standards now include role boot gate and team-agent familiarization gate for non-PM roles.
- Revisit trigger:
  - If user explicitly requests reactivation of Antigravity as primary execution model.
