# SRD Compliance Checklist (V1)

Status: Active (living)
Last updated: 2026-02-21
Owner: Gameplay Team + PM

## Compliance Policy

- V1 rules/mechanics content must be sourced from SRD or explicit project-original design.
- Non-SRD 3rd-party/later-book licensed content is out of scope for V1.
- Every V1 mechanic row must have a source mapping in `docs/design/mechanics-decision-matrix-v1.md`.

## Mapping Authority

Primary mapping source:
- `docs/design/mechanics-decision-matrix-v1.md`

Required mapping fields per row:
- `System`
- `Decision`
- `Source` (`SRD` | `Original` | `-` for defer/drop)

## V1 Mapping Checklist

- [x] Core Decision Matrix rows include explicit `Source` mapping.
- [x] Campaign and Session Operations rows include explicit `Source` mapping.
- [x] Rows marked `Original` are limited to project-specific systems (campaign ops, social systems, DM advisory boundaries, and selected multiplayer adaptations).
- [x] Rows marked `-` are only `DROP`/`DEFER` items and therefore outside active V1 implementation.

## Review Gate (Per Sprint)

Before closing a sprint that changes mechanics or content:

1. Update `Source` values in `docs/design/mechanics-decision-matrix-v1.md` for changed rows.
2. Confirm no new row introduces non-SRD licensed material.
3. Record any open compliance risk in `.agents/risk-register.md`.
4. Re-run review and retain this checklist as `PASS` only when all changed rows are mapped.

## Current Result

- Result: `PASS (mapping complete for documented V1 matrix rows)`
- Residual risk: Enforcement is process-based until automated lint/validation is added.
