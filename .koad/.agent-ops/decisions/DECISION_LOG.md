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

## 2026-02-21 - PR-required dual-review merge gate for lane completion
- Decision:
  - Require every coder lane to submit a PR before completion.
  - Use dual review gates: user performs GitHub review and Koad performs local git review.
  - Treat lane/backlog task as complete only after merged PR confirmation.
- Why:
  - Centralizes review authority with the user in GitHub while preserving local deterministic verification through Koad git review.
  - Prevents premature task closure before integration.
- Impact:
  - Handoff artifacts must now include PR URL/base/head/latest commit plus both review dispositions.
  - PM/task closure now requires merged-PR evidence, not just handoff claims.
  - Standards and role docs now encode this gate for future sessions.
- Revisit trigger:
  - If repository governance changes to a different approval/merge model.

## 2026-02-21 - Enforce PR governance via template + required status check
- Decision:
  - Add a standard PR template for lane handoff metadata and review gates.
  - Add GitHub workflow check `validate-pr-governance` to validate required PR sections and checked review-gate boxes.
  - Use branch protection on `master` to require PR approval and `validate-pr-governance` before merge.
- Why:
  - Converts process-only review requirements into enforceable GitHub merge controls.
- Impact:
  - Lane PRs now have a consistent structure and machine-validated governance gate.
  - Branch protection setup has a concrete required check for repository configuration.
- Revisit trigger:
  - If branch strategy changes from `master` or governance checks migrate to a different CI/policy system.

## 2026-02-21 - Set `v1` as active release base branch for coder lanes
- Decision:
  - Treat `v1` as the active shared base branch for coder-lane work.
  - Require lane branches to be created from `v1` and lane PRs to target `v1` until PM declares a replacement release base.
  - Update branch-protection guidance to protect `v1` instead of `master`.
- Why:
  - Keeps all in-flight lane work aligned to one integration line during V1 delivery.
- Impact:
  - Lane onboarding and handoff metadata now assumes `v1` as the expected PR base.
  - Governance documentation now points branch protection at `v1`.
- Revisit trigger:
  - When V1 release line closes or a new release base branch is activated.

## 2026-02-21 - Account-first authentication for character management
- Decision:
  - Require login to authenticate a user account first.
  - Scope all character management/join operations to the authenticated account identity.
  - Treat name-derived account handles as insufficient for ownership enforcement.
- Why:
  - Character-name-derived identity allows ownership checks to be bypassed by presenting a known character name.
- Impact:
  - `BL-012` acceptance criteria now requires account-authenticated session flow before character operations.
  - `S1-P2` packet scope/acceptance is updated to implement account-first auth semantics.
- Revisit trigger:
  - If a full external auth provider/session model replaces current local account/session handling.

## 2026-02-21 - Enforce `koad-os` branch scope for Koad/agent support artifacts
- Decision:
  - Reserve `koad-os` for Koad/agent support and workflow-governance artifacts.
  - Block those artifacts on non-`koad-os` PRs so release-lane PRs stay focused on product/runtime delivery.
  - Add required status check `validate-koad-os-scope` and include it in branch-protection guidance.
- Why:
  - Mixed support/process updates and feature code in the same release-lane PRs increase review noise and integration risk.
  - Enforced branch/file scope keeps delivery lanes clean while preserving a dedicated stream for agent-learning/process evolution.
- Impact:
  - Added `STD-010` to standards registries and sprint-plan standards list.
  - Added CI workflow `.github/workflows/koad-os-scope-gate.yml`.
  - Updated onboarding/policy docs to route Koad/agent support edits through `koad-os`.
- Revisit trigger:
  - If branch strategy changes to a different support branch name or policy engine.

## 2026-02-21 - `koad-os` sync exception + lightweight branch governance
- Decision:
  - Allow support-file promotion PRs from `koad-os` into `v1` (and future release lines), constrained to Koad/agent support scope.
  - Keep `koad-os` governance lightweight to permit occasional direct maintainer commits.
  - Add PR persona-signature field enforcement for clearer authorship attribution in mixed human/agent flows.
