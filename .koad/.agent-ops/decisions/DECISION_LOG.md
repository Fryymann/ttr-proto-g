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
