# 08 — Memory Layer

> **Target Audience:** All agent roles; anyone adding a store or sync path.
> **Purpose:** One memory abstraction for everything the system learns:
> vocabulary, playbooks, behavioral patterns, calibration, catch logs.
> **Status:** Governing document

---

## The problem this solves

The legacy design had four disconnected stores: Cloudflare KV (vocab),
Vectorize (behavior memory), local SQLite (telemetry), and flat files
(playbooks, backups). Agents had to know which store held what, sync was a
feature bolted onto vision, and nothing was versioned. An agent-centric
system needs memory that is **addressable, versioned, local-first, and
queryable by machines**.

## The VesselMemory trait

One abstraction, one implementation today (SQLite + content-addressed file
store), swappable later. Everything the system learns lives behind it:

```rust
trait VesselMemory {
    fn put(&self, ns: Namespace, key: &str, value: Value, prov: Provenance) -> Hash;
    fn get(&self, ns: Namespace, key: &str) -> Option<(Value, Provenance)>;
    fn history(&self, ns: Namespace, key: &str) -> Vec<Version>;   // memory never forgets, it supersedes
    fn query(&self, ns: Namespace, q: Query) -> Vec<Hit>;          // structured or semantic
    fn digest(&self, range: Range) -> Digest;                      // for sync + audit
}
```

### Namespaces

| Namespace | Contents | Written by | Read by |
|-----------|----------|-----------|---------|
| `vocab` | captain's term mappings ("feed layer" = biomass band 30–40fm) | Analyst, human | all agents, vision pipeline |
| `playbooks` | registry: id → stage, hash, evidence refs, live stats | gate machinery | kernel, Engineer, Auditor |
| `patterns` | learned behavior segments (state-window → human action) | Analyst | Engineer, missions |
| `calibration` | hydrodynamic params, compass deviation table, per-sea-state gains | Analyst/calibration driver | playbooks, envelope (read-only) |
| `catches` | catch log entries | human (one tap), Operator | Analyst, Fleet |
| `missions` | named mission configs + history | human, Operator | Operator |
| `escalations` | every escalation + resolution | kernel | Engineer, Auditor |
| `fleet` | imported cross-vessel knowledge (vetted) | Fleet agent | Engineer |

### Rules

1. **Every write carries provenance** (same shape as bus events: actor,
   basis, confidence). Unsourced memory is not knowledge, it's rumor.
2. **No destructive updates.** `put` creates a new version; `history` is
   the audit trail. Rollback of *learning* must be as easy as rollback of
   code.
3. **Local-first, always.** The boat is fully intelligent with zero
   connectivity. Sync is strictly additive.
4. **Vocabulary is a first-class citizen.** It's the trust interface
   (legacy Insight-004): agents must speak the captain's language in
   escalations and reports, and the vision pipeline must hear it.

## Learning loop, end to end

```
human drives + talks
   → human.voice.transcript + telemetry (time-aligned)
   → Analyst mines patterns: "before headwind turns, +5% throttle, 14 instances"
   → pattern stored in `patterns` (provenance: those 14 event ids)
   → Engineer drafts playbook citing the pattern
   → gate (docs/07) → SHADOW → ACTIVE
   → shadow/active stats accumulate in `playbooks` registry
   → Analyst's next report is better informed
```

The loop is closed entirely by events and memory — no ad-hoc scripts, no
hand-carried CSVs. Voice timestamps go through the alignment-confidence
check (legacy RQ-001) before a transcript may be cited as pattern basis;
misaligned pairs are quarantined, not trained on.

## Sync architecture

```
VESSEL (source of truth)                CLOUD EDGE (amplifier)
┌──────────────────────┐                ┌──────────────────────────┐
│ SQLite + CAS files    │   opportunistic│ Workers KV: vocab mirror  │
│ (namespaces above)    │───────────────▶│ Vectorize: pattern embeds │
│ black box: append-only│   digests,     │ playbook registry mirror  │
│                       │◀───────────────│ fleet playbook candidates │
└──────────────────────┘   fleet pulls   └──────────────────────────┘
```

- Sync unit is the **digest**: hash-stamped namespace slices. Conflicts
  resolve by provenance precedence (local human > local agent > fleet) —
  last-writer-wins is forbidden for anything learned locally.
- Telemetry lane never syncs raw. It syncs as digests and curated pattern
  windows. Bandwidth is assumed hostile.
- Fleet imports land in `fleet` namespace as **candidates**: they can
  inspire drafts but cannot enter the gate without local replay evidence.
  Your boat, your water, your proof.

## Query interface for agents

`boatctl memory query` supports:

- structured: `ns=patterns where sea_state=rough and action=throttle_ramp`
- semantic: `ns=patterns similar "headwind turn preparation"` (local
  embeddings; falls back gracefully offline)
- provenance walk: `boatctl memory why <playbook_id>` → the full chain:
  rule → pattern → transcripts → raw event ids → replay report

The `why` walk is the audit superpower. Every learned behavior must be
explainable back to evidence in under a second.

## What memory deliberately does NOT do

- **No unbounded growth.** Namespaces have retention policies; the Analyst
  compacts superseded versions into digests. (Addresses legacy RQ-005/RQ-010
  by policy, not hope.)
- **No direct writes from generated code.** Playbooks are pure; they read
  calibration via the kernel, they never write memory.
- **No cloud-only memory.** If a fact can't survive the satellite going
  dark, it doesn't count as known.

---

**Next:** `09_HUMAN_IN_THE_LOOP.md` — the supervision contract.
