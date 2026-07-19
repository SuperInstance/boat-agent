# 16 — Ecosystem Integration: tzpro-agent & sonar-vision Family

> **Target Audience:** Agents and engineers building the vision/observation
> subsystem, the replay harness, or fleet sync.
> **Purpose:** How four sibling repositories map onto this architecture,
> what to adopt vs. adapt vs. avoid, and the concrete integration order.
> **Status:** Analysis + plan · Date: 2026-07-19

---

## The repositories

| Repo | Lang | Status | What it actually is |
|------|------|--------|---------------------|
| [tzpro-agent](https://github.com/SuperInstance/tzpro-agent) | Python | **Operational** on F/V EILEEN (Ketchikan, chum trolling) | Production sensor node: TZ Pro echogram screen-capture daemon, OpenCV analyzer, SQLite mirror, Bayesian species vocabulary, catch linking, alerts, NMEA bridge, Cloudflare Worker sync |
| [sonar-vision](https://github.com/SuperInstance/sonar-vision) | Python | Toolkit | Sonar ping/echo **simulation**, signal processing, multi-object tracking, spatial mapping |
| [sonar-vision-rs](https://github.com/SuperInstance/sonar-vision-rs) | Rust | Library, zero-dep | Rust port of the same: `Signal`, `Sonar`, `SpatialMap`, `ObjectTracker`, ASCII display |
| [plato-sonar-text](https://github.com/SuperInstance/plato-sonar-text) | Rust | Library | Acoustic scene → **text descriptions for agents** (audio domain: sound events, voice activity, ambient profiles, alerts) |

## The headline

**tzpro-agent is the G-2 "frame-to-event" vision IO already running in
production — and its design principles are independently the same as
ours.** Local SQLite as source of truth, cloud as replication target not
control plane, never-overwrite schema versioning, capture never blocks on
analysis, fire-and-forget ingest, Bayesian-not-neural vocabulary because
Laplace smoothing works with 1 report and neural needs 10,000. That's
axiom-aligned engineering. We don't rebuild it; we **adopt it at the
event boundary**.

And the data it is capturing *right now* — timestamped, GPS-linked,
catch-linked echogram frames with OpenCV feature extraction — is exactly
the labeled dataset legacy RQ-002 said we'd have to partner with vessels
to get. We already own it. It is the seed of the replay corpus and the
vocab namespace.

## Mapping onto the architecture

### tzpro-agent → vision subsystem + first fleet node

| tzpro piece | boat-agent home | Notes |
|-------------|-----------------|-------|
| `capture_v3.py` (screen capture daemon) | L0 `driver.screencap` (B-DRV-5) | Its "capture never blocks on analysis" rule = our lane discipline |
| `analyzer.py` (OpenCV: blobs, bottom, thermoclines, haze, interference lines) | Perception sidecar emitting narrative-lane events (`sensor.acoustics.*` — new kinds per docs/06 procedure) | Runs as supervised child, same model as playbook host |
| `vocabulary.py` (Bayesian species-at-depth) | `vocab` + `patterns` namespaces (Analyst job) | Laplace-smoothed priors are exactly the "works with 1 report" discipline docs/08 wants |
| `catch_link.py` | Outcome labeling in the learning loop (docs/08) | catch report ↔ nearest capture = our "catch logs double as outcome labels" |
| `alerts.py` (5 rule types + dedup) | Escalation layer (docs/09) | Its dedup is attention-budget thinking; port rules to structured escalations with fallbacks |
| NMEA bridge (GPS COM6 → TCP multicast) | `driver.nmea0183` + `driver.com_splice` (B-DRV-1/3) | Already solved legacy Insight-005 (COM port sharing) — via TCP fan-out, the G-1 pattern |
| Ship Log Cloudflare Worker + Vectorize + planned D1 | Fleet agent home + memory sync targets (docs/08, docs/10) | Their anonymized-patterns rule ("vocabulary shared, catch counts private") becomes fleet policy |
| QGIS export, bathymetry corrections | Analyst outputs; chart-correction playbook evidence | New: chart-vs-sounder deltas are a data product other vessels will pay for |
| `captures/v3/YYYY-MM-DD_LAT/` corpus | **Replay corpus seed** (B-CORE-1) | Importer converts capture JSONL → replay slices; labels → expectations |

### sonar-vision / sonar-vision-rs → simulation & tracking

1. **Synthetic replay corpora.** The gate needs edge cases (sensor
   degradation, sea clutter, dropouts) that real trips produce rarely.
   A simulator generates them on demand with ground truth — this extends
   B-CORE-1 from "replay what happened" to "replay what could happen."
   The Rust port means simulation can live inside the test harness
   without a Python dependency.
2. **`ObjectTracker` for cross-frame blob persistence.** tzpro analyzes
   frames independently; tracking blobs *across* frames (school moving
   through the water column) is what turns "530 returns" into "a school
   holding at 31 fm for 4 minutes" — a much stronger pattern for the
   Analyst. New event kinds: `acoustics.track.*`.
3. **Future raw-signal driver.** Screen-scraping is a TZ Pro constraint,
   not a goal. `Signal`/`Sonar` are the seed of a driver that reads
   transducer data directly (ethernet/N2K) when hardware allows — higher
   fidelity, no palette guessing. (Q-IO-1 territory.)

### plato-sonar-text → wheelhouse audio driver + a design template

- **Direct use:** voice-activity detection gating transcription for
  `human.voice.transcript` (cheaper and cleaner than always-on Whisper),
  acoustic anomaly alerts (unusual engine note, alarm sounds) as
  narrative-lane events.
- **As a template:** "perception → structured text *for agents*" is the
  G-2 contract in another modality. Its `AcousticScene`/`SoundEvent`/
  severity-ranked alert shapes are a good reference when we define
  `sensor.acoustics.*` payloads for the echogram path.

## Adaptation notes (what NOT to do)

- **Don't merge runtimes.** tzpro has its own agent runtime (router,
  tool server, baton locks, MCP wrapper). Integrate at the **data/event
  boundary**: it becomes a supervised sidecar emitting VesselEvents, or
  (immediately) stays standalone with an importer. Two agent runtimes on
  one boat is one too many.
- **Don't import its TODOs as our design.** Its own pipeline audit scored
  itself 4/10 ship-ready; its rendering bug and stats-endpoint bug are
  known. Adopt the *data and the principles*, port the *analysis logic*
  under our contracts and tests.
- **Don't let the vocabulary fork.** tzpro's `.vocabulary_cache.json` and
  our `vocab` namespace must converge on one schema — pick ours
  (provenance-mandatory, versioned), migrate their entries with
  provenance="import:tzpro-agent".
- **Screen-scraping is technical debt we're choosing to carry.** Fine —
  it's the only interface TZ Pro gives. But every new perception module
  should be written against the G-2 config-driven spec so the day we get
  raw sounder data, only the driver changes.

## Integration order

1. **B-INT-1: Corpus importer** (days, pure win) — `captures/v3` JSONL →
   replay corpus manifests + labeled frame dataset. Unblocks RQ-002 and
   gives B-CORE-1 real material. No live integration risk.
2. **B-INT-2: Vocabulary migration** — tzpro vocab → `vocab` namespace
   with provenance; one schema going forward.
3. **B-INT-3: tzpro as event-emitting sidecar** — analyzer outputs become
   `sensor.acoustics.*` narrative events via NDJSON stdout (the playbook
   host protocol). The kernel doesn't care it's Python.
4. **B-INT-4: Ship Log Worker ↔ memory sync** — Fleet agent role home;
   digest sync per docs/08; anonymized-pattern policy enforced here.
5. **B-INT-5: ObjectTracker sidecar** — cross-frame blob tracks as
   first-class events (Rust, uses sonar-vision-rs).
6. **Later:** raw-signal driver, wheelhouse audio driver
   (plato-sonar-text), D1 timeline queries feeding Analyst.

## The strategic read

boat-agent's docs describe a fleet-learning vision system; tzpro-agent
*is* one, one boat deep, with real water under it. The two projects
converged independently on the same principles (local-first,
never-overwrite, vocabulary compounding, cloud-as-amplifier) — that
convergence is the strongest validation either design has. The division
of labor going forward:

- **boat-agent (this repo):** the trusted core, the control plane, the
  contracts, the gate.
- **tzpro-agent:** the perception workhorse and the data engine — first
  vessel node of the fleet, source of the corpus.
- **sonar-vision(-rs):** the simulator and tracker library.
- **plato-sonar-text:** the audio modality and the text-perception
  pattern.

One fleet, four specialties, one event contract between them.

---

**Cross-references:** bounties B-INT-1..5 in `docs/14`; G-1/G-2/G-3
generalization tracks; Q-IO-1; legacy RQ-002 (answered with our own
corpus), legacy Insight-005 (solved by tzpro's TCP bridge pattern).
