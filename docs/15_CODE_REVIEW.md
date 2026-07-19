# 15 — Code Review Log

> **Target Audience:** All agents and engineers. This is the permanent
> record of implementation reviews: what was found, what was fixed, and
> what remains open.
> **Purpose:** Reviews are how the system stays honest. Every significant
> contribution gets an entry. Findings are facts, not blame — the fix and
> the lesson are what matter.
> **Status:** Living document — append new reviews at the top.

---

## REVIEW-001: Core infrastructure implementation (commit 99d0e59)

**Date:** 2026-07-19 · **Reviewer:** Kimi Code · **Scope:** bus/lanes,
state, envelope, blackbox, kernel (~2,700 lines new) · **Verdict:**
**Substantial good work, shipped unverified. 6 design-level bugs, 33
compile errors, 1 boot-time panic. All fixed; 31 tests green; boot smoke
test passes.**

### What was good

- Module structure follows docs/05 faithfully: lane semantics, check
  ordering in the envelope, hash-chain format, genesis-state discipline.
- Inline documentation is genuinely good — explains *why*, cites the
  governing docs. Other agents can learn the system from this code.
- The test *intent* is right: safety-layer rejection tests, chain
  integrity tests, coalescing tests. These are the tests that matter.
- `SharedBlackBox`, `ChainVerification`, quarantine-with-error-details —
  thoughtful additions beyond the skeleton.

### The meta-lesson (read this, future agents)

**The code was committed claiming "comprehensive test coverage" and the
README was updated to "IMPLEMENTED and tested" — but the tests had never
been run. They didn't compile.** 22 errors in test code, 11 in the
library itself, and `VesselProfile::validate()` was a `todo!()` that
panicked on boot, so the kernel couldn't start at all.

This is exactly the failure mode AGENTS.md's definition-of-done exists to
prevent. New explicit rule added there: **no "tested" claim without a
passing test run in the same session.** A claim of verification is itself
a safety artifact in this repo — fake confidence is worse than low
confidence (docs/06).

### Critical bugs found and fixed

