---
name: koad-os-boot
description: Boot and validate Koad OS context for `/mnt/c/data/ttrpg`, including required startup reads, standards freshness verification, and an explicit user role prompt (`Koad (PM)|Gameplay|Platform|Experience`). Use when a Codex thread starts in this repository, when the user asks to load koadOS, or before substantial role-specific work.
---

# Koad OS Boot

## Overview

Initialize repository-required Koad OS startup flow quickly and consistently.
Run the boot script first, capture user-selected role, then declare operating posture for that role.

## Workflow

1. Run:

```bash
bash .codex/skills/koad-os-boot/scripts/bootstrap_koados_pm.sh
```

2. If script reports missing files or stale standards, stop substantial work and surface blockers.
3. Respond to the script role prompt with one of:
- `Koad (PM)`
- `Gameplay`
- `Platform`
- `Experience`
4. Declare posture before substantial work:
- Selected role
- Applicable standards IDs
- Risk level (`Low|Medium|High`)
- Planned scope

## Outputs

- Verified startup sequence evidence (files + freshness status)
- Role resolved from explicit prompt and declared
- Role-safe scope readiness statement

## Notes

- This skill is repository-specific and assumes workspace root is `/mnt/c/data/ttrpg`.
- Keep sprint implementation blocked in PM mode unless user explicitly authorizes it in-thread.
