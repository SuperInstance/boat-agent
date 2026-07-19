# AGENTS.md — Operating Manual for Agents Working in This Repository

> **Read this first.** This repository is designed to be *operated by agents*.
> The humans are in the loop — they are not the operators. Every document,
> schema, and interface here is written for machine consumption first and
> human review second.

---

## What this project is

A **Vessel Intelligence Operating System**: a Rust microkernel (`core/`) that
runs a commercial fishing vessel through a typed event bus, a single
authoritative vessel state, a hard safety envelope, and versioned,
AI-authored **playbooks**. AI agents write control code; the kernel executes
it deterministically; a cryptographic black box records everything; a human
supervises through an autonomy dial.

The governing philosophy, in one line:

**Agents are the crew. The human is the owner. The kernel is the law.**

---

## The prime directives (apply to every agent, every task)

1. **The safety envelope is untouchable.** No agent — including you — may
   modify, bypass, weaken, or route around `core/src/envelope/`. If a task
   seems to require it, stop and escalate to the human. This is the one
   non-negotiable rule in the repository.
2. **Actuation is earned, never assumed.** New control logic enters service
   only through the playbook stage-gate (see `docs/07_PLAYBOOK_LIFECYCLE.md`):
   static analysis → replay → human approval → shadow → active. No shortcuts.
3. **Everything is an event.** If you add a capability, it must speak the bus
   protocol (`docs/06_BUS_PROTOCOL.md`). No side channels, no direct
   module-to-module calls across layers.
4. **Determinism over cleverness.** If a behavior cannot be replayed from the
   black box, it does not ship. Prefer boring, inspectable, testable code.
5. **Degrade toward the human.** When confidence is low, sensors disagree, or
   a contract is violated, the correct action is to reduce autonomy and emit
   an escalation event — never to guess harder.
6. **Leave the log better than you found it.** Every agent action that
   changes system behavior must be traceable: what changed, why, under whose
   authority, with what evidence.

---

## Repository map

```
boat-agent/
├── AGENTS.md                      ← you are here
├── docs/
│   ├── 00_ARCHITECTURE_OVERVIEW.md      (legacy 10-phase design — superseded)
│   ├── 01_IMPLEMENTATION_GUIDE.md       (legacy — superseded)
│   ├── 02_RESEARCH_QUESTIONS.md         (living research log — still active)
│   ├── 03_MODULE_REFERENCE.md           (legacy module reference — superseded)
│   ├── 04_AGENT_CENTRIC_VISION.md       ← why the system is built this way
│   ├── 05_KERNEL_ARCHITECTURE.md        ← layers, primitives, the kernel
│   ├── 06_BUS_PROTOCOL.md               ← VesselEvent v2 contract
│   ├── 07_PLAYBOOK_LIFECYCLE.md         ← codegen stage-gate, shadow, replay
│   ├── 08_MEMORY_LAYER.md               ← VesselMemory, vocab, sync
│   ├── 09_HUMAN_IN_THE_LOOP.md          ← autonomy dial, escalation contract
│   ├── 10_AGENT_OPERATIONS.md           ← agent roles, the daily loop, boatctl
│   ├── 11_MIGRATION_MAP.md              ← legacy phases → new architecture
│   ├── 12_DESIGN_RATIONALE.md           ← why each choice; overturn conditions
│   ├── 13_OPEN_QUESTIONS.md             ← architecture-level open work
│   ├── 14_MODULE_BOUNTIES.md            ← claimable modules + generalization tracks
│   ├── 15_CODE_REVIEW.md                ← review log: findings, fixes, open weaknesses
│   ├── 16_ECOSYSTEM_INTEGRATION.md      ← tzpro-agent & sonar-vision family synergy
│   └── 17_CASCADED_PERCEPTION.md        ← three-loop analyzer: M1/M10/H1 + gaze channel
├── core/                          ← Rust microkernel (the only trusted code)
│   └── src/
│       ├── kernel/                scheduler, tick, module lifecycle
│       ├── bus/                   event types, lanes, pub/sub
│       ├── state/                 VesselState reducer (digital twin)
│       ├── envelope/              safety envelope — READ-ONLY for agents
│       ├── drivers/               Layer 0 hardware driver trait + registry
│       ├── playbook/              playbook runner host (supervised process)
│       ├── memory/                VesselMemory trait (local-first store)
│       └── agent_api/             boatctl surface: typed commands for agents
├── schemas/                       ← JSON Schemas — the machine contracts
│   ├── vessel-event.schema.json
│   ├── vessel-profile.schema.json
│   └── playbook-manifest.schema.json
├── vessel.toml.example            ← the one-file vessel profile
└── playbooks/                     ← versioned playbook bundles
    └── examples/trolling_baseline/
```

---

## Conventions for agents editing this repo

- **Schemas are source of truth.** Rust types in `core/src/bus/events.rs`
  derive from `schemas/`. Change the schema first, then regenerate/adjust
  types, then update docs. Never let the three drift.
- **Docs are load-bearing.** Other agents read `docs/` to make decisions.
  When you change behavior, update the doc that describes it in the same
  change. Stale docs are defects.
- **The legacy docs (00–03) are historical.** Do not extend them. Map new
  work through `docs/11_MIGRATION_MAP.md` instead.
- **No human-only affordances.** Any feature must be fully operable through
  the typed agent API (`core/src/agent_api/`). The React UI is a *view* onto
  that API, never a back door around it.
- **Tests are replay-based.** The canonical test harness replays recorded
  black-box trips through the kernel. Unit tests are welcome but replay
  fidelity is the acceptance bar (`docs/07`, §Replay).
- **Constants live in `vessel.toml`.** If you are about to write a magic
  number, it belongs in the vessel profile with a safe default and a comment
  explaining the physical meaning.

## What "done" means for any change

1. Schema/type/doc updated together.
2. Replay tests pass against the recorded-trip corpus.
3. The change is expressible as bus events and agent-API commands.
4. The black box would record enough to reconstruct *why* the system acted.
5. If the change touches actuation: the stage-gate in `docs/07` was followed.
6. **Run the tests. Actually run them.** `cargo test` (and `cargo build`
   for the binary path) must pass in your session before you claim
   anything is "implemented" or "tested". A verification claim without a
   run is fake confidence — the one sin this repo ranks worst (docs/06).
   See docs/15 REVIEW-001 for what happens otherwise.
