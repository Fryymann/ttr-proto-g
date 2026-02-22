# Gemini CLI - ttrpg Project Base of Operations

## Project Mandates

1.  **koadOS Integration**:
    - Consult `~/.koad-os/core` for system-wide rules.
    - Treat `/home/ideans/data/ttrpg/.koad` as the primary source of truth for project configurations.
    - Adhere to the standards defined in `.koad/.standards/standards_registry.md`.

2.  **Context & Continuity**:
    - Store project-specific context in `.gemini/context/`.
    - Record significant events and decisions in `.gemini/learning/journal.md`.
    - Record structured patterns in `.gemini/learning/patterns.json`.

3.  **Governance Compliance**:
    - **STD-001 (Change Safety)**: Prefer small, verifiable increments.
    - **STD-005 (Execution Gate)**: Do not execute sprints without explicit user authorization.
    - **STD-006 (Role Boot)**: Confirm role before substantial work. Current role: Gemini CLI (Support/Automation).
    - **STD-010 (Scope Separation)**: Keep Koad/agent support artifacts on the `koad-os` branch.

## Project Context
- **Domain**: Multiplayer, scene-based, turn-based text RPG.
- **Stack**: Rust + Tokio.
- **Planning**: Artifacts in `.agents/`.

## Workspace
- Root: `/home/ideans/data/ttrpg`
- Koad OS: `~/.koad-os`