- Why:
  - Previous branch-scope rule blocked `koad-os` -> `v1` promotion PRs.
  - User requires flexibility for manual updates on `koad-os` while preserving clean release-lane scope and audit clarity.
- Impact:
  - `validate-koad-os-scope` now permits `koad-os` source sync PRs with support-only files.
  - Branch-protection runbook now sets strict requirements on `v1` and lightweight guidance for `koad-os`.
  - PR template/gate now requires a filled `Persona signature` line.
- Revisit trigger:
  - If repository governance moves to signed commits/attestations with automated identity verification.

## 2026-02-21 - Multi-role saveup continuity model
- Decision:
  - Adapt `saveup` to be role-aware across `Koad (PM)`, `Gameplay`, `Platform`, and `Experience`.
  - Require saveup ledger rows to include `role` and `context_ref`.
  - Enforce role-boundary mirror behavior during saveup (`Koad (PM)` may mirror `.agents/*`; team roles log proposed PM deltas in ops logs without direct reprioritization).
- Why:
  - Shared continuity records without role/context attribution become ambiguous in multi-role execution and reduce recovery quality.
- Impact:
  - `SAVEUP_PROTOCOL.md`, `SAVEUP_CALLS.md`, and session log template now require role/context metadata.
  - Standards registries include `STD-011` to keep role-aware saveup behavior durable.
- Revisit trigger:
  - If saveup automation is introduced with explicit runtime role/session context capture.
## 2026-02-21 - Add shared Koad OS workflow CLI (`.koad/scripts/koad`)
- Decision:
  - Add one shared CLI entrypoint for high-frequency Koad OS operations.
  - Initial commands are `lane-start`, `pr-open`, and role-aware `saveup`.
- Why:
  - These actions are repeated across PM and team-role lanes and benefit from consistent, non-interactive automation.
- Impact:
  - Lane/worktree setup, PR template population, and saveup logging become faster and more standardized.
  - Scripted output aligns with existing branch/worktree and PR-governance rules.
- Revisit trigger:
  - If team workflow adds merge-queue automation or expands command set beyond current three actions.

## 2026-02-21 - Add continuously refreshed root progress dashboard
- Decision:
  - Maintain a root `PROJECT_PROGRESS.md` as a generated status dashboard aligned to roadmap + backlog + sprint/queue state.
  - Add `koad progress-sync` command and wire `koad saveup` to refresh the dashboard by default.
- Why:
  - User needs a low-friction, always-available view of project progress against roadmap intent.
- Impact:
  - Repository now has one canonical quick-view progress artifact.
  - `STD-012` codifies dashboard continuity and sourcing rules.
- Revisit trigger:
  - If dashboard generation should move to CI or external PM tooling instead of local script sync.

## 2026-02-21 - Scope-gate exception for root progress dashboard
- Decision:
  - Treat `PROJECT_PROGRESS.md` as an in-scope Koad/agent support artifact under `koad-os` branch-scope policy.
- Why:
  - The new dashboard is intentionally a support/process artifact, and scope gate failures would block normal `koad-os` sync PRs.
- Impact:
  - `validate-koad-os-scope` now allows `PROJECT_PROGRESS.md` on `koad-os` PRs and `koad-os`-sourced sync PRs.
  - Scope-policy docs/standards now explicitly include the dashboard file.
- Revisit trigger:
  - If progress reporting moves out of repository into external PM tooling.

## 2026-02-21 - Add compact `koad status` progress command
- Decision:
  - Add `koad status` command to print a one-line project progress summary from `PROJECT_PROGRESS.md`.
  - Support `--refresh` so status can regenerate dashboard before output when needed.
- Why:
  - User requested a faster terminal-first view of roadmap progress without opening markdown manually.
- Impact:
  - Progress checks now become a single command (`bash .koad/scripts/koad status`).
  - Script docs now include status command usage.
- Revisit trigger:
  - If status output needs structured machine format (e.g., JSON) for external dashboards.

