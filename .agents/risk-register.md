# Risk Register

Status: Active
Last updated: 2026-02-21
Owner: Project Manager

## Scale

- Probability: `Low`, `Medium`, `High`
- Impact: `Low`, `Medium`, `High`

## Active Risks

| ID | Risk | Probability | Impact | Owner Team | Mitigation Plan | Trigger/Indicator | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R-001 | Rules ambiguity causes inconsistent server behavior | Medium | High | Gameplay | Convert all adapted mechanics to explicit tie-breaker rules and tests | Frequent edge-case regressions or contradictory behavior reports | Open |
| R-002 | Scene simulation complexity slows delivery | High | High | Gameplay | Phase features: V1 single occupancy + basic terrain, defer advanced rules | M2 slips due to pathing/collision edge cases | Open |
| R-003 | Multiplayer sync bugs under concurrent actions | Medium | High | Platform | Deterministic queue, integration tests with concurrent command streams | Divergent client views or order-dependent outcomes | Open |
| R-004 | Protocol churn breaks client-server compatibility | Medium | Medium | Platform | Version protocol structs and gate breaking changes | Frequent client decode errors after merges | Open |
| R-005 | Terminal UX becomes hard to read in encounters | Medium | Medium | Experience | Define render conventions and run usability playtests each milestone | Players miss turn state or map meaning | Open |
| R-006 | AI-driven NPC cost/perf spikes when introduced | Medium | Medium | Platform | Event-triggered calls, per-scene budgets, scripted fallback policy | Latency spikes or budget overrun in encounter scenes | Open |
| R-007 | Scope creep before first full playable loop | High | High | PM | Enforce hard scope cuts and milestone gates | New subsystem work starts before current gate closure | Open |
| R-008 | Single-save campaign model risks irreversible progress loss | Medium | High | Platform | Snapshot/backup cadence, atomic writes, restore drill in CI/manual ops | Save corruption or failed restart after crash | Open |
| R-009 | Campaign-lock and account binding errors can orphan characters | Medium | High | Platform | Explicit lock-state schema, validation on join, and permission-gated audited admin unlock tools | Player cannot join despite valid character or wrong campaign binding | Open |
| R-010 | Combat quit/disconnect policy can be exploited or feel unfair | Medium | High | Gameplay | Define staged deterministic fallback (defensive rounds -> limited AI), reconnect windows, and abuse playtests | Players intentionally disconnect to avoid consequences | Open |
| R-011 | Split-feed client mode increases UX scope and maintenance burden | Medium | Medium | Experience | Keep terminal-first baseline as default; feature-flag richer client surfaces | UI regressions or delayed milestones due to client complexity | Open |
| R-012 | Pull-radius misconfiguration causes missing or over-inclusive encounter participants | Medium | Medium | Gameplay | Provide validated scene profile defaults, config linting, and encounter pull diagnostics in logs | Unexpected participant lists in combat start events | Open |

## Closed Risks

Move resolved risks here with closure date and outcome summary.

## Review Cadence

- Weekly PM review:
  - Re-evaluate probability/impact
  - Update mitigation progress
  - Escalate top 3 risks in milestone notes
