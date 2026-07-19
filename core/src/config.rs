//! Vessel profile loading — the one file that describes the whole boat.
//! Click-and-play = this file + auto-discovery (docs/04, docs/05).
//! Schema: schemas/vessel-profile.schema.json. Example: vessel.toml.example.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselProfile {
    pub vessel: VesselInfo,
    pub envelope: EnvelopeLimits,
    pub drivers: HashMap<String, toml::Value>, // per-driver config, schema-validated
    pub agents: HashMap<String, AgentGrants>,
    pub missions: HashMap<String, toml::Value>,
    pub cloud: Option<CloudConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselInfo {
    pub id: String,
    pub name: String,
    pub vessel_type: String, // "troller", "longliner", ...
    pub data_dir: String,    // profile-relative; no hardcoded paths anywhere
}

/// THE safety numbers. Legacy consts became defaults; per-vessel tuning
/// lives here and ONLY here (AGENTS.md: no magic numbers in code).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeLimits {
    pub max_rudder_deg: f32,            // default 15.0
    pub min_autopilot_speed_kn: f32,    // default 2.0
    pub max_trolling_rpm: u32,          // default 1800
    pub max_throttle_step_pct: f32,     // default 15.0 (per adjustment)
    pub trolling_throttle_ceiling_pct: f32, // default 40.0
    pub watchdog_heartbeat_ms: u64,     // default 300 (legacy Insight-001)
    pub watchdog_miss_threshold: u32,   // default 3 (≈800–900 ms to trip)
    pub shoaling_reject_m_per_min: f32, // depth-trend guard
    pub min_command_interval_ms: u64,   // default 200 — min ms between actuations
    pub max_rudder_step_deg: f32,       // default 15.0 — max rudder delta per command
    pub max_swing_dps: f32,             // default 20.0 — compass swing guard threshold
}

/// Capability grants per agent role (docs/10 §Capability model).
/// Enforced at bus ingest — structural, not polite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentGrants {
    #[serde(default)]               // map key names the role; field optional
    pub role: String,                    // "operator" | "engineer" | ...
    pub may_emit: Vec<String>,           // kind globs: ["agent.shadow.*"]
    pub may_invoke: Vec<String>,         // boatctl command groups
    pub escalation_budget_per_hour: u32, // attention is scarce (docs/09)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub worker_url: String,        // Cloudflare edge
    pub sync_enabled: bool,
    pub vessel_key_path: String,   // signs digests; fleet imports verified
}

impl VesselProfile {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let profile: Self = toml::from_str(&text)?;
        profile.validate()?;
        Ok(profile)
    }

    /// Cross-field validation beyond serde: sane limit relationships,
    /// and no playbook/agent granted write access to envelope limits.
    /// A profile that fails validation must never boot — fail LOUD here,
    /// at the dock, not at sea.
    fn validate(&self) -> anyhow::Result<()> {
        let e = &self.envelope;
        anyhow::ensure!(
            e.max_throttle_step_pct <= e.trolling_throttle_ceiling_pct,
            "envelope: max_throttle_step_pct ({}) exceeds trolling ceiling ({})",
            e.max_throttle_step_pct,
            e.trolling_throttle_ceiling_pct
        );
        anyhow::ensure!(
            e.max_rudder_step_deg <= e.max_rudder_deg,
            "envelope: max_rudder_step_deg ({}) exceeds max_rudder_deg ({})",
            e.max_rudder_step_deg,
            e.max_rudder_deg
        );
        anyhow::ensure!(
            e.watchdog_miss_threshold >= 1,
            "envelope: watchdog_miss_threshold must be >= 1"
        );
        anyhow::ensure!(
            !self.vessel.id.is_empty() && !self.vessel.data_dir.is_empty(),
            "vessel: id and data_dir are required"
        );
        // Analyst must never hold a control.* grant, even if a profile
        // tries to give it one (docs/10 — structural, not polite).
        for (name, grants) in &self.agents {
            if name == "analyst" {
                anyhow::ensure!(
                    !grants.may_emit.iter().any(|g| g.starts_with("control.")),
                    "agents.analyst: control.* emission grants are forbidden"
                );
            }
        }
        Ok(())
    }
}
