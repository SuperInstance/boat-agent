# boat-agent

**A deckhand with perfect memory. Talks back. Never sleeps. Working on the water today.**

*For the working deck. Not a tech project. A second mate.*

---

It's 1 PM, you're running gear, gloves on, the rail's wet with chum. You don't stop. You just say it:

**"Twelve chum over the rail. Two of them bruisers."**

And you go back to work.

A week later you're idling in the same water, coffee in hand, and you ask: **"How'd we do here last Tuesday?"** It tells you — what you caught, when the bite turned, what the tide was doing, what the sounder looked like. You didn't write anything down. You didn't open a laptop. You talked.

That's the whole point of this thing.

You hire a deckhand who never sleeps, never complains, and never touches the wheel without asking. He watches. He remembers. He writes it down so you don't have to. When he's sure, he suggests. When he's not, he asks. The wheel is still yours. Always.

---

## Where this is today

> **Looking for the working software?** The sounder watch, memory twin,
> and scrubber UI live in the vessel-side repo:
> **[tzpro-agent](https://github.com/SuperInstance/tzpro-agent)** — start
> at its [`FIRST_BOAT.md`](https://github.com/SuperInstance/tzpro-agent/blob/master/FIRST_BOAT.md).
> This repo (boat-agent) is the kernel, contracts, and design system.

**Working on the water right now:**
- **Sounder watch (LEVEL 2):** The daemon runs live on F/V EILEEN. Every minute it notes what changed. Every ten minutes it saves a record. End of day, a full briefing. The scrubber UI lets you replay any day. No logbook to keep; it writes itself.

**Coming next season:**
- **Voice mate (LEVEL 1):** Memory is ready — transcripts filed by time and GPS. The end-to-end voice conversation (talk to it, it talks back) is getting wired in now.
- **Eyes in the engine room (LEVEL 3):** Designed. Forward cam for logs and buoys in the dark. IR cam for hot spots before they're failures.
- **Wake-word autopilot (LEVEL 4):** Designed. Say the word, he steers. Your hand on the wheel outranks everything, always.
- **Build-it-together (LEVEL 5):** Designed. He interviews you, you two sketch the plan, he hands you a wiring diagram for YOUR boat and code you can read.

Straight talk: the sounder watch works today. The rest is in the water and coming. We're not overselling it.

---

## What you get, plain

**1. A mate with perfect memory.** Talk to him like a deckhand. Mid-set, gloves on. He writes everything down as a transcript, stamped with the time and the GPS spot in the file name. Ask him anything later — "how was last week off the point?" "what did the sounder look like Tuesday dawn?" "show me every time I said birds working last month." He searches it all. The AI model doesn't matter. The memory is the product, and the memory lives on your boat. Even if you never switch on anything else on this page, that alone earns its keep.

**2. A mate who watches your screens.** You know how you stare at the sounder all day, keeping the picture in your head? He does that now. Every minute he notes what changed. Every ten minutes he saves a short record. Once an hour, a one-screen summary on your phone. End of trip, a full debrief. You never save a file. You never name a folder. He does — and because the location's in the filename, "show me every Tuesday off the rock" just works. The sounder watch and your voice notes are one searchable pile, grouped by place. You're not keeping a logbook anymore. You're just fishing, and the logbook writes itself.

**3. A mate with eyes in the engine room.** A cam facing forward — logs, buoys, boats coming at you in the dark. A second set of eyes on the bow when you're on the gear, not the water ahead. An IR cam in the engine room — hot spots before they're failures, a leak before it's a story, bilge level creeping up at 3 AM at the dock. It texts you. Same memory, same conversation: "any alarms last night?" is a question you can ask over coffee.

**4. A mate you call by name.** Say the word, he's listening. **"Take the helm"** — he steers. **"I've got her"** — you're driving again. No buttons to find with wet gloves. Your hand on the wheel or the throttle outranks everything he thinks, every time. That's not a setting. That's the design.

**5. A mate you build with.** Need the throttle to handle itself while you troll — easing up when you turn into the wind, back down when you fall off? Tell him. He interviews you like a sharp deckhand would — cable or hydraulic? Micro Commander? chain and sprocket? how long's the run from helm to engine? — and you two sketch the plan together. He helps you pick the parts and order them. When the box arrives, he hands you a wiring diagram drawn for YOUR boat and the code to match — code you can read, in plain English, with the reasons written in. You wrench it in. You're the hands; he's the head. The result is a boat that trolls the way YOU troll, because he learned from watching you do it. And he remembers how he built it — next season, when something needs adjusting, you ask.

---

## What he will NEVER do

- Drive without you at the wheel.
- Override your hand on the wheel or the throttle.
- Delete a day's record without reading it once.
- Push your boat's data anywhere without you saying so first.
- Pretend he knows when he doesn't.

If you grab the wheel, he goes quiet and politely asks why. Out loud. Even a grunt is teaching him.

---

## Getting started

This is crew-run software, not shrink-wrap. Getting it going takes a runbook, not a download. But once it's running, it works.

1. **Talk to us.** We'll walk you through what fits your boat.
2. **Plug in your GPS** — and the sounder screen, when you're ready for level 2.
3. **Say something.** "Start the log." That's it — he's remembering.

The dial starts at **1 — Coach**: he suggests, you decide. Move it up when he's earned it, trip by trip. Slow is fine. The boat is yours.

Cameras, the autopilot, and the building-stuff-together part come later, when you want them. Everything works offline; the cloud is a backup, not a requirement.

*If something here doesn't match what the boat does, the boat is right and this page is wrong. Tell us.*

---

---

# For the technical crew

**A Vessel Intelligence Operating System — designed to be operated by
agents, supervised by humans.** The page above is the product; below is
the machine.
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

**Core infrastructure is implemented, reviewed, and tested** (31/31 unit
tests passing, boot smoke test verified — see
[docs/15_CODE_REVIEW.md](docs/15_CODE_REVIEW.md) for the review record,
including safety-critical bugs found and fixed after the initial
implementation).

### ✅ Completed (v0.1.0 - Foundation)

| Module | Status | Notes |
|--------|--------|-------|
| Lane Router (`bus/lanes.rs`) | ✅ Complete | coalescing + capability tests |
| State Reducer (`state/mod.rs`) | ✅ Complete | deterministic, replay-pure |
| Safety Envelope (`envelope/mod.rs`) | ✅ Complete | regression-tested (Coach dial, clamped-command checks) |
| Black Box (`blackbox/mod.rs`) | ✅ Complete | chain resume + verification tested |
| Kernel Loop (`kernel/mod.rs`) | ✅ Complete | boot smoke test passes |

Known open weaknesses are tracked explicitly in
[docs/15_CODE_REVIEW.md](docs/15_CODE_REVIEW.md) (W-1 through W-9) —
read them before building on these modules.

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
