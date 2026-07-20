# Marine Repos Synergy — SuperInstance Cluster Study

**Date:** 2026-07-19 · **Analyst:** mini-agent (research), Kimi (synthesis)
**Scope:** Marine/fishing-facing repos in the SuperInstance org, assessed
for synergy with boat-agent + tzpro-agent.

---

## Per-repo findings

| Repo | What it actually is | Maturity | Synergy | Cost |
|------|--------------------|----------|---------|------|
| **signalk-bridge** | Signal K → Oracle relay bridge; auto-logs boat instrument events (NMEA→Signal K→events) | Working (Python, ~26KB source) | **HIGH.** Signal K is the open marine data standard — our L0 drivers (B-DRV-1/2) can consume Signal K instead of hand-parsing NMEA-0183/N2K; AIS contacts become bus events for free | Adapter |
| **ship-log-modules** | 12 built vessel-ops modules: crew-logbook, map-view (Leaflet), fuel-tracker, maintenance-scheduler, trip-planner, **tide-predictor**, weather-feed, **ais-tracker** (Signal K WS), engine-hours, export-csv, fishcoin-ledger, vessel-quest (gamification) | Built (HTML/Python, spec docs) | **HIGH for overlays:** tide/weather/AIS layers for the scrubber timeline (docs/22) and Analyst context (sea-state classification input). Module spec/matrix docs are genuine integration engineering | Adapter |
| **ship-log-search** | Semantic + spatial + timeline search Worker over time/location-stamped vessel records | Working, deployed | Precedent + likely host for the cloud-twin search tier (docs/19); its API shape already matches "when/where/what" vessel queries | Adapter |
| **ship-log-sync** | Offline-first local SQLite sync for ship-log-search — runs on the boat | Working (Python) | Direct overlap with docs/19 sync protocol; a working offline-queue implementation to study for our digest sync | Adapter |
| **fleet-weather** | Fleet-wide operational weather Worker (conditions + forecasts) | Working, deployed | Weather overlay for scrubber; wind/sea-state context feed for the Analyst and envelope guards | Drop-in (API) |
| **fishinglog-ai** | Edge AI fishing vessel: Jetson-powered species classification, captain voice interface | Working | The Jetson edge-inference hardware path for species ID (beyond our screen-scraping); voice interface pairs with catch logging | Research |
| **activelog-app** | Domain-agnostic voice transcriber with location annotation | Working | Voice catch logging → `human.catch.log` events + transcripts for the learning loop (docs/08); pairs with plato-sonar-text audio path | Adapter |
| **trawl** | Commercial fishing on Working Animal Architecture: routes-as-pastures, quotas-as-fences — route/quota/regulatory/catch Python modules | Working (clean dataclass code) | Quota & regulatory tracking (commercial compliance — a real product feature); route recommendations as mission-layer (L3) input | Adapter |
| **module-registry** | KV-backed directory/registry for ship-log modules with health metadata | Working | Pattern for our plugin registry question (Q-ARCH-1); module health probing | Research |
| **activelog-ai** | AI fitness/activity tracker (sibling product) | Working | None direct; pattern donor only | Skip |

## Key observations

1. **A vessel software suite already exists in the org.** ship-log-modules +
   ship-log-search + ship-log-sync + fleet-weather + signalk-bridge form a
   working ship's-log product. Our differentiation must stay: trusted
   control kernel, safety envelope, staged-trust playbooks, provenance.
   We adopt their data-plane and overlays; they adopt our analysis records.
2. **Signal K is the right marine data plane.** Rather than growing custom
   NMEA/N2K parsers for every instrument, the Signal K ecosystem
   (open standard, existing gateways) should feed our drivers. signalk-bridge
   proves the org already speaks it.
3. **The tzpro/ship-log duplication needs one owner.** tzpro-agent posts to
   "Ship Log" today; ship-log-sync/search is the formal version. When B-INT-4
   lands, the M10 `echogram_record` should flow through the docs/19 sync
   protocol into the same search tier — one searchable vessel memory, not two.

## Top 3 recommended adoptions (value ÷ cost)

1. **signalk-bridge → L0 driver input.** Marine data-plane standardization:
   GPS/depth/engine/AIS via Signal K, one driver instead of four parsers.
2. **ship-log-modules tide/weather/AIS trio → scrubber overlay layers.**
   Tide-predictor, weather-feed, ais-tracker are built; they become timeline
   tracks (T3, docs/23) and Analyst context with adapter-level effort.
3. **ship-log-sync + ship-log-search → cloud-twin ingest/search.**
   Working offline-queue sync and deployed search give the docs/19 pipeline
   a running start; align schemas to `sensor.acoustics.echogram_record`.

**Deferred:** fishinglog-ai Jetson path (hardware phase), trawl quota
(product phase 3+), activelog-ai (pattern donor).
