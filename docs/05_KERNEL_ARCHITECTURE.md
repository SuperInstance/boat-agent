# 05 — Kernel Architecture

> **Target Audience:** Agents and engineers building or modifying the core.
> **Purpose:** The structural contract: layers, primitives, data flow, and
> the rules about what may talk to what.
> **Status:** Governing document — supersedes `00_ARCHITECTURE_OVERVIEW.md`

---

## The shape of the machine

The system is a **microkernel with five primitives**. The kernel is small,
pure Rust, and trusted. Everything else is a module that speaks the bus.

```
                        ┌──────────────────────────────────────────┐
                        │                 HUMAN                     │
                        │   dial · veto (jog lever) · teach · yes/no│
                        └───────────────┬──────────────────────────┘
                                        │ (also just bus events)
   ┌────────────────────────────────────┼────────────────────────────────────┐
   │ L4  AGENTS        Operator · Engineer · Analyst · Auditor · (Fleet)      │
   │                   propose / author / analyze / verify                    │
   ├─────────────────────────────────────────────────────────────────────────┤
   │ L3  MISSIONS      trolling · transit · anchor-watch · custom             │
   │                   state machine; sets targets & selects playbooks        │
   ├─────────────────────────────────────────────────────────────────────────┤
   │ L2  PLAYBOOKS     AI-authored deterministic rules (supervised sandbox)   │
   │                   pure f(VesselState) → Intent, runs at kernel tick      │
   ├─────────────────────────────────────────────────────────────────────────┤
   │ L1  ENVELOPE      hard bounds · rate limits · watchdog · human override  │
   │                   owns ALL actuator writes · pure Rust · agent-read-only │
   ├─────────────────────────────────────────────────────────────────────────┤
   │ L0  DRIVERS       NMEA-0183 · N2K/CAN · RTSP · screen-cap · jog lever    │
   │                   normalize hardware ⇄ bus events; containable panics    │
   └─────────────────────────────────────────────────────────────────────────┘
                     ▲                                      │
                     │            ┌──────────────────┐      │
                     └────────────│   THE BUS (typed,│◄─────┘
                                 │   priority lanes)│
                                 └────────┬─────────┘
                                          │
                    ┌─────────────────────┼─────────────────────┐
                    ▼                     ▼                     ▼
             ┌────────────┐       ┌──────────────┐       ┌────────────┐
             │ VESSEL     │       │ BLACK BOX    │       │ MEMORY     │
             │ STATE      │       │ (hash-chained│       │ (local-    │
             │ (reducer,  │       │  append-only │       │  first,    │
             │  1 source) │       │  replayable) │       │  synced)   │
             └────────────┘       └──────────────┘       └────────────┘
```

## The laws of layering

1. **Authority decreases upward.** L1 can veto L2–L4. L2 cannot refuse L1.
2. **Speed decreases upward.** L0/L1 run at kernel tick (10–50 Hz). L2 at
   tick. L3 at human timescales. L4 whenever it has something worth saying.
3. **No sideways calls.** Modules communicate only through bus events and
   the shared state snapshot. A module that imports another module's
   internals is a defect.
4. **Actuation has exactly one door.** Every physical output flows:
   `Playbook → Intent event → Envelope → Driver → hardware`.
   There is no second path. The envelope is the sole owner of actuator
   file handles/ports.
5. **Agents never touch L0/L1 code.** The envelope directory is read-only
   for agents (`AGENTS.md`, prime directive 1). Drivers may be *added* by
   agents but cannot modify the envelope's write path.

## The five primitives

### 1. The Bus (`core/src/bus/`)

Typed, priority-laned pub/sub. Three lanes:

| Lane | Class | Semantics | Examples |
|------|-------|-----------|----------|
| **critical** | safety | preempts; processed before next tick completes | jog-lever move, watchdog trip, thermal runaway |
| **telemetry** | state | sampled, coalesced into the tick reducer | GPS, RPM, depth, wind |
| **narrative** | log/analysis | never blocks; batched | sonar interpretation, agent reasoning, sync |

Full contract in `docs/06_BUS_PROTOCOL.md`.

### 2. Vessel State (`core/src/state/`)

One authoritative struct — the digital twin. A pure reducer folds telemetry
events into state each tick:

```
state_{t+1} = reduce(state_t, events_t)     // deterministic, no I/O
```

Everything reads the same snapshot: playbooks, missions, envelope sanity
checks, the UI (via diff subscription), and the black box. There is no
second copy of the truth anywhere in the system.