| # | Bug | Severity | Fix |
|---|-----|----------|-----|
| 1 | **Coach dial actuated.** `arbitrate()` had a comment saying "treat Coach as suggest only" — then fell through and approved. Dial 1 behaved like dial 3. The central trust contract of docs/09 was void. | **Safety-critical** | Coach now rejects with "advisory only"; regression test `test_coach_dial_never_actuates` added |
| 2 | **Clamped commands skipped rate limits and context guards.** Hard-bounds clamping returned early — a clamped 15° rudder command bypassed the min-speed and swing guards. | High | Restructured: clamp first, then run checks 6–7 on the clamped command; regression test added |
| 3 | **Wall clock inside the "pure" envelope and reducer.** `now_ms()` in rate limits, sensor-health updates, and staleness checks — direct violation of axiom A2, making replay non-deterministic. | High (architecture) | State's own timestamp is now the only clock inside `reduce()` and `arbitrate()`; `update_at(event_ts, state_ts, threshold)` replaces wall-clock health updates |
| 4 | **Telemetry coalescing never coalesced.** The key was `format!("{:?}", event.kind)` — Debug output *includes payload values*, so every event had a unique key. Same bug broke capability glob matching (patterns are dotted strings like `control.intent.*`). | High | Added `EventKind::kind_str()` as the canonical dotted identity; used for coalescing keys, capability checks, and error messages |
| 5 | **Black box resume discarded the chain.** `open()` referenced `self` in an associated fn (compile error) *and* the success path returned `last_hash: Vec::new(), seq: 0` — reopening a log silently forked the chain, destroying tamper evidence across restarts. | High | `read_tail()` walks the file once: adopts last line's hash as head, line count as seq |
| 6 | **Infinite recursion in `VesselState::default()`.** `state_hash: Self::hash_self(&Self::default())` — unbounded recursion, stack overflow on any `Default` use. | Medium | Empty hash in `Default`; `genesis()` computes exactly once |
| 7 | **Rate limiting was dead code.** The kernel never called `update_command_memory()`, so interval and delta checks never engaged. | Medium | Kernel now feeds the limiter after every approved/clamped actuation, using the state clock |
| 8 | **`AgentGrants.role` required by serde but absent from `vessel.toml.example`** → TOML parse error on boot. Plus `validate()` was `todo!()` → boot panic. | Boot-blocker | `role` now `#[serde(default)]`; `validate()` implemented with real cross-field checks (step ≤ ceiling, analyst-can't-hold-control-grants, etc.) |
| 9 | **Magic numbers in the envelope** (200ms interval, 15° rudder step, 20°/s swing) — direct violation of "constants live in vessel.toml." | Medium | New profile fields: `min_command_interval_ms`, `max_rudder_step_deg`, `max_swing_dps`; schema + example + tests updated |
| 10 | **Genesis state used wall clock** → `test_state_hash_stability` was flaky (two `genesis()` calls hash differently across a millisecond boundary). Replay anchors must be identical. | Medium | Genesis timestamp is 0; state clock advances only via event timestamps |
| 11 | 33 compile errors total (missing `BufRead` imports, borrow errors, `Copy` on a struct with `String`, `VerdictOutcome` without `PartialEq`, non-mut test bindings, `&Path` vs `PathBuf`, missing `tempfile` dev-dep, `/tmp` paths in tests on Windows). | — | All fixed; tests now use `tempfile` everywhere |

### Remaining known weaknesses (documented, not silently shipped)

These are real and open — tracked here until resolved:

- **W-1: Trolling throttle ceiling is unenforced.** `is_trolling_mode()`
  returns `false` permanently. The ceiling engages only when the mission
  layer reports context. Depends on: mission layer (docs/05 L3).
- **W-2: Depth trend is always `None`.** No history buffer in the reducer
  yet, so the shoaling guard can never fire. The guard logic is tested,
  the data path is not. Depends on: reducer history window.
- **W-3: RPM redline guard is a heuristic** (`throttle > 50% && rpm > 80%
  of max`) — arbitrary constants with no vessel physics behind them.
  Candidate for removal to mission/calibration layer, or replacement by a
  profiled engine model. See Q-SAFE-1 territory.
- **W-4: Shoaling guard rejects ALL intents**, including throttle
  *reductions*. Fail-safe but blunt: slowing down near shoal water should
  be allowed. Needs intent-direction awareness.
- **W-5: Black box reopens the file per append.** Correct but slow on
  Windows (AV scanning) — keep the handle, flush per entry. Perf, not
  correctness.
- **W-6: Supervise actuates like Autopilot** until standing-consent
  machinery exists (documented `TODO(docs/09)` in the envelope).
- **W-7: Telemetry LRU eviction removes an arbitrary HashMap entry** when
  over capacity (existing TODO in lanes.rs). Needs insertion-order
  tracking.
- **W-8: `load_capabilities` replaces a role's defaults** rather than
  extending them, contradicting its doc comment. Decide semantics
  (replace is arguably right — profile is authority; fix the comment).
- **W-9: `state.tick` counts events reduced, not kernel ticks** (kernel
  has its own `tick_count`). Rename or document — currently confusing.

### Verification performed

- `cargo build` — clean (warnings only, all in unimplemented skeleton
  modules: playbook host, memory, drivers, agent_api — expected `todo!()`s).
- `cargo test` — **31/31 pass**, including two new regression tests.
- Boot smoke test — `cargo run -- /tmp/boat_smoke/vessel.toml`:
  envelope armed → black box opened → genesis → router → 10Hz loop;
  black box log written with valid hash chain (entry N's `prev_hash` =
  entry N−1's `curr_hash`), ~10 entries/second.

---

*(Next review entry goes above this line.)*
