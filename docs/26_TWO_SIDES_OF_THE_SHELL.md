# 26 — Two Sides of the Shell: The Operational UX

> **Target Audience:** Everyone building anything the captain or the
> agents touch while the vessel is underway.
> **Purpose:** The UX of the *relationship*, not the screens. How the
> human's loop and the agent's loop interlock through a full operating
> day — what each side sees, does, and owes the other at every phase.
> **Status:** Governing vision for human surfaces and agent behavior
> · Date: 2026-07-20 · Cross-refs: docs/09 (contract), docs/20/22/23 (surfaces)

---

## The shell

A conch shell has two sides and one wall. The human captains the
outside: the sea, the gear, the judgment, the wheel. The agent captains
the inside: the watching, the remembering, the comparing, the proposing.
Neither can do the other's job. The wall between them is thin enough to
speak through and thick enough that neither side can grab the other's
helm. That wall — the shared surface — is what this document designs.

**The human's loop** is event-driven and physical. It runs on the
water's tempo: long quiet watches punctuated by bursts of work with
busy hands and no attention to spare. Its input is the sea. Its output
is judgment, voice, and occasionally a touch.

**The agent's loop** is clock-driven and retrospective. It runs on
ticks and windows: every minute a note, every ten a record, every hour
a summary, every evening a reckoning. Its input is sensors and
transcripts. Its output is records, proposals, and questions.

The two tempos are incompatible by nature. The shell's job is to let
each side keep its own rhythm and meet only where rhythms agree. Every
design failure in this system will be, at root, one loop forcing the
other's tempo: the agent interrupting during a burst, or the human
demanding the agent reason at engine speed.

## The shared surfaces (all four, no more)

1. **Voice** — the primary surface. Hands-busy, eyes-forward, zero
   screen. Used for: catch events, observations, questions, commands.
2. **The scrubber** — the retrospective surface. The day replayed.
   Used when the day is over or the moment allows.
3. **The escalation card** — the exceptional surface. Structured,
   rare, expensive on purpose. The agent's only way to ask for judgment.
4. **The dial** — the authority surface. The only control that is
   always the human's, never negotiated.

Everything else — briefings, Telegram, debriefs page — is a view onto
these four, never a fifth channel.

---

## The operating day, both loops at once

### 05:00 — The dock (the morning ritual)

**Human loop:** coffee, keys, sixty seconds of attention while the
engine warms. He hears the morning digest — spoken, one minute:
*"Yesterday you made 3 sets, best water at the north edge. Tide's high
at 09:40, wind building NW after lunch. I checked the logs overnight —
nothing wrong. One thing worth knowing: the thermocline's been sitting
4 fathoms deeper all week than last July."*

**Agent loop:** spent the night verifying the black-box chain, checking
module health, pulling tide and weather for the day's position, and
compressing yesterday into sixty honest seconds. Its whole night's work
is judged on whether that minute was worth the captain's coffee sip.

**The shared moment:** the digest. It is a *ritual*, not a
notification — same time, same shape, every morning. The captain never
opens an app for it; it plays through the wheelhouse speaker like a
deckhand reporting for watch.

