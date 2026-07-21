# 13 — Open Questions

> **Target Audience:** Agents and engineers looking for high-leverage work.
> **Purpose:** Architecture-level questions we deliberately left open.
> Each entry has context, why it matters, candidate directions, and what a
> good answer looks like. Claim one by opening an issue referencing the ID.
> **Relationship to legacy docs:** `02_RESEARCH_QUESTIONS.md` tracks
> *domain* research (PID, sonar, alignment). This file tracks *system*
> questions. Both are active.
> **Status:** Living document

---

## Conventions

- IDs are stable: `Q-<AREA>-<n>`. Never renumber; mark resolved ones
  `RESOLVED → link` and leave them in place.
- A "good answer" usually means: evidence (replay data, benchmarks, field
  logs) + a decision recorded in `12_DESIGN_RATIONALE.md` + docs/code
  updated in the same change.
- If you pick one up, respect the prime directives — several questions
  below are traps that look like invitations to bypass the envelope. They
  are not.

---

## Q-ARCH-1: Process-isolated plugin runtime for third-party modules

**Status:** Open · **Area:** modularity · **Touches:** D10

Third-party authors will want to ship drivers and Analyst jobs without
compiling into the core tree. Today modules are traits compiled in. A
supervised plugin *process* model (like the playbook host, generalized)
would give isolation + language freedom.

**Why it matters:** "modular pieces, added or removed for different
applications" is the product thesis; compile-in is a friction ceiling.

**Directions:** (a) generalize `PlaybookHost` into a `ModuleHost`
(supervised child + NDJSON protocol + capability manifest); (b) WASM
component model as the plugin format; (c) signed module registry with the
same stage-gate discipline as playbooks.

**Good answer:** a plugin runs unprivileged, speaks bus protocol, crashes
without touching the tick loop, and its install/uninstall is two
`boatctl` commands.

---

## Q-ARCH-2: Event schema evolution

**Status:** Open · **Area:** bus · **Touches:** docs/06, D2

Kinds will evolve: fields added, payloads restructured. Playbooks and
fleet imports will lag. We have no versioning story beyond "change schema
first."

**Why it matters:** a fleet of vessels on mixed versions is coming;
schema drift is how replay corpora rot.

**Directions:** (a) semver per kind (`sensor.gps.fix@2`) with kernel-side
downgrade adapters; (b) additive-only rule enforced at CI; (c) schema
registry stored in Memory so replay can interpret old events with old
schemas.

**Good answer:** a 6-month-old black-box log replays correctly against
current code, and the mechanism proving it runs in CI.

---

## Q-ARCH-3: Tick rate and lane capacities under real load

**Status:** Open · **Area:** kernel · **Touches:** docs/05

10 Hz, 4096 telemetry depth, 16384 narrative depth are reasoned guesses.
Nobody has measured a fully-loaded vessel day (N2K chatter, camera
metadata, five agents, sync running).

**Why it matters:** coalescing that drops decision-relevant telemetry is
the one failure mode D2 flagged as its own overturn condition.

**Directions:** instrument tick drift + lane occupancy; replay a synthetic
worst-day; derive capacities from measurements.

**Good answer:** a `kernel.limits` table in vessel.toml justified by
measured percentiles, plus alerts at 70% occupancy.

---

## Q-ARCH-4: How far can static verification of generated code go?

**Status:** Open · **Area:** playbooks · **Touches:** D4, D6, legacy RQ-007

Stage 2 is AST scan + type check + clamp verification. Could we prove
stronger properties — output boundedness, termination, monotonicity —
for the restricted subset playbooks are allowed to use?

**Why it matters:** every property proven statically is one the envelope
doesn't have to catch at runtime, and one the human approval screen can
state as fact.

**Directions:** (a) restrict playbook Python to a verifiable subset and
  run abstract interpretation; (b) WASM migration makes fuel/termination
  structural — maybe this question dissolves into D5's migration;
(c) property-based replay (generative state sequences, not just recorded).

