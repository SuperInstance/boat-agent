//! The kernel: tick, schedule, supervise, record (docs/05 §The kernel).
//!
//! Deliberately boring. Contains NO policy — it doesn't know what
//! trolling is. It knows what safe is.

use crate::blackbox::BlackBox;
use crate::bus::events::*;
use crate::bus::{Event, LaneRouter};
use crate::config::VesselProfile;
use crate::envelope::SafetyEnvelope;
use crate::memory::LocalMemory;
use crate::playbook::PlaybookHost;
use crate::state::VesselState;

/// Default control rate. Playbooks evaluate per-tick, non-blocking.
const TICK_HZ: u32 = 10;

pub struct Kernel {
    profile: VesselProfile,
    router: LaneRouter,
    state: VesselState,
    envelope: SafetyEnvelope,   // L1 — owns all actuation
    playbooks: PlaybookHost,    // L2 — supervised child processes
    memory: LocalMemory,        // learned knowledge
    blackbox: BlackBox,         // append-only, hash-chained
    // supervisor: module lifecycle (drivers, sidecars) with backoff restart
}

impl Kernel {
    pub async fn bootstrap(profile: VesselProfile) -> anyhow::Result<Self> {
        // Boot order matters:
        //   1. envelope FIRST (safe state asserted before anything moves)
        //   2. black box + memory
        //   3. drivers (supervised) begin emitting
        //   4. playbook host warms the ACTIVE playbook against one replay
        //      tick before the live loop starts
        // Dial ALWAYS restarts at Coach (docs/09: autonomy re-earned each
        // power-up, never silently resumed).
        todo!()
    }

    /// The main loop. One tick:
    ///
    ///   0. drain CRITICAL lane → apply immediately (jog lever, watchdog)
    ///   1. drain telemetry (coalesced) → state.reduce()
    ///   2. mission targets (L3) — from memory/missions config
    ///   3. playbooks.evaluate(snapshot, targets) — non-blocking;
    ///      Timeout → hold last safe command, count the miss
    ///   4. envelope.arbitrate(intent, state) — the one door
    ///   5. actuator flush via envelope-owned drivers
    ///   6. blackbox.append(snapshot_hash, intent, verdict, dial, actor)
    ///   7. drain critical again before sleeping to next tick
    ///
    /// Between ANY two steps, critical events preempt.
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let _tick_interval = std::time::Duration::from_millis(1000 / TICK_HZ as u64);
        loop {
            self.handle_critical();
            self.reduce_telemetry();
            let intent = self.evaluate_control().await;
            let verdict = self.arbitrate_and_actuate(intent).await;
            self.record(&verdict);
            self.handle_critical();
            todo!("sleep to next tick boundary; track drift for TickHeartbeat")
        }
    }

    fn handle_critical(&mut self) {
        for event in self.router.drain_critical() {
            match &event.kind {
                EventKind::JogLeverMove(_) => {
                    // Absolute preemption: override flag set in state,
                    // current playbook demoted to shadow, drivers already
                    // handed control back to hardware (docs/09 §Veto).
                    todo!()
                }
                EventKind::DialSet(_) | EventKind::EscalationAnswer(_) => {
                    self.state.reduce(&[event]);
                }
                _ => self.state.reduce(&[event]),
            }
        }
    }

    fn reduce_telemetry(&mut self) {
        let events = self.router.drain_telemetry_coalesced();
        self.state.reduce(&events);
        self.envelope.heartbeat(self.state.tick);
    }

    async fn evaluate_control(&mut self) -> Option<Intent> {
        todo!("playbook host with tick budget; shadow evaluation in parallel")
    }

    async fn arbitrate_and_actuate(&mut self, intent: Option<Intent>) -> Option<Verdict> {
        todo!("envelope.arbitrate → if Approved/Clamped: envelope-owned driver actuate()")
    }

    fn record(&mut self, verdict: &Option<Verdict>) {
        // Every actuation tick appends: state hash, intent, verdict,
        // authority basis. If it can't be reconstructed from the log,
        // it didn't happen (docs/04 A2).
        let _ = verdict;
        todo!()
    }
}

/// Emit an event onto the bus (used by modules holding a router handle).
pub fn emit(_router: &mut LaneRouter, _event: Event) {
    todo!()
}
