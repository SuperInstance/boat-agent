//! boatctl: the typed agent command surface (docs/10).
//!
//! Everything — UI included — goes through this. The Tauri UI is a
//! client; agents are never second-class. If a feature can't be driven
//! from here, it doesn't exist for agents, and it doesn't ship.
//!
//! Transport: newline-delimited JSON over local IPC. Structured in,
//! structured out; errors carry codes; writes are capability-checked
//! and logged with actor identity.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Command {
    // ── discovery (agents call this first; varies with dial/hardware) ──
    Capabilities,

    // ── reads (unauthenticated locally) ──
    StateSnapshot { since_seq: Option<u64> },
    BusTail { lane: Option<String>, kind_glob: Option<String>, limit: u32 },
    MemoryQuery { ns: String, query: String },
    MemoryWhy { artifact_ref: String },
    AuditVerify { from_seq: Option<u64>, to_seq: Option<u64> },
    DialGet,

    // ── control (capability-checked) ──
    MissionStart { name: String, params: Option<toml::Value> },
    MissionStop,
    PlaybookList,
    PlaybookShow { id: String },
    PlaybookPropose { bundle_path: String },     // Engineer: enters the gate
    PlaybookActivate { id: String },             // post-gate only; also rollback
    PlaybookDemote { id: String, reason: String },
    Escalate { request_json: String },           // rate-limited (docs/09)
    EscalationRespond { id: String, option_id: String }, // human path via UI
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<CommandError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandError {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ErrorCode {
    /// actor lacks the grant — see vessel.toml [agents.*]
    EpermCapability,
    /// referenced state snapshot no longer available
    EstaleState,
    /// playbook stage-gate not satisfied
    EgateViolation,
    /// escalation budget exhausted this window
    EattentionBudget,
    /// hardware degraded; command unavailable right now
    Edegraded,
    /// malformed request
    Einvalid,
}

/// Dispatch is the ONLY path from agents/UI into the kernel besides bus
/// events. Same capability checks as bus ingest (docs/06).
pub async fn dispatch(_cmd: Command, _actor: &crate::bus::Actor) -> Response {
    todo!("capability check → route to kernel/memory/playbook registry → structured response")
}
