# 10 — Agent Operations: The Crew and the Daily Loop

> **Target Audience:** Agent runtimes (OpenClaw sidecar, cloud workers) and
> the engineers who build them.
> **Purpose:** The five agent roles, their capabilities and constraints,
> the operational loop across a trip, and the `boatctl` command surface.
> **Status:** Governing document

---

## Capability model

Agents hold **capabilities**, not permissions-in-general. A capability is:
`<action-class> on <resource-class> within <constraint>`, granted by
config, revocable by dial/envelope/human. Capabilities are declared in
`vessel.toml` under `[agents.*]` and enforced at the bus ingest and the
envelope — an agent emitting outside its grants is quarantined.

## The five roles

### OPERATOR (real-time, kernel-adjacent)

- **Job:** execute missions through active playbooks; narrate at dial 1.
- **Reads:** VesselState snapshots, mission layer, calibration namespace.
- **Writes:** `control.intent.*`, shadow deltas, coach narration.
- **May never:** modify playbooks, alter its own goals, escalate without
  a structured request, run unconstrained inference in the control loop.
- **Key discipline:** the Operator is a *host* for playbooks, not a mind.
  Its intelligence lives in authored, gated code.

### ENGINEER (offline, sidecar)

- **Job:** turn evidence into playbooks. Runs the stage-gate machinery:
  draft → static analysis → replay → present for approval → manage
  shadow → monitor active.
- **Reads:** patterns, escalations, replay corpus, fleet candidates.
- **Writes:** playbook drafts, gate evidence, `agent.proposal.playbook`.
- **May never:** activate a playbook (only the human gate + kernel can),
  skip replay, or edit a sealed artifact.
- **Cadence:** batch-oriented. Works when the boat doesn't need it:
  at anchor, dockside, or on low-priority narrative cycles.

### ANALYST (offline, sidecar/cloud)

- **Job:** mine. Voice-telemetry alignment, pattern extraction, vocabulary
  refinement, replay corpus curation, dial-trust metrics, attention-budget
  reports.
- **Reads:** black box (read-only), all memory namespaces, escalations.
- **Writes:** `patterns`, `vocab` proposals, `agent.analysis.finding`.
- **May never:** touch actuation paths — it has no capability grant for
  any `control.*` kind, enforced at ingest.

### AUDITOR (real-time, minimal, paranoid)

- **Job:** verify. Hash-chain integrity, provenance completeness,
  confidence calibration per agent, escalation false-alarm rates, lane
  discipline violations, playbook stat anomalies.
- **Reads:** everything.
- **Writes:** `agent.audit.result`, demotion triggers, degraded-mode
  requests.
- **May never:** be disabled, be rate-limited, or have its log writes
  filtered. If the Auditor is silent, that itself is an alarm condition.
- **Size discipline:** the Auditor must stay small enough to be obviously
  correct. It checks; it does not think.

### FLEET (cloud, opportunistic)

- **Job:** aggregate across vessels: distill common patterns, maintain the
  playbook exchange, push *candidates* to boats.
- **May never:** write to any local namespace other than `fleet`, and
  candidates there are inert until locally gated. One vessel's evidence
  is another vessel's hypothesis.

## The daily loop (a fishing day, agent's-eye view)

```
PRE-DEPARTURE (dock, dial 1)
  Auditor:   verify black-box chain from last trip; check module health
  Engineer:  report shadow stats; propose any pending playbook approvals
  Analyst:   overnight sync digests; fleet candidates summarized
  Human:     reviews digest (one screen, 60 seconds), sets dial, goes

DEPARTURE → TRANSIT (dial 2–3, transit mission)
  Operator:  runs transit playbook; Engineer idle; Analyst live-aligns
             any voice notes against telemetry
  Auditor:   continuous verification; lane/provenance spot checks

ON THE GROUNDS (trolling mission, dial 2–3)
  Operator:  trolling playbook active; coach narration if dial 1–2
  Analyst:   real-time shadow deltas; catch logs labeled onto patterns
  Human:     fishes. Interrupts only for structured escalations.

ANOMALIES (any time)
  Kernel:    sensor disagreement → degraded mode → safe playbook hold
  Auditor:   anomaly → playbook auto-demote if implicated
  Operator:  structured escalation with context bundle; timeout fallback
             acts if human is busy landing a fish — always

EVENING (anchor/dock)
  Engineer:  full replay of the day against pending drafts; gate reports
  Analyst:   day's patterns committed; vocab updates proposed; trust
             metrics updated ("matched your calls 94%")
  Auditor:   day's chain sealed and checkpointed; sync digest emitted
  Human:     one-screen digest. Approvals if earned. Done.
```

## boatctl: the agent command surface

Everything — UI included — goes through `boatctl` semantics (typed JSON
commands over local IPC; the Tauri UI is a client). Self-describing:
`boatctl capabilities` returns the full schema of what this vessel
supports right now (varies with dial, hardware, degraded mode).

Core groups:

```
boatctl state snapshot [--since seq]        # digital twin reads
boatctl bus tail --lane narrative --kind agent.*
boatctl playbook list|show|propose|replay|activate|demote
boatctl memory query|why|history
boatctl mission start|stop|status
boatctl dial get                            # set is human-only, hardware path
boatctl escalate respond <id> <option>      # how the human answers via UI
boatctl audit verify [--range]
boatctl capabilities                        # discovery for agents
```

Design rules for the surface:

- **Structured in, structured out.** JSON always; exit codes meaningful;
  errors carry codes (`EPERM_CAPABILITY`, `ESTALE_STATE`, …).
- **Idempotent where possible**; where not, commands take idempotency keys.
- **Read paths are unauthenticated locally** (single-operator vessel),
  **write paths are capability-checked** and logged with actor identity.
- If a feature can't be driven by `boatctl`, it doesn't exist for agents —
  and features that don't exist for agents don't ship (AGENTS.md).

## Failure etiquette for agents

1. Emit what you know, flag what you don't (`confidence`, `basis`).
2. On contract violation (schema rejection, quarantine): stop, emit
   `agent.analysis.finding` with the rejection, do not retry-storm.
3. On degraded mode: narrow your ambitions, keep provenance clean.
4. When overridden by the human: treat it as the highest-value signal of
   the day, not as an error.

---

**Next:** `11_MIGRATION_MAP.md` — how the legacy 10-phase design folds in.
