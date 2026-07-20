# 23 — UX Deep Research: Evidence Base for Human Surfaces

> **Target Audience:** Everyone building captain-facing surfaces.
> **Purpose:** The research foundation under docs/20 and docs/22. Three
> deep-research streams (marine touchscreen HCI, timeline-review
> patterns, AI-trust presentation), distilled to principles, with every
> principle sourced and every inference flagged.
> **Status:** Governing · Research date: 2026-07-19 · Tags: [R]=research-backed, [I]=inference

---

## The executive deltas (what changes today)

1. **Strip "91% conf" from captain-facing surfaces.** Raw model scores
   aren't probabilities and don't calibrate trust — they inflate it on
   good days and nuke it on the first visible miss. Show **calibrated hit
   rates** instead: "17 of my last 20 calls like this held." [R]
2. **The dial display becomes an FMA.** Annunciate *effective* autonomy
   (what the system is actually doing), not dial position — with the gap
   and its cause visible. Aviation made this law after killing people
   with the alternative. [R]
3. **Escalation cards: evidence first, recommendation behind a tap.**
   Recommendation-first + 15-second target = a rubber-stamp machine that
   trains itself on its own answers. [R]
4. **Scrubbing = one variable-rate drag gesture.** Resolves docs/20's
   open question: drag owns scrubbing (vertical offset = finer rate);
   1×/2×/10× presets are only for unattended playback. No jog-wheel UI,
   ever — the strongest negative result in the research. [R]
5. **The "94% agreement" metric is a sycophancy detector, not a
   competence score** — show the full contingency table including the
   times disagreement was correct, weighted by outcomes. [R]

---

## Stream 1 — Marine touchscreen HCI

