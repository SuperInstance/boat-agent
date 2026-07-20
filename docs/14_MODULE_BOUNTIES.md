# 14 — Module Bounties & Generalization Tracks

> **Target Audience:** Agents (and humans) looking to contribute a
> self-contained piece. This is the menu.
> **Purpose:** Two lists. Part 1: concrete modules ready to be built,
> each with its contract and acceptance criteria. Part 2: generalization
> tracks — places where a purpose-built mechanism should grow into a
> general IO function usable by anything related.
> **Status:** Living document. Claim work by opening an issue referencing
> the ID (e.g. `B-DRV-2`). Completed bounties move to the bottom with a
> link, never deleted.

---

## The rules of engagement (read before claiming anything)

1. **Contracts first.** Every bounty names the docs/schemas it must
   satisfy. If your module can't be driven by `boatctl` and doesn't speak
   bus protocol, it doesn't ship (AGENTS.md).
2. **Small and complete beats large and partial.** A bounty is done when
   its acceptance criteria pass — including replay tests where applicable
   — and its doc section is written.
3. **The envelope is read-only.** Several bounties orbit the envelope.
   None modify it. If you believe one must, that's an escalation
   (Q-SAFE-*), not a PR.
4. **Generalize at the seam, not the core.** Part 2 tracks exist so we
   widen *interfaces*, never loosen *guarantees*.
5. **Config, not constants.** Anything a different vessel/application
   would tune belongs in the profile.

---

# Part 1 — Module bounties

## Drivers (L0)

### B-DRV-1: NMEA-0183 driver with auto-discovery
- **Contract:** `core/src/drivers/mod.rs` (`Driver` trait); emits
  `sensor.gps.fix`, `sensor.compass.heading`, `sensor.depth.sounder`.
- **Build:** serial reader with baud sniffing (4800/38400), sentence
  validation with checksum (legacy Insight-002), graceful `PortLocked`
  errors that feed onboarding auto-diagnosis (legacy Insight-005).
- **Acceptance:** replay test from recorded NMEA logs; discovery writes a
  valid `[drivers.gps]` block; panic-free under garbage input fuzz.

### B-DRV-2: N2K/CAN driver (PGN 127488 family)
- **Contract:** same trait; emits `sensor.engine.rpm` (+tank/temp kinds
  registered per docs/06 procedure).
- **Build:** CAN frame decode for engine params rapid-update; gateway
  serial and native CAN (socketcan on Pi) backends.
- **Acceptance:** golden-frame decode tests; config-driven PGN allowlist.

### B-DRV-3: COM multi-cast splicer (legacy Phase 4, hardened)
- **Contract:** driver pair (physical-in, virtual-out); no `expect()`.
- **Build:** lock-conflict detection with escalation event; backpressure
  handling so a slow consumer can't stall the feed.
- **Acceptance:** legacy failure modes (nav software holds port) produce
  structured escalation, not a hang. **Generalization:** implement against
  the G-1 byte-stream multiplexer interface from day one.

### B-DRV-4: Jog lever driver (GPIO + serial variants)
- **Contract:** emits `human.jog_lever.move` on the **critical** lane —
  one of the only modules permitted to.
- **Build:** debounce + vibration discrimination (legacy RQ-009 data
  welcome); fail-*active* wiring check (a disconnected lever must look
  like "unknown," never like "no input").
- **Acceptance:** fault-injection tests; latency measurement lever→event
  under 20 ms.

### B-DRV-5: Screen-capture driver (vision source)
- **Contract:** frames to CAS storage, `sensor.camera.frame_meta` to bus.
- **Build:** async capture (never the tick thread), crop/quantize config,
  adaptive rate on narrative-lane pressure.
- **Acceptance:** bounded memory under 8h soak; **G-2-ready** (source is
  config, not code).

### B-DRV-6: Autopilot + throttle actuator drivers
- **Contract:** `actuate()` callable ONLY by the envelope; confirm via
  `control.actuation.confirm`.
- **Build:** $GPAPB generation with checksum (legacy Phase 5 code ports
  cleanly); servo/digital-pot throttle backends behind one trait.
- **Acceptance:** hardware-in-loop bench test; every actuation traceable
  to a verdict id.

## Kernel & infrastructure

### B-CORE-1: Replay harness
- **Contract:** `replay` feature flag; feeds black-box slices as bus
  events; compares verdict streams.