## 2026-02-21 - Activate S2-P1 as next dispatch packet
- Decision:
  - Advance active queue state to dispatch `S2-P1` (Platform) as the next implementation lane.
  - Keep `S2-E1` queued and dependency-gated until `S2-P1` merges to `v1`.
- Why:
  - `S1-P2` is merged, so the next critical-path M2 work is deterministic scene queue + scene protocol (`BL-004`, `BL-002`).
- Impact:
  - Operator shortcut now maps directly to user-preferred command style:
    - `Your next task is S2-P1.`
  - Queue docs and sprint status notes are aligned to current dispatch order.
- Revisit trigger:
  - If urgent defects or branch constraints force reordering ahead of `S2-P1`.

## 2026-02-21 - Advance queue to S2-E1 after S2-P1 merge
- Decision:
  - Mark `S2-P1` complete and merged to `v1`.
  - Promote `S2-E1` to active next dispatch packet.
- Why:
  - `S2-P1` dependency is satisfied, so `S2-E1` is now the immediate M2 follow-on lane.
- Impact:
  - Operator dispatch shortcut is now:
    - `Your next task is S2-E1.`
  - Backlog and progress dashboard now reflect completed `BL-002`/`BL-004` and updated focus window.
- Revisit trigger:
  - If regression/production defects from S2-P1 require hotfix reprioritization before S2-E1 starts.

## 2026-02-22 - Advance queue to S3-G1 after S2-E1 merge
- Decision:
  - Mark `S2-E1` complete and merged to `v1`.
  - Promote `S3-G1` to active next dispatch packet with `S3-P1` and `S3-E1` dependency-queued behind it.
- Why:
  - S2 scene-render baseline is complete, so the roadmap now enters S3 encounter-loop implementation.
- Impact:
  - Queue and packet docs now support immediate Gameplay dispatch:
    - `Your next task is S3-G1.`
  - Backlog/progress artifacts reflect `BL-003` done and updated Now/Next focus.
- Revisit trigger:
  - If urgent post-S2 stabilization issues force short-term reprioritization.

## 2026-02-22 - Enable lane-isolated saveup journaling for developer lanes
- Decision:
  - Default team-role lane saveups to lane-scoped journals instead of shared global saveup ledgers.
  - Keep global saveup ledgers for PM/global mode and explicit forced global calls.
- Why:
  - Shared saveup files create avoidable merge conflicts when multiple developer lanes run in parallel.
- Impact:
  - `koad saveup` now supports lane-isolated mode and auto-selects it for team-role `lane/...` contexts.
  - Lane journals are kept as local/gitignored continuity artifacts to stay outside feature PR scope.
  - Standards/startup/agent docs now include lane-isolated saveup expectations (`STD-013`).
- Revisit trigger:
  - If saveup reconciliation is fully automated and global ledgers can be updated conflict-free from all lanes.

