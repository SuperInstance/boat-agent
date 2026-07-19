//! ═══════════════════════════════════════════════════════════════════
//!  THE SAFETY ENVELOPE — AGENT READ-ONLY (prime directive 1)
//!
//!  The vessel's reflexes. Every physical output in the system passes
//!  through `arbitrate()` and every actuator write is performed by this
//!  module. There is no second door (docs/05 law 4).
//!
//!  This code trusts NOTHING above it: playbooks are assumed adversarial,
//!  agents are assumed fallible, networks are assumed absent. It trusts
//!  only: vessel.toml limits, hardware watchdog heartbeats, and the
//!  human's hands.
//!
//!  Any agent that believes this module needs to change: STOP.
//!  Emit an escalation. A human reviews envelope changes. No exceptions.
//! ═══════════════════════════════════════════════════════════════════

use crate::bus::events::{AutonomyLevel, Intent, Verdict, VerdictOutcome};
use crate::config::EnvelopeLimits;
use crate::state::VesselState;
use serde::{Deserialize, Serialize};

/// Rate-limit memory. Kept minimal and explicit.
#[derive(Debug, Default)]
pub struct EnvelopeMemory {
    last_rudder_deg: Option<f32>,
    last_throttle_pct: Option<f32>,
    last_command_ms: u64,
    consecutive_rejections: u32,
    missed_kernel_heartbeats: u32,
}

pub struct SafetyEnvelope {
    limits: EnvelopeLimits, // from vessel.toml — never hardcoded per boat
    memory: EnvelopeMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Arbitration {
    Approved(Intent),
    Clamped { command: Intent, reason: String },
    Rejected(String),
}

impl SafetyEnvelope {
    pub fn new(limits: EnvelopeLimits) -> Self {
        Self {
            limits,
            memory: EnvelopeMemory::default(),
        }
    }

    /// The one door. Called once per tick per intent.
    ///
    /// Check order is deliberate — cheapest and most absolute first:
    ///   1. HUMAN VETO       override_active → reject everything, drivers
    ///                       already preempted on the critical lane
    ///   2. WATCHDOG         kernel heartbeat stale → safe state, no command
    ///   3. DIAL CEILING     dial < required → downgrade to advisory/reject
    ///   4. SENSOR SANITY    actuating on stale sensors → reject
    ///   5. HARD BOUNDS      rudder ±limit, throttle range, RPM redline
    ///   6. RATE LIMITS      per-tick deltas, cooldowns
    ///   7. CONTEXT GUARDS   min speed for autopilot, shoaling trend
    pub fn arbitrate(&mut self, intent: &Intent, state: &VesselState) -> Arbitration {
        if state.human.override_active {
            return Arbitration::Rejected("human override active".into());
        }
        if self.memory.missed_kernel_heartbeats >= self.limits.watchdog_miss_threshold {
            return Arbitration::Rejected("watchdog tripped: safe state held".into());
        }
        // 3–7: bounds, rates, context guards — pure functions of
        // (intent, state, limits, memory). No I/O. Replay-verifiable.
        todo!("implement checks 3–7 exactly in the documented order")
    }

    /// Effective autonomy = min(dial, what current conditions allow).
    /// Sensor degradation lowers the ceiling automatically (docs/09).
    pub fn effective_autonomy(&self, state: &VesselState) -> AutonomyLevel {
        let dial = state.human.dial;
        if state.degraded.is_some() || self.memory.missed_kernel_heartbeats > 0 {
            return dial.min(AutonomyLevel::Supervise);
        }
        dial
    }

    /// Kernel heartbeat, once per tick. Missing ticks are counted by the
    /// CALLER's absence — the watchdog hardware relay is armed here.
    pub fn heartbeat(&mut self, tick: u64) {
        self.memory.missed_kernel_heartbeats = 0;
        let _ = tick;
    }

    /// Convert an arbitration into the bus-visible verdict event.
    pub fn to_verdict(&self, intent_id: ulid::Ulid, arb: Arbitration) -> Verdict {
        match arb {
            Arbitration::Approved(cmd) => Verdict {
                intent_id,
                outcome: VerdictOutcome::Approved,
                reason: String::new(),
                final_command: Some(cmd),
            },
            Arbitration::Clamped { command, reason } => Verdict {
                intent_id,
                outcome: VerdictOutcome::Clamped,
                reason,
                final_command: Some(command),
            },
            Arbitration::Rejected(reason) => Verdict {
                intent_id,
                outcome: VerdictOutcome::Rejected,
                reason,
                final_command: None,
            },
        }
    }
}
