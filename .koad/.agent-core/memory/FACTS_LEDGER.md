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
