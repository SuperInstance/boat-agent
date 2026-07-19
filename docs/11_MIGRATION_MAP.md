# 11 — Migration Map: Legacy 10-Phase Design → Kernel Architecture

> **Target Audience:** Agents and engineers porting legacy work.
> **Purpose:** Exact mapping from docs 00–03 concepts to the new
> architecture, with disposition: keep / fold / replace / drop.
> **Status:** Governing document for transition

---

## Reading guide

Nothing in docs 00–03 is thrown away blindly — most of it was right about
*what* and wrong only about *topology*. The table below is the authority
when legacy docs and docs 04–10 disagree.

## Phase-by-phase mapping

| Legacy (docs 00–03) | Disposition | New home |
|---------------------|-------------|----------|
| Phase 1: Tauri shell + sidecar supervisor | **Keep, demoted** | Tauri is an *adapter* around `core/`. Sidecar lifecycle moves to kernel module supervision. SIGKILL-on-close logic stays, generalized to all modules. |
| Phase 2: Setup wizard | **Fold** | Wizard output becomes `vessel.toml`. Steps become hardware auto-discovery (drivers probe ports/baud). Wizard = profile builder UI, nothing more. |
| Phase 3: Vision pipeline | **Keep, rewired** | Screen capture is an L0 driver emitting `sensor.camera.frame_meta` + (via vision worker) narrative-lane interpretation events with vocab applied. Cloudflare Worker endpoints map to Memory sync (`docs/08`). |
| Phase 4: COM multi-cast splicer | **Keep, hardened** | An L0 driver pair (physical-in, virtual-out). Remove `expect()` panics, add lock-conflict detection → escalation (legacy Insight-005 becomes auto-diagnosis). |
| Phase 5: Autopilot guardrails | **Fold into envelope** | `autopilot_guard.rs` logic merges into `core/src/envelope/` steering arbiter. No separate module; there is one door. |
| Phase 6: Propulsion control | **Split** | N2K PGN 127488 parsing → L0 driver. PID/rate limits → envelope + calibration namespace. Adaptive gains → Analyst + replay experiments (legacy RQ-003). |
| Phase 7: Behavioral cloning | **Keep, formalized** | Becomes the Analyst's pattern pipeline with alignment-confidence gating (legacy RQ-001 enforced, not hoped for). Data structure survives as `patterns` namespace entries. |
| Phase 8: Code generation | **Keep, upgraded** | pyo3-in-process → supervised child process (see below). Timestamped backups → content-addressed registry. Ad-hoc execution → stage-gate (`docs/07`). |
| Phase 9: Universal hardware bus | **Promoted** | The bus stops being a module and becomes the architecture. `VesselEvent` upgraded to v2 with lanes, provenance, schema validation (`docs/06`). |
| Phase 10: Black box | **Promoted** | From passive logger to system spine: replay source, audit base, trust-metric substrate. Hash chain kept as-is (it was right). |

## Module-file mapping (legacy `src-tauri/src/`)

| Legacy file | New location | Notes |
|-------------|--------------|-------|
| `main.rs` | `core/src/main.rs` + shell adapter | Builder glue shrinks to adapter wiring. |
| `service.rs` | `core/src/kernel/supervisor.rs` | Generalized from OpenClaw-only to all modules; graceful-shutdown research question resolved: 500 ms grace, then kill, logged. |
| `screenshot.rs` | `core/src/drivers/screencap.rs` | Async, off the main thread; frames go to disk, metadata to bus. |
| `serial.rs` | `core/src/drivers/nmea0183.rs` | + auto-baud sniffing; `RelayState` → driver instance config in `vessel.toml`. |
| `autopilot_guard.rs` | `core/src/envelope/steering.rs` | Depth-trend shoaling check kept; becomes envelope input. |
| `propulsion.rs` | `core/src/drivers/n2k.rs` + `core/src/envelope/throttle.rs` | Parsing vs. limiting separated. |
| `universal_bus.rs` | `core/src/bus/` | Rebuilt around lanes + validation. |
| `blackbox.rs` | `core/src/blackbox/` | Unchanged core logic; now written by kernel every actuation tick. |
| `rollback.rs` | **deleted** | Replaced by playbook registry (`boatctl playbook activate <id>`). |
| `calibration.rs` | `core/src/memory/` (calibration ns) + Analyst | System ID becomes an Analyst job against replay, not a live command. |
| `parser.rs` | playbook tooling | Still renders rules+provenance for UI; reads bundles, not live files. |