**Good answer:** a named set of properties with a checker that runs in
the gate in seconds, plus an honest list of what remains unprovable (and
therefore still needs replay + shadow).

---

## Q-SAFE-1: Degraded-mode policy matrix

**Status:** Open · **Area:** envelope/missions · **Touches:** docs/05, docs/09

When sensors degrade, `effective_autonomy` drops and the kernel
safe-holds. But *which* hold is safe depends on mission and sea state:
"hold throttle" in a following sea near a lee shore is not the same as on
open water.

**Why it matters:** the difference between a safe system and a
dangerously confident one is what it does when it's wrong.

**Directions:** per-mission degradation tables in vessel.toml; Analyst
mines past degradations; escalation fallbacks chosen from the table.

**Good answer:** a reviewed matrix (mission × sensor-loss class → safe
behavior) that ships as config, with replay evidence for each cell.

---

## Q-SAFE-2: Independent hardware interlock

**Status:** Open · **Area:** safety · **Touches:** D4

The envelope is software. A cheap independent microcontroller watching
the same heartbeat and holding a physical relay would survive total
laptop failure modes software can't see.

**Why it matters:** commercial deployment and insurance will ask "what if
the whole computer lies?"

**Directions:** watchdog MCU on USB/serial with its own power path;
envelope streams signed heartbeats; relay opens on miss or signature
failure.

**Good answer:** a bench prototype + failure-injection test results +
updated docs/05 diagram. This is a *supplement* to the envelope, and any
framing of it as a replacement is a misunderstanding of D4.

---

## Q-HITL-1: Attention budget calibration

**Status:** Open · **Area:** human loop · **Touches:** docs/09, D8

≤2 critical interrupts/trip, ≤1 batched/10min are hypotheses. Real
captains on real trips will tell us the truth.

**Why it matters:** set too tight, agents suppress escalations that
mattered; too loose, captains start dismissing.

**Directions:** log escalation-answer latency/quality as implicit
fatigue signal; A/B batch windows across vessels; per-captain budgets in
the profile.

**Good answer:** budget defaults with field data behind them, and the
Auditor's false-alarm metric validated against actual dismissal behavior.

---

## Q-HITL-2: Escalation fallback policy

**Status:** Open · **Area:** human loop · **Touches:** docs/09

`fallback_on_timeout` is currently authored per-escalation by the
proposing agent. Agents authoring their own defaults is a calibration
hazard: the fallback is the action that happens when the human is busiest.

**Why it matters:** the fallback executes *by definition* under maximum
human load. It deserves more scrutiny than the proposal.

**Directions:** (a) fallback restricted to a kernel-approved safe-action
set per mission; (b) Auditor reviews fallback/outcome pairs weekly;
(c) fallback selection moves to the mission layer, agents only propose.

**Good answer:** no fallback outside the approved set, plus data showing
fallback outcomes ≈ outcomes when humans answered "hold/safe" manually.

---

## Q-LEARN-1: Pattern confidence decay

**Status:** Open · **Area:** Analyst/memory · **Touches:** docs/08

A pattern learned in summer may be wrong in winter (different water,
different gear). Memory versions never die; confidence should arguably
decay without corroboration.

**Why it matters:** stale confidence is fake confidence — the sin the bus
protocol explicitly ranks worst.

**Directions:** time-decayed corroboration scores in the patterns
namespace; Analyst revalidation passes against new trips; decay curves
per pattern class.

**Good answer:** playbooks citing patterns show the *current* confidence
on the approval screen, and at least one case where decay correctly
demoted a stale rule to shadow.

---

## Q-LEARN-2: Fleet knowledge trust model

**Status:** Open · **Area:** Fleet · **Touches:** docs/08, docs/10

Fleet imports are inert candidates until locally gated. But which vessels'
candidates deserve priority? A sister troller in the same fishery vs. a
different vessel type entirely?

**Why it matters:** fleet learning is the growth engine (legacy RQ-011);
a bad trust model either imports noise or strangles the exchange.

