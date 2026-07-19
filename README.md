# boat-agent

**A Vessel Intelligence Operating System — designed to be operated by
agents, supervised by humans.**

AI agents crew the vessel all day: one drives within its authority, one
writes and tests control code, one mines the logs, one watches the
watchers. The human captain sets the autonomy dial, answers structured
escalations, teaches by doing, and holds the physical veto. AI authors
control code; a small trusted Rust kernel executes it deterministically;
a hash-chained black box records everything; any trip can be replayed.

> **Agents are the crew. The human is the owner. The kernel is the law.**

## Architecture in one breath

A policy-free microkernel — fixed-tick loop, typed three-lane event bus,
single authoritative vessel state, and a safety envelope that owns every
actuator write — surrounded by replaceable modules: hardware drivers,
AI-authored playbooks (staged trust gate: static analysis → replay →
human approval → shadow → active), a local-first memory layer, and five
agent roles. The UI is just another client.

## Where to start

| You are | Start here |
|---|---|
| An agent working in this repo | **[AGENTS.md](AGENTS.md)** — prime directives, conventions, definition of done |
| Anyone wanting the philosophy | [docs/04_AGENT_CENTRIC_VISION.md](docs/04_AGENT_CENTRIC_VISION.md) |
| An engineer building the core | [docs/05_KERNEL_ARCHITECTURE.md](docs/05_KERNEL_ARCHITECTURE.md) |
| Reviewing *why* it's built this way | [docs/12_DESIGN_RATIONALE.md](docs/12_DESIGN_RATIONALE.md) |
| Looking for work | [docs/13_OPEN_QUESTIONS.md](docs/13_OPEN_QUESTIONS.md) · [docs/14_MODULE_BOUNTIES.md](docs/14_MODULE_BOUNTIES.md) |
| Porting legacy docs 00–03 | [docs/11_MIGRATION_MAP.md](docs/11_MIGRATION_MAP.md) |

## Repository layout

```
AGENTS.md            operating manual for agents — read first
core/                Rust microkernel (the only trusted code)
schemas/             JSON Schemas — the machine contracts (source of truth)
vessel.toml.example  the one-file vessel profile (click-and-play)
playbooks/           versioned AI-authored control bundles + example
docs/                vision, architecture, protocols, rationale, bounties
```

## Status

Architecture and scaffolding phase. The kernel skeleton
(`core/src/`) defines the real types and contracts with `todo!()` bodies;
docs 04–14 are the governing design. Legacy docs 00–03 are preserved for
history. See [docs/14](docs/14_MODULE_BOUNTIES.md) for the highest-leverage
first build (the replay harness).