## Constants migration

All legacy `const` safety limits move to `vessel.toml` `[envelope]` with
these legacy values as the *defaults* (see `vessel.toml.example`):
rudder ±15°, min autopilot speed 2.0 kn, max trolling RPM 1800, throttle
step ≤15%, trolling ceiling 40%, watchdog 800 ms (Insight-001's 300 ms
question resolved: heartbeat 300 ms, trip threshold 800 ms, configurable).

Hardcoded paths (`C:\Users\Public\openclaw\...`) → profile-relative data
dir. Hardcoded COM ports → discovery + profile. Hardcoded models →
`[agents]` config.

## Cloudflare worker mapping

| Legacy endpoint | New role |
|-----------------|----------|
| `POST /v1/vision/sounder` | Kept; response now arrives on the boat as a narrative event with vocab applied; per-vessel vocab pulled from KV mirror of `vocab` namespace. |
| `POST /v1/memory/train` | Becomes vocab-namespace sync (vocab proposals are Analyst-gated; endpoint accepts only signed digests). |
| `POST /v1/memory/sync` / `query` | Pattern-digest sync to Vectorize; semantic query brokered through `boatctl memory query`. |

## React components mapping

| Legacy component | New role |
|------------------|----------|
| `Wizard.jsx` | Profile builder: discovery results + one-tap confirm → writes `vessel.toml`. |
| `CatchDashboard.jsx` | Unchanged spirit; now a bus subscriber + `boatctl` client. |
| `VersionControlPanel.jsx` | Playbook panel: stage, evidence, replay reports, one-tap rollback = `playbook activate`. |
| `HardwareDashboard.jsx` | Driver registry view; node status from bus; no direct serial poking. |
| (new) | Dial control + escalation inbox — the human's two primary surfaces. |

## Research questions disposition

| RQ | Disposition in new architecture |
|----|--------------------------------|
| RQ-001 alignment precision | Enforced as a **gate**: transcripts below alignment confidence can't be pattern basis. Research sets the threshold. |
| RQ-002 sonar model failure | Vocab + narrative events structure the feedback; Analyst owns the confusion-matrix work. |
| RQ-003 PID across sea states | Offline gain-scheduling experiments via replay; results land in `calibration` ns per sea state. |
| RQ-004 filter topology | Reducer-internal decision; benchmarked via replay, swappable without touching bus contract. |
| RQ-005 vocab convergence | Memory retention policy + Analyst metrics; lock threshold becomes a vocab-namespace setting. |
| RQ-006 compression | L0 screencap driver config; measurable end-to-end via replay of stored frames. |
| RQ-007 codegen safety | Answered structurally: AST gate + sealed replay + shadow + envelope backstop. Formal methods remain open. |
| RQ-008 compass calibration | Calibration namespace versions deviation tables; Analyst recommends intervals from drift data. |
| RQ-009–012 | Unchanged, now executable: override latency from black-box stats; storage via digest compaction; fleet learning via exchange; offline model bake-offs via replay. |

## Transition order (safe, incremental)

1. Stand up `core/` skeleton: bus v2, state reducer, black box — passive
   mode, legacy modules keep running alongside.
2. Migrate drivers behind the bus (serial, N2K, screencap).
3. Move guardrail constants into envelope + `vessel.toml`; route legacy
   actuation through envelope (envelope in permissive-logging mode first,
   then enforcing).
4. Introduce playbook registry; convert the live `trolling_rules.py` into
   the first bundle (it becomes `trolling_baseline`, grandfathered to
   ACTIVE with its first replay evidence generated retroactively).
5. Retire `rollback.rs`; point UI at the registry.
6. Stand up shadow mode + Analyst loop; dial ships defaulting to 1.
7. Cloud endpoints rewired to Memory sync last (least safety-critical).

Each step is independently revertible and leaves the boat operable.

---

**Back to:** `AGENTS.md` for the working rules; `docs/05` for the target.
