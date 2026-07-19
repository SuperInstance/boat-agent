# 12 — Design Rationale: Why the Architecture Is What It Is

> **Target Audience:** Reviewing agents and engineers. This is the
> reasoning record — read it before proposing structural change.
> **Purpose:** For every significant architectural choice: the forces at
> play, the alternatives seriously considered, why the chosen path won,
> and what evidence would overturn the decision.
> **Status:** Living document — append new decisions; never silently edit
> old ones. If a decision here is overturned, mark it SUPERSEDED with a
> pointer to the replacement.

---

## How to read this document

Each decision follows the same shape:

- **Decision** — what we chose, one line.
- **Forces** — the constraints and tensions that made the choice hard.
- **Alternatives** — what else was genuinely on the table.
- **Reasoning** — why this one won.
- **Overturn if** — the evidence that would prove this wrong. A decision
  without a falsification condition is dogma; we don't keep those.

---

## D1. Microkernel + five primitives instead of a module collection

**Decision:** The system is a small trusted kernel (tick loop, bus, state
reducer, safety envelope) with everything else — drivers, playbooks,
agents, UI — as replaceable modules speaking one event protocol.

**Forces:** The legacy design had ten phase-modules, each with its own
state structs, Tauri commands, and direct hardware access. It worked on
paper but had three structural diseases: (1) N×M coupling — every module
that needed GPS data knew about the serial module; (2) no single truth —
four modules held four partial, differently-stale copies of vessel state;
(3) safety logic scattered across files, so "is this actuation safe?"
had no single answerable location.

**Alternatives considered:**

- *Keep the 10 phases, just clean up interfaces.* Rejected: the coupling
  was topological, not cosmetic. Polishing interfaces on a star-topology
  tangle leaves the tangle.
- *Full microservice mesh (each module a process, message broker between).*
  Rejected: operational weight on a wheelhouse laptop; latency variance in
  a safety loop; and most "services" here are 150-line drivers that don't
  deserve a process boundary.
- *Actor framework (e.g. an Erlang-style supervision tree in Rust).*
  Seriously considered. Close cousin of what we chose; the kernel's
  supervised modules are actor-ish. We declined a framework dependency in
  the trusted core — the kernel must be auditable in an afternoon, and
  frameworks smuggle policy into their runtime.

**Reasoning:** A microkernel minimizes the code that must be *correct* and
maximizes the code that must merely be *contained*. That asymmetry is the
whole safety argument. The five primitives (bus, state, envelope,
playbooks, memory) are not an arbitrary partition — each exists because
exactly one system-wide concern needs exactly one home: communication,
truth, actuation, learned behavior, and knowledge.

**Overturn if:** the kernel grows policy (it knows what trolling is) or a
primitive needs a second home (e.g., two state stores). Growth of kernel
policy is the early-warning sign that this decision is eroding.

---

## D2. Event bus as the ONLY inter-module channel

**Decision:** All inter-module communication is typed events on a
three-lane bus. No direct calls across layers, no shared mutable state
besides the reducer-owned snapshot.

**Forces:** Agents operate this system. Agents need to observe, reason
about, and replay everything — which requires all significant
communication to be *data*, not function calls. Also: real-time control
needs bounded latency, while analytics need throughput — one queue can't
serve both.

**Alternatives considered:**

- *Tauri commands everywhere (legacy).* Rejected: RPC hides causality.
  You cannot replay a function call graph; you can replay an event log.
- *Single FIFO queue.* Rejected: a burst of narrative-lane agent chatter
  would delay a jog-lever event. Safety traffic needs preemption.
- *Full pub/sub with arbitrary topics.* Rejected: unbounded topic space
  is unbounded schema drift. A fixed, schema-validated kind taxonomy is
  what keeps the bus a contract instead of a firehose.

**Reasoning:** Three lanes (critical / telemetry / narrative) encode the
system's actual priority physics: safety preempts, state coalesces,
narrative sheds load. Making lanes a *privilege* (only L0/envelope emit
critical) converts a whole class of bugs — a chatty agent starving the
control loop — from runtime failures into ingest rejections. The
provenance fields (`authority`, `confidence`, `basis`) exist because a
log that records *what happened* is data, but a log that records *why the
system believed it* is institutional memory.

**Overturn if:** profiling shows coalescing loses decision-relevant
telemetry (tune capacities first), or a legitimate cross-layer pattern
emerges that events genuinely cannot express.

---

## D3. Single authoritative state via a pure reducer

**Decision:** One `VesselState`, updated only by
`reduce(state, events) → state`, pure and deterministic. Everything —
playbooks, envelope, UI, black box — reads this same snapshot.

**Forces:** Control decisions are only as good as the world-model they
run on. Multiple partial copies guarantee disagreeing modules. But a
single state store accessed by concurrent modules reintroduces races
unless updates are serialized through one owner.