- **Build:** corpus loader (Analyst-curated manifests), deterministic
  clock injection, verdict-diff reporter.
- **Acceptance:** the example playbook's `tests/` actually run in CI.
  This bounty unblocks every other one — **highest leverage first.**

### B-CORE-2: Kernel supervisor with backoff
- **Contract:** module lifecycle (drivers, playbook host, sidecars);
  restart policy in profile; module health events on bus.
- **Acceptance:** kill -9 any module; system degrades + recovers per
  policy, envelope holds safe state throughout, Auditor saw everything.

### B-CORE-3: boatctl IPC server + CLI
- **Contract:** `core/src/agent_api/`; NDJSON over local socket.
- **Build:** capability checks, idempotency keys, `capabilities`
  discovery endpoint reflecting live dial/degraded state.
- **Acceptance:** full docs/10 command groups; Tauri UI ported onto it as
  a pure client.

## Agent runtimes (L4)

### B-AGT-1: Auditor agent (first agent to build — deliberately)
- **Why first:** it's small, read-mostly, and makes every later agent
  safer. The crew member that watches the crew.
- **Build:** chain verification, provenance-completeness scoring, lane
  discipline checks, escalation false-alarm accounting.
- **Acceptance:** detects a hand-corrupted black box line; its own output
  passes its own checks; <500 lines.

### B-AGT-2: Analyst pattern miner (v1: voice↔telemetry alignment)
- **Contract:** consumes black box + transcripts; writes `patterns` with
  provenance; alignment-confidence gate per legacy RQ-001.
- **Acceptance:** on a synthetic corpus with planted patterns, recovers
  them with stated confidence and cites the right event ids.

### B-AGT-3: Engineer gate machinery (Stages 1–3 automation)
- **Contract:** draft → AST gate → replay → approval packet.
- **Build:** AST allowlist checker, clamp verifier, sealed-evidence
  writer, plain-language approval diff generator (uses `vocab`).
- **Acceptance:** the example bundle passes end-to-end; a deliberately
  unsafe draft (hidden `while True`, unclamped output) is rejected with
  machine-readable reasons.

## UI panels (React, pure boatctl clients)

### B-UI-1: Dial + escalation inbox (the human's two primary surfaces)
### B-UI-2: Playbook panel (stage, evidence, replay/shadow reports, one-tap rollback)
### B-UI-3: Profile builder wizard (discovery results → one-tap vessel.toml)

All UI bounties share one acceptance rule: **zero kernel calls besides
boatctl** — proven by inspection.

---

# Part 2 — Generalization tracks

