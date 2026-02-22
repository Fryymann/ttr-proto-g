# Facts Ledger

Append-only record of confirmed durable facts.

## 2026-02-21
- Project language/runtime focus: Rust, Tokio server/client architecture.
- PM operational artifacts live under `.agents/`.
- Koad framework is located under `.koad/` in this repository.
- User-approved policy decisions:
  - Campaign-lock is admin-reversible (audited).
  - Disconnect fallback is staged (defensive rounds, then limited AI).
  - Encounter pull uses radius/zone config in V1.
- Koad standards sync script defaults now target `.koad/.standards/sync_manifest.json` and `.koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md`.
- Local standards freshness check is currently `FRESH` using the `.koad`-scoped manifest and required sources.

## 2026-02-21 (decision update)
- Supersedes prior encounter-pull assumption: V1 encounter participation is party-based, not pull-radius/zone based.
- V1 mechanics/content scope is SRD-only plus original project-authored material.
- DM-agent remains advisory by default; scripted narrative/NPC policy is authoritative.

## 2026-02-21 (process update)
- Development workflow now uses Koad as PM and Antigravity agents for parallel sprint execution lanes.
- Root sprint prompt pack lives at `ANTIGRAVITY_SPRINT_PROMPTS.md` and is maintained by PM.

## 2026-02-21 (workflow policy update)
- Parallel coder lanes were standardized to one lane per dedicated git worktree/branch (`lane/<PROMPT_ID>/<scope-slug>`) during Antigravity phase.
- Lane handoffs now require PR metadata with merge dependency ordering, with default one PR per code lane.
- Koad standards include `STD-007` for worktree-isolated lane execution policy enforcement.

## 2026-02-21 (execution model pivot)
- Supersedes Antigravity-as-primary execution model: Antigravity is paused for this repository.
- Active model is Koad PM lead plus three Codex team-role instances (`Gameplay`, `Platform`, `Experience`).
- Codex instances must boot general Koad OS first, then resolve role via `.koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md`.
- Active prompt pack is `CODEX_ROLE_PROMPTS.md`.

## 2026-02-21 (koad-os governance update)
- `koad-os` is the dedicated support branch for Koad/agent and workflow-governance artifacts.
- `koad-os` may remain lightly protected to allow occasional direct maintainer commits.
- Support updates are promoted into the active release line via `koad-os` -> `v1` PRs constrained to support-scope files.
- PR governance now includes a required `Persona signature` line for authorship clarity.

## 2026-02-22 (queue/process update)
- `S2-E1` is merged to `v1`; M2 scene-render baseline is complete and queue entry has moved into S3.
- Saveup flow now supports lane-isolated journals under `.koad/.agent-core/sessions/lane-saveups/` (local gitignored artifacts) to reduce cross-lane merge conflicts.
- `S3-G1` is merged to `v1` (PR #13); backlog items `BL-005` and `BL-015` are complete.
- Active next task packet is `S3-P1` (Platform), with `S3-E1` queued after protocol/interface confirmation.

## 2026-02-22 (execution environment update)
- Codex team-role agents in this repository run inside WSL2 (typically Ubuntu).
- For Codex task packets in this repository, workspace paths should use `/mnt/c/data/ttrpg` rather than `C:\data\ttrpg`.

## 2026-02-22 (governance automation update)
- `koad-os` now auto-syncs from `v1` via `.github/workflows/sync-koad-os-from-v1.yml`.
- PR governance gate validates review-gate line presence, while checkbox states are informational (GitHub approvals + required checks remain authoritative merge gates).