**The hardware truth:** marine MFDs ship at 900–1300 nits; a consumer
laptop is 300–400. Brightness is the bottleneck, not palette — dark vs
light theme matters little; **luminance + contrast drives legibility**
([Buchner 2009](https://www.psychologie.hhu.de/fileadmin/redaktion/Oeffentliche_Medien/Fakultaeten/Mathematisch-Naturwissenschaftliche_Fakultaet/Psychologie/AAP/Publikationen/2009/Buchner_Mayr_Brandt__2009_.pdf), [Dobres 2017, MIT AgeLab](https://jdobr.es/pdf/Dobres-etal-2017-Ambient.pdf)). The professional answer exists and is free: **OpenBridge's four vetted palettes** (Bright/Day/Dusk/Night) ([openbridge.no](https://www.openbridge.no/)). Night vision is wrecked by one bright glance ([USCG R&D](https://www.plaisance-pratique.com/IMG/pdf/6_-_Rothblum_-_Night_Vision_and_Nighttime_Lighting_for_Mariners_2_.pdf)).

**Touch at sea:** 44 px is a dry-office floor (~11.6 mm of a 10.5–26 mm
static range). Vibration nearly doubles touch error (10%→17%,
[avionic study](https://www.mendeley.com/catalogue/4e4b05e7-e266-3fa2-89a9-1a7503156d30/)). **Thumb-on-bezel (braced) beats freehand under vibration** — the same reason knobs survive. Wet capacitive failure is hardware physics ([MicroTouch](https://microtouch.com/wp-content/uploads/2021/06/WP-Noise_Supression_v2.pdf)); firmware mitigations ([RainCheck, Wobbrock ICMI 2018](https://faculty.washington.edu/wobbrock//pubs/icmi-18.02.pdf)) are unavailable to a web app. Raymarine shipped touch-only Axiom and the market punished it — "crap with wet fingers or in fog" ([Cruisers Forum](https://www.cruisersforum.com/forums/f2/ray-marine-axiom-series-224452.html)); bridge officers are openly hostile to touch for vital functions ([Danielsen 2022](https://link.springer.com/article/10.1007/s10111-022-00700-8)).

**Principles:**
- **U1. Four OpenBridge palettes**, auto-switched, one-tap override. [R]
- **U2. Design for 350 nits:** fat strokes, no gray-on-gray, matte
  anti-glare film recommended; echogram stays dark, chrome high-contrast. [R]
- **U3. Type by visual angle:** HUD numerals ≥ 8 mm cap height (~30 px),
  body ≥ 18–20 px, humanist sans ([ISO 15008](https://cdn.standards.iteh.ai/samples/50805/8b8f0af110d34e91985fb1e6c182abe7/ISO-15008-2009.pdf), [MIT AgeLab](https://web.mit.edu/reimer/www/pdfs/AgeLab_typeface_white_paper_2012.pdf)). [R]
- **U4. Primary targets 64 px, nothing interactive under 48 px** with
  dead space between. [R for floor+motion penalty; I for exact figure]
- **U5. Every core interaction works braced:** timeline at the bottom
  edge (heel-of-hand on bezel), momentum drag, no precision gestures for
  anything important, stepper buttons beside sliders. [R/I]
- **U6. Keyboard is the rotary encoder:** arrows scrub, space plays,
  0–3 sets dial — every function also keyboard-reachable, so a $15
  keypad covers wet-screen days. [R]
- **U7. Motion-adaptive sizing experiment:** when roll/pitch RMS crosses
  threshold, grow targets a tier (48→64→80) and suppress low-priority
  controls. Precedent: [Kane 2008 movement-adaptive targets](https://3dvar.com/Kane2008Getting.pdf). Validate on replay corpus. [I]

## Stream 2 — Timeline / day-review patterns

The meta-finding: **every mature review domain (CCTV, sports, bodycam,
aviation) converged on jump-driven review** — events, highlights,
thumbnails, playlists. Playback speed is the detail tool, not the review
tool ([Milestone Rapid REVIEW](https://doc.milestonesys.com/sc/pdf/2025r2/es-ES/MilestoneXProtectSmartClient_UserManual_es-ES.pdf), [Hudl coach's guide](https://blog.callplaybook.com/blog/coach-video-review-software-hudl-dartfish-alternatives), [CloudAhoy](https://www.cloudahoy.com/)).

**Pattern catalog (the ones we're adopting):**

- **T1. Variable-rate drag** (iOS Hi-Speed Scrubbing): drag along bar to
  scrub; slide finger *down* while holding for finer rates. One gesture
  covers 12-hour sweeps and frame-landing. ([recipe](https://arthurhammer.de/2020/03/uislider-with-scrubbing-speeds/)) [R]
- **T2. The frame IS the thumbnail.** YouTube storyboards prove the
  skim mechanism ([Mux storyboards](https://www.mux.com/articles/extract-thumbnails-from-a-video-with-ffmpeg)); at 10-min cadence our 72
  stills/day paint live at cursor (≤100 ms budget already set). At 30s
  cadence, server-side storyboard strip. [R]
- **T3. Provenance-separated event tracks + adaptive clustering.**
  Machine marks (schools/anomalies) and human marks (deck presets) get
  different tracks and different visual weight ([Axon Evidence](https://www.axon.com/help/axon-evidence-legacy/software/axon-evidence-legacy/evidence/review-evidence/clips-and-markers.htm)); overflow
  merges to "+N" that expands on zoom ([KronoGraph](https://cambridge-intelligence.com/blog/designing-intuitive-data-experiences-with-graph-visualizations/)). Enforced in the component, not as user discipline. [R]
- **T4. Two-level overview+detail, linear time.** Whole-day bar + zoomed
  window strip. **Never fisheye** — distortion warps time judgments
  ([Cockburn 2007 review](https://worrydream.com/refs/Cockburn_2007_-_A_Review_of_Overview+Detail,_Zooming,_and_Focus+Context_Interfaces.pdf)); zoom-to-detail event timelines work for non-technical
  users ([LifeLines](http://www.cs.umd.edu/users/ben/papers/Plaisant1998LifeLines.pdf)). [R]
- **T5. Highlight landing + padded loop + event-hop buttons.** Auto-curation with context padding ([Hudl Replay lead/lag](https://www.hudl.com/releases/replay), [BlackVue pre-event buffer](https://cloudmanual.blackvue.com/docs/cloud-features/playback-4/)); "next/prev event" as two 64 px buttons = event-first review with zero aiming. [R]

**Anti-patterns:** skeuomorphic jog/shuttle controls (professionals
ignore them: [ProVideo Coalition](https://www.provideocoalition.com/review-logitech-craft-advanced-keyboard-with-creative-input-dial/)); Christmas-tree timelines; playback-speed thinking for review. [R]

## Stream 3 — AI trust presentation

The canonical frameworks: [Lee & See 2004, appropriate reliance](https://internationalc2institute.org/s/015.pdf); [Parasuraman & Riley 1997, misuse/disuse taxonomy](https://ir.library.oregonstate.edu/downloads/z603r571k). The uncomfortable findings:

- **Numeric confidence doesn't calibrate trust; explanations raise trust
  whether the AI is right or wrong** ([Zhang 2020, FAT*](https://dl.acm.org/doi/10.1145/3351095.3372852); [Bansal 2021, CHI](https://arxiv.org/html/2409.10250v1)). What works: **continuous system-ability displays** (faster takeovers, appropriately *lower* trust: [Helldin 2013](https://utoronto.scholaris.ca/server/api/core/bitstreams/9bcf38a0-213a-440d-97df-6870e45b4afb/content)) and **empirical track records**.
- **Friction beats explanation:** cognitive forcing functions cut
  overreliance; cheap always-on explanations become trust theater
  ([Buçinca 2021, CSCW](https://arxiv.org/html/2409.10250v1); [Vasconcelos 2023](https://arxiv.org/html/2606.25489v1)).
- **Alarm fatigue is measured in deaths** (Joint Commission SEA 50: 80
  deaths; 85–99% of alarms non-actionable); EEMUA 191/ISA-18.2 encode
  the fix: every alarm must alert, inform, *and guide*; floods are
  system failures. Our attention budget already exceeds EEMUA targets. [R]
- **The opposite pole is just as deadly:** out-of-the-loop decay
  ([Endsley & Kiris 1995](https://www.researchgate.net/publication/238726310_The_Out-of-the-Loop_Performance_Problem_and_Level_of_Control_in_Automation)) — rare interrupts + high reliability = a captain
  who can't catch the first real failure. Engagement must be *engineered*.
- **Mode annunciation is regulated for a reason** ([14 CFR 25.1329](https://hfcc.dot.gov/publications/docs/GeneralGuidance/zz_FAA_GeneralGuidanceDoc_Chapter_04_Section_03.pdf)): show actual state,
  not selector position; transitions and reversions must be announced.
  The canonical questions: *what is it doing? why? what will it do next?*

**Principles:**
- **A1. Track record, not scores.** Per-class calibrated hit rates from
  the black box; raw confidences stay internal (Auditor-only). [R]
- **A2. Uncertainty as system state:** persistent coarse ability
  indicator (sonar quality, sensor agreement, envelope headroom). [R]
- **A3. Escalation inventory is rationalized forever** (EEMUA lifecycle);
  floods = system defect. [R]
- **A4. Annunciate effective mode + armed modes + what's next, always.** [R]
- **A5. Evidence behind one tap, before the recommendation.** [R]
- **A6. Engineer engagement:** daily after-action digest as ritual
  surface; periodic coach-back drills; "days since meaningful captain
  input" as an Auditor metric that *lowers* effective autonomy. [R]
- **A7. Verified vocabulary, never generic ML terms** — terms carry
  testable referents ("you call this the feed layer — I'm seeing it at
  28–34 fm, right?"). [R-adjacent: fisheries participatory literature; I]

## The five trust risks in our current design (and fixes)

| # | Risk | Fix |
|---|------|-----|
| R1 | "91% conf" = uncalibrated score posing as measurement | A1 — strip from captain surfaces |
| R2 | "94% agreement" = sycophancy metric pushing the dial up | full contingency table, outcome-weighted, misses always shown; weekly digest only |
| R3 | Recommendation-first escalations = rubber-stamp machine training itself | A5; measure rubber-stamp rate with known-answer probes |
| R4 | Dial shows ceiling while boat runs at floor = built-in mode confusion | A4 — effective autonomy is the primary readout; gap annunciated with cause |
| R5 | Timeouts + batching teach that silence is a complete strategy | A6 — fallback-activated badges persist until acknowledged; engagement metrics feed autonomy |

## Amendments to existing docs

- **docs/20:** presets-vs-rate-drag question RESOLVED (T1: drag owns
  scrubbing; presets only for playback). 44 px rule amended by U4
  (64 px primary / 48 px floor). Palette rule amended by U1 (four
  OpenBridge palettes). New: U6 keyboard fallback, U7 motion-adaptive
  experiment.
- **docs/22:** components 2/3 adopt T1/T2; event tracks per T3; zoom per
  T4; highlight loop gets configurable lead/lag + next/prev-event
  buttons (T5). New explicit anti-requirement: no jog-wheel UI.
- **docs/09:** A4/A6 amendments — effective-autonomy annunciation
  requirement, engagement engineering, fallback badges. R2 changes the
  trust-metric display; R3 changes escalation card ordering.
- **docs/13 (open questions):** add Q-UX-1 (validate U7 motion-adaptive
  sizing on replay corpus) and Q-UX-2 (wet-finger hardware path — bonded
  marine panel vs keypad fallback).

---

**Method note:** three parallel deep-research agents with web sources;
all principles tagged [R]/[I]; gaps flagged (no maritime roll-angle
target-size study exists; vocabulary-trust evidence is adjacent-domain).
