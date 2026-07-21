# 28 — Shell Ecology: Molting, Distillation, and the Shape of the Vessel

> **Target Audience:** Everyone building git-agents (docs/27) and
> planning the hardware/instance lifecycle of the fleet on the boat.
> **Purpose:** How agents inherit shells, outgrow them, and molt —
> with distillation as the mechanism that lets a small mind wear a big
> one's armor until it grows its own.
> **Status:** Governing design · Date: 2026-07-20
> **Lineage:** hermit-crab-ecology (the 67K-word spec in poetry),
> docs/27 (git-agents), the claw/tminus scout (2026-07-20)

---

## The claim in one breath

An agent is a hermit crab. Its shell is the *shape of its vessel*:
hardware, sandbox, permissions, autonomy level, and innate role — all
one thing, not five. Shells are not permanent. The crab that outgrows
its shell must molt; the vacated shell does not stay empty long.

## The shell taxonomy (hardware is destiny, honestly priced)

| Shell | Vessel | Fits | Won't do well | Autonomy tier |
|-------|--------|------|---------------|---------------|
| **nerite** | ESP32-class microcontrollers | threshold watchers, bilge, heartbeat | inference of any kind | 0 — reflex only |
| **conch** | Raspberry Pi | single-camera vision, pulse agents, small models | multi-stream perception, fast VLMs | 1 — narrow judgment, supervised |
| **green** | Jetson-class edge | real-time vision, distilled models, the watcher roster | large VLMs, training | 2 — independent judgment, gated |
| **murex** | Wheelhouse laptop/PC | the cascade, the twin, the scrubber, distillation runs | — (it IS the big water) | 3 — the workshop |
| **whelk** | Cloud edge (Workers) | fleet sync, heavy batch, cross-vessel learning | anything latency-bound or offline | 2+ — powerful but absent-minded |
| **magpie** | borrowed/foreign hardware | opportunistic compute on anything reachable | nothing — trust is scoped per hop | 0–1, tightly fenced |

The rule the taxonomy enforces: **a role is shaped by its shell, never
assigned in spite of it.** A conch-shell agent doing magpie work is a
design smell, not a clever hack.

## Turbo shells: the agent that eases its own job

Some loops contain a piece that cannot be made an algorithm — a
judgment call in a narrow crevice. A **turbo shell** is the agent
placed into exactly that crevice, with one standing order beyond the
job itself: **make your own job easier.**

Concretely: the agent watches which of its judgments are actually
repeated patterns. It crystallizes them — a lookup, a threshold, a
tiny rule — so that next time a *simpler* model (or no model) suffices.
The big model is called less; the cheap path handles more; the agent's
own outputs become the curriculum for its replacement. γ grows, η
shrinks, C holds — the conservation law operating on a single role
over time.

