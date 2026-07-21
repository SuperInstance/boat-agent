# 27 — Git-Agents: Small Modular Minds on a Shared Harness

> **Target Audience:** Agents building sensor/perception daemons, and
> the humans who'll assemble hardware for them.
> **Purpose:** The pattern that takes the cascade (one perception
> family) and generalizes it into a roster of narrow, single-purpose
> agents — each small, each honest, each disposable — watching weather,
> wind, engine heat, and anything else with a pulse.
> **Status:** Governing design · Date: 2026-07-20
> **Cross-refs:** docs/17 (cascade — the prototype), docs/26 (the day),
> docs/08 (memory), docs/06 (bus contract)

---

## The pattern

A **git-agent** is a narrow mind with three organs and one contract:

```
senses  ──▶  a small brain  ──▶  records (timestamped, located, versioned)
                │
             memory = git files. Its state is a repo. Its history is commits.
```

- **Narrow by design.** One realm per agent: weather, wind, engine heat,
  bilge. An agent that knows two realms is two agents. Narrowness is
  what makes small models sufficient, failures containable, and honesty
  checkable.
- **Git-native state.** Every git-agent's working memory is files in a
  repository: its notes, its baselines, its calibration, its own config.
  State changes are commits. "What did the wind watcher believe last
  Tuesday?" is `git log`. Rollback of *learning* is `git revert`.
  Harnesses will change; the repo is the durable artifact (docs/18's
  premise, applied to agent minds).
- **Small models on a well-made harness.** The harness does the heavy
  lifting: scheduling, I/O, validation, retention, retries. The model
  does one narrow judgment. Speed is irrelevant for most realms — an
  hourly weather synthesis has an hour of slack; a bilge reading has
  seconds, and that's a threshold check, not an LLM call.
- **Local first.** If the hardware has capacity, inference happens
  on-boat (Ollama-class, or a tiny specialist model). Cloud is a
  fallback lane for bursts, never a dependency.

## The harness contract (every git-agent, no exceptions)

Five channels, same shapes everywhere:

| Channel | Direction | What it carries |
|---------|-----------|-----------------|
| **pulse** | in | timestamped, location-stamped sensor readings: `wind {speed, dir}`, `temp {zone, celsius}`, `level {cm}` |
| **gaze** | in | attention directives from human, H1, or *other git-agents*: "watch the lee side of the island", "baseline the #2 alternator this week" |
| **trigger** | in | correlation wake-ups from the mesh: "event X happened, analyze now" (below) |
| **record** | out | the canonical outputs: notes (transient), records (canonical), escalations (rare) — same retention contract as docs/17 |
| **heartbeat** | out | liveness + queue depth + model used. Every agent is watchdog-able |

Plus the standing rules: atomic writes, idempotent reruns, quarantine
with reasons, GC only after final read, provenance on everything. These
are the docs/17 cascade constraints, promoted to the roster standard.

## The roster (first three)

### SCOUT — the weather/news analyst
Hourly cadence (or on trigger). Compares the NOAA feeds (tide, gridpoint
forecast — `cascade/tools/daily_context.py` is its seed) against what
the vessel's own sensors are *actually seeing*, and against every other
realm's activity. Its product is not weather data but **forecast
awareness**: "the forecast says NW 15 by 15:00; your wind pulse says
it's already here, 40 minutes early; the ebb starts at 14:00; wind
against tide by mid-afternoon." Small model is plenty — the reasoning
is comparative, not generative.

### PULSE — the wind watcher
The simplest possible git-agent and the proof of the pattern: each
reading is a pulse `{ts, lat, lon, speed_kn, dir_deg}`; the agent's job
is patterns — ramps, veers, gust structure, the difference between
forecast and felt. No LLM needed for most of it (statistics), small
model for the narrative. This is exactly the sounder program's shape:
timestamped, located pulses → records → briefings.