### 3. The Safety Envelope (`core/src/envelope/`)

The vessel's reflexes. Receives every `Intent`, returns
`Approved(command) | Rejected(reason) | Clamped(command, reason)`, and owns
actuator writes. Enforces:

- hard physical bounds (rudder ±, throttle range, RPM redline)
- rate limits (per-second deltas, cooldowns)
- the hardware watchdog (heartbeat from kernel; miss → safe state + buzzer)
- human override detection (jog lever = absolute preemption)
- the autonomy dial ceiling (dial 1 → all actuation becomes advisory)

Configuration comes from `vessel.toml` — never hardcoded per boat, never
weakened by any agent.

### 4. Playbooks (`core/src/playbook/`, `playbooks/`)

AI-authored, versioned, human-approved control bundles. A playbook is a
directory: manifest (`playbook.toml`), pure-function rules (Python today,
WASM-ready interface), provenance (voice transcripts, confidence), and
replay-test evidence. The kernel runs the **active** playbook in a
supervised child process: pure `f(state) → intent`, fuel-limited,
heartbeat-monitored, killed and rolled back on fault.

Lifecycle (stage-gate) in `docs/07_PLAYBOOK_LIFECYCLE.md`.

### 5. Memory (`core/src/memory/`)

Local-first store behind a trait: vocabulary, catch logs, playbook
registry, behavioral patterns, calibration. Syncs opportunistically to the
cloud edge when bandwidth exists. The boat is fully intelligent offline;
the cloud is an amplifier, not a dependency. See `docs/08_MEMORY_LAYER.md`.

## The kernel itself (`core/src/kernel/`)

Deliberately boring. Four jobs:

1. **Tick.** Fixed-rate loop (10 Hz default). Per tick: drain telemetry
   lane → reduce state → run active playbook (non-blocking IPC; stale
   result → hold last safe command) → envelope arbitration → actuator
   flush → black box append.
2. **Schedule.** Critical lane preempts between any two steps above.
3. **Supervise.** Module lifecycle: drivers, playbook host, sidecars.
   Crash = restart with backoff, envelope holds safe state meanwhile.
4. **Record.** Every tick that produces an actuation appends a black-box
   entry with full provenance (state hash, intent, envelope verdict,
   authority basis).

The kernel contains **no policy**. It doesn't know what trolling is. It
knows what safe is.

## Data flow: one tick of trolling

```
 1. NMEA driver: GPS + compass sentences        → telemetry lane
 2. N2K driver:  engine RPM (PGN 127488)        → telemetry lane
 3. Reducer: fold into VesselState snapshot #48211
 4. Mission (trolling): current targets {sog: 2.3kn, track: 147°}
 5. Playbook process: f(snapshot, targets) → Intent{throttle: +2%, rudder: -3°}
 6. Envelope: bounds ✓ rate ✓ dial=3 ✓ watchdog ✓ → Approved
 7. Drivers: write $GPAPB to autopilot port; step throttle servo
 8. Black box: append entry {snapshot_hash, intent, verdict, dial, actor}
 9. UI: receives state diff; captain sees intent vs. action, live
10. Bus narrative lane: Analyst agent logs agreement stat for shadow metrics
```

If the playbook process hangs at step 5: kernel uses last safe command,
envelope counts the miss, Auditor escalates after N misses, mission layer
degrades. The boat never waits on AI.

## Why this is modular in practice

- **A driver is ~150 lines behind a trait.** Add a sensor = add a driver
  that emits valid events. No other file changes.
- **A playbook is a folder.** Share it, diff it, roll it back, replay it.
- **An agent role is a consumer.** The Analyst could be rewritten in any
  language that speaks JSON-over-IPC; the kernel wouldn't know.
- **The UI is a view.** React subscribes to state diffs and renders.
  Everything it can do, `boatctl` can do — agents are never second-class.
- **The kernel is replaceable in principle.** Because policy lives in
  playbooks and profiles, the trusted core is small enough to audit in an
  afternoon. That is the real safety argument.

## Portability

Domain core (bus, state, envelope, playbook host) has zero platform
dependencies — no Tauri, no Win32. Adapters (serial, screen capture,
shell) sit at L0/edges. The same kernel runs: Tauri shell on the wheelhouse
laptop (today), headless on a Raspberry Pi (near future), or in the replay
simulator on CI (always).

---

**Next:** `06_BUS_PROTOCOL.md` — the event contract every module signs.
