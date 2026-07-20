# 21 — Phase 2 Plan: Durability → Visibility → Backup → Replay

> **Target Audience:** Everyone. This is the operating plan for the next
> 2–4 weeks.
> **Purpose:** Record the planning council (four model perspectives),
> the adjudication of disagreements, and the resulting phase plan with
> checkpoints.
> **Status:** Active plan · Council date: 2026-07-19
> **Review:** weekly, against the checkpoints below

---

## The council

Four positions solicited on the same brief (kernel done, cascade live,
twin designed-not-built, data-loss incident fresh, captain wants visible
value):

| Position | Priorities | Biggest risk flagged | Don't build yet |
|----------|-----------|---------------------|-----------------|
| **Kimi** (me) | twin impl → permanence → scrubber → backup-lite | scope split across threads | actuation, fleet, frameworks |
| **Claude** | storage pipeline → replay+scrubber → cloud sync | migration data loss | moondream, gate, novelty calibration |
| **mini-agent** | local twin this week → scrubber wk2 → cascade-into-twin wk3 | orphaned cascade daemon (permanence belongs IN priority 1) | cloud sync, replay *framework*, historical importer |
| **Hermes** | meta.db+WAL → replay harness → scrubber | cloud-first fallacy (sync before local schema stabilizes) | stage-gate automation |

## Where they agree (this is the plan's spine)

1. **Durability first.** Unanimous #1: implement the local twin now.
   Every day without it, data flows into a sieve.
2. **The scrubber is the credibility moment.** Unanimous top-3: the
   captain must SEE his day replayed. "The first thing he'll show his
   wife" (mini-agent).
3. **No playbook stage-gate automation.** Unanimous: manually verify
   with simple replay first; automating an unproven process is a faster
   way to fail.
4. **No replay *framework*.** Ad-hoc beats premature abstraction
   (mini-agent); hermes wants replay as a *laboratory* — both agree on
   simple and useful over general and early.

## Disagreements, adjudicated

**Cloud sync: now (Claude) vs not yet (mini-agent, Hermes)?**
→ **Split it.** *Backup-lite* (R2 multipart upload of signed bundles +
manifests — dumb, resumable, one-way) ships in week 3: it directly
closes the durability trauma and has no schema-coupling risk. The
*smart* cloud (D1 index, Vectorize, Workers AI analysis) waits until the
local schema has 30 days of stability. Hermes's "cloud-first fallacy"
applies to the index, not to a dumb bucket.

**Historical corpus import?**
→ **Dropped** (mini-agent's call, correct): the old data is gone; what
the cascade produces now is higher quality. Importer scope reduced to
*going-forward* ingest of the existing capture folders.

**Daemon permanence: separate thread?**
→ **Folded into priority 1** (mini-agent's catch): the cascade is
verified but orphaned — one reboot from silent death. Permanence without
a twin is pointless; a twin without a running daemon starves. Same week,
same work item.

## The plan

### Week 1 — Durability: the twin becomes real
- Implement `meta.db` (docs/18 schema) + strict write path
  (blob→fsync→rename→row).
- B-INT-1 (reduced): ingest `captures/v3/**` going forward; today's 18
  frames backfilled.
- Cascade rewired: M1 notes, M10 records, H1 briefings write to the
  twin, not loose files.
- Daemon permanence: Task Scheduler entry + heartbeat watchdog +
  kill-recovery test.
- **Checkpoint:** reboot the laptop mid-capture; daemon auto-restarts;
  zero frames lost; every frame hash-verified in meta.db.

### Week 2 — Visibility: the scrubber MVP
- Day scrubber reading the twin via boatctl queries (docs/20 decisions:
  direct-manipulation timeline, overlay layers, session state, ≥44px
  targets).
- Tech: keep it the thinnest thing that renders — local web UI over
  meta.db is acceptable; no new framework without a fight.
- **Checkpoint:** captain scrubs a full 12-hour day in under 2 minutes,
  unassisted, and finds "that school at 14:30" without help.

### Week 3 — Backup-lite: off-boat copies
- R2 upload of signed hourly bundles + manifests (docs/19 upload
  protocol only — no D1/Vectorize yet); resume-on-reconnect.
- USB manifest backup with post-copy re-hash verification.
- Restore drill: wipe a day locally, restore from R2, verify hashes.
- **Checkpoint:** restore drill passes; GC's two-verified-copies rule
  now satisfiable and enforced.

### Week 4 — Replay v0 + calibration
- Ad-hoc replay: feed recorded twin data through reducer+envelope,
  diff verdicts. NOT a framework — a script that answers "would we have
  done the same thing?"
- Novelty calibration against the accumulated corpus (the threshold
  problem found in the field, cascade README).
- **Checkpoint:** replaying day N reproduces day N's state hashes
  exactly; novelty retention lands in a sane band (5–25% of frames).

## Deferred (explicitly not this phase)

Playbook stage-gate automation · cloud index/AI analysis (D1, Vectorize,
Workers AI) · fleet anything · moondream (quality-of-life, not critical
path) · WASM migration · historical corpus archaeology · scrubber
playback-speed debates (decide with captain in seat).

## Acceptance for the phase

One sentence, from mini-agent's test: **"Can the captain use it on
Saturday without me on the phone?"** — for scrubbing his day and
trusting nothing is being lost. If yes, Phase 2 succeeded.

---

**Cross-references:** docs/18 (twin), docs/19 (cloud — backup-lite
subset only this phase), docs/20 (scrubber decisions), docs/17 (cascade),
docs/14 (bounties B-CORE-1, B-INT-1 modified by this plan).
