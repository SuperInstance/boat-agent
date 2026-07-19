//! VesselMemory: one local-first abstraction for everything learned
//! (docs/08). SQLite + content-addressed files today, trait-swappable.
//!
//! RULES:
//!   - every write carries provenance (unsourced memory is rumor)
//!   - no destructive updates — `put` supersedes, `history` retains
//!   - playbooks (generated code) can NEVER write memory
//!   - telemetry never syncs raw; digests only

use crate::bus::Provenance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Namespace {
    Vocab,       // captain's term mappings — the trust interface
    Playbooks,   // registry: id → stage, hash, evidence, live stats
    Patterns,    // learned behavior segments (state-window → human action)
    Calibration, // hydrodynamic params, compass deviation, per-sea-state gains
    Catches,     // one-tap outcome labels
    Missions,    // named mission configs + history
    Escalations, // every escalation + resolution (attention audit trail)
    Fleet,       // imported cross-vessel candidates — inert until locally gated
}

pub type Hash = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub hash: Hash,
    pub value: serde_json::Value,
    pub provenance: Provenance,
    pub timestamp_ms: u64,
}

/// Semantic or structured query. Offline-capable: local embeddings with
/// graceful fallback to structured-only (docs/08 §Query interface).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Query {
    Structured { filter: String },
    Semantic { text: String, k: u8 },
}

pub trait VesselMemory: Send + Sync {
    fn put(&self, ns: Namespace, key: &str, value: serde_json::Value, prov: Provenance) -> Hash;
    fn get(&self, ns: Namespace, key: &str) -> Option<Version>;
    fn history(&self, ns: Namespace, key: &str) -> Vec<Version>;
    fn query(&self, ns: Namespace, q: &Query) -> Vec<Version>;

    /// Hash-stamped namespace slice — the sync unit. Conflicts resolve by
    /// provenance precedence (local human > local agent > fleet), never
    /// last-writer-wins for locally learned facts.
    fn digest(&self, ns: Namespace, since_ms: u64) -> Digest;

    /// THE audit superpower: full provenance walk from any learned
    /// artifact back to raw evidence (rule → pattern → transcripts →
    /// event ids → replay report). Must answer in <1s.
    fn why(&self, artifact_ref: &str) -> Vec<Version>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Digest {
    pub ns: Namespace,
    pub range: (u64, u64),
    pub head_hash: Hash,
    pub entries: Vec<Version>,
}

/// Local implementation. CAS files for blobs (audio, replay reports),
/// SQLite for indexes. The boat is fully intelligent offline; the cloud
/// is an amplifier, not a dependency.
pub struct LocalMemory {
    // db: rusqlite::Connection,
    // cas_dir: PathBuf,
}

impl VesselMemory for LocalMemory {
    fn put(&self, _ns: Namespace, _key: &str, _v: serde_json::Value, _p: Provenance) -> Hash {
        todo!("append new version; never overwrite")
    }
    fn get(&self, _ns: Namespace, _key: &str) -> Option<Version> { todo!() }
    fn history(&self, _ns: Namespace, _key: &str) -> Vec<Version> { todo!() }
    fn query(&self, _ns: Namespace, _q: &Query) -> Vec<Version> { todo!() }
    fn digest(&self, _ns: Namespace, _since: u64) -> Digest { todo!() }
    fn why(&self, _artifact_ref: &str) -> Vec<Version> { todo!() }
}
