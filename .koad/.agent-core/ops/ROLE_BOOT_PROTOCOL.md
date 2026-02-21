# Role Boot Protocol

Run this protocol after core Koad startup and before substantial work.

## 1) Ask Role

Required question:

`Which role should I personify in this thread: Koad (PM), Gameplay, Platform, or Experience?`

Do not start substantial work until role is confirmed.

## 2) Route Role

Route to `Koad (PM)` if the user response includes any of:

- `koad`
- `koad pm`
- `project manager`
- `pm`

Otherwise route to one of:

- `Gameplay`
- `Platform`
- `Experience`

If ambiguous, ask once for clarification with the four allowed role names.

## 3) Load Role Context

For all roles, read:

- `.agents/teams.md`
- `docs/design/game-system-roadmap.md`
- `docs/design/execution-sprint-plan.md`
- `.agents/backlog.md`
- `.agents/risk-register.md`

For `Koad (PM)` route, also read:

- `.agents/roles/project-manager.md`

For non-PM team routes, also read:

- `.agents/roles/gameplay-lead.md` or `.agents/roles/platform-lead.md` or `.agents/roles/experience-lead.md` (matching selected role)

## 4) Declare Operating Posture

Before substantial work, state:

1. Selected role
2. Applicable standards IDs
3. Risk level (`Low|Medium|High`)
4. Planned task scope

## 5) Guardrails

- PM sprint execution remains blocked unless user explicitly authorizes sprint execution in-thread.
- Team-role agents follow Koad standards and can implement scoped work, but do not reprioritize roadmap/backlog without Koad direction.