### WATCHER — the engine-room IR eye
An IR camera is a slow, rich sensor. WATCHER builds **baselines**:
normal operating temperature per zone per engine state. Then it watches
for the three deviations: hot spots forming (investigate before
failure), cold lines on a wall (a leak's signature), bilge level trends.
Zone definitions live in its git repo as versioned config — the captain
can draw new zones, and the change is a commit.

## The trigger mesh (correlation as a first-class citizen)

Gaze is *attention*; triggers are *correlation*. Any git-agent (or the
human, or H1) may subscribe an agent to events elsewhere in the system
matching time, location, entity, or pattern:

```yaml
# in SCOUT's repo: triggers.yaml — versioned, reviewable
- when: pulse.wind.speed_kn > 15 and vessel.near(island="Bold")
  then: analyze lee_profile(island="Bold", depth_h=2h)
- when: watcher.zone("alternator").temp_trend > +2C/hour
  then: escalate(priority=1, context=baseline_diff)
- when: sounder.record contains "bait ball"
  then: pulse.annotate(reason="feeding window?")
```

The worked example the pattern exists for: **the chart becomes a
git-agent too.** CHART holds land altitudes and coastline shape as
versioned data. SCOUT subscribes: *wind from bearing B + vessel near
island X → consult CHART's terrain model → estimate the lee.* When the
captain asks *"what's the wind doing behind that island?"* the answer
comes from three narrow agents correlating — wind felt (PULSE), terrain
known (CHART), synthesis (SCOUT) — and the reasoning chain is in all
three repos. This is G-2's frame-to-event generalization applied to
*everything with a pulse*: any realm can correlate with any other
through the same two channels.

## The camera co-design loop (both labor inversions)

Hardware assembly is where the labor inversion flows *both* ways, and
naming that honestly is the design:

**Human gives specs, agent labors the details.** "Point them port and
starboard, at least 5 feet up, 3 feet wider apart" — the agent works
out mounting, coverage math, wiring, zones, config. This is the docs/07
playbook pattern in physical space.

**Agent gives specs, human labors the physical.** WATCHER scores camera
placements continuously: coverage, occlusion, sightline overlap (depth
perception needs parallax), thermal contrast per zone. With a limited
camera budget it *proposes* the placement that maximizes information
per camera: *"move the engine cam 40 cm left — the alternator's edge is
out of frame, and the bilge line is in shadow."* The human wrenches.
The agent re-scores. Iterate.

**Each must learn the other's mind.** The human learns how the agent
specifies (precise, evidenced, checkable); the agent learns how the
human works (fast, approximate, gloves on). The co-design loop is the
relationship from docs/26 applied to hardware: two sides of the shell,
one boat.

## Capacity policy (honest scheduling)

Every git-agent declares its cadence and its deadline; the harness
schedules by *value ÷ compute*, dropping to cheaper models or longer
intervals when the laptop is busy. Priorities: safety thresholds (never
model-gated) > escalations > records > notes > background analysis
(most compressible). An hour late on a weather synthesis is nothing; a
second late on a bilge alarm is failure — the policy must know the
difference structurally, not hopefully.

## The quality bar (every git-agent, before it joins the roster)

1. Speaks the five channels, nothing else.
2. Its repo contains: config, baselines, calibration, and a README a
   new agent could rebuild it from.
3. Tests: replay of recorded pulses → expected records (the cascade
   suite's shape, docs/15).
4. A recorded "first day" in the review log — including its mistakes.
5. GC contract honored: nothing deleted unread, nothing retained
   without a reason a captain could verify.

## Open questions (to docs/13)

- **Q-AGENT-1:** trigger mesh delivery — bus subscriptions in
  `core/src/bus` (kernel-mediated) vs. a simple file-watcher mesh
  (repo-mediated)? Kernel-mediated is the architecture; repo-mediated
  is the pragmatics. Decide when the second realm ships.
- **Q-AGENT-2:** baseline drift for WATCHER — how does "normal
  operating temperature" age across seasons, and who approves a
  baseline reset (the stage-gate pattern applied to baselines)?
- **Q-AGENT-3:** the lee model's truth standard — terrain-derived wind
  estimates are inference, not measurement. How are they labeled so the
  captain (and the envelope) never confuse them with felt wind?
  (Candidate: provenance.class = "modeled" vs "measured" on every record.)

---

**Next:** audits (`docs/research/VAAS_SPECTRO_AUDIT.md`) and
writings-informed R&D (`docs/research/WRITINGS_INFORMED_RND.md`).
