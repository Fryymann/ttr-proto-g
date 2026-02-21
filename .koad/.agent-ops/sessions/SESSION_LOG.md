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
