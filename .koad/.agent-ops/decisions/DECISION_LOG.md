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
