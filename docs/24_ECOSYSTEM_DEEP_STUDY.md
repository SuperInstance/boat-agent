# 24 — SuperInstance Ecosystem Deep Study: Synthesis

> **Target Audience:** All agents. The map of what the org already owns
> that this project should use, mine, or ignore.
> **Purpose:** Consolidate the four-stream deep study (2026-07-19) into
> one adoption plan. Sources: claude code ([FLUX/PLATO](research/FLUX_PLATO_SYNERGY.md)),
> mini-agent ([marine cluster](research/MARINE_REPOS_SYNERGY.md)),
> two explore subagents (agent-infra, cloud/search — findings inline).
> **Status:** Governing for adoption decisions · Review quarterly

---

## The landscape (57 active repos, studied ~45)

Four clusters, four researchers. Headline: **the org has accidentally
built most of our support infrastructure already** — cloud scaffold,
marine data plane, policy enforcement, ops tooling — while the trusted
control core (our actual job) exists nowhere else. The division of labor
is clear and worth defending: **we are the only safety-critical repo in
the org.**

## Tier 1 — Adopt now (high value, low cost)

| Repo | Use | Cluster source |
|------|-----|----------------|
| **fleet-platform** | Fork as the docs/19 cloud-twin scaffold: deploy script + R2/D1/Vectorize wiring already built; replace its D1 schema with one generated from `schemas/` | cloud |
| **oracle-relay** | Boat↔cloud↔browser event link: deployed WebSocket relay with ring-buffer catch-up — the offline-reconnect answer. Needs a typed VesselEvent bridge | agent-infra |
| **signalk-bridge** | Marine data plane: Signal K feeds our L0 drivers (GPS/depth/engine/AIS) instead of bespoke NMEA parsers | marine |
| **search-superinstance-ai** | Cloud-twin semantic search tier: Workers-AI embed → Vectorize pipeline with free-tier math done | cloud |
| **ship-log-modules** (tide/weather/AIS trio) | Scrubber overlay tracks + Analyst sea-state context; built, spec'd | marine |
| **baton-protocol** | Session handoff YAML for our own agent crew (prime directive 6). Drop-in CLI, zero code | agent-infra |
| **roam-graph** | Docs-hygiene CI: orphan/hub detection over our 24+ load-bearing docs ("stale docs are defects" — this is the detector) | cloud |
| **edge-weight** | Link-quality-adaptive autonomy: satellite-degraded → conservative thresholds; already designed for our boat (docs/09) | cloud |

## Tier 2 — Evaluate / mine the pattern (real value, real porting cost)

| Repo | Pattern to mine | Why not the code |
|------|----------------|------------------|
| **conservation-enforcer-rs** | Deterministic policy bytecode for the stage-gate (Rust, WASM-ready, 150+ tests) — claude's top pick | FLUX dialect governs text/LLM output, not real-time control; needs a control-loop dialect (see Q-ARCH-5 below) |
| **plato-room-deployment-approval** | Human approval gate as an engine — our docs/07 Stage 4 institutionalized | Room machinery ≠ our gate evidence model |
| **whistle** | DSL→validate→compile pipeline with referential integrity — shape for playbook-manifest authoring | Compiles to PLATO configs, not our schemas |
| **exocortex-rs** | Tiered memory decay, shadow rendering (event→narrative for scrubber) | No persistence/async (self-admitted); patterns only |
| **SmartCRDT** | Answer to Q-SYNC-1: LWW-Map + OR-Set in ~200 lines of Rust | Platform is a 4-service Docker stack; mine types only |
| **othismos-reef** | Erosion-based GC with blast_radius ("what breaks if we retract this playbook") | Python; design donor for docs/08 GC |
| **vetcheck** | Model drift detection + quarantine for playbook monitoring and edge models | Python/model-API oriented; reimplement in gate |
| **a2ui** | Intent→InterfaceSpec for ad-hoc captain/ops views | Keyword parsing; IR consumption only |
| **trawl** | Quota/regulatory modules (commercial compliance, phase 3) | Product-phase feature, not now |

## Tier 3 — Skip / watch

polln (impressive README, unproven) · A2A-native-notebookLM (most rows
"Planned") · tminus-os & breed-registry (**404 — link rot; ecosystem
docs reference them anyway**) · hermes-memory-mcp (empty shell) ·
domain-landing, smart-404 (trivial) · activelog-ai (fitness, unrelated) ·
email-oracle (half-baked relay path).

## The five decisions this study forces

1. **The cloud twin starts from fleet-platform, not from scratch.** The
   Phase-2 cold start is gone; the work is schema alignment + auth, not
   infrastructure.
2. **Signal K is our marine data plane.** One standard feed instead of
   N bespoke parsers. signalk-bridge + ais-tracker prove the org speaks
   it; docs/06 gets a `driver.signalk` entry.
3. **Policy bytecode is worth a real look — but not in the envelope.**
   conservation-enforcer-rs + flux-policy-tester could harden the
   stage-gate's *analysis* stages (Docs/07 Stage 2) and agent-output
   governance. The envelope itself stays hand-written Rust (docs/12 D4).
   → New open question **Q-ARCH-5**: a FLUX dialect for control-loop
   policy, evaluated against the replay harness.
4. **Ship-log and tzpro must converge on one searchable vessel memory.**
   tzpro posts to "Ship Log" today; ship-log-sync/search is the formal
   version. B-INT-4 routes `echogram_record` through docs/19 sync into
   that tier. One memory, not two.
5. **Cheap drops land immediately:** baton-protocol (crew handoffs),
   roam-graph (docs CI), emergency-alerts (once the cloud twin has ≥2
   Workers), plato-room-directory→fleet registry (when vessel #2 exists).

## New open questions (to docs/13)

- **Q-ARCH-5:** FLUX-for-control-loops dialect — can bytecode policy
  express rate limits/bounds verifiably enough to earn gate Stage 2?
  Evaluation path: replay harness, never live actuation.
- **Q-SYNC-1 (updated):** SmartCRDT's type catalog (LWW-Map, OR-Set) is
  the porting shortlist; platform rejected.
- **Q-UX-3:** vessel-quest gamification — does catch-quest XP fit the
  captain, or is it noise? Test with the deck-mode presets (docs/20).

---

**Cross-references:** docs/16 (first ecosystem map — this supersedes its
"other repos" scope), docs/18/19 (twin), docs/21 (phase plan — Tier 1
items fold into weeks 3–4), docs/13 (new questions).
