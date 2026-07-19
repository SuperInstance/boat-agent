//! The Bus: typed, priority-laned pub/sub. The ONLY way modules talk.
//! Contract: docs/06_BUS_PROTOCOL.md. Schema: schemas/vessel-event.schema.json.
//!
//! Lane privileges are enforced at ingest:
//!   - `critical` may only be emitted by L0 drivers and the envelope.
//!   - payloads failing schema validation go to the quarantine log,
//!     never onto the bus, and the Auditor is notified.

pub mod events;
mod lanes;

pub use events::*;
pub use lanes::{LaneRouter, QuarantineLog};

use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Delivery priority. See docs/06 §Lanes for backpressure semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lane {
    /// Safety. Preempts between any two kernel steps. Drivers + envelope only.
    Critical,
    /// State updates. Coalesced per producer (stale GPS is worse than none).
    Telemetry,
    /// Logs, agent reasoning, sync. Never blocks; dropped-oldest under load.
    Narrative,
}

/// Who/what produced an event. `authority` is re-verified by the envelope
/// before any actuation — emitters attest, the envelope decides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub module: String,             // "driver.nmea0183", "agent.engineer", ...
    pub instance: Option<String>,   // "com3@4800"
    pub actor: Actor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Actor {
    System,
    Human,
    Agent(AgentRole),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentRole {
    Operator,
    Engineer,
    Analyst,
    Auditor,
    Fleet,
}

impl AgentRole {
    /// Stable string form for capability keys and provenance ("operator", ...).
    pub fn as_ref(&self) -> &'static str {
        match self {
            AgentRole::Operator => "operator",
            AgentRole::Engineer => "engineer",
            AgentRole::Analyst => "analyst",
            AgentRole::Auditor => "auditor",
            AgentRole::Fleet => "fleet",
        }
    }
}

/// Provenance turns the log into a graph (docs/04 axiom A7, docs/06 §Rules).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// e.g. "driver", "human", "playbook:halibut_trolling_v3@b3f9…"
    pub authority: String,
    /// Emitter's self-assessed certainty. Fake confidence is worse than low.
    pub confidence: f32,
    /// Event ids this event was derived from. Replay depends on this.
    pub basis: Vec<Ulid>,
}

/// The universal envelope. Payload is validated against the schema at
/// ingest; `seq` is assigned by the kernel (emitters leave it 0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Ulid,
    pub seq: u64,
    pub timestamp_ms: u64,
    pub lane: Lane,
    pub kind: EventKind,
    pub source: Source,
    pub provenance: Provenance,
}

impl Event {
    pub fn new(lane: Lane, kind: EventKind, source: Source, provenance: Provenance) -> Self {
        Self {
            id: Ulid::new(),
            seq: 0,
            timestamp_ms: now_ms(),
            lane,
            kind,
            source,
            provenance,
        }
    }
}

/// Errors at ingest. Structured, machine-actionable (docs/04 axiom A1).
#[derive(Debug, Clone, thiserror::Error)]
pub enum IngestError {
    #[error("lane violation: {module} may not emit on critical lane")]
    LaneViolation { module: String },
    #[error("schema rejection for kind {kind}: {reason}")]
    SchemaViolation { kind: String, reason: String },
    #[error("capability denied: {actor} cannot emit {kind} (EPERM_CAPABILITY)")]
    CapabilityDenied { actor: String, kind: String },
}

pub fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