**Design rules:** one minute max, always audible, always the same
structure (what happened / what's ahead / one thing worth knowing). If
there is nothing worth knowing, the agent says *"nothing worth knowing
today"* — the sentence that builds more trust than any finding.

### 07:30 — Transit (the quiet company)

**Human loop:** driving. Relaxed but watchful. Talks naturally:
*"heading for the rock."* — half to himself.

**Agent loop:** attentive silence. Logs the intent, builds the day's
spatial spine (position, track, sea state), watches the screens.
Heard "the rock" and filed it as a candidate vocabulary term — to be
confirmed later, never assumed.

**The shared moment:** almost none, and that is the design. The worst
thing the agent can do in transit is talk. Its presence is felt through
absence of interruption. The only permitted utterances: safety, and
the rare high-value observation timed for a lull.

**Design rules:** attention budget enforced by the kernel. Transit is
where the agent earns the right to be listened to later.

### 10:00 — On the grounds (the working rhythm)

**Human loop:** reading water, setting gear, watching birds, feeling
the day. Fast, physical, conversational in bursts.

**Agent loop:** the M1/M10 loops at full tempo — minute notes, ten-minute
records. But its real work is *comparison*: today against every
remembered day. It is waiting for the moment its memory becomes
relevant: *"bait's stacking at 30 fathoms, same as Tuesday when the
bite turned."* One sentence, timed for a lull, in the captain's own
vocabulary.

**The shared moment:** the well-timed single sentence. The agent's
highest art. It interrupts only when (value × confidence) clears the
bar and the human's tempo allows. When it's wrong, the captain says so,
and that correction is worth more than the observation was.

**Design rules:** observations arrive in the captain's vocabulary or
not at all. One at a time. Never during a burst. The agent's confidence
is shown as track record, never as a score on the screen ("last three
times I called this, two held").

### 11:20 — The catch moment (voice only)

**Human loop:** full burst. Gear running, gloves on, fish coming over
the rail. Zero screen, zero patience, still talking: *"twelve chum over
the rail — two bruisers!"*

**Agent loop:** this is what it was built for. The catch event lands
with time, place, and everything that preceded it — the marks, the
tide, the bottom, the last hour's notes. It links the catch to the
pattern *as the day continues*, so the next set can be better informed.

**The shared moment:** one spoken sentence becomes a permanent,
searchable, teachable record. The human did nothing but work and talk.

**Design rules:** voice transcription must survive engine noise and
gloves-on speech; a missed transcription is logged as a miss (never
guessed). The catch log is the system's most precious input — it is
the outcome label that turns observation into knowledge.

### 13:40 — The anomaly (the expensive moment)

**Human loop:** something's off — or he's about to learn it is. He gets
the escalation card: calm, structured, one screen or one spoken
paragraph: *"Depth's shoaling fast on this track — 6 fathoms in four
minutes. Options: come port 20°, hold and watch, or you've got her.
I'd suggest coming port. Default in 60 seconds: hold and watch."*

**Agent loop:** detected the trend, already degraded its own behavior
(dropped effective autonomy, flagged the state), and spent its effort
on the *context bundle*: the last four minutes of depth, the track, the
options with consequences. It prepared for the answer *and* for
silence.

**The shared moment:** the escalation ritual — deliberately rare,
deliberately structured, deliberately priced. The captain's answer (or
his silence, which is also an answer) is recorded as training data. If
he grabs the wheel mid-sentence, the card vanishes; muscle outranks
conversation, always.

**Design rules:** every card has options, consequences, a
recommendation, a default, and a timeout. Evidence behind one tap,
recommendation behind a second (rubber-stamp defense, docs/23). After:
a calm one-line close — *"holding and watching, logged"* — so the loop
closes audibly.

### 15:30 — The long quiet (the trust exercise)

**Human loop:** the system has been right all week. He checks it less.
This is the goal *and* the risk.

**Agent loop:** working the learning loop in the background — comparing
its calls to his, mining yesterday for patterns, preparing tomorrow's
gaze. And watching for its own decay: a system that's never questioned
becomes a system that's never verified.

**The shared moment:** NOT a mid-day pop quiz — field review (crush,
2026-07-20) killed that: no fisherman in a lull wants his electronics
quizzing him, and a muted quiz erodes what the morning digest earned.
Instead the agent *volunteers its own record unprompted*: the coach-back
happens in the evening debrief, where "what I would have done vs. what
you did, here's where I was wrong" is a section the captain reads on his
own time. Mid-day, the rule stays: silence.

**Design rules:** "days since meaningful input" is a health metric —
but the response is transparency, not interruption. The agent offers its
report card; it never demands a grade.

### 19:00 — The evening debrief (where trust compounds)

**Human loop:** docked, dinner soon, two relaxed minutes. He scrubs
the day — lands on the highlight, skips through the sets, smiles at
the 12-chum moment. Then the debrief: what the day looked like, what
the agent proposes to change, what it wants to try tomorrow. One or
two approvals, given or refused in a sentence.

**Agent loop:** the evening's hard work: final read of every discarded
minute (nothing deleted unread), GC, writing the debrief with the
day's honest scorecard — including *its own misses, first*: *"I called
bait at noon that wasn't there. You were right to ignore it."* Its
proposals are packaged with replay evidence and a plain-language diff.

**The shared moment:** the daily review. This is where the dial
moves — never by the agent's request, only by accumulated evidence the
captain can feel.

**Design rules:** the debrief leads with the agent's mistakes.
Proposals without replay evidence are not presented. Refusals are
recorded with reasons when offered — they're the best training signal
in the system.

### 02:00 — Overnight (the invisible shift)

**Human loop:** asleep. Not a surface in sight.

**Agent loop:** verifies the hash chain, syncs to the cloud if there's
bandwidth, mines the week for patterns, drafts tomorrow's digest, and
watches the IR camera and the bilge. If something's wrong at 3 AM, the
text goes out. Otherwise: silence, and a verified log by morning.

**The shared moment:** none — which is the entire point. The night
shift proves itself in the morning, in sixty seconds, with coffee.

---

## The moments the first draft missed (field review additions)

Crush's blunt review (2026-07-20) found four scenes a real day has that
the draft ducked. They're in the loop now:

### The gear betrayal (any time, always at the worst time)

A bird's nest, a snapped mainline, a hydraulic stutter. The human loop
goes from rhythm to chaos in one second — this is the one moment the
agent must *raise* its tempo: record everything (it's the most valuable
failure data there is), keep the screens quiet except the one relevant
readout, and offer exactly one thing: *"logging it. Nothing from me
until you're clear."* Gear trouble is where the agent earns the right
to be heard on the good days — by shutting up on the bad ones, on
request, without being told twice.

### Other humans aboard

A deckhand, a greenhorn, a voice on the radio. The system must know the
difference between the captain's voice and everyone else's — and treat
the captain's as the only authority. Voice-owner identification is an
open question (docs/13 Q-UX-4); until it's solved, commands are
captain-only by proximity (the wheelhouse mic, the paired phone), and
the agent says *who* it heard in every log line. Crew is also the
agent's best amplifier: "show the greenhorn yesterday's debrief" is a
legitimate use of the shell by the captain, not a new feature.

### The weather turn (the real scary moment)

Shoaling is slow danger; wind-against-tide with gear in the water is
fast danger. The agent's weather context (tide + forecast) exists
exactly for this: when forecast deterioration crosses the day's
operating envelope, the escalation is *earlier than feels necessary* —
"wind's building against the ebb by 15:00, gear should be up by 14:00."
The rule: weather escalations cite the forecast, the tide, and the
gear state in one breath, because that's the computation the captain
is already doing in his head and the agent must show it can do too.

### The dock (where the catch log pays off)

Offload, grading, price, the buyer. The day doesn't end at the last
set — it ends at the sale. The catch log's payoff scene: the agent
produces the trip summary the buyer and the captain both want —
counts by species and set, times, positions, the exportable record.
*"Tell the buyer: 47 chum across three sets, here's the log."* This is
the moment the system stops being a fishing tool and becomes a
business record — and for many captains, the moment it pays for itself.

---

## What each side owes the other (the compact — amended)

**The agent owes the human:** silence by default; the captain's own
vocabulary; evidence before recommendations; mistakes first in every
debrief; an honest "I don't know"; and never, ever a fifth channel.

**The human owes the agent:** — and here the first draft overcharged,
as the field review said. The honest version: the agent's education
*costs the captain attention*, and a real compact names the tax. So:
the human owes the agent **corrections when it's wrong and the daily
two minutes** — that is the whole price, stated plainly. Judgment
calls only when they're actually needed. And the agent's side of the
bargain is to keep the tax shrinking: better calibration every week
means fewer corrections needed, which is the only growth metric the
relationship is allowed to have.

**The shell owes both:** the same story from every surface. What the
voice says, the scrubber shows, and the log records must never
disagree. One truth, four surfaces, two tempos, one boat.

---

**Cross-references:** docs/09 (the contract this dramatizes), docs/17
(the cascade tempos), docs/23 (the evidence behind every rule here),
docs/20/22 (the surfaces), CAPTAINS_GUIDE.md (the human-facing version).