## 2026-02-22 - Advance queue to S3-P1 after S3-G1 merge
- Decision:
  - Mark `S3-G1` complete and merged to `v1` (PR #13).
  - Promote `S3-P1` to active next dispatch packet.
  - Keep `S3-E1` queued pending `S3-P1` protocol/interface confirmation.
- Why:
  - S3 Gameplay foundation is landed (`BL-005`, `BL-015`), so the critical path now moves to deterministic timeout/fallback infrastructure (`BL-006`).
- Impact:
  - Operator dispatch shortcut is now:
    - `Your next task is S3-P1.`
  - Backlog/progress artifacts now reflect `BL-005` and `BL-015` as `done`.
- Revisit trigger:
  - If merge follow-up defects from S3-G1 require temporary Gameplay hotfix prioritization ahead of S3-P1.

## 2026-02-22 - Enforce saveup tracked-artifact branch scope on `koad-os`
- Decision:
  - Treat tracked saveup artifacts (`.koad/**` saveup ledgers/logs and `PROJECT_PROGRESS.md`) as `koad-os`-only commit scope.
  - Enforce this at tooling level by blocking global-ledger saveup writes outside `koad-os`.
- Why:
  - Team-role developer lanes should not carry tracked `.koad/**` continuity artifacts in feature branches.
  - Policy-only guidance is insufficient; CLI enforcement prevents accidental branch-scope drift.
- Impact:
  - `koad saveup` now errors when global-ledger mode is invoked off `koad-os`.
  - `koad saveup --sync-progress-in-lane` now requires `koad-os` because it writes `PROJECT_PROGRESS.md`.
  - Saveup/standards/agent prompt docs now explicitly direct developer lanes to lane-isolated journals and `koad-os` reconciliation for tracked artifacts.
- Revisit trigger:
  - If saveup artifacts are migrated to an external continuity store and no longer tracked in git.

## 2026-02-22 - Codex execution path standard set to WSL paths
- Decision:
  - Standardize Codex team-role prompt workspace paths to WSL mount paths for this repository (`/mnt/c/data/ttrpg`).
  - Retire Windows-native `C:\...` path guidance for Codex prompts.
- Why:
  - Codex agents in this environment execute inside WSL2 (Ubuntu), so Windows-native paths are error-prone and can fail resolution.
- Impact:
  - Updated Codex task packet/path prompts to `/mnt/c/data/ttrpg`.
  - Updated memory/preferences/facts to preserve the environment assumption for future sessions.
- Revisit trigger:
  - If execution moves from WSL2 to a Windows-native shell environment.

## 2026-02-22 - Formalize review sequence and rename final approval gate
- Decision:
  - Standardize lane review sequence as: coding agent self-review, Koad git review, Ian GitHub review, then merge.
  - Rename final PR-body review gate checkbox from `User GitHub review approved` to `Ian review approved`.
  - Enforce self-review evidence in governance checks via `Coding agent self-review completed`.
- Why:
  - User requested explicit reviewer ownership and ordered gate progression for PR closure.
- Impact:
  - PR template + governance workflow now require the updated review-gate labels and self-review checkbox.
  - PM/team operating docs now align on `Koad git review + Ian review` terminology and ordered review flow.
  - `koad pr-open` now emits the self-review checkbox as checked and supports `--ian-approved` (with `--user-approved` alias retained for compatibility).
- Revisit trigger:
  - If final approver identity or review sequencing policy changes again.

## 2026-02-22 - Reduce koad-os sync friction with automation + non-blocking review checkboxes
- Decision:
  - Add automated `v1` -> `koad-os` synchronization workflow (`.github/workflows/sync-koad-os-from-v1.yml`) triggered on pushes to `v1`.
  - Change `validate-pr-governance` to require review-gate line presence, not checked-state enforcement.
  - Keep checkbox states as human audit metadata while relying on GitHub approvals + required checks as authoritative merge gates.
- Why:
  - Manual koad-os maintenance and checkbox-driven CI failures were creating avoidable delivery drag.
- Impact:
  - `koad-os` stays near-current with `v1` without repeated manual merge choreography.
  - Conflict scenarios route to explicit `v1` -> `koad-os` sync PR handling instead of silent branch drift.
  - PR body governance checks remain structured while reducing merge friction from administrative checkbox timing.
- Revisit trigger:
  - If automated sync causes recurring conflict churn or if branch-scope policy is simplified further.

## 2026-02-22 - Advance queue to S3-E1 after S3-P1 merge
- Decision:
  - Mark `S3-P1` complete and merged to `v1` (PR #17).
  - Promote `S3-E1` to active next dispatch packet.
- Why:
  - Turn timer/timeout fallback infrastructure (`BL-006`) is landed, so the immediate follow-on is Experience turn tracker delivery (`BL-007`).
- Impact:
  - Operator dispatch shortcut is now:
    - `Your next task is S3-E1.`
  - Queue/backlog/progress artifacts now reflect `BL-006` as `done` and `S3-E1` as active next.
- Revisit trigger:
  - If post-merge S3-P1 defects require urgent Platform hotfixes before S3-E1 starts.
