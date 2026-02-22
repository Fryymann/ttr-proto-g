# Ops Session Log

## YYYY-MM-DD - session title
- Scope:
- Changes:
- Evidence:

## 2026-02-21 - Koad PM role adaptation
- Scope:
  - Adapt `.koad` operating system to Project Manager role and workspace layout.
- Changes:
  - Normalized Koad path references to `.koad/...`.
  - Added PM standards/source requirements and startup context for `.agents` artifacts.
  - Seeded memory files with current project facts/preferences/patterns.
- Evidence:
  - Updated docs under `.koad/AGENTS.md`, `.koad/.agent-core/*`, `.koad/.agent-ops/*`.

## 2026-02-21 - Saveup continuity checkpoint
- Scope:
  - Persist PM/Koad operational continuity after design and OS integration updates.
- Changes:
  - Logged durable lessons, patterns, preferences, and facts.
  - Finalized saveup ledger entry and aligned core/ops session records.
- Evidence:
  - Updated `.koad/.agent-core/sessions/SAVEUP_CALLS.md` and memory ledgers.

## 2026-02-21 - IAN decision integration for V1 scope constraints
- Scope:
  - Convert latest `IAN.md` decisions into actionable PM governance and planning artifacts.
- Changes:
  - Updated backlog for SRD-only V1 scope, advisory DM-agent behavior, and party-based encounter participation.
  - Updated risk register to reflect SRD compliance and advisory-boundary failure risks.
  - Logged durable product decisions in Koad decision records.
- Evidence:
  - Updated `.agents/backlog.md`, `.agents/risk-register.md`, and `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - Roadmap and system outline baseline
- Scope:
  - Create a living game-system roadmap to centralize foundations, flows, sequencing, and focus.
- Changes:
  - Added `docs/design/game-system-roadmap.md` as primary roadmap reference.
  - Aligned existing design docs to party-based encounter participation, SRD-only V1 scope, and advisory DM-agent boundaries.
  - Added roadmap link in root `README.md` for discoverability.
- Evidence:
  - Updated `docs/design/game-system-roadmap.md`, `docs/design/combat-framework-outline.md`, `docs/design/mechanics-decision-matrix-v1.md`, and `README.md`.

## 2026-02-21 - Execution sprint plan baseline
- Scope:
  - Convert roadmap strategy into concrete delivery slices with owner/file/test focus for M2-M3.
- Changes:
  - Added `docs/design/execution-sprint-plan.md`.
  - Added backlog `Active Focus Window` tied to immediate `Now` and `Next` priorities.
  - Added root README pointer for sprint plan discoverability.
- Evidence:
  - Updated `docs/design/execution-sprint-plan.md`, `.agents/backlog.md`, and `README.md`.

## 2026-02-21 - Antigravity sprint orchestration integration
- Scope:
  - Integrate Antigravity as execution layer and formalize Koad PM prompt-packet responsibilities.
- Changes:
  - Added root prompt pack `ANTIGRAVITY_SPRINT_PROMPTS.md` with starter prompts for S1-S4 lanes.
  - Updated PM role, team handoff contract, and execution sprint plan to include Antigravity lane mapping.
  - Updated memory and decision logs for durable workflow continuity.
- Evidence:
  - Updated `ANTIGRAVITY_SPRINT_PROMPTS.md`, `.agents/roles/project-manager.md`, `.agents/teams.md`, `docs/design/execution-sprint-plan.md`, and Koad memory/ops logs.

## 2026-02-21 - Sprint S1-P1 campaign bootstrap and lock seam scaffold
- Scope:
  - Resume execution from planning into Sprint S1-P1 implementation for campaign startup and persistence/account scaffolding.
- Changes:
  - Added campaign manifest loader and explicit active-campaign selection flow (`--campaign` / `TTRPG_CAMPAIGN_ID` / interactive prompt).
  - Added in-memory account/character persistence scaffolding with campaign-lock metadata and join-time lock enforcement seam.
  - Integrated create/login path with persistence validation and updated operator docs + sample campaign manifest.
- Evidence:
  - Updated `crates/ttrpg-server/src/main.rs`, `crates/ttrpg-server/src/campaign/mod.rs`, `crates/ttrpg-server/src/account/mod.rs`, `crates/ttrpg-server/src/persistence/mod.rs`, `campaigns/manifest.json`, and `README.md`.
  - Verified with `cargo check` and `cargo test -p ttrpg-server` (6 tests passing).

## 2026-02-21 - Operating guardrail for sprint execution authorization
- Scope:
  - Enforce a Koad operating-system guardrail that blocks PM sprint implementation unless explicitly requested by user.
- Changes:
  - Added `STD-005` sprint execution authorization gate to standards registry.
  - Added explicit sprint-authorization requirements to bootstrap/startup, identity, PM role, and kit README guidance.
  - Updated user preference memory to reflect sprint execution ownership boundary.
- Evidence:
  - Updated `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/AGENTS.md`, `.koad/.agent-core/ops/STARTUP_CHECKLIST.md`, `.koad/.agent-core/IDENTITY.md`, `.agents/roles/project-manager.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/README.md`.

## 2026-02-21 - Documentation update for S1-P1 completion
- Scope:
  - Update planning artifacts to reflect completion status of Sprint S1 lane `S1-P1`.
- Changes:
  - Marked `BL-011` as `done` and removed it from `Now` active focus.
  - Added explicit `S1-P1` completion status note in execution sprint plan with remaining scope callout for `BL-012`.
  - Annotated `ANTIGRAVITY_SPRINT_PROMPTS.md` `S1-P1` block with completion status and follow-up boundaries.
- Evidence:
  - Updated `.agents/backlog.md`, `docs/design/execution-sprint-plan.md`, and `ANTIGRAVITY_SPRINT_PROMPTS.md`.

## 2026-02-21 - Antigravity Windows path alignment
- Scope:
  - Align Antigravity prompt and PM guidance with Windows-layer path resolution.
- Changes:
  - Updated sprint prompt pack to use `C:\data\ttrpg` workspace paths and added explicit path-translation guidance.
  - Updated PM role workflow and user preference memory to enforce Windows path form for Antigravity delegation packets.
- Evidence:
  - Updated `ANTIGRAVITY_SPRINT_PROMPTS.md`, `.agents/roles/project-manager.md`, and `.koad/.agent-core/memory/USER_PREFERENCES.md`.

## 2026-02-21 - Antigravity onboarding tightening for lane alignment
- Scope:
  - Strengthen Antigravity coder-lane onboarding and completion criteria to reduce scope drift on shared branch/worktree execution.
- Changes:
  - Added lane context contract, mandatory onboarding command/evidence gate, and required handoff quality gate to prompt pack.
  - Updated PM role workflow to require onboarding acknowledgement and acceptance checklist evidence before lane closure.
  - Added new standards entry (`STD-006`) and AGENTS operational rule for Antigravity onboarding enforcement.
- Evidence:
  - Updated `ANTIGRAVITY_SPRINT_PROMPTS.md`, `.agents/roles/project-manager.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, and `.koad/AGENTS.md`.

