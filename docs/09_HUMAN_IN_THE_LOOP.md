# 09 — Human in the Loop: The Supervision Contract

> **Target Audience:** Everyone. This defines the human's four jobs and
> the escalation contract that protects their attention.
> **Purpose:** Precise semantics for the autonomy dial, escalation,
> veto, and teaching.
> **Status:** Governing document

---

## Reframe: the human is an authority source, not a controller

Agents operate the vessel all day. The human contributes exactly four
things machines cannot manufacture: **authority** (the dial), **judgment**
(escalation answers), **demonstration** (teaching by doing), and
**veto** (physical override). The system's job is to make those four
inputs cheap, unambiguous, and maximally informative — and to never waste
them.

## The autonomy dial

A single physical-and-on-screen control, position always visible, always
honest. Recorded in the black box with every actuation.

| Dial | Name | Agents may | Playbook intents become |
|------|------|-----------|------------------------|
| **0** | LOG | Observe, record, learn. No control output at all. | dropped (logged as shadow deltas) |
| **1** | COACH | Suggest: intents displayed to human in real time with rationale. | advisory cards on the dash |
| **2** | SUPERVISE | Act within envelope, but each *class* of action needs standing human consent; novel situations escalate. | executed after envelope + consent check |
| **3** | AUTOPILOT | Act within envelope autonomously; escalate only on anomaly/uncertainty. | executed after envelope |

Rules:

- The dial is a **ceiling, not a floor** — the envelope can always further
  restrict (it drops effective autonomy on sensor degradation, watchdog
  events, or playbook demotion).
- Dial changes are `human.dial.set` events (critical lane). The black box
  attributes every actuation to a dial position. Liability clarity is a
  feature, not paperwork.
- Default after any restart: **dial 1**. Autonomy re-earned each power-up,
  never silently resumed.

## Escalation contract

Escalation is the system's most expensive operation — it spends human
attention. It is therefore a **structured transaction**, not an alert:

```json
{
  "kind": "agent.escalation.request",
  "payload": {
    "question": "Swell period shifted to 6s; current gains oscillate at 0.4°. Switch to rough-sea gain set?",
    "options": [
      {"id": "switch", "label": "Switch to rough-sea gains", "consequence": "smoother steering; slightly slower response"},
      {"id": "hold", "label": "Hold current gains", "consequence": "minor oscillation continues"},
      {"id": "manual", "label": "I'll take the helm", "consequence": "dial → 1, shadow resumes"}
    ],
    "recommendation": "switch",
    "confidence": 0.82,
    "context_bundle_ref": "mem:escalations/e_8841",
    "expires_s": 120,
    "fallback_on_timeout": "hold"
  }
}
```

Contract terms:

1. **Every escalation ships its own context bundle** (state snapshot,
   recent black-box slice, evidence) so the human decides in seconds, and
   so the answer is auditable.
2. **Every escalation has a declared timeout fallback.** The boat never
   waits for the human. Silence selects the safe default.
3. **Rate-limiting is enforced by the kernel.** Non-critical escalations
   batch into a digest at human timescales (max 1 interrupt / 10 min unless
   critical). Agents compete for attention budget; the Auditor tracks
   false-alarm rate per agent and reports it.
4. **Answers are training data.** `human.escalation.answer` events feed
   the Analyst: repeated "yes" to a class → Engineer proposes promoting it
   to standing consent; repeated "no" → the proposing agent's confidence
   calibration is adjusted down.

## Veto: the physical layer

- Jog lever, throttle lever, wheel: `human.jog_lever.move` on the critical
  lane. Absolute, unverifiable-by-software, preempts everything in the same
  tick. No agent, no envelope setting, no bug can outrank muscle.
- Veto triggers automatic mode response: current playbook demoted to
  shadow, kernel asks (batched, non-blocking) *why* via voice. The answer —
  even grunted — is the most valuable training signal the system gets.
- Watchdog: kernel heartbeat missed → relays open, safe state, buzzer,
  `control.watchdog.trip`. Hardware path, not software-mediated.

## Teaching: demonstration as first-class input

Humans teach by doing + narrating. The system treats this as the primary
learning channel (superior to preference clicks):

- Captain drives normally; Analyst aligns voice transcripts with telemetry
  (alignment-confidence gated, see `docs/08`).
- One-tap catch logging doubles as outcome labeling: the pattern that
  preceded a catch is stronger evidence than one that didn't.
- "Coach-back": at dial 1, the Operator narrates what it *would* do in the
  captain's vocabulary. Agreement/disagreement is logged per situation
  class and becomes the trust metric shown to the human: **"I matched your
  calls 94% this trip."** That number — not marketing — is what moves the
  dial.

## The attention budget (design targets)

| Metric | Target |
|--------|--------|
| Critical interrupts per trip | ≤ 2 |
| Non-critical interrupts per hour | ≤ 1, batched |
| Time to answer an escalation | ≤ 15 s to comprehend |
| Escalations with full context bundle | 100% |
| Human actions required for routine ops | 0 (dial 2+) |

An agent that exceeds budget is a defective agent. The Auditor's weekly
report ranks agents by interrupts-caused-per-useful-outcome; the Engineer
uses it to fix calibration.

---

**Next:** `10_AGENT_OPERATIONS.md` — the crew, their daily loop, boatctl.
