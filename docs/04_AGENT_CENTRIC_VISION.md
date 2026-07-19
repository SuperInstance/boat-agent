# 04 — Agent-Centric Vision

> **Target Audience:** All agents and engineers. Read after `AGENTS.md`.
> **Purpose:** The philosophical foundation. Why this system is built
> agent-first, and what "the human is just in the loop" actually means.
> **Status:** Governing document

---

## The inversion

Most "AI systems" are built for humans with AI features bolted on. This
system is the inverse:

```
CONVENTIONAL:  Human operates → AI assists → human decides
THIS SYSTEM:   Agents operate → human supervises → human sets the bounds
```

The vessel is worked all day by agents: one drives within its authority,
one writes and tests control code, one mines the logs, one watches the
watchers. The human captain does four things and only four things:

1. **Set the autonomy dial** (how much authority agents have right now).
2. **Answer escalations** (approve/deny when agents hit their bounds).
3. **Teach** (speak while driving; the system learns from demonstration).
4. **Veto** (the jog lever and the physical world always belong to the human).

Everything else — steering corrections, throttle management, sonar
interpretation, rule authorship, log analysis, vocabulary learning,
self-testing — is agent work. This isn't a goal to grow into; it is the
design constraint every component is built under.

## Design axioms

An agent-centric system has different physics than a human-centric one.
These axioms follow from who the operator is:

### A1. Machine legibility beats human polish

Agents don't need persuasion, dashboards, or confirmations dialogs — they
need **contracts**. Every interface in this system is typed, versioned, and
discoverable: JSON Schemas in `schemas/`, a self-describing command surface
(`boatctl`), structured errors with codes, never prose. The human UI is a
renderer of the same contracts, not the source of them.

### A2. Determinism is the substrate of trust

An agent can only be trusted to the extent its behavior is reproducible.
Therefore: the control loop is fixed-tick, the playbook code is pure
functions of state, the black box is append-only and hash-chained, and any
trip can be **replayed** byte-for-byte. "What would the system have done?"
must always have a computable answer.

### A3. Authority is a gradient, not a switch

Autonomy isn't on/off. It is a dial (0–3, see `docs/09`) combined with
per-domain grants. Agents hold *capabilities*, not blanket permission, and
every capability can be revoked by the dial, the envelope, or the human —
in that order of speed.

### A4. The envelope is the only trusted code

Agents write code. That is the point of the system. Therefore the small
Rust kernel — scheduler, bus, state reducer, safety envelope — must be
correct *without trusting anything above it*. Generated code runs in a
supervised sandbox, its outputs are bounds-checked, and the envelope owns
every actuator write. Agents can be brilliant; the kernel assumes they are
adversarial.

### A5. Uncertainty routes to the human; certainty doesn't

The system's scarcest resource is the captain's attention. Agent design
must therefore be **escalation-oriented**: act autonomously inside the
envelope of confidence, and outside it, emit a structured escalation —
context bundle, options, recommendation — rather than either guessing or
stalling. The measure of a good agent here is not how often it acts but
how rarely it interrupts unnecessarily.

### A6. Memory is shared, versioned, and local-first

Agents operating across days, trips, and vessels need continuity: what the
captain calls things, what worked, what the boat does in a following sea.
That lives in one memory layer (`docs/08`) — local-first because the ocean
has no bandwidth guarantee, synced opportunistically, and versioned so
learning is auditable and reversible.

### A7. Every agent action is narrated in the log

Not prose narration — structured provenance. Each consequential event
carries: actor (which agent/module), basis (what state/evidence), authority
(dial position, capability grant), and outcome. This makes the black box
readable by *other agents*, which is what turns a log into an institution's
memory.

## The crew model

The system is staffed by five agent roles (detailed in `docs/10`):

| Role | Lives in | Job | Never does |
|------|----------|-----|------------|
| **Operator** | kernel, real-time | Run missions: steering/throttle via active playbooks | Exceed the dial; modify its own rules |
| **Engineer** | sidecar, offline | Write playbooks, run the stage-gate, propose changes | Activate code without the gate |
| **Analyst** | sidecar/cloud, offline | Mine black box + memory; propose vocabulary, tuning, experiments | Touch actuation at all |
| **Auditor** | kernel, real-time | Verify hash chain, watch for anomalies, enforce provenance | Be disabled |
| **Fleet** | cloud, opportunistic | Aggregate playbooks/patterns across vessels | Push unreviewed changes to a boat |

The human stands above all five, holding the dial.

## What "click-and-play but very very powerful" means here

- **Click-and-play** is a *profile*: one `vessel.toml`, auto-discovered
  hardware, a wizard whose only output is that file. An agent can install
  and provision a vessel end-to-end because every step is a typed command.
- **Very powerful** is a *consequence of composition*: bus + state +
  envelope + playbooks + memory + replay. Each primitive is simple; their
  composition yields fleet learning, self-testing control code, coaching
  that becomes autonomy, and a legal-grade audit trail — from the same
  mechanisms, not from features.

## Non-goals (say these out loud)

- **No real-time LLM inference in the control loop.** Ever. AI authors
  code; code runs the boat.
- **No cloud dependency for safety.** Loss of satellite narrows the system,
  it never endangers it.
- **No hidden autonomy.** The dial is always visible, always physical-world
  honest, and the black box records every actuation's authority basis.
- **No cleverness that can't be replayed.** If it can't be reconstructed
  from the log, it didn't happen — and it doesn't ship.

---

**Next:** `05_KERNEL_ARCHITECTURE.md` — how the machine is actually shaped.
