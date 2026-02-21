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