Each track: the specific thing we built for boats, the general IO
function it wants to become, and the seam where the widening happens.
These are how "modular pieces for different applications" stops being a
slogan. Picking up a track means doing the generalization *and*
retrofitting the marine use onto it (proof it's truly general).

### G-1: Byte-stream multiplexer
- **Specific:** COM splicer fans one GPS feed out to nav software + agent
  stack.
- **General:** `stream_mux` — N inputs → M outputs fan-out/fan-in for
  serial, UDP, TCP, with per-output backpressure policy and tap points
  (passive listeners that can't stall the flow).
- **Harnessable by:** any deployment merging redundant sensors or sharing
  one instrument across consumers — tractors (RTK GPS → autosteer +
  logger), aquaculture (sensor buses), USVs.
- **Seam:** B-DRV-3 implements it as the general component with
  `com_splice` as config.

### G-2: Frame-to-event vision IO
- **Specific:** sonar screen → edge vision model → text interpretation.
- **General:** a `vision_source` driver spec: {capture source, crop,
  model/prompt, output schema, event kind, vocab namespace}. Any pixels →
  validated structured events. Prompt changes go through the same
  stage-gate discipline as code changes.
- **Harnessable by:** analog gauges, chartplotters, thermal cameras,
  dock-side paperwork photos, greenhouse monitors.
- **Seam:** Q-IO-1; B-DRV-5 built config-first.

### G-3: Domain-language layer
- **Specific:** captain's vocabulary ("feed layer") maps model output to
  the user's terms.
- **General:** `vocab` is a domain-adaptation namespace for ANY expert
  jargon — agronomist, engineer, medic. Same trust dynamics (legacy
  Insight-004 is universal: experts distrust systems that speak generic).
- **Harnessable by:** every vertical; zero kernel changes, it's memory
  + prompt plumbing already.
- **Seam:** docs/08 already namespaces it; the work is proving it on a
  second domain (see Q-PROD-1).

### G-4: Staged-trust pipeline for AI-authored code
- **Specific:** playbook gate (static → replay → approve → shadow →
  active).
- **General:** the gate is a **trust compiler** for any AI-authored
  artifact that acts on the world: control code, alert rules, maintenance
  schedules, irrigation plans. Inputs: artifact + evidence corpora.
  Output: artifact + graduated authority.
- **Harnessable by:** any agentic product with actuation; potentially the
  most exportable component we own.
- **Seam:** parameterize gate stages by artifact type; the marine
  playbook is `artifact_type = "control_rules"`.

### G-5: Human-decision broker
- **Specific:** captain escalations with options, consequences, timeout
  fallbacks, budgets.
- **General:** a structured attention market between agents and humans:
  role-addressed, multi-channel, delegation chains, auditable. (Q-IO-2.)
- **Harnessable by:** fleet managers, shore crews — and honestly any
  agent system outside this repo.
- **Seam:** escalation kinds get a `role` field; transport becomes a
  driver (UI today, SMS/app tomorrow).

### G-6: Provenance log / audit chain
- **Specific:** black box for vessel liability.
- **General:** append-only, hash-chained, replayable event store with
  provenance graphs — useful anywhere "why did the system do that" has
  legal or operational weight.
- **Harnessable by:** any regulated automation.
- **Seam:** extract `core/src/blackbox/` into a standalone crate with the
  vessel entry type as one schema instance.

### G-7: Hostile-bandwidth sync
- **Specific:** digest-based, local-first memory sync over satellite.
- **General:** condition-scoped, provenance-precedence sync for any
  intermittently-connected fleet (rural ag, disaster response, research
  stations).
- **Harnessable by:** all of the above verticals.
- **Seam:** Q-SYNC-1's resolution defines the general conflict model.

---

## Ecosystem integration (see docs/16 — tzpro-agent & sonar-vision family)

### B-INT-1: Corpus importer (tzpro captures → replay corpus)
- **Contract:** replay corpus manifests per docs/07; labels → `expectations.toml` shape.
- **Build:** convert `captures/v3/**` JSONL (timestamped, GPS-linked, catch-linked echogram analyses) into replay slices + a labeled frame dataset. Pure data work, zero live-integration risk. **Do this first — it seeds RQ-002's dataset and B-CORE-1's material.**

### B-INT-2: Vocabulary migration (tzpro Bayesian vocab → `vocab` namespace)
- **Contract:** docs/08 namespaces; every entry carries `provenance="import:tzpro-agent"`. One schema going forward — ours.

### B-INT-3: tzpro analyzer as event-emitting sidecar
- **Contract:** emits `sensor.acoustics.*` narrative events as NDJSON on stdout (playbook-host protocol). New kinds registered per docs/06 procedure. Supervised child process; kernel doesn't care it's Python.

### B-INT-4: Ship Log Worker ↔ memory digest sync
- **Contract:** docs/08 sync (digests, provenance precedence) + anonymized-pattern fleet policy (vocabulary shared, catch counts private).

### B-INT-5: Cross-frame blob tracker (sonar-vision-rs)
- **Contract:** new `acoustics.track.*` events; ObjectTracker sidecar in Rust. Turns per-frame blob counts into persistent school tracks — much stronger Analyst patterns.

---

## Standalone tools (spun out as their own repos)

| Tool | Repo | Generalizes | Status |
|------|------|-------------|--------|
| **perception-cascade** | [SuperInstance/perception-cascade](https://github.com/SuperInstance/perception-cascade) | G-2/G-4: tiered perception loops (M1/M10/H1 + gaze) for ANY frame time-series; prompts are env config | Live — verified on marine echograms |
| **provenance-log** | [SuperInstance/provenance-log](https://github.com/SuperInstance/provenance-log) | G-6: hash-chained append-only audit log as a crate | Live — 5/5 tests incl. tamper detection |

## Completed bounties

*(none yet — the menu opened today)*

---

**Contribution flow:** pick an ID → open an issue referencing it →
contract review against the named docs → build with replay tests →
Auditor + human review → land. Welcome aboard.
