# 06 — Bus Protocol (VesselEvent v2)

> **Target Audience:** Every module and agent author. This is the contract
> you sign when you join the system.
> **Purpose:** Complete event taxonomy, envelope format, lanes, provenance,
> and the rules for adding new event types.
> **Status:** Governing document — machine schema lives in
> `schemas/vessel-event.schema.json`

---

## Why a protocol document

Agents operate this system. Agents need contracts, not conventions. The bus
is the only way anything talks to anything, so its schema is the single
most important file in the repository. Rule: **schema first, types second,
code third** — in that order, always.

## Envelope format

Every event on the bus has this outer shape:

```json
{
  "id": "evt_01J4Z8...",
  "seq": 482113,
  "timestamp_ms": 1711974008123,
  "lane": "telemetry",
  "kind": "sensor.gps.fix",
  "source": {
    "module": "driver.nmea0183",
    "instance": "com3@4800",
    "actor": "system"
  },
  "provenance": {
    "authority": "driver",
    "confidence": 1.0,
    "basis": []
  },
  "payload": { "type-specific": "fields" }
}
```

### Field semantics

| Field | Meaning | Set by |
|-------|---------|--------|
| `id` | ULID — unique, sortable | emitter |
| `seq` | Monotonic kernel sequence number | kernel (on ingest) |
| `timestamp_ms` | Epoch ms at the source, not at ingest | emitter |
| `lane` | `critical` \| `telemetry` \| `narrative` | emitter, validated by kernel |
| `kind` | Dotted taxonomy string (see below) | emitter |
| `source.module` | Module id, e.g. `driver.n2k`, `agent.engineer` | emitter |
| `source.actor` | `system` \| `human` \| `agent:<role>` | emitter |
| `provenance.authority` | `driver` \| `playbook:<id>@<hash>` \| `mission:<name>` \| `human` \| `agent:<role>` | emitter, **verified by envelope** |
| `provenance.confidence` | 0.0–1.0; emitter's self-assessed certainty | emitter |
| `provenance.basis` | Event ids this event was derived from | emitter |
| `payload` | Per-kind typed body, validated against schema | emitter |

### Hard rules

1. **Payloads are validated at ingest.** An event that fails schema
   validation is rejected to a quarantine log, never onto the bus.
2. **Lane is a privilege.** Only L0 drivers and the envelope may emit
   `critical`. Anything else attempting it is quarantined and the Auditor
   is notified.
3. **Provenance is not optional and not self-attested for actuation.**
   The envelope re-verifies `authority` against the playbook registry
   hash before any actuator write.
4. **`basis` is how the log becomes a graph.** An intent derived from
   state snapshot + sonar event lists those event ids. Replay and audit
   depend on it. Emitters that omit basis lose trust (Auditor metric).

## Taxonomy

Kinds are dotted: `<domain>.<entity>.<action>`. Current registry:

### Sensing (L0 → bus, telemetry lane)

| Kind | Payload summary |
|------|-----------------|
| `sensor.gps.fix` | lat, lon, sog_kn, cog_deg, hdop, sats |
| `sensor.compass.heading` | heading_deg, swing_rate_dps |
| `sensor.depth.sounder` | depth_m, bottom_hardness? |
| `sensor.engine.rpm` | engine_id, rpm, throttle_pct |
| `sensor.wind.apparent` | speed_kn, angle_deg |
| `sensor.rudder.angle` | angle_deg |
| `sensor.thermal.reading` | zone, celsius, sensor_id |
| `sensor.camera.frame_meta` | camera_id, ts, hash (frames stay off-bus) |
| `sensor.custom.*` | driver-defined, schema-registered |

### Human input (L0/UI → bus)

| Kind | Lane | Payload summary |
|------|------|-----------------|
| `human.jog_lever.move` | **critical** | direction, magnitude — absolute override |
| `human.dial.set` | critical | autonomy_level 0–3, by_whom |
| `human.mission.command` | narrative | start/stop/switch mission |
| `human.voice.transcript` | narrative | text, word_timestamps[], audio_hash |
| `human.escalation.answer` | critical | escalation_id, decision, note? |
| `human.catch.log` | narrative | species, depth_fm, location?, note? |

### Control (L2/L3 → envelope → L0)

| Kind | Lane | Payload summary |
|------|------|-----------------|
| `control.intent.steering` | telemetry | requested_rudder_deg, horizon_s |
| `control.intent.throttle` | telemetry | requested_throttle_pct, horizon_s |
| `control.verdict` | telemetry | intent_id, approved/clamped/rejected, reason, final_command |
| `control.actuation.confirm` | telemetry | verdict_id, driver_ack, measured_state_after |
| `control.watchdog.trip` | **critical** | missed_heartbeats, last_known_state_hash |

### Agent activity (L4 → bus, narrative lane unless noted)

| Kind | Payload summary |
|------|-----------------|
| `agent.proposal.playbook` | playbook_id, diff_summary, evidence_refs[] |
| `agent.escalation.request` | question, options[], recommendation, context_bundle_ref |
| `agent.analysis.finding` | finding_type, detail, supporting_event_ids[] |
| `agent.shadow.delta` | playbook_id, what_agent_would_do, what_human_did, agreement |
| `agent.audit.result` | chain_verified, anomalies[], checked_range |

### System (kernel → bus)

| Kind | Lane | Payload summary |
|------|------|-----------------|
| `system.tick.heartbeat` | telemetry | tick_n, drift_ms, state_hash |
| `system.module.up/down` | telemetry | module_id, reason |
| `system.mission.enter/exit` | telemetry | mission_name, params |
| `system.degraded_mode` | **critical** | cause, remaining_capabilities[] |

## Lanes: delivery semantics

| Lane | Queue | Backpressure policy | Ordering |
|------|-------|--------------------|----------|
| critical | unbounded, lock-free | never dropped; kernel halts to safe state if producer floods | strict seq |
| telemetry | bounded (4096) | coalesce by kind (keep newest per producer) | seq |
| narrative | bounded (16384) | drop oldest, count drops, Auditor alerted if >0 | seq |

Coalescing rule exists because a stale GPS fix is worse than a missing one.

## Adding a new event type (procedure for agents)

1. Add the payload schema to `schemas/vessel-event.schema.json` under the
   appropriate domain. Follow `<domain>.<entity>.<action>` naming.
2. Regenerate/extend the Rust types in `core/src/bus/events.rs`.
3. If the kind can influence actuation: say so explicitly in the schema
   description; the Auditor will track it.
4. Document the kind in this file's registry, same change.
5. Emitters must populate `provenance` honestly. Fake confidence is a
   worse sin than low confidence.

## Serialization on the wire

- In-process: Rust enums (serde-tagged), zero-copy where possible.
- Cross-process (playbook host, sidecars, `boatctl`): newline-delimited
  JSON over local sockets/stdio. Same schema, no exceptions.
- Cloud sync: NDJSON batches, gzip, only narrative lane + black-box
  checkpoints. Telemetry lane never leaves the boat raw; it leaves as
  digests (`docs/08`).

---

**Next:** `07_PLAYBOOK_LIFECYCLE.md` — how AI-authored code earns the helm.
