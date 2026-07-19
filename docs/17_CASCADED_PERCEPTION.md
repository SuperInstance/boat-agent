# 17 — Cascaded Perception: The Three-Loop Analyzer

> **Target Audience:** Agents building or operating the perception tier
> (tzpro-agent integration, B-INT-3, G-2 track).
> **Purpose:** Formal design for time-tiered perception: fast narrow
> loops below, slow wide loops above, attention flowing down, records
> flowing up. Born from the captain's racehorse metaphor; this document
> makes it a contract.
> **Status:** Governing document for the `cascade/` daemon in tzpro-agent
> **Date:** 2026-07-19

---

## The problem

OpenClaw's cron woke up to analyze whenever OpenClaw felt like it. When
OpenClaw died, analysis died with it — captures kept flowing, but the
interpretation layer went dark and days of echogram analysis were lost.
Two lessons:

1. **Perception must not depend on a general-purpose agent's mood.**
   It needs its own constrained, independent daemon ("zeroclaw"): one
   job, its own scheduler, kill-safe, no external agent runtime.
2. **One analysis cadence is the wrong shape.** A sounder day has
   signal at three timescales — what just appeared (seconds), what the
   last ten minutes mean (the searchable record), what the day adds up
   to (the briefing). Different timescales want different models,
   different context widths, different retention.

## The metaphor, kept

- **Minute loop = racehorses with blinders.** Small context, narrow
  gaze, fast. They take notes. They see one frame, not the day.
- **10-minute loop = the scribe.** Reads the racehorses' notes (mostly
  for lat/lon and sequence — the spatial track), plus the canonical
  10-minute frame, and writes the *searchable record*: word-based +
  structured JSON. It also steers the racehorses — it can rewrite their
  blinders.
- **Hourly loop = the analyst.** Reads the day's scribe records and
  writes the briefing: summary, impact, recommendations with
  confidences. Available on demand at any time.

The captain can steer any loop from the main chat at any time. Steering
is attention, not interruption.

## Mapping to boat-agent laws

This is not a new architecture — it is docs/05's law applied to
perception: **speed decreases upward, authority (over attention)
increases upward.**

| Loop | Cadence | Model tier | Context | Output (bus mapping) |
|------|---------|-----------|---------|---------------------|
| **M1** minute | 60 s | tiny local vision (moondream-class; gemma4:12b until pulled) | one frame + gaze hint | minute note (transient; novel ones → narrative lane) |
| **M10** 10-min | on capture | medium local vision (gemma4:12b) | canonical frame + ~10 M1 notes + gaze | canonical record → `sensor.acoustics.echogram_record` |
| **H1** hourly | 60 min + on-demand | largest local / cloud when online | day's M10 records + retained M1 notes | briefing → `agent.analysis.finding` |

### The gaze channel (attention flows down)

`gaze.json` is the only downward channel. Any tier (M10, H1, or the
human via chat/command file) may write a focus directive:

```json
{
  "focus": "watch the 25-35fm band for thermocline breaks",
  "set_by": "M10|H1|human",
  "ts_utc": "2026-07-19T23:10:00Z",
  "ttl_s": 3600
}
```

M1 reads it fresh before every run and injects it into its prompt —
that is the blinders being adjusted. Conflict rule: **human > H1 > M10**;
expired directives vanish. Downward attention and upward records never
share a channel.

### Retention (the GC contract)

- **M1 notes are transient by default.** They live in a ring buffer and
  are garbage-collected. Exception: a note whose novelty score crosses
  threshold is retained — zero-shot observations of how something
  *looked* are training data (this is the vocab/patterns pipeline's raw
  ore, docs/08).
- **M10 records are canonical.** Never GC'd. They are the searchable,
  vectorizable payload (A2A-native JSON).
- **H1 briefings are canonical.** Never GC'd.
- **Evening final read.** Before the day's discarded minute frames are
  GC'd, the evening pass reads them one last time — a cheap batch scan
  for anything the blinders missed. Findings append to the day briefing.
  Only then does GC run. Nothing is deleted unread.
- **PNGs:** the 10-minute canonical frames are kept. Minute-cadence
  frames (when capture cadence increases) are GC'd after the evening
  final read unless an M1 note marked them novel.

### The "zeroclaw" constraints (independence contract)

1. Single process, own scheduler, own heartbeat file. Windows Task
   Scheduler restart-on-failure. **No dependency on OpenClaw, MCP, or
   any external agent runtime.**
2. Read-only on the captures tree. Writes go only to its own dirs and
   the canonical record sidecar files.
3. Kill-safe between any two frames: every output is written
   atomically (temp + rename); a kill mid-cycle loses at most one
   loop's work.
4. Idempotent: re-running any loop over the same input produces the
   same output file, not a duplicate.
5. Model-degraded mode: if Ollama is down, the daemon keeps watching
   and queues; it does not crash, spam, or invent analysis. Quiet hours
   are logged, not filled.
6. Everything the daemon decides is itself a record: which frames it
   analyzed, which it skipped, why a note was kept. The Auditor's
   provenance rule applies to perception too.

## Why the small-context racehorses are a feature

Large context is not free accuracy — for anomaly spotting it is noise.
A model seeing one frame with one job ("what is here, is it novel")
outperforms a model reasoning about the whole day for *detection*, just
as a watchstander beats a historian for "mark, bearing 270." The tiers
exist so each model is asked only the question its context window can
honestly answer. The scribe's job is synthesis; the racehorse's job is
noticing. Don't blur them.

## Interaction with the stage-gate and vocab

- M10 records carry the vocabulary terms from `vocab` (the captain's
  words), and unknown patterns are flagged `unclassified` — they feed
  the Analyst's vocabulary-proposal loop (docs/08).
- Retained novel M1 notes are candidates for the RQ-002 fine-tuning
  dataset. They land in memory with provenance, never loose files.
- When tzpro speaks to boat-agent (B-INT-3), the canonical M10 record
  IS the `sensor.acoustics.echogram_record` payload. One schema, no
  translation layer.

---

**Implementation:** `cascade/` in tzpro-agent (see its README).
**Cross-references:** docs/05 (timescale law), docs/08 (retention),
docs/09 (attention economy), docs/16 (ecosystem), G-2 (frame-to-event).