**Alternatives considered:**

- *Modules query drivers directly when they need data.* Rejected: this
  was the legacy topology; staleness and disagreement were the cost.
- *CRDT-style replicated state.* Rejected: solves a distributed problem
  we don't have on one boat; adds merge semantics to a domain where a
  total order (the kernel tick) is available and natural.
- *Database as state store (SQLite as source of truth).* Rejected: I/O
  in the control loop; and "current world state" is not a query, it's a
  value.

**Reasoning:** The reducer's purity is the load-bearing property. Because
`reduce` has no I/O, no clocks, and no randomness, *any* recorded event
stream reproduces the exact state history — which is what makes replay,
simulation, codegen testing, and incident reconstruction the same
mechanism. Sensor fusion (EKF/UKF choice, legacy RQ-004) deliberately
lives *inside* the reducer as an implementation detail: the bus contract
doesn't care which filter fuses the compass.

**Overturn if:** reducer compute per tick exceeds budget (then optimize,
don't distribute), or multi-vessel coordinated control becomes a real
requirement (that's a Fleet-layer federation question, not a reason to
fragment local truth).

---

## D4. The safety envelope as sole actuator owner

**Decision:** Every physical output flows `playbook → intent → envelope →
driver → hardware`. The envelope owns actuator file handles. It is pure
Rust, configured by `vessel.toml`, and read-only to agents by prime
directive.

**Forces:** The system's core premise is that AI authors control code —
so the architecture must remain safe even if that code is wrong,
adversarial, or insane. "Validate AI output before use" (the legacy
autopilot_guard approach) is a check; we needed a *boundary*.

**Alternatives considered:**

- *Per-module guardrails (legacy).* Rejected: scattered checks meant a
  new actuation path could bypass them without touching them. A guard
  that can be routed around is a suggestion.
- *Formal verification of generated code.* Aspirational, retained as
  open question Q-ARCH-4 (docs/13). Static analysis catches classes of
  badness; it cannot catch "reasonable-looking wrongness." Verification
  is a layer *above* the envelope, never a substitute for it.
- *Hardware-only safety (external PLC as the interlock).* Deferred, not
  rejected — for commercial deployment an independent hardware interlock
  is a genuinely good idea (Q-SAFE-2). But the first vessel is a laptop;
  the envelope is the software floor today.

**Reasoning:** Owning the *handles*, not just the policy, is what makes
bypass structurally impossible rather than socially discouraged. The
check order (veto → watchdog → dial → sensors → bounds → rates → context)
is ordered by absoluteness and cost: cheapest and most absolute first.
Config-from-profile (never hardcoded limits) exists because a 40-foot
troller and a 60-foot seiner have different physics — but the *location*
of the limits (one schema-validated table agents can't write) is
universal.

**Overturn if:** never weakened; only strengthened or supplemented.
Legitimate evolution: hardware interlock beneath it, formal proofs about
its check functions, broader context guards. Any proposal to let a higher
layer "temporarily" skip arbitration is a defect report, not a feature
request.

---

## D5. Playbooks in a supervised child process (not pyo3-in-core)

**Decision:** AI-authored Python runs in a separate, sandboxed child
process with a tick budget; timeouts and faults are first-class outputs
the kernel handles. WASM (wasmtime) is the declared migration target
behind the same host trait.

**Forces:** The legacy plan embedded Python via pyo3 in the core process
at 10 Hz. That makes generated code a *core stability dependency*: a
segfault or GIL deadlock in AI-authored code kills the safety core. Also:
the control loop must never block — a hung rule engine must degrade to
"hold last safe command," not "freeze the boat."

**Alternatives considered:**

- *pyo3 in-process (legacy).* Rejected for the stated reason: shared fate
  between untrusted code and trusted core.
- *WASM from day one.* Seriously considered and still the destination.
  Fuel metering kills infinite loops by construction; memory safety is
  structural; the sandbox is total. Deferred because: captains and the
  Engineer agent iterate in Python, the provenance-docstring pattern is
  native to it, and the team has no WASM toolchain in place yet. The
  `PlaybookHost` trait is the seam that makes this a swap, not a rewrite.
- *DSL / restricted expression language.* Rejected as primary:
  expressiveness matters (PID-ish math, conditionals, trend logic), and a
  home-grown DSL is a compiler project with worse tooling than Python.
  Retained as a possibility for tiny rule fragments.
- *Rust-codegen instead of Python.* Rejected: compile step in the trust
  path, terrible iteration latency, unreadable by captains.

**Reasoning:** The child-process design converts catastrophic failure
modes (core crash) into managed ones (Timeout/Fault → hold → restart →
demote). Non-blocking evaluation with a tick budget is the concrete
meaning of "the boat never waits on AI." The interface —
`f(snapshot, targets) → intent`, pure — is runtime-agnostic, which is
what makes Python-today/WASM-tomorrow a config change.

**Overturn if:** IPC overhead measurably threatens the tick budget at
target rates (then in-proc WASM moves up the priority list — it was
always the plan, just sooner), or Python's ecosystem becomes a
provenance/audit liability.

---

## D6. Stage-gate trust pipeline instead of "validate then run"

**Decision:** Generated control code advances through
Draft → Analyzed → Replayed → Human-approved → Shadow → Active, with
sealed evidence per stage and automatic demotion triggers.

**Forces:** "AI writes boat code" fails socially and legally if activation
is one agent's judgment call. It also fails practically if validation is
only static (reasonable-looking wrongness passes) or only live (learning
by crashing boats). And captains adopt autonomy gradually — the system
must *earn* trust with evidence, not ask for it.

**Alternatives considered:**

- *Human reviews code diff, then activate (legacy rollback.rs world).*
  Rejected: code review alone doesn't answer "what will the boat DO
  differently" — simulation does. Also, file-copy rollback is not version
  control; content-addressed bundles are.
- *Continuous online learning (rules self-update at runtime).* Rejected
  firmly: learning inside the control loop makes behavior non-replayable
  and non-attributable — it breaks A2. Learning happens through the gate,
  never in the loop.
- *Full autonomy from first install.* Rejected on the legacy evidence
  itself (Insight-003: captains want to coach first).

**Reasoning:** Each gate stage answers a distinct question no other stage
can: static analysis ("is this code structurally sane?"), replay ("does
it behave on real history, including edge cases?"), human approval ("do
you consent?"), shadow ("does it agree with you on *your* water, live?"),
active monitoring ("does it keep deserving this?"). Sealed, immutable
evidence is what makes the gate auditable rather than theatrical. Shadow
mode doubles as the trust metric — "94% agreement this trip" is a number
a captain can act on and a court can read.

**Overturn if:** shadow agreement proves non-predictive of active-mode
safety (would need incident data; revisit the graduation criteria, not
the gate's existence), or replay corpora prove unrepresentative (expand
corpus curation — the Analyst's job — rather than dropping the stage).

---

## D7. One memory layer, local-first, provenance-mandatory

**Decision:** All learned knowledge (vocab, patterns, playbooks registry,
calibration, catches, escalations, fleet imports) lives behind one
`VesselMemory` trait. Local-first; cloud sync is digest-based and
additive; every write carries provenance; nothing is ever destructively
updated.

**Forces:** The legacy design scattered knowledge across KV, Vectorize,
SQLite, and flat files — meaning agents needed to know *where* a fact
lived before they could use it, and sync was per-store bespoke. The
ocean imposes hostile bandwidth: any design requiring connectivity to be
intelligent is dead on the water. And learned behavior must be
*explainable*: "why does the boat do this?" needs a sub-second answer.

**Alternatives considered:**

- *Cloud-primary with local cache.* Rejected: inverted dependency —
  satellite loss would lobotomize the boat exactly when conditions are
  worst.
- *Event-sourced memory (memory = projections of the black box).* Elegant
  but rejected as the *only* mechanism: recomputation cost for semantic
  queries and embeddings; some knowledge (imported fleet patterns) isn't
  locally derived. Digests + namespaces are the pragmatic middle.
- *CRDT sync.* Rejected for now (Q-SYNC-1 keeps it open): merge
  semantics add complexity, and provenance-precedence resolution
  (local human > local agent > fleet) covers the actual conflict cases.

**Reasoning:** The namespace table is the system's epistemology made
concrete: it says what kinds of things may be known, who may assert them,
and who may act on them. Versioned-never-deleted storage turns "the AI
learned something wrong" from a crisis into a `history()` call and a
superseding write. The `why()` provenance walk is the audit superpower —
rule → pattern → transcript → raw events → replay report — and it only
works because provenance was mandatory from the first write, not
retrofitted.

**Overturn if:** namespace proliferation (agents inventing parallel
stores) signals the trait is too weak — extend the trait, never bless
the parallel store. CRDTs return if multi-writer sync conflicts actually
materialize in fleet operation.

---

## D8. Human-in-the-loop as an attention economy, not a UI problem

**Decision:** The human's role is four primitive inputs (dial, judgment,
demonstration, veto). Escalation is a structured, rate-limited transaction
with mandatory timeout fallbacks. Attention metrics are engineering
targets tracked by the Auditor.

**Forces:** The scarcest resource in the system is the captain's
attention — it is also the least reliable (they may be gaffing a fish).
Alert-driven systems train humans to ignore alerts. But full autonomy
without human judgment is both a trust failure and a liability
catastrophe.

**Alternatives considered:**

- *Notification center / alert stream.* Rejected: alerts without options
  and consequences make the human do the agent's structuring work;
  unbounded rate trains dismissal.
- *Confidence-threshold autonomy (act if confident, ask if not).* Tempting
  but rejected as the *sole* mechanism: agent self-confidence is
  miscalibrated by nature. The dial (human-set ceiling) × envelope
  (condition-set ceiling) × escalation (structured asks) triangulates
  instead of trusting one number.
- *Blocking confirmation for risky actions.* Rejected: a boat that waits
  for confirmation is a boat that drifts. Every escalation declares its
  `fallback_on_timeout`; silence selects the safe default.

**Reasoning:** Treating attention as a budget (≤2 critical interrupts per
trip, ≤1 batched non-critical per 10 min, agents ranked by
interrupts-per-useful-outcome) converts UX into an engineering discipline
with measurable targets. Dial-restarts-at-1 each power-up encodes
"autonomy is re-earned, never assumed" in one line of kernel code.
Veto stays physical (jog lever on the critical lane) because muscle must
never wait for software — including our own.

**Overturn if:** field data shows escalation answers are uninformative
(humans rubber-stamp) — then strengthen option design and consequence
labeling; or timeout fallbacks prove wrong often — then the fallback
policy matrix (Q-HITL-2) needs work, not the contract.

---

## D9. Everything agents do goes through `boatctl` semantics

**Decision:** One typed, self-describing command surface for all agents
and the UI. Features that can't be driven through it don't ship.

**Forces:** If the UI can do things agents can't, agents are second-class
operators — but agents *are* the operators. Conversely, if agents use
side channels, the audit story collapses.

**Alternatives considered:**

- *UI-first with agent API catching up.* Rejected: guarantees the
  two-classes-of-citizen outcome.
- *Agents emit bus events only, no command surface.* Rejected: many
  operations are request/response by nature (queries, activation,
  approvals); shoehorning them into fire-and-forget events breeds
  correlation-id spaghetti.

**Reasoning:** `capabilities` discovery makes the system self-describing
under varying conditions (dial, degraded mode, hardware present) — an
agent never has to guess what it's allowed to try. Capability-checked
writes with actor identity logged means the black box can always answer
"who did this, under what grant." The UI being *just another client* also
future-proofs: phone app, fleet dashboard, voice interface — all clients.

**Overturn if:** the command surface ossifies (new features secretly
bypassing it) — treat each instance as a P0 architecture bug.

---

## D10. Modularity via contracts, not via frameworks

**Decision:** Modules are plain code behind narrow traits (`Driver`,
`VesselMemory`, `PlaybookHost`), configured by profile, discovered by
schema. No plugin framework, no dynamic loading in the trusted core.

**Forces:** "Modular pieces added or removed for different applications"
is the product thesis — but plugin systems tend to become frameworks,
and frameworks smuggle policy into runtimes (see D1). Also: hot-plug in
a safety core is a risk surface, not a feature.

**Alternatives considered:**

- *Dynamic library loading (DLL/.so plugins).* Rejected for the core:
  ABI fragility, no sandbox, untrusted code in-process. (Drivers may
  evolve toward supervised plugin *processes* — Q-MOD-1.)
- *Everything compiled in, configured out.* Rejected as sole mechanism:
  defeats the add/remove-for-different-applications goal for third-party
  module authors.
- *Marketplace/registry of modules.* Destination, not starting point —
  the playbook exchange is its first instance (docs/10 Fleet role).

**Reasoning:** The modularity claim rests on contract discipline, not
machinery: a driver is ~150 lines behind a trait and touches nothing
else; a playbook is a folder; an agent role is a consumer of documented
events. Generalization pressure is applied deliberately (docs/14): each
purpose-built mechanism has a named general form, so today's sonar
pipeline can become tomorrow's any-camera-to-text IO without redesign.

**Overturn if:** third-party module development is actually blocked by
the lack of a process-isolated plugin runtime — Q-MOD-1 is the standing
question for exactly that evidence.

---

## Cross-cutting: why "AI as compiler, not controller" survived every review

The legacy docs' central insight was right and every decision above
deepens it. Real-time LLM inference in a control loop fails on latency
(unbounded), determinism (none), auditability (post-hoc rationalization),
and connectivity (ocean). Generated, gated, replayable code fails on none
of these. The system spends its AI budget where AI is strong —
synthesis, pattern extraction, language — and its Rust budget where
machines must be strong: bounds, timing, and never being surprised.

The corollary worth stating: **the architecture's job is to make trust
cheap.** Trust in AI-authored actuation can't be argued for; it can only
be *engineered* — via envelopes, gates, replays, shadow statistics, and
logs that answer "why" in under a second. Every mechanism in docs 04–11
is, at bottom, a trust-manufacturing machine. That framing should survive
any individual decision in this document being overturned.

---

**Next:** `13_OPEN_QUESTIONS.md` — what we deliberately haven't decided.
