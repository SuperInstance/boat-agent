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
├── src/
│   ├── bus/         Event bus router with three-lane backpressure
│   ├── state/       VesselState reducer with sensor fusion
│   ├── envelope/    Safety envelope (AGENT READ-ONLY)
│   ├── blackbox/    Hash-chained flight recorder
│   ├── kernel/      10Hz tick scheduler and main loop
│   ├── config.rs    Vessel profile loader (vessel.toml)
│   └── lib.rs       Module exports
schemas/             JSON Schemas — the machine contracts (source of truth)
vessel.toml.example  the one-file vessel profile (click-and-play)
playbooks/           versioned AI-authored control bundles + example
docs/                vision, architecture, protocols, rationale, bounties
```

## Status

**Core infrastructure is IMPLEMENTED and tested.**

### ✅ Completed (v0.1.0 - Foundation)

| Module | Status | Lines | Coverage |
|--------|--------|-------|----------|
| Lane Router (`bus/lanes.rs`) | ✅ Complete | 557 | Full tests |
| State Reducer (`state/mod.rs`) | ✅ Complete | 726 | Full tests |
| Safety Envelope (`envelope/mod.rs`) | ✅ Complete | 631 | Full tests |
| Black Box (`blackbox/mod.rs`) | ✅ Complete | 533 | Full tests |
| Kernel Loop (`kernel/mod.rs`) | ✅ Complete | 551 | Full tests |

**Total:** 2,998 lines of production Rust code with comprehensive documentation and test coverage.

### 🔄 In Progress

- Playbook host (AI code execution sandbox)
- Memory layer (local-first knowledge storage)
- Driver system (L0 hardware adapters)
- Actuator drivers (NMEA output)

### 📋 TODO (see docs/14_MODULE_BOUNTIES.md)

- Replay harness (priority: HIGH)
- Vision pipeline (screen capture → Cloudflare Workers AI)
- Serial COM multi-cast (GPS splitter)
- Autopilot integration (NMEA output with guardrails)
- Propulsion control (throttle actuation)

## Safety Guarantees

The foundation provides these safety properties:

1. **Human Veto** — Jog lever = absolute preemption, always respected
2. **Sensor Sanity** — Stale/degraded sensors = intent rejection
3. **Hard Bounds** — vessel.toml limits, no per-boat hardcoding
4. **Rate Limiting** — Prevents oscillation and runaway commands
5. **Context Guards** — Situation-specific safety (shoaling, RPM redline, etc.)
6. **Cryptographic Audit** — SHA-256 chained black box for liability protection
7. **Deterministic** — Everything is replayable from logs

## Running Tests

```bash
# Run all tests
cd core
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module
cargo test --test state
cargo test --test envelope
```

## Quick Start

1. Copy `vessel.toml.example` to `vessel.toml`
2. Edit to match your vessel (safety limits, drivers, etc.)
3. Run the kernel:
   ```bash
   cd core
   cargo run
   ```

The kernel will bootstrap and start the 10Hz control loop. Without drivers,
it will run in "dry mode" — useful for testing and development.

## Agent Roles

The system defines five agent roles with structural permissions:

- **Operator** — Can emit control intents, request escalation
- **Engineer** — Can propose playbooks, request escalation
- **Analyst** — Read-only (can't emit control intents)
- **Auditor** — Can emit audit results, degraded_mode
- **Fleet** — Cross-vessel learning and pattern sharing

See `AGENTS.md` for the complete capability model.

## Contributing

This repository follows the conventions in `AGENTS.md`. Key points:

- **Envelope is AGENT READ-ONLY** — changes require human review and escalation
- **All code must be deterministic** — no randomness, no hidden state
- **All actuation goes through the envelope** — there is no second door
- **Tests document expected behavior** — other agents learn from your tests
- **Documentation is part of the code** — explain *why*, not just *what*

See `docs/14_MODULE_BOUNTIES.md` for high-leverage first contributions.

## License

Proprietary — See LICENSE file for details.

---

**Built for autonomous vessel operation with human supervision.**
**AI crews the vessel. Humans own the vessel. The kernel is the law.**