## 2026-02-21 - S1-G1 review remediation and closure sync
- Scope:
  - Remediate S1-G1 review findings and align tracker/docs to actual completed gameplay-lane deliverables.
- Changes:
  - Wired `scene` module into server crate so occupancy unit tests compile/run with normal server test commands.
  - Replaced SRD checklist content with evidence-oriented mapping gate tied to mechanics matrix source columns.
  - Updated sprint/backlog/prompt status to mark `BL-001` and `BL-019` done and annotate `S1-G1` as completed.
  - Removed stray `mod.pdb` artifact from workspace root.
- Evidence:
  - Updated `crates/ttrpg-server/src/main.rs`, `docs/design/srd-compliance-checklist-v1.md`, `.agents/backlog.md`, `docs/design/execution-sprint-plan.md`, and `ANTIGRAVITY_SPRINT_PROMPTS.md`.
  - Verified with `cargo test -p ttrpg-server` (9 passing, includes `scene::*` tests) and filtered `test_occupancy_rules` run.

## 2026-02-21 - Git worktree policy rollout for coder-lane parallelism
- Scope:
  - Configure agent workflow to use git worktrees for parallel coder lanes and codify PR strategy without team-structure changes.
- Changes:
  - Added one-lane-per-worktree/branch policy and PR metadata requirements to Antigravity prompt pack and PM execution docs.
  - Added `STD-007` worktree-isolated lane execution standard and propagated policy across Koad bootstrap/identity/startup/memory docs.
  - Updated role handoff checklists to include branch/worktree/PR evidence for coder-agent work.
