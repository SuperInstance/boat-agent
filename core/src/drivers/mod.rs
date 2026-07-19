//! L0: hardware drivers. Normalize hardware ⇄ bus events.
//!
//! A driver is ~150 lines behind this trait. Add a sensor = add a driver;
//! no other file changes (docs/05 §Modularity).
//!
//! CONTAINMENT RULE: drivers run in supervised tasks. A panicking driver
//! is restarted with backoff by the kernel supervisor; its failure must
//! never reach the envelope or the tick loop. Legacy `expect()`/`unwrap()`
//! on hardware I/O is forbidden here — every failure is a Result.

use crate::bus::Event;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// What a driver says about itself — used by auto-discovery during
/// onboarding (the wizard's real job: probe, then write vessel.toml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverDescriptor {
    pub id: String,                    // "driver.nmea0183"
    pub displays_as: String,           // "GPS / NMEA-0183 serial"
    pub config_schema: serde_json::Value, // JSON Schema for its [drivers.*] entry
    pub can_autodetect: bool,
}

/// Result of probing a candidate port/device during discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub confidence: f32,               // sniffed NMEA sentences at 4800? 0.95
    pub suggested_config: toml::Value, // ready to merge into vessel.toml
    pub conflict: Option<String>,      // e.g. "port locked by other process"
                                       //   → feeds auto-diagnosis (legacy Insight-005)
}

#[async_trait]
pub trait Driver: Send + Sync {
    fn descriptor(&self) -> DriverDescriptor;

    /// Try to identify hardware on a candidate resource (port, IP, …).
    /// Default: not detectable.
    async fn detect(_candidate: &str) -> Option<Detection>
    where
        Self: Sized,
    {
        None
    }

    /// Start producing events. Runs until the kernel cancels the token
    /// or the driver returns an error (→ supervised restart with backoff).
    async fn run(
        &mut self,
        emit: Box<dyn FnMut(Event) + Send>,
        cancel: tokio_util::sync::CancellationToken,
    ) -> Result<(), DriverError>;

    /// Actuation side, ONLY for drivers that own an actuator — and even
    /// then, the only caller is the envelope (docs/05 law 4). Receiving
    /// this call from anywhere else is a defect worth an audit finding.
    async fn actuate(&mut self, _command: &crate::bus::events::Intent) -> Result<(), DriverError> {
        Err(DriverError::NotAnActuator)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("hardware unavailable: {0}")]
    Unavailable(String),
    #[error("port locked by another process: {0}")]
    PortLocked(String),
    #[error("malformed data: {0}")]
    Malformed(String),
    #[error("this driver has no actuator")]
    NotAnActuator,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

// Planned driver modules (see docs/11 §Module-file mapping):
//   pub mod nmea0183;   // GPS/compass/depth; auto-baud sniff 4800/38400
//   pub mod n2k;        // CAN bus, PGN 127488 engine params
//   pub mod com_splice; // physical→virtual multi-cast (legacy Phase 4)
//   pub mod jog_lever;  // GPIO/serial — THE critical-lane override source
//   pub mod screencap;  // wheelhouse screen → frames on disk, meta on bus
//   pub mod autopilot;  // actuator: $GPAPB out, checksum always (Insight-002)
//   pub mod throttle;   // actuator: servo / digital potentiometer
