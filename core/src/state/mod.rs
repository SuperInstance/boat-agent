//! VesselState: the one authoritative digital twin (docs/05 §Primitive 2).
//!
//! INVARIANTS:
//!   - Pure reducer: reduce(state, events) → state. No I/O, no clocks
//!     other than event timestamps, no randomness. Replayable by A2.
//!   - This is the ONLY copy of the truth. Playbooks, envelope, UI, and
//!     black box all read this same snapshot.
//!   - Sensor fusion (legacy RQ-004: EKF/UKF choice) lives INSIDE the
//!     reducer — an implementation detail invisible to the bus contract.

use crate::bus::events::*;
use crate::bus::Event;
use serde::{Deserialize, Serialize};

/// Quality metadata so consumers can reason about staleness instead of
/// trusting silently-bad numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SensorHealth {
    pub last_update_ms: u64,
    pub stale: bool,
    pub degraded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationState {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub sog_kn: Option<f32>,
    pub cog_deg: Option<f32>,
    pub heading_deg: Option<f32>,
    pub swing_rate_dps: Option<f32>,
    pub depth_m: Option<f32>,
    /// Recent depth window for shoaling trend (legacy autopilot_guard.rs
    /// depth analysis, now a first-class state input).
    pub depth_trend_m_per_min: Option<f32>,
    pub gps_health: SensorHealth,
    pub compass_health: SensorHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropulsionState {
    pub rpm: Option<u32>,
    pub throttle_pct: Option<f32>,
    pub rudder_angle_deg: Option<f32>,
    pub engine_health: SensorHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub wind_speed_kn: Option<f32>,
    pub wind_angle_deg: Option<f32>,
    /// Classified by the reducer from wind/swing statistics; selects the
    /// calibration gain set (legacy RQ-003 gain scheduling).
    pub sea_state: SeaState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SeaState {
    Calm,
    Moderate,
    Rough,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanState {
    pub dial: AutonomyLevel,
    /// True from jog-lever move until N quiet seconds — veto preemption.
    pub override_active: bool,
    pub last_override_ms: Option<u64>,
}

/// The snapshot. `state_hash` chains snapshots so black-box entries can
/// reference an exact world-state (replay + audit anchor).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselState {
    pub tick: u64,
    pub timestamp_ms: u64,
    pub nav: NavigationState,
    pub propulsion: PropulsionState,
    pub environment: EnvironmentState,
    pub human: HumanState,
    pub degraded: Option<String>,
    pub state_hash: String,
}

impl VesselState {
    pub fn genesis() -> Self {
        todo!("all-None sensors, dial=Coach (restart default, docs/09), SeaState::Unknown")
    }

    /// THE reducer. Deterministic. This function's purity is what makes
    /// replay, simulation, and codegen testing possible — guard it.
    pub fn reduce(&mut self, events: &[Event]) {
        for event in events {
            match &event.kind {
                EventKind::GpsFix(p) => self.apply_gps(p, event.timestamp_ms),
                EventKind::CompassHeading(p) => self.apply_compass(p, event.timestamp_ms),
                EventKind::DepthSounder(p) => self.apply_depth(p, event.timestamp_ms),
                EventKind::EngineRpm(p) => self.apply_engine(p, event.timestamp_ms),
                EventKind::WindApparent(p) => self.apply_wind(p, event.timestamp_ms),
                EventKind::RudderAngle(p) => self.apply_rudder(p, event.timestamp_ms),
                EventKind::DialSet(p) => self.human.dial = p.level,
                EventKind::JogLeverMove(_) => {
                    self.human.override_active = true;
                    self.human.last_override_ms = Some(event.timestamp_ms);
                }
                EventKind::DegradedMode(p) => self.degraded = Some(p.cause.clone()),
                _ => {} // narrative-lane kinds never touch state
            }
        }
        self.update_derived(); // trends, sea state, staleness, state_hash
    }

    fn apply_gps(&mut self, _p: &GpsFix, _ts: u64) { todo!() }
    fn apply_compass(&mut self, _p: &CompassHeading, _ts: u64) { todo!() }
    fn apply_depth(&mut self, _p: &DepthSounder, _ts: u64) { todo!() }
    fn apply_engine(&mut self, _p: &EngineRpm, _ts: u64) { todo!() }
    fn apply_wind(&mut self, _p: &WindApparent, _ts: u64) { todo!() }
    fn apply_rudder(&mut self, _p: &RudderAngle, _ts: u64) { todo!() }

    /// Derived quantities: depth trend, sea-state classification, sensor
    /// staleness flags, and the snapshot hash.
    fn update_derived(&mut self) { todo!() }
}

/// Frozen, immutable view handed to playbooks. Playbooks get a snapshot,
/// never a live reference — this is what keeps them pure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot(VesselState);