- Evidence:
  - Updated `ANTIGRAVITY_SPRINT_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `.agents/roles/*.md`, `.agents/teams.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.agent-core/ops/STARTUP_CHECKLIST.md`, `.koad/.agent-core/IDENTITY.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `.koad/README.md`, `.koad/.agent-core/memory/WORKING_MEMORY.md`, and `.koad/.agent-core/memory/USER_PREFERENCES.md`.

## 2026-02-21 - Saveup continuity checkpoint after worktree policy rollout
- Scope:
  - Execute saveup protocol after workflow-policy propagation to preserve continuity for future PM/coder-lane sessions.
- Changes:
  - Registered saveup call, performed duplicate checks, and updated memory ledgers with policy-propagation learning/pattern/facts.
  - Added core session log checkpoint for this saveup execution.
- Evidence:
  - Updated `.koad/.agent-core/sessions/SAVEUP_CALLS.md`, `.koad/.agent-core/sessions/LOG.md`, `.koad/.agent-core/memory/LEARNINGS.md`, `.koad/.agent-core/memory/PATTERNS.md`, `.koad/.agent-core/memory/FACTS_LEDGER.md`, and `.koad/.agent-core/memory/USER_PREFERENCES.md`.

## 2026-02-21 - Codex role-instance pivot and boot-sequence update
- Scope:
  - Pivot development orchestration from Antigravity to multi-instance Codex role agents and update repository boot behavior.
- Changes:
  - Added role-routing boot protocol requiring startup role question and route-specific context loading.
  - Added active `CODEX_ROLE_PROMPTS.md` and marked `ANTIGRAVITY_SPRINT_PROMPTS.md` paused.
  - Updated standards, PM/team-role docs, and sprint-plan execution model to Codex role-instance orchestration.
  - Updated Koad memory artifacts to reflect the execution-model pivot and role-selection requirements.
- Evidence:
  - Updated `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.agent-core/IDENTITY.md`, `.koad/.agent-core/ops/STARTUP_CHECKLIST.md`, `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `.agents/roles/*.md`, `.agents/teams.md`, `.agents/README.md`, `README.md`, and memory ledgers under `.koad/.agent-core/memory/`.

## 2026-02-21 - PR-required dual-review merge workflow activation
- Scope:
  - Configure coder-lane workflow so tasks close only after reviewed and merged PRs.
- Changes:
  - Updated Codex role prompt pack, PM role definition, team handoff contracts, and sprint execution plan to enforce PR-required dual review (`Koad git review` + `user GitHub review`) before closure.
  - Added standards entry `STD-009` and boot-rule references for merged-PR completion gating.
  - Updated user preference memory and Koad ops decision log for durable continuity.
- Evidence:
  - Updated `CODEX_ROLE_PROMPTS.md`, `.agents/teams.md`, `.agents/roles/project-manager.md`, `.agents/roles/gameplay-lead.md`, `.agents/roles/platform-lead.md`, `.agents/roles/experience-lead.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `.koad/AGENTS.md`, `AGENTS.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, and `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - PR governance template and branch-protection enforcement pack
- Scope:
  - Add enforceable PR governance artifacts for dual-review merge workflow.
- Changes:
  - Added `.github/pull_request_template.md` with required handoff metadata and review-gate checkboxes.
  - Added `.github/workflows/pr-template-gate.yml` (`validate-pr-governance`) to validate PR body structure and review gates.
  - Added `docs/ops/github-branch-protection.md` with exact GitHub branch protection settings for `master`.
  - Updated PM/standards docs to reference template, workflow check, and branch-protection runbook.
- Evidence:
  - Updated `CODEX_ROLE_PROMPTS.md`, `.agents/roles/project-manager.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Added `.github/pull_request_template.md`, `.github/workflows/pr-template-gate.yml`, `docs/ops/github-branch-protection.md`.

## 2026-02-21 - Release-line branch policy update to `v1`
- Scope:
  - Align agent lane workflow and governance docs to use `v1` as the active release base branch.
- Changes:
  - Updated lane workflow docs to require branching from `v1` and PR targets into `v1`.
  - Updated branch-protection runbook target branch from `master` to `v1`.
  - Updated standards, preferences, and learnings with release-line branching behavior.
- Evidence:
  - Updated `CODEX_ROLE_PROMPTS.md`, `.agents/roles/project-manager.md`, `docs/design/execution-sprint-plan.md`, `docs/ops/github-branch-protection.md`, `.github/pull_request_template.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-core/memory/LEARNINGS.md`, and `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - Codex lane task packets published for immediate queue
- Scope:
  - Prepare actionable developer-agent handoff packets for the current `Now`/`Next` queue on `v1`.
- Changes:
  - Added active packet queue in `CODEX_ROLE_PROMPTS.md` with packet IDs, ownership, dependencies, and suggested lane branches.
  - Published detailed packet briefs for `S1-P2` (`BL-012`), `S2-P1` (`BL-004` + `BL-002`), and `S2-E1` (`BL-003`).
  - Updated sprint-plan status notes to record packet publication and dependency order.
- Evidence:
  - Updated `CODEX_ROLE_PROMPTS.md` and `docs/design/execution-sprint-plan.md`.

## 2026-02-21 - BL-012 scope correction to account-first login semantics
- Scope:
  - Clarify active Platform lane requirements after review finding on ownership enforcement bypass risk.
- Changes:
  - Updated `BL-012` to require account-authenticated login before character operations.
  - Updated `S1-P2` task packet objective/scope/acceptance to enforce account-scoped character management.
  - Updated sprint-plan S1 platform validation gate and status notes to reflect account-first auth requirement.
  - Logged durable PM decision for account-first authentication policy.
- Evidence:
  - Updated `.agents/backlog.md`, `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, and `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - Koad-OS branch-scope enforcement rollout
- Scope:
  - Enforce separation between Koad/agent support updates and runtime feature delivery lanes.
- Changes:
  - Added `.github/workflows/koad-os-scope-gate.yml` (`validate-koad-os-scope`) to enforce branch/file scope policy on all PRs.
  - Updated bootstrap/policy docs to reserve `koad-os` for Koad/agent support artifacts and keep development lanes focused on feature/runtime changes.
  - Added standards entry `STD-010` in both standards registries and updated sprint-plan standards list + status notes.
  - Updated branch-protection runbook to require `validate-koad-os-scope` for both `v1` and `koad-os`.
  - Updated user preference memory for durable continuity.
- Evidence:
  - Updated `AGENTS.md`, `.koad/AGENTS.md`, `CODEX_ROLE_PROMPTS.md`, `docs/ops/github-branch-protection.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Added `.github/workflows/koad-os-scope-gate.yml`.

## 2026-02-21 - Koad-OS promotion path + persona-signature governance refinement
- Scope:
  - Unblock `koad-os` -> `v1` support-sync PR flow and align governance to user preference for lighter `koad-os` restrictions.
- Changes:
  - Updated `.github/workflows/koad-os-scope-gate.yml` to allow support-only PRs when source branch is `koad-os`.
  - Added `Persona signature` field to PR template and validation in `validate-pr-governance`.
  - Updated branch-protection runbook to keep `v1` strict and `koad-os` lightweight/direct-commit friendly.
  - Updated standards/bootstrap/memory artifacts to encode sync exception and lightweight `koad-os` expectation.
- Evidence:
  - Updated `.github/workflows/koad-os-scope-gate.yml`, `.github/pull_request_template.md`, `.github/workflows/pr-template-gate.yml`, `docs/ops/github-branch-protection.md`, `CODEX_ROLE_PROMPTS.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, and `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - Role-aware saveup protocol rollout
- Scope:
  - Adapt saveup continuity workflow for all Koad OS roles (`Koad (PM)`, `Gameplay`, `Platform`, `Experience`).
- Changes:
  - Updated saveup protocol with role preconditions, role/context call metadata, role-focused learning extraction, and role-boundary mirror rules.
  - Updated saveup ledger schema to include `role` and `context_ref`; backfilled existing entries.
  - Updated saveup session-log template to include role/context fields.
  - Added `STD-011` to standards registries and updated sprint-plan standards list/status notes.
  - Updated startup/bootstrap/readme and preference memory to encode role-aware saveup usage.
- Evidence:
  - Updated `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`, `.koad/.agent-core/sessions/SAVEUP_CALLS.md`, `.koad/.agent-core/sessions/LOG.md`, `.koad/.agent-core/ops/STARTUP_CHECKLIST.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/README.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
## 2026-02-21 - Shared Koad workflow CLI scaffold
- Scope:
  - Implement shared scripts for frequently repeated Koad OS actions.
- Changes:
  - Added `.koad/scripts/koad` wrapper and `.koad/scripts/koad_cli.py` command implementation.
  - Implemented commands:
    - `lane-start`: create lane branch/worktree and emit onboarding evidence.
    - `pr-open`: generate governance-template PR body and call `gh pr create`.
    - `saveup`: append role-aware saveup ledger row and session log entry.
  - Added usage guide `.koad/scripts/README.md` and linked script purpose in `.koad/README.md`.
  - Ran smoke checks for command help and dry-run paths.
- Evidence:
  - Added `.koad/scripts/koad_cli.py`, `.koad/scripts/koad`, `.koad/scripts/README.md`.
  - Updated `.koad/README.md`.
  - Verified with:
    - `.koad/scripts/koad --help`
    - `.koad/scripts/koad lane-start --help`
    - `.koad/scripts/koad pr-open --help`
    - `.koad/scripts/koad saveup --help`
    - dry-run calls for each command.

## 2026-02-21 - Root project progress dashboard automation
- Scope:
  - Provide an easy, continuously refreshed roadmap-vs-progress view in repo root.
- Changes:
  - Added `koad progress-sync` command to generate `PROJECT_PROGRESS.md` from canonical planning sources.
  - Wired `koad saveup` to refresh `PROJECT_PROGRESS.md` by default (`--no-progress-sync` to skip).
  - Added script docs and propagated continuity rules into AGENTS/standards/sprint-plan artifacts.
  - Added standards entry `STD-012` for dashboard continuity.
- Evidence:
  - Updated `.koad/scripts/koad_cli.py`, `.koad/scripts/README.md`, `.koad/README.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Generated `PROJECT_PROGRESS.md` via `koad progress-sync`.

## 2026-02-21 - Scope-gate allowlist update for progress dashboard
- Scope:
  - Resolve `validate-koad-os-scope` failure for root progress dashboard updates.
- Changes:
  - Added `PROJECT_PROGRESS.md` to scope-gate allowlist in `.github/workflows/koad-os-scope-gate.yml`.
  - Updated branch-scope policy wording in AGENTS + standards docs to explicitly include root dashboard artifact.
  - Logged durable decision for dashboard scope treatment.
- Evidence:
  - Updated `.github/workflows/koad-os-scope-gate.yml`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - Add compact status command for progress dashboard
- Scope:
  - Provide one-command terminal summary of current roadmap-vs-progress state.
- Changes:
  - Added `koad status` command to print compact snapshot from `PROJECT_PROGRESS.md`.
  - Added `--refresh` option to regenerate dashboard before status output.
  - Updated script docs and Koad readme utility command list.
- Evidence:
  - Updated `.koad/scripts/koad_cli.py`, `.koad/scripts/README.md`, `.koad/README.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Verified with `bash .koad/scripts/koad status` and `bash .koad/scripts/koad status --refresh`.

## 2026-02-21 - Queue status refresh and next packet dispatch setup
- Scope:
  - Advance PM queue after merged support PR and publish next task handoff shortcut.
- Changes:
  - Updated active queue statuses in `CODEX_ROLE_PROMPTS.md` (`S2-P1` set to active next; `S2-E1` marked queued on dependency).
  - Added explicit operator dispatch shortcuts matching user-preferred style.
  - Added sprint status note recording post-merge queue advance.
  - Logged durable queue decision in decision log.
- Evidence:
  - Updated `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-21 - Post-S2-P1 merge queue + backlog progression
- Scope:
  - Reflect S2-P1 completion and activate S2-E1 as the next dispatch lane.
- Changes:
  - Updated packet queue statuses in `CODEX_ROLE_PROMPTS.md` (`S2-P1` done, `S2-E1` active next).
  - Updated operator dispatch shortcut to `Your next task is S2-E1.`
  - Updated backlog states (`BL-002` + `BL-004` to `done`) and re-ranked focus window.
  - Added sprint status note and durable queue decision log entry.
- Evidence:
  - Updated `.agents/backlog.md`, `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-22 - Post-S2-E1 merge queue + status progression
- Scope:
  - Reflect S2-E1 completion and advance active queue into S3.
- Changes:
  - Marked `BL-003` as `done` in backlog and updated focus window.
  - Updated queue statuses (`S2-E1` done, `S3-G1` active next, `S3-P1`/`S3-E1` queued on dependency).
  - Added/updated S3 task packets in role prompt pack and sprint status notes.
  - Regenerated root progress dashboard.
- Evidence:
  - Updated `.agents/backlog.md`, `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `PROJECT_PROGRESS.md`.

## 2026-02-22 - Lane-isolated saveup prep for parallel developer lanes
- Scope:
  - Reduce saveup merge conflicts during concurrent team-role lane execution.
- Changes:
  - Updated `koad saveup` to support lane-isolated journaling and default it for team-role `lane/...` contexts.
  - Added lane saveup journal path `.koad/.agent-core/sessions/lane-saveups/` as local gitignored artifact storage.
  - Updated saveup protocol, startup guidance, standards, and agent docs for the new mode.
- Evidence:
  - Updated `.koad/scripts/koad_cli.py`, `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`, `.koad/.agent-core/ops/STARTUP_CHECKLIST.md`, `.koad/.agent-core/sessions/SAVEUP_CALLS.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/AGENTS.md`, `AGENTS.md`, `.koad/scripts/README.md`, `.koad/README.md`.

## 2026-02-22 - Post-S3-G1 merge queue + status progression
- Scope:
  - Reflect S3-G1 completion and prepare next task dispatch.
- Changes:
  - Updated backlog states (`BL-005`, `BL-015` -> `done`) and re-ranked focus window.
  - Updated queue statuses (`S3-G1` done, `S3-P1` active next, `S3-E1` queued after S3-P1 interface confirmation).
  - Updated sprint status notes and durable queue decision log entry.
  - Regenerated root progress dashboard snapshot.
- Evidence:
  - Updated `.agents/backlog.md`, `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-core/memory/FACTS_LEDGER.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`, `PROJECT_PROGRESS.md`.

## 2026-02-22 - Saveup tracked-artifact scope enforcement (`koad-os` only)
- Scope:
  - Enforce user-requested branch-scope rule for developer saveup behavior around tracked `.koad/**` artifacts.
- Changes:
  - Added CLI guard in `.koad/scripts/koad_cli.py` so global-ledger saveup writes are blocked outside `koad-os`.
  - Added CLI guard for `--sync-progress-in-lane` to require `koad-os` because it writes `PROJECT_PROGRESS.md`.
  - Updated saveup protocol, standards registries, AGENTS docs, and Codex role prompts to require tracked saveup artifacts to be committed via `koad-os`.
  - Recorded explicit user preference for saveup tracked-artifact branch scope.
- Evidence:
  - Updated `.koad/scripts/koad_cli.py`, `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `AGENTS.md`, `.koad/AGENTS.md`, `CODEX_ROLE_PROMPTS.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Verified script compiles: `python3 -m py_compile .koad/scripts/koad_cli.py`.

## 2026-02-22 - Experience lane S2-E1 continuity checkpoint
- Scope:
  - Preserve role-scoped continuity for Experience lane delivery of CLI scene rendering (`S2-E1` / `BL-003`).
- Changes:
  - Landed scene renderer module and wired `SceneSnapshot`/`SceneDelta` paths to terminal map rendering with stable `@`/`P`/`N` symbols.
  - Added Experience-focused learning for multiplayer identity hydration in scene-symbol classification.
  - Opened Experience lane PR for review/merge against `v1`.
- Evidence:
  - Updated `crates/ttrpg-client-cli/src/render_scene.rs`, `crates/ttrpg-client-cli/src/main.rs`, `.koad/.agent-core/memory/LEARNINGS.md`.
  - PR: `https://github.com/Fryymann/ttr-proto-g/pull/11`.

## 2026-02-22 - Codex WSL pathing correction for prompt pack and core memory
- Scope:
  - Correct Codex execution environment assumptions after user clarification that Codex runs inside WSL2.
- Changes:
  - Updated Codex prompt pack workspace paths from `C:\data\ttrpg` to `/mnt/c/data/ttrpg`.
  - Updated core memory artifacts (`WORKING_MEMORY`, `USER_PREFERENCES`, `LEARNINGS`, `FACTS_LEDGER`) to encode WSL pathing as the durable default for Codex agents.
  - Logged durable PM decision for WSL path standardization.
- Evidence:
  - Updated `CODEX_ROLE_PROMPTS.md`, `.koad/.agent-core/memory/WORKING_MEMORY.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-core/memory/LEARNINGS.md`, `.koad/.agent-core/memory/FACTS_LEDGER.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-22 - Review workflow update (self-review -> Koad -> Ian)
- Scope:
  - Apply user-requested PR review sequence and approval-label updates across governance enforcement and PM/team operating docs.
- Changes:
  - Updated PR template review gates to include `Coding agent self-review completed` and renamed final gate to `Ian review approved`.
  - Updated `validate-pr-governance` workflow requirements to enforce the new checkbox set.
  - Updated PM/team governance docs and standards to replace `user GitHub review` language with `Ian review` and describe ordered review flow.
  - Updated `koad pr-open` body generator to emit self-review as checked and support `--ian-approved` (retaining `--user-approved` alias).
  - Updated user preference memory to reflect the new review sequence.
- Evidence:
  - Updated `.github/pull_request_template.md`, `.github/workflows/pr-template-gate.yml`, `docs/ops/github-branch-protection.md`, `CODEX_ROLE_PROMPTS.md`, `.agents/roles/project-manager.md`, `.agents/roles/platform-lead.md`, `.agents/roles/gameplay-lead.md`, `.agents/roles/experience-lead.md`, `.agents/teams.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/scripts/koad_cli.py`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Verified CLI syntax with `python3 -m py_compile .koad/scripts/koad_cli.py`.

## 2026-02-22 - koad-os sync friction reduction rollout
- Scope:
  - Reduce recurring branch-maintenance overhead while preserving koad-os support-branch governance.
- Changes:
  - Added workflow automation to sync `koad-os` from `v1` on every push (`.github/workflows/sync-koad-os-from-v1.yml`), including conflict fallback PR creation (`v1` -> `koad-os`).
  - Updated PR governance check to require review-gate line presence rather than checked-state enforcement.
  - Updated scope-gate allowlist for the new sync workflow and added `v1` -> `koad-os` sync exception.
  - Updated PM/operator docs and prompts to reflect automated sync and informational checkbox semantics.
  - Updated standards source/registry and user preference memory for durable continuity.
- Evidence:
  - Updated `.github/workflows/sync-koad-os-from-v1.yml`, `.github/workflows/pr-template-gate.yml`, `.github/workflows/koad-os-scope-gate.yml`, `.github/pull_request_template.md`, `docs/ops/github-branch-protection.md`, `CODEX_ROLE_PROMPTS.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Updated `.koad/.agent-core/memory/WORKING_MEMORY.md`, `.koad/.agent-core/memory/FACTS_LEDGER.md`, `.koad/.agent-core/memory/LEARNINGS.md`.

## 2026-02-22 - Post-PR17 PM status refresh and S3-E1 dispatch prep
- Scope:
  - Sync PM planning artifacts after `S3-P1` merge and prepare the next task queue state.
- Changes:
  - Updated backlog focus window and marked `BL-006` as `done`.
  - Updated queue state in `CODEX_ROLE_PROMPTS.md` (`S3-P1` done, `S3-E1` active next) and refreshed S3-E1 dependency text.
  - Added sprint status note for PR #17 merge progression in `docs/design/execution-sprint-plan.md`.
  - Logged durable queue decision and refreshed standards sync manifest timestamp.
- Evidence:
  - Updated `.agents/backlog.md`, `CODEX_ROLE_PROMPTS.md`, `docs/design/execution-sprint-plan.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`, `.koad/.standards/sync_manifest.json`, `PROJECT_PROGRESS.md`.

## 2026-02-22 - Merge-driven dashboard automation rollout
- Scope:
  - Replace per-change dashboard maintenance with post-merge automation on `v1`.
- Changes:
  - Added `.github/workflows/update-v1-dashboard.yml` to regenerate dashboard content and upsert managed issue `V1 Project Dashboard` after every push to `v1`.
  - Added new workflow to scope-gate allowlist and updated branch-protection runbook docs.
  - Updated standards/startup/saveup/docs so dashboard publishing is merge-driven and `saveup` progress sync is opt-in.
  - Updated memory to capture new user preference and operating model.
- Evidence:
  - Updated `.github/workflows/update-v1-dashboard.yml`, `.github/workflows/koad-os-scope-gate.yml`, `docs/ops/github-branch-protection.md`, `.koad/scripts/koad_cli.py`, `.koad/.agent-core/ops/SAVEUP_PROTOCOL.md`, `.koad/scripts/README.md`, `.koad/README.md`, `AGENTS.md`, `.koad/AGENTS.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `CODEX_ROLE_PROMPTS.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-core/memory/WORKING_MEMORY.md`, `.koad/.agent-core/memory/LEARNINGS.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-22 - koad-os promotion PR lifecycle automation rollout
- Scope:
  - Offload recurring PM overhead for support-branch promotion into `v1`.
- Changes:
  - Added `.github/workflows/promote-koad-os-to-v1.yml` to auto-create/update one managed `koad-os` -> `v1` promotion PR when effective deltas exist, and auto-close it when no deltas remain.
  - Added new workflow to `.github/workflows/koad-os-scope-gate.yml` allowlist.
  - Updated branch-protection/runbook docs, standards, startup guidance, prompt pack, and memory to describe merge-flow automation.
- Evidence:
  - Updated `.github/workflows/promote-koad-os-to-v1.yml`, `.github/workflows/koad-os-scope-gate.yml`, `docs/ops/github-branch-protection.md`, `CODEX_ROLE_PROMPTS.md`, `.koad/.standards/standards_registry.md`, `.koad/.agent-ops/STANDARDS_REGISTRY.md`, `.koad/AGENTS.md`, `AGENTS.md`, `.koad/.agent-core/memory/USER_PREFERENCES.md`, `.koad/.agent-core/memory/WORKING_MEMORY.md`, `.koad/.agent-core/memory/LEARNINGS.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.

## 2026-02-22 - PM PR gate review script offload (`koad pr-gate`)
- Scope:
  - Offload repeated PM PR review checks and checkbox updates into a single scripted command.
- Changes:
  - Added `pr-gate` subcommand to `.koad/scripts/koad_cli.py` to evaluate mergeability, required check state, branch-scope policy, and required PR evidence markers.
  - Added optional `--apply-koad-approved` flow to check `Koad git review approved` in PR body when verdict is `approve`.
  - Added optional `--comment` flow for standardized Koad PM approval comment posting.
  - Updated script docs to include the new command and usage examples.
- Evidence:
  - Updated `.koad/scripts/koad_cli.py`, `.koad/scripts/README.md`, `.koad/README.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Verified with `python3 -m py_compile .koad/scripts/koad_cli.py`, `bash .koad/scripts/koad pr-gate --help`, and `bash .koad/scripts/koad pr-gate --pr 25 --dry-run`.

## 2026-02-22 - Promotion PR required-check seeding fix
- Scope:
  - Remove missing-check deadlock on workflow-created `koad-os` promotion PRs.
- Changes:
  - Updated `.github/workflows/promote-koad-os-to-v1.yml` with `checks: write` permission and a new step that seeds `validate-pr-governance` + `validate-koad-os-scope` check-runs for managed promotion PR create/update outcomes.
  - Seeded check logic validates governance marker presence and branch-scope policy against live PR body/file list before publishing pass/fail conclusions.
  - Logged durable decision for workflow-created PR check seeding.
- Evidence:
  - Updated `.github/workflows/promote-koad-os-to-v1.yml`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Verified YAML parse with `python3 -c "import yaml; yaml.safe_load(...)"`.

## 2026-02-22 - `pr-gate --watch` check-settle automation
- Scope:
  - Remove manual PR-check polling from PM gate review loop.
- Changes:
  - Extended `.koad/scripts/koad_cli.py` `pr-gate` command with `--watch`, `--watch-timeout-seconds`, and `--poll-seconds`.
  - Added transient-blocker detection so watch mode waits for pending/missing check states but exits immediately on hard blockers.
  - Updated script docs to include watch-mode usage in review flow examples.
- Evidence:
  - Updated `.koad/scripts/koad_cli.py`, `.koad/scripts/README.md`, `.koad/README.md`, `.koad/.agent-ops/decisions/DECISION_LOG.md`.
  - Verified with `python3 -m py_compile .koad/scripts/koad_cli.py`, `bash .koad/scripts/koad pr-gate --help`, and `bash .koad/scripts/koad pr-gate --pr 26 --watch --watch-timeout-seconds 5 --poll-seconds 1`.
