//! Lane queues with per-lane backpressure policy (docs/06 §Lanes).

use super::{AgentRole, Event, IngestError, Lane};
use std::collections::VecDeque;
use tokio::sync::mpsc;

const TELEMETRY_CAPACITY: usize = 4096;
const NARRATIVE_CAPACITY: usize = 16384;

/// Sink for events that fail ingest. Never silently dropped — the Auditor
/// reads this. A module that lands here repeatedly is defective.
pub struct QuarantineLog {
    // append-only; same hash-chain discipline as the black box
}

/// Routes events into lane queues; enforces lane privilege at ingest.
pub struct LaneRouter {
    critical_tx: mpsc::UnboundedSender<Event>,
    telemetry: VecDeque<Event>, // coalesced by (kind, producer): newest wins
    narrative: VecDeque<Event>, // drop-oldest; drops counted, Auditor alerted
}

impl LaneRouter {
    /// The single entry point for every event in the system.
    ///
    /// Enforcement order: schema → capability → lane privilege.
    /// Failures go to quarantine with a structured `IngestError`.
    pub fn ingest(&mut self, event: Event) -> Result<(), IngestError> {
        // 1. Schema validation against schemas/vessel-event.schema.json.
        self.validate_schema(&event)?;

        // 2. Capability check: is this actor allowed to emit this kind?
        //    (grants from VesselProfile [agents.*])
        self.check_capability(&event)?;

        // 3. Lane privilege: critical lane is for L0 drivers + envelope only.
        if event.lane == Lane::Critical && !self.may_emit_critical(&event.source.module) {
            return Err(IngestError::LaneViolation {
                module: event.source.module.clone(),
            });
        }

        todo!("route into the lane queue with its backpressure policy")
    }

    fn validate_schema(&self, _event: &Event) -> Result<(), IngestError> {
        todo!("validate payload against registered kind schema")
    }

    fn check_capability(&self, _event: &Event) -> Result<(), IngestError> {
        // Analyst holds no control.* grant; enforcement here makes that
        // structural rather than polite (docs/10 §Capability model).
        todo!("check actor grants from vessel profile")
    }

    fn may_emit_critical(&self, module: &str) -> bool {
        module.starts_with("driver.") || module == "envelope" || module == "kernel"
    }

    /// Drain all critical events. Called between every two kernel steps —
    /// this is what "preempts" means concretely.
    pub fn drain_critical(&mut self) -> Vec<Event> {
        todo!()
    }

    /// Coalescing drain for the reducer: newest event per (kind, producer).
    pub fn drain_telemetry_coalesced(&mut self) -> Vec<Event> {
        todo!()
    }
}

/// Marker: agents that may only ever consume, never produce, control kinds.
/// Used by capability checks; zero-sized, compile-time documentation.
pub struct ReadOnlyRole(pub AgentRole);