The honesty rule: a turbo-shell agent *reports its own
crystallization rate* ("this week, 61% of my calls needed no model —
up from 44%"). Self-easing is a measured behavior, not a hope.

## Distillation: power armor first, molting later

The maturation path for a narrow role, in order:

1. **The large model does the job first.** It owns the loop: the
   alignment, the harness, the data access, the prompt shapes, the
   failure handling — proven working end-to-end by a model that can
   handle the crevice's full width.
2. **The outputs become a curriculum.** Every judgment the large model
   makes inside that harness is logged as input→output pairs with
   context. This is training data *of the exact job*, not of the
   general domain.
3. **A LoRA is distilled — power armor.** A small model dons the
   adapter and takes over the narrow role inside the *same* harness,
   with the large model on call for out-of-distribution cases
   (shadow→active, the docs/07 gate pattern applied to models).
4. **The molt.** When the distilled model's measured competence
   crosses the bar, it sheds the armor dependency: it becomes its own
   model for that role — retrained periodically as new data arrives,
   versioned like a playbook, rollback-able like a playbook.

A distilled model inherits provenance: which large model taught it,
from which data window, with what measured competence at graduation.
Molting is a **stage-gated promotion**, never a silent swap — the same
evidence rules as code (replay, shadow, approval) because to the
envelope there is no difference.

## The vacancy chain (mission creep is a molting signal)

The worked example, exactly as it will happen:

A conch shell (Raspberry Pi) runs a basic vision model on a camera.
It does *such* a good job that the captain and cocapn want more from
that camera — more streams, more classes, faster cadence. The conch is
outgrown: that's not a failure, it's the ecology working.

1. **Molt:** the vision hermit moves to a green shell (Jetson) — same
   role, same git repo of state, bigger vessel. The move is a config
   delta and a documented promotion, not a rewrite.
2. **Vacancy:** the freed conch shell gets a *fresh* hermit — a new
   narrow role (pulse watcher, second camera, threshold sensor). Cheap
   shells should never sit empty; a fresh crab is nearly free.
3. **Migration:** the conch's old role grows elsewhere — its baseline
   and calibration (already git-versioned) inform the new tenant's
   start. Nothing learned is lost in the move.

The signal-to-watch: **sustained success is the molting trigger.** When
an agent's role keeps expanding (mission creep by demand), the harness
should *propose* the molt — "this role wants a bigger shell" — as a
structured escalation, not a silent strain.

## Temporal/spatial awareness: the extraction plan

Scout findings (2026-07-20) on what the sibling repos give this
ecology — adopted with their wake conditions checked:

| Mechanism | From | Cost | Role in the ecology |
|-----------|------|------|---------------------|
| **Heartbeat roster + staleness reaping** | swarm-anchor | hours (vendor) | every shell writes its heartbeat; the directory is the roster. The spatial ground truth: who's alive, where, seeing what |
| **Quorum countdowns + deadline cascade** | t-minus / t-minus-rs, **shipped as `swarm-tminus` on PyPI (~300 tests)** | zero — the wake condition MET | the temporal spine: predict-and-confirm between shells; missed quorum degrades toward the human by construction |
| **Ternary classification (Avoid/Unknown/Choose) + adaptive thresholds** | claw + edge-weight (sketch-tier) | ~½ day | the perception filter inside each watcher; Unknown escalates, never guesses |
| **`.swarm/` shared-state directory convention** | tminus-os (private, wiring-only) | 1 hour | one inspectable root for all shell state — `cat`-able, crash-proof, offline-proof |

Adoption notes: heartbeats and profiles are emitted as **bus events
validated against `schemas/`** (we do not adopt swarm-anchor's
no-schema stance). Anything touching actuation goes through the
stage-gate, never through a baton or prediction file's say-so.

## The honesty ledger

- `tminus-os` is private and wiring-only — take the directory
  convention, nothing else.
- `edge-weight` is v0.1.0 sketch, untested — port the profile math
  (~80 lines), not the Worker.
- `tminus-ecosystem-review`'s own status section was stale within a
  day of writing (it said swarm-tminus wasn't pushed; PyPI says
  otherwise). Trust registries over READMEs.
- The shell taxonomy is a policy, not a guarantee: hardware capability
  claims (especially VLM-on-conch) must be measured per deployment and
  recorded in the shell's repo.

## Open questions (to docs/13)

- **Q-SHELL-1:** the molt promotion bar for distilled models — what
  measured competence, on what replay corpus, graduates a LoRA to a
  standalone model? (Candidate: the playbook gate's own criteria,
  re-pointed at weights.)
- **Q-SHELL-2:** vacancy policy — should shell assignment be
  human-approved (profile change, dial-adjacent) or harness-proposed
  with captain consent via one escalation card?
- **Q-SHELL-3:** magpie shells — under what conditions, if ever, may a
  role run on hardware we don't own? (Default answer: never for
  anything touching actuation or private data.)

---

**Cross-references:** docs/27 (the roster), docs/07 (the gate),
docs/12 D6 (why promotion is staged), hermit-crab-ecology (the source
metaphor made spec), docs/13 (Q-SHELL-1..3).
