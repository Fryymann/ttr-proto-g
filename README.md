# TTRPG M1 Scaffold (Rust)

This repository contains a minimal multiplayer text RPG scaffold:

- `crates/ttrpg-protocol`: shared JSON protocol types
- `crates/ttrpg-server`: Tokio TCP authoritative server
- `crates/ttrpg-client-cli`: minimal terminal client
- `ANTIGRAVITY_SPRINT_PROMPTS.md`: starter prompts for parallel Antigravity sprint lanes
- `docs/design/game-system-roadmap.md`: primary roadmap and system outline (living)
- `docs/design/execution-sprint-plan.md`: implementation sprint sequencing (living)
- `docs/design/combat-framework-outline.md`: living combat/scene architecture plan
- `docs/design/mechanics-decision-matrix-v1.md`: keep/adapt/drop mechanics matrix

## Implemented in M1

- Multi-client server
- Character create/login with interactive editor
- Campaign manifest loading + explicit active campaign selection at server boot
- Commands: `look`, `go`, `say`, `who`, `sheet`, `help`
- Shared room state and room chat
- Line-delimited JSON protocol

## Quick start

1. Install prerequisites (Linux/WSL):

```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config
```

2. Install Rust (`rustup`) and load Cargo into your shell if needed:

```bash
source "$HOME/.cargo/env"
```

3. Run server:

```bash
cargo run -p ttrpg-server -- --campaign greenhollow
```

4. In two terminals, run clients:

```bash
cargo run -p ttrpg-client-cli
cargo run -p ttrpg-client-cli
```

5. In each client:

```text
/create
look
say hello
who
go north
```

## Notes

- Default server address: `127.0.0.1:7000`
- Default campaign manifest: `campaigns/manifest.json`
- If `--campaign` is not provided, the server prompts for selection from the manifest.
- Override with environment variable:

```bash
TTRPG_SERVER_ADDR=127.0.0.1:7001 cargo run -p ttrpg-server
```

- Non-interactive campaign selection:

```bash
TTRPG_CAMPAIGN_ID=greenhollow cargo run -p ttrpg-server
```
