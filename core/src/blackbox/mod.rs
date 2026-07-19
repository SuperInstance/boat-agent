//! Black box: append-only, SHA-256 hash-chained flight recorder.
//!
//! Promoted from passive logger to system spine (docs/11): it is the
//! replay source, the audit base, and the trust-metric substrate.
//! Core logic kept from the legacy design — the chain was right.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// What gets recorded every actuation tick. Fields chosen so that
/// "why did the boat do that?" is always computable (docs/04 A7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackBoxEntry {
    pub timestamp_epoch_ms: u64,
    pub tick: u64,
    pub state_hash: String,          // exact world-state anchor
    pub dial_level: u8,              // authority basis
    pub actor: String,               // "playbook:…@hash" | "human" | "envelope"
    pub intent_json: Option<String>,
    pub verdict_json: Option<String>,
    pub human_override_detected: bool,
}

pub struct BlackBox {
    last_hash: Vec<u8>,
    log_path: std::path::PathBuf,
}

impl BlackBox {
    pub fn open(log_path: std::path::PathBuf) -> std::io::Result<Self> {
        // Resume: read last line, adopt its hash as chain head. If the
        // file was truncated or corrupted, the FIRST new entry records
        // the discontinuity explicitly — gaps are data, not errors.
        todo!()
    }

    /// Append one entry. prev_hash|curr_hash|json — same format as the
    /// legacy implementation, verified by `boatctl audit verify`.
    pub fn append(&mut self, entry: &BlackBoxEntry) -> std::io::Result<String> {
        let json = serde_json::to_string(entry)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let mut hasher = Sha256::new();
        hasher.update(&self.last_hash);
        hasher.update(json.as_bytes());
        let current_hash = hasher.finalize().to_vec();

        // format + atomic append + flush (legacy blackbox.rs logic, kept)
        let _line = format!("{}|{}|{}\n", hex::encode(&self.last_hash), hex::encode(&current_hash), json);
        self.last_hash = current_hash;
        todo!("write + flush; return current hash hex")
    }

    /// Verify a range of the chain. The Auditor runs this continuously;
    /// `boatctl audit verify` exposes it to agents and humans.
    pub fn verify_range(&self, _from_seq: u64, _to_seq: u64) -> bool {
        todo!()
    }
}
