# 22 — Day Scrubber MVP: Design

> **Target Audience:** Agents building the scrubber (Week 2, docs/21).
> **Purpose:** Architecture decision and component plan for the captain's
> primary human surface: replaying a fishing day.
> **Status:** Governing for the Week-2 build · Ideation: mini-agent
> (2026-07-19), adjudicated against docs/20 decisions · Date: 2026-07-19

---

## The decision

Four architectures considered:

| Option | Verdict |
|--------|---------|
| **A. Local web UI** (stdlib/ASGI server + vanilla HTML/JS in Edge kiosk) | ✅ **PICK** — browser gives canvas, drag, touch, zoom, PNG rendering for free; SQLite via Python; ships in days |
| B. Tauri/Electron native shell | ⏳ Later — true fullscreen, but toolchain cost kills the week; the API surface is the contract, so the swap is cheap |
| C. Python + PySide6/tkinter | ❌ touch on tkinter is painful; canvas redraw at scrub speed is flaky on Windows |
| D. Terminal TUI | ❌ PNG on a 14" wheelhouse screen is a non-starter |

**The API is the contract.** A local server (`scrubber/serve.py`)
reads the twin directly (`twin.Twin`, docs/18) and serves JSON + blobs
over loopback; the UI is one `index.html`. Any future shell (Tauri,
phone) is a client of the same endpoints — same discipline as boatctl
(docs/10).

## Endpoints (the whole contract)

```
GET /api/day/<YYYY-MM-DD>   → {frames: [{frame_id, ts_utc, lat, lon, sog,
                               cog, sha256, tier, novelty, keep_reason}],
                               records: [{frame_id, record_json...}]}
GET /api/blob/<sha256>      → the PNG (from blobs/, or cold-tier pointer)
GET /api/day/<date>/highlight → the "holy shit" cursor (below)
```

Performance budgets (docs/20: trust is the risk — these are correctness
requirements): `/api/day` < 300 ms for 24 h of frames+records;
frame-at-cursor < 100 ms warm; trace sync error < 1 frame.

## Layout (docs/20 rules applied: ≥44 px targets, direct manipulation,
no dialogs, state survives restart)

```
┌──────────────────────────────────────────────────────────────────┐
│  ◀ Mon 19 Jul   14:30:12   55°47.6′N 131°40.1′W   2.1 kt   ⏯ ⏪⏩  │  ← HUD bar (48 px)
├──────────────────────────────────────────────────────────────────┤
│            ╭────────── ECHOGRAM PNG @ cursor ──────────╮         │
│            │  ░▒▓ school 26–27 fm ▓▒░  (overlay on)     │         │  ← Frame pane (~60%):
│            │       · · · cyan GPS track · · ·           │         │    70% grayscale,
│            ╰────────────────────────────────────────────╯         │    analysis overlay
│  [overlay ▓▓▓▓▓░░░]  [track ▓▓▓▓▓▓░░]   Minimal|Standard|Detailed │  ← big sliders + preset chips
├──────────────────────────────────────────────────────────────────┤
│ ▕━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▏           │  ← Timeline (bottom third):
│   06:00        ▲14:30        18:00    │tick marks: records,       │    drag-to-scrub,
│                cursor                 schools, catches, novelties │    tap-drag A/B loop
└──────────────────────────────────────────────────────────────────┘
```

## Build order (five components, acceptance criteria from ideation)

1. **Twin reader + day API** — `/api/day/:date` returns frames + joined
   records. AC: one call, < 300 ms.
2. **Timeline component** — canvas, drag-to-scrub, A/B loop, event
   ticks from records + labels. AC: `frame_at(ts)` ≤ 100 ms warm.
3. **Frame viewer** — blob `<img>` + overlay composited from
   record_json. AC: pixel-accurate render, pinch-zoom, double-tap fit.
4. **HUD + transport** — clock, lat/lon/sog/cog; big ⏯/⏪/⏩ (≥ 64 px),
   1×/2×/10×. AC: state survives restart (localStorage).
5. **Layer opacity + presets** — two large sliders (overlay, track),
   chip row (Minimal/Standard/Detailed). AC: one toggle per layer, no
   red/green palette, ≥ 44 px.

## The "holy shit" moment (ship it in v1)

On first launch, the scrubber does NOT open on a blank timeline. It
lands the cursor on the day's **highest-novelty moment** (max
`frames.novelty` joined with a confident record), draws a 5-second A/B
loop around it, and pops a one-line caption at the top:

> **14:28 — school at 26 fm, hard bottom at 50 fm, 91% conf.**

The captain did nothing and already knows what happened today. Trust
established before he touches a control. (`/api/day/<date>/highlight`
implements this server-side.)

## Defer list (ruthless, per ideation)

Annotations/label capture (deck-mode presets are a separate MVP) ·
multi-day/calendar UI · geographic mini-map (cyan track only) · phone
companion (docs/19 API covers it later) · novel-frame badges · FTS
search · briefings reader · command palette · Tauri rewrite, installer,
autostart.

## Amendment (docs/23, 2026-07-19)

- Component 2 (timeline): **variable-rate drag** — drag scrubs, finger
  offset downward = finer rate. No jog-wheel UI, ever (strongest
  negative result in the research).
- Component 3 (frame viewer): the frame IS the thumbnail at 10-min
  cadence; server-side storyboard strip when cadence reaches 30s.
- New: provenance-separated event tracks (machine vs human marks) with
  "+N" clustering enforced in the component; two-level zoom, **never
  fisheye**; highlight loop gets configurable lead/lag + big
  next/prev-event buttons.
- New component 0: **keyboard fallback** (arrows/space/0–3) — covers
  wet-screen days.

## Where it lives

`tzpro-agent/scrubber/` (serve.py + static/index.html) for the MVP —
it reads the same twin the cascade writes. Graduates to boat-agent's UI
when the kernel grows its shell adapter (docs/11).

---

**Cross-references:** docs/20 (UX rules, trust budget), docs/18 (twin
schema + query interface), docs/21 (Week-2 checkpoint: "captain scrubs
a 12-hour day in 2 minutes, unassisted").
