# 20 — GUI & Presets: Taste-Level Decisions for Human Surfaces

> **Target Audience:** Agents building human-facing surfaces (dashboard,
> scrubber, settings).
> **Purpose:** The small set of human-facing design decisions where taste
> matters more than architecture. External review: Claude Code 2.0.65 and
> mini-agent perspectives, 2026-07-19 (this document records the decisions,
> dissent goes in review log).
> **Status:** Governing for UI work · Constraint: the UI is a boatctl
> client, always (docs/10)

---

## Context that shapes everything

The human surface is a **wheelhouse touchscreen**: bright sun, salt
spray, wet hands, a boat that moves. The captain is non-technical and
supervises; agents operate. Every pixel competes with fishing.

## The day scrubber (the primary human surface)

The scrubber replays a fishing day: echogram timelapse + analysis
overlay + GPS track (the DAW-dashboard idea from tzpro's VISION,
formalized here).

**Three critical decisions:**

1. **High-contrast overlay layers.** Echogram at ~70% grayscale; vision
   analysis in distinct palettes (no red/green — sun and colorblindness);
   GPS track thin semi-transparent cyan. Layer opacity by **large
   sliders**, never nested menus.
2. **Direct-manipulation timeline.** The bottom third is one physical
   scrub bar: drag-to-scrub, tap-and-drag for A/B loop ranges. **No
   "seek to timestamp" dialogs, ever.**
3. **State preservation across sessions.** Last-viewed time, layers,
   zoom — exactly as left, surviving app close and Windows restart.

**The mistake everyone makes:** tiny playback controls and modal seek
dialogs. On a bouncing boat in sunlight: targets ≥44×44px, immediate
visual feedback. "File → Open → Select Date" is a non-starter.

**Data binding:** the scrubber reads docs/18's twin directly — `frames`
(time index) + `echogram_records` (overlay) + `frames_geo` (track) —
through boatctl queries, never through file paths.

## Presets: what goes where

The tiering principle: **captain-facing knobs are things he decides
weekly; installer knobs are set once; ML-tuned parameters are buried
because exposing them creates confusion, not control.**

| Tier | Knobs |
|------|-------|
| **One-tap presets (GUI)** | Capture mode: Cruising \| Fishing \| Docked · Retention: 3 \| 7 \| 14 \| 30 days · Upload policy: WiFi-only \| Satellite (with warning) \| Off · Overlay density: Minimal \| Standard \| Detailed |
| **Buried config (vessel.toml)** | Capture cadence (Hz) · novelty threshold · compression levels · Parquet batch sizes · vision confidence thresholds · warm_days |
| **Hardcoded (kernel)** | Content-addressing algorithm · time-sync protocol · SQLite schema · worker auth flow · crypto parameters |

Preset changes write to vessel.toml via boatctl (capability-checked,
logged with actor identity — a preset change is an auditable event, not
a file edit). Presets map onto config; they never bypass it.

## Quick modifiability: the command palette pattern

Non-technical captain, zero TOML exposure:

```
CTRL+SPACE (or on-screen ⚙)
┌─────────────────────────────────────┐
│ > Keep more days                    │
│ > Capture faster when fishing       │
│ > Upload over satellite             │
│ > Show less vision overlay          │
└─────────────────────────────────────┘
```

- Natural-language matching to preset changes.
- **Preview before confirm:** "Will keep 30 days, uses ~2 GB more storage."
- **One-tap undo:** "Reverted to previous settings."
- Touch-optimized: large targets, swipe to cancel.

Why it works: no file paths, no syntax, no save/apply distinction. The
same palette exposes health checks ("Show storage usage") without
cluttering the main UI. And because it speaks boatctl, **the palette is
also an agent interface** — the Operator can invoke the same commands
with the same audit trail.

## Relationship to the autonomy dial and escalation UX

The dial (docs/09) is the one control more important than any preset:
physical, always visible, always honest. Presets live *below* it in the
visual hierarchy. Escalation cards follow the scrubber's rules: ≥44px
targets, options with consequences, timeout fallback visible
("defaults to Hold in 1:47").

## Deck-mode event presets (mini-agent perspective — accepted)

Distinct from system knobs: the events a captain marks *while fishing*,
one tap each, auto-stamped with GPS+depth+speed+time:

- **Mark School** — annotation box on echogram, tied to position/depth.
- **Haul Event** — snapshots the echogram range, prompts catch count on retrieval.
- **Gear Issue** — captures last 60 s of echogram + sensor trace, flags for review (no diagnostic jargon).
- **Bycatch Alert** — freezes annotation, audible chime, queues photo prompt.
- **End of Set** — closes the log segment, auto-summarizes catch/effort.

Anything requiring frequency/gain/threshold tweaks lives in **engineer
mode behind a PIN** — never on the deck screen. These presets write
`labels` and `catch.log` rows (docs/18) — they ARE the outcome-labeling
channel the learning loop eats (docs/08).

## The scrubber's real risk (mini-agent — accepted)

Not density of controls but **loss of trust**: if the scrubber stutters,
desyncs from depth/speed traces, or loads too slowly to recall a moment
mid-deck-handover, the captain reverts to paper and the system dies
silently. Performance budgets are therefore correctness requirements:
frame-at-cursor < 100 ms from warm tier, trace sync error < 1 frame.
Secondary risk: annotation Christmas-tree — preset color-coding with
**one toggleable layer at a time** is non-negotiable.

## Open taste questions (flagged, not decided)

- Scrubber playback speed presets (1×/2×/10×) vs continuous rate drag —
  decide with the first captain session, not in advance.
- Whether "novel frame" badges appear inline on the timeline (risk:
  badge fatigue; mitigations: density preset).
- Phone companion: read-only digest vs full scrubber. Start digest-only
  (docs/19 API Worker already serves it).

---

**Cross-references:** docs/09 (dial, escalation, attention budget),
docs/10 (UI = boatctl client), docs/18 (data binding), docs/19 (phone API).
