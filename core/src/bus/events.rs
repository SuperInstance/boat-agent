//! Typed event payloads. DERIVED FROM schemas/vessel-event.schema.json —
//! change the schema first (AGENTS.md conventions). Kind strings follow
//! `<domain>.<entity>.<action>` (docs/06 §Taxonomy).

use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Serde-tagged union of all payload types. The tag IS the kind string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum EventKind {
    // ── Sensing (L0 → bus, telemetry) ────────────────────────────────
    #[serde(rename = "sensor.gps.fix")]
    GpsFix(GpsFix),
    #[serde(rename = "sensor.compass.heading")]
    CompassHeading(CompassHeading),
    #[serde(rename = "sensor.depth.sounder")]
    DepthSounder(DepthSounder),
    #[serde(rename = "sensor.engine.rpm")]
    EngineRpm(EngineRpm),
    #[serde(rename = "sensor.wind.apparent")]
    WindApparent(WindApparent),
    #[serde(rename = "sensor.rudder.angle")]
    RudderAngle(RudderAngle),
    #[serde(rename = "sensor.thermal.reading")]
    ThermalReading(ThermalReading),

    // ── Human input ──────────────────────────────────────────────────
    #[serde(rename = "human.jog_lever.move")]
    JogLeverMove(JogLeverMove), // critical lane — absolute override
    #[serde(rename = "human.dial.set")]
    DialSet(DialSet), // critical lane
    #[serde(rename = "human.voice.transcript")]
    VoiceTranscript(VoiceTranscript),
    #[serde(rename = "human.escalation.answer")]
    EscalationAnswer(EscalationAnswer), // critical lane
    #[serde(rename = "human.catch.log")]
    CatchLog(CatchLog),

    // ── Control (L2/L3 → envelope → L0) ─────────────────────────────
    #[serde(rename = "control.intent")]
    Intent(Intent),
    #[serde(rename = "control.verdict")]
    Verdict(Verdict),
    #[serde(rename = "control.watchdog.trip")]
    WatchdogTrip(WatchdogTrip), // critical lane

    // ── Agent activity (narrative) ───────────────────────────────────
    #[serde(rename = "agent.proposal.playbook")]
    PlaybookProposal(PlaybookProposal),
    #[serde(rename = "agent.escalation.request")]
    EscalationRequest(EscalationRequest),
    #[serde(rename = "agent.shadow.delta")]
    ShadowDelta(ShadowDelta),
    #[serde(rename = "agent.audit.result")]
    AuditResult(AuditResult),

    // ── System (kernel) ──────────────────────────────────────────────
    #[serde(rename = "system.tick.heartbeat")]
    TickHeartbeat(TickHeartbeat),
    #[serde(rename = "system.degraded_mode")]
    DegradedMode(DegradedMode), // critical lane
}

// ── Sensing payloads ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsFix {
    pub lat: f64,
    pub lon: f64,
    pub sog_kn: f32,
    pub cog_deg: f32,
    pub hdop: f32,
    pub sats: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompassHeading {
    pub heading_deg: f32,
    pub swing_rate_dps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthSounder {
    pub depth_m: f32,
    pub bottom_hardness: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineRpm {
    pub engine_id: String,
    pub rpm: u32,
    pub throttle_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindApparent {
    pub speed_kn: f32,
    pub angle_deg: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RudderAngle {
    pub angle_deg: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalReading {
    pub zone: String, // "ENGINE_ROOM", ...
    pub celsius: f32,
    pub sensor_id: String,
}

// ── Human payloads ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JogLeverMove {
    pub direction: i8,
    pub magnitude: f32,
}

/// The autonomy dial. Ceiling, not floor (docs/09).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DialSet {
    pub level: AutonomyLevel,
    pub by_whom: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutonomyLevel {
    Log = 0,       // observe only
    Coach = 1,     // suggest; intents become advisory
    Supervise = 2, // act with standing consent; novel situations escalate
    Autopilot = 3, // act within envelope; escalate on anomaly
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTranscript {
    pub text: String,
    /// Word-level timestamps; alignment-confidence gated before training
    /// use (docs/08, legacy RQ-001).
    pub word_timestamps: Vec<(String, u64, u64)>,
    pub audio_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationAnswer {
    pub escalation_id: Ulid,
    pub option_id: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchLog {
    pub species: String,
    pub depth_fm: f32,
    pub note: Option<String>,
}

// ── Control payloads ─────────────────────────────────────────────────────

/// What a playbook/mission WANTS. Never reaches hardware directly —
/// the envelope owns the only door (docs/05 law 4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub requested_rudder_deg: Option<f32>,
    pub requested_throttle_pct: Option<f32>,
    pub horizon_s: f32,
}

/// The envelope's answer to an Intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verdict {
    pub intent_id: Ulid,
    pub outcome: VerdictOutcome,
    pub reason: String,
    pub final_command: Option<Intent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerdictOutcome {
    Approved,
    Clamped,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogTrip {
    pub missed_heartbeats: u32,
    pub last_known_state_hash: String,
}

// ── Agent payloads ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookProposal {
    pub playbook_id: String,
    pub diff_summary: String,
    pub evidence_refs: Vec<String>,
}

/// Structured escalation — the attention-spending transaction (docs/09).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRequest {
    pub question: String,
    pub options: Vec<EscalationOption>,
    pub recommendation: String,
    pub context_bundle_ref: String,
    pub expires_s: u32,
    pub fallback_on_timeout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationOption {
    pub id: String,
    pub label: String,
    pub consequence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowDelta {
    pub playbook_id: String,
    pub what_agent_would_do: Intent,
    pub what_human_did: Option<Intent>,
    pub agreement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub chain_verified: bool,
    pub anomalies: Vec<String>,
    pub checked_range: (u64, u64),
}

// ── System payloads ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickHeartbeat {
    pub tick_n: u64,
    pub drift_ms: i64,
    pub state_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradedMode {
    pub cause: String,
    pub remaining_capabilities: Vec<String>,
}