**Directions:** vessel similarity vectors (type, fishery, size, water);
transfer-discount factor applied to imported confidence; local replay
remains mandatory regardless.

**Good answer:** a similarity metric + discount schedule with measured
shadow-graduation rates for imported vs. homegrown playbooks.

---

## Q-SYNC-1: Are provenance-precedence conflict rules enough?

**Status:** Open · **Area:** memory sync · **Touches:** D7, docs/08

Conflicts resolve local human > local agent > fleet. This is simple and
auditable but lossy — two vessels can both be right about different
conditions, and precedence picks one.

**Why it matters:** as the fleet grows, sync conflicts shift from rare to
routine.

**Directions:** (a) keep precedence, add condition-scoping (facts carry
their validity envelope: sea state, region, season); (b) CRDTs for
specific namespaces (vocab is naturally a grow-only map); (c) conflict
surfaces as escalation digest items.

**Good answer:** condition-scoped facts (a) probably wins — it's the
smallest change that stops throwing away true knowledge. Prove with a
two-vessel simulation.

---

## Q-IO-1: Generalizing the vision pipeline beyond sonar

**Status:** Open · **Area:** drivers/edge AI · **Touches:** docs/14 G-2

Screen-capture → edge vision model → structured text works for sonar.
Engine-room gauges, chartplotters, thermal cameras, even paper logbooks
photographed at the dock are the same shape: pixels → structured events.

**Why it matters:** each bespoke vision path is a fork; a general
"frame-to-event" IO driver is a product surface.

**Directions:** a driver config declaring {capture source, crop,
prompt/schema, output kind, vocab namespace}; per-source confidence
tracking; same stage-gate discipline for prompt changes as for code.

**Good answer:** adding a new visual source is config + a replay corpus
of labeled frames, zero new Rust.

---

## Q-IO-2: Generalizing the escalation contract beyond the helm

**Status:** Open · **Area:** agents · **Touches:** docs/09, docs/14 G-5

The structured escalation (options, consequences, context bundle,
timeout fallback, budget) was designed for the captain — but maintenance
crews, fleet managers, and shore-side owners face identical "agent needs
a human decision" problems.

**Why it matters:** a general human-decision broker is a component other
agent systems would adopt; it also forces our contract to be honest.

**Directions:** role-addressed escalation (who can answer what);
channels beyond the wheelhouse UI (SMS/app); delegation chains with
audit.

**Good answer:** a shore-side maintenance escalation answered from a
phone, logged with the same provenance, zero changes to the kernel.

---

## Q-PROD-1: Non-marine deployments of the same kernel

**Status:** Open · **Area:** product · **Touches:** everything

Agriculture (autonomous tractors), aquaculture (feeding barges), small
ferries, research USVs, even fixed plant (pump stations) share the
physics: hostile connectivity, safety-bounded actuation, expert operators
worth learning from.

**Why it matters:** the kernel was deliberately built policy-free (D1) —
which means it *already* isn't a boat kernel. Knowing which vertical is
the cheapest proof shapes what we generalize first (docs/14).

**Directions:** one pilot outside marine; measure what had to change
(profile? drivers? missions?) vs. what didn't (kernel, gate, memory).

**Good answer:** a pilot where the diff against this repo is <10% and
almost entirely config + drivers. That result *is* the modularity proof.

---

## Q-UX-3: vessel-quest gamification — does catch-quest XP fit the captain, or is it noise? Test with the deck-mode presets (docs/20).

## Q-UX-4: Voice-owner identification

**Status:** Open · **Area:** human loop · **From:** docs/26 field additions

When crew is aboard, the system must know whose voice carries command
authority. Options: (a) wake-word + proximity (wheelhouse mic, paired
phone) — current default; (b) speaker embeddings per registered voice;
(c) command confirmation for any non-captain voice. Good answer: commands
restricted to captain by default with a named-crew registry, and every
log line records who the system *believed* it heard.

---

## Resolved

*(none yet — this file started today)*

---

**Next:** `14_MODULE_BOUNTIES.md` — concrete modular pieces to build.
