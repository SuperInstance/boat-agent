# 25 — Research Landscape: Science for the Next Phases (2021–2026)

> **Target Audience:** All agents planning work beyond Phase 2.
> **Purpose:** Consolidate four parallel deep-research streams into one
> cited, adjudicated map: what the literature supports, what it refutes,
> and what it says to build next. Every paper verified via web search.
> **Status:** Governing for Phase 3+ planning · Date: 2026-07-19
> **Sources:** 4 explore agents, ~60 verified papers. Full citations inline.

---

## The three cross-cutting findings (read these first)

**1. The oracle is the gate; intelligence is the drafting aid.**
Three independent streams converge: LLM self-review is worthless without
an external test oracle ([Demystifying GPT Self-Repair, NeurIPS 2023](https://arxiv.org/abs/2306.09896)); explanations raise trust whether
the AI is right or wrong ([Bansal CHI 2021](https://arxiv.org/html/2409.10250v1));
AI-assisted users write less secure code while believing the opposite
([CCS 2023](https://arxiv.org/abs/2211.03622)). Our stage-gate's
structure — replay oracle + human gate + shadow period — is exactly the
architecture the evidence supports. **Never let any LLM self-critique
become a gate stage.**

**2. Small-data is a solved problem — if you use the right recipe.**
Every stream's small-data finding points the same way: pretrain on your
own unlabeled data, fine-tune with tens of labels, never train from
scratch. U-Net segmentation on **18 days of expert-annotated echograms
hit 90% recall** ([oceanstream 2026](https://oceanstream.io/training-a-deep-learning-model-for-echogram-semantic-segmentation/));
self-supervised pretraining on unlabeled acoustics is the exact recipe
for our regime ([Pala et al., Ecological Informatics 2024](https://pmc.ncbi.nlm.nih.gov/articles/PMC12115645/));
execution-verified skill libraries beat fine-tuning for offline
personalization ([Voyager, TMLR 2024](https://arxiv.org/abs/2305.16291)).

**3. The simplex/envelope pattern is now formally grounded.**
Black-Box Simplex ([FM 2022](https://arxiv.org/abs/2102.12981),
extended [2024](https://link.springer.com/article/10.1007/s11334-024-00553-6))
and CBF-at-guidance-level ([Marley CAMS 2021](https://ntnuopen.ntnu.no/ntnu-xmlui/bitstream/handle/11250/2982156/Marley_CAMS_2021.pdf))
give our envelope↔playbook split its academic formalization: the safety
layer need not model the advanced controller's internals. We built the
right thing before the literature caught up; now we have the citation
trail for certification conversations.

---

## Stream 1 — Fisheries acoustics ML

**Adopt:**

- **U-Net segmentation with pretrained encoder + focal loss** replaces
  our OpenCV blob heuristic. Brautaset 2020 ([ICES JMS](https://doi.org/10.1093/icesjms/fsz235))
  proves the architecture on echograms (~94%); [Pala 2023](https://pmc.ncbi.nlm.nih.gov/articles/PMC12115645/)
  proves focal loss handles the <1%-target-pixel regime. Bootstrap with
  heuristic-generated weak masks + handful of hand corrections.
  ~8M params, runs on laptop CPU, deterministic — fits the determinism
  directive better than a VLM in the loop.
- **SSL pretraining on our own archive** ([Pala 2024](https://pmc.ncbi.nlm.nih.gov/articles/PMC12115645/)):
  masked-reconstruction on unlabeled frames from OUR instrument, which
  also solves the domain-shift that kills survey-data models fed
  consumer fish-finder screen captures.
- **Physics priors in the species vocabulary:** swimbladder resonance
  bands and TS(f) response ([Pedersen 2023, MEE](https://besjournals.onlinelibrary.wiley.com/doi/full/10.1111/2041-210X.14261))
  as Bayesian priors instead of pixel-only species guesses.
- **Sim-to-real augmentation:** [Allken 2019/2021](https://doi.org/10.1093/icesjms/fsy147)
  proves synthetic-echogram training transfers; vary shape/density/depth
  on our few real school examples.

**Refuted:**
- Training deep models from scratch on a handful of catch reports
  ([Yassir 2023 systematic review](https://www.sciencedirect.com/science/article/pii/S0165783623001832)).
- GAN/diffusion synthetic data as real-data substitute at our scale
  ([Steiniger 2025](https://elib.dlr.de/216919/1/main.pdf)).

**Assets found:** [Echopype](https://academic.oup.com/icesjms/advance-article/doi/10.1093/icesjms/fsae133/7815946)
+ NOAA NCEI Alaska survey archive (Zarr, same waters/species);
[LSSS open-sourced 2025](https://www.norceresearch.no/en/news/avansert-analyseverktoy-for-fiskeriakustikk-blir-na-tilgjengelig-som-apen-kildekode)
(commercial-grade heuristics reference); VIAME's annotate-a-little
workflow.

## Stream 2 — Marine autonomy & control

**Adopt (highest payoff per line of code):**

- **Wave filtering on heading/yaw-rate** so first-order wave motion
  never drives rudder/throttle ([Fossen & Perez, IEEE CSM 2009](https://www.semanticscholar.org/paper/Kalman-filtering-for-positioning-and-heading-of-and-Fossen-P%C3%A9rez/68fb43070da9a4328ff7012bd9f6859040aad4ca);
  [Selimovic 2020 review](https://www.mdpi.com/2077-1312/8/4/234)).
  Deterministic, boring, calms compass-swing and actuator wear.
- **Vessel-as-wave-buoy sea-state estimation** from a cheap motion
  sensor ([Nielsen 2020](https://orbit.dtu.dk/files/235977117/OE_2020_Vol216_pp107781_PostPrint.pdf);
  [Majidian 2022 review]) — measured sea state feeds gain scheduling AND
  becomes replay metadata. Sea trials prove adaptive steering pays:
  ~3% fuel, better heading variance (Steermaster, via [Åström & Murray](https://www.cds.caltech.edu/~murray/books/AM08/pdf/fbs-author_29Mar19.pdf)).
- **Counterfactual replay via disturbance observer** ([Menges & Rasheed,
  Ocean Eng 2023](https://doi.org/10.1016/j.oceaneng.2023.115412)):
  estimate lumped wind/wave/current forcing from recorded trips so
  replays evaluate playbooks that deviate from the recorded track —
  the upgrade from reproduction to counterfactual testing. Re-sync the
  twin as hull fouls ([Berg 2025](https://doi.org/10.1038/s41598-025-93635-9)).

**Refuted:**
- End-to-end deep RL in the actuation path — simulation-only, reward-
  sensitive, self-admitted assurance gap even in the best sea-trial
  paper ([Martinsen 2020](https://www.frontiersin.org/journals/robotics-and-ai/articles/10.3389/frobt.2020.00032)).
- Full COLREGs stacks for small vessels — untestable as binary property
  ([MIT 2024](https://dspace.mit.edu/bitstream/handle/1721.1/155858/molina-mnmolina-eng-ms-me-eecs-thesis.pdf));
  a troller engaged in fishing is the privileged vessel under Rule 18;
  our real needs are track-keeping, CPA alarms, fast escalation —
  already covered.

## Stream 3 — Edge AI & agent architectures

**Adopt:**

- **Role-split by evidence:** small VLM (Qwen2.5-VL-3B / SmolVLM) for
  frame understanding, Chronos-class model for numeric forecasting
  ([Chronos 2024](https://arxiv.org/abs/2403.07815)), LLM only for
  interpretation — because removing the LLM *beats* LLM-based forecasters
  on raw series ([Tan et al., NeurIPS 2024 Spotlight](https://arxiv.org/abs/2406.16964)).
- **Execution-verified skill libraries + reasoning memory** for vessel
  personalization instead of weight updates ([Voyager](https://arxiv.org/abs/2305.16291);
  [ReasoningBank 2025](https://arxiv.org/abs/2509.25140) — failures carry
  much of the signal; [Dynamic Cheatsheet, ICML 2025](https://arxiv.org/abs/2504.07952)).
  Log failed playbook attempts as first-class memory.
- **Memory architecture map for the cascade:** MemGPT's tiered memory is
  the ancestor of our M1/M10/H1 ([MemGPT](https://arxiv.org/abs/2310.08560));
  A-MEM's linked-note memory fits vocabulary learning ([A-MEM, NeurIPS 2025](https://arxiv.org/abs/2502.12110));
  Zep's temporal KG with fact invalidation fits a twin whose facts change
  ([Zep 2025](https://arxiv.org/abs/2501.13956)).
- **Compliance framing:** EU AI Act Article 12 requires automatic
  lifetime logging for post-incident reconstruction — our black box is
  Article-12-by-design. State it in docs.

**Refuted:**
- "GPT-4V-level" tiny VLM claims — benchmark-selective; moondream has
  **no paper at all**. Sub-3B VLMs are fine for narrow fine-tuned tasks,
  mediocre general reasoners ([SmolVLM](https://arxiv.org/abs/2504.05299)).
- LLM self-correction as any part of a safety mechanism.

## Stream 4 — Learning from demonstration + sensor fusion

**Adopt:**

- **Two-stage alignment, timestamped at the sensor.** Whisper's native
  timestamps fail the 500ms poisoning budget ([HAL 2024](https://hal.science/hal-04404777v1/document)
  calls them "very inaccurate"); forced alignment passes it
  ([WhisperX, ICASSP 2023](https://arxiv.org/abs/2303.00747));
  attention-head filtering hits **20–50ms tolerance** ([Yeh 2025](https://arxiv.org/pdf/2509.09987)).
  Gate training windows on per-word confidence ([whisper-timestamped](https://github.com/linto-ai/whisper-timestamped)).
- **Hindsight pairing over dense alignment:** narrated-play learning
  works with <1% labeled frames ([Lynch & Sermanet, RSS 2021](https://arxiv.org/abs/2005.07648);
  ablation: [Mees 2022](https://arxiv.org/abs/2204.06252)). The telemetry
  trajectory is ground truth; language is auxiliary labels. Data quality
  bounds you, not quantity ([Belkhale, NeurIPS 2023](https://arxiv.org/abs/2306.02437)).
- **Shadow mode as a ThriftyDAgger loop** ([CoRL 2021](https://arxiv.org/abs/2106.00063)):
  risk/novelty-gated escalation decides when to page the captain; every
  takeover is a labeled corrective demonstration feeding the next
  playbook revision. Shadow testing becomes the data engine.
- **Robust factor-graph fusion** for the reducer: FGO > EKF for GNSS/INS
  ([Wen 2021, NAVIGATION](https://arxiv.org/pdf/2304.10142v2));
  shipborne robust FGO on real sea data ([Hu 2024](https://doi.org/10.1049/rsn2.12521));
  graduated non-convexity rejects multipath ([Wen 2022](https://arxiv.org/pdf/2109.00667v1)).
  Middle step: SVD-UKF + covariance-growing dead reckoning with
  autonomy dial-down keyed to covariance (also the spoofing defense —
  [Bhatti & Humphreys 2017](https://mirror.explodie.org/Bhatti_et_al-2017-Navigation.pdf)).
- **LLM-proposes-structure, search-fits-parameters** for rule discovery
  ([LLM-SR 2024](https://arxiv.org/abs/2404.18400); [Landajuela ICML 2021](https://proceedings.mlr.press/v139/landajuela21a.html))
  — exactly our architecture, now with precedent.

**Refuted:**
- Pure symbolic regression (PySR/gplearn) fitted to raw noisy telemetry —
  SRBench++ shows sharp degradation under noise ([SRBench](https://arxiv.org/html/2508.21484v1)).
  Interpretability comes from the LLM's code skeleton, not from hoping GP
  finds clean expressions.

---

## Action map (into the plan and bounties)

| Action | Where | Evidence |
|--------|-------|----------|
| U-Net segmentation replaces OpenCV blobs | B-INT-5 + cascade M1 | Brautaset, Pala, oceanstream |
| SSL pretrain on our archive | new bounty B-ML-1 | Pala 2024 |
| Wave filter + wave-buoy sea state | envelope-adjacent driver + replay metadata | Fossen, Nielsen, Majidian |
| Counterfactual replay via disturbance observer | B-CORE-1 upgrade | Menges & Rasheed |
| Shadow→ThriftyDAgger escalation loop | docs/07 amendment | Hoque, Diff-DAgger |
| Two-stage forced alignment for voice learning | Analyst pipeline | WhisperX, Yeh, HAL |
| Chronos-class forecaster for numeric prediction | H1 loop | Chronos, Tan |
| ReasoningBank-style failure memory | docs/08 amendment | ReasoningBank, Voyager |
| Factor-graph reducer (or SVD-UKF middle step) | docs/13 new Q-FUSE-1 | Wen, Hu |
| EU AI Act Article 12 framing | docs/10 + README | statute |

## New open questions (docs/13)

- **Q-FUSE-1:** full factor-graph reducer vs SVD-UKF middle step —
  decide after wave-filter + outage-mode data accumulates.
- **Q-ML-1:** does SSL pretraining on TZ Pro screen captures (consumer
  palette, not raw backscatter) transfer as well as on raw EK80 data?
  Our images are a different domain than all published acoustics ML.
- **Q-SAFE-3:** formalize the envelope as a black-box simplex switch —
  reachability-checked handoff conditions, logged, for certification.

---

**Method note:** four parallel explore agents, ~60 papers, each verified
to exist via web search. Preprints flagged where load-bearing is
uncertain. The oceanstream 18-days-to-90% figure is a vendor demo, not
peer review — treat as encouraging, not proven.
