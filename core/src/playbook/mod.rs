//! Playbook host: runs AI-authored control bundles in a SUPERVISED CHILD
//! PROCESS — never in-process (docs/05, docs/07).
//!
//! Why a child process and not pyo3-in-core: a segfault or GIL deadlock in
//! generated code must not be able to take down the safety core. The
//! kernel holds the heartbeat; the child is killable; the envelope holds
//! the last safe command while a replacement boots. (WASM via wasmtime is
//! the planned migration for fuel-metered determinism — same trait.)
//!
//! Playbooks are PURE: f(StateSnapshot, Targets) → Intent. No I/O, no
//! imports outside the allowlist, no state between calls. Enforced at the
//! stage-gate (AST scan), re-verified here by sandboxing (no fs/net).

use crate::bus::events::Intent;
use crate::state::StateSnapshot;
use serde::{Deserialize, Serialize};

/// Mission-provided targets (L3 → L2). The playbook's only goal input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Targets {
    pub target_sog_kn: Option<f32>,
    pub target_track_deg: Option<f32>,
    pub params: toml::Value, // mission-specific extras
}

/// Identity + stage of a bundle, from the registry in Memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Draft,
    Analyzed,
    Replayed,
    Approved,
    Shadow,
    Active,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookRef {
    pub id: String,     // content hash — "halibut_trolling_v3@b3f9…"
    pub stage: Stage,
    pub path: std::path::PathBuf,
}

pub struct PlaybookHost {
    active: Option<PlaybookRef>,
    shadow: Vec<PlaybookRef>,
    // child: supervised process handle + heartbeat (kernel supervisor owns)
}

impl PlaybookHost {
    /// Per-tick evaluation. NON-BLOCKING: if the child hasn't answered
    /// within the tick budget, the kernel uses the last safe command and
    /// the miss is counted (Auditor escalates after N misses — the boat
    /// never waits on AI, docs/05).
    pub async fn evaluate(
        &mut self,
        snapshot: &StateSnapshot,
        targets: &Targets,
    ) -> PlaybookOutput {
        todo!("send snapshot+targets to child over NDJSON stdio; await with tick budget")
    }

    /// Shadow evaluation runs in parallel with the active playbook but its
    /// results are logged as `agent.shadow.delta` — never actuated.
    /// This is the trust engine (docs/09: "I matched your calls 94%").
    pub async fn evaluate_shadow(
        &mut self,
        snapshot: &StateSnapshot,
        targets: &Targets,
    ) -> Vec<(String, Intent)> {
        todo!()
    }

    /// Swap the active playbook. Only reachable via the stage-gate
    /// (kernel command after human approval + Auditor certification).
    /// Rollback is this same function with an older id — content-addressed,
    /// no file copying (legacy rollback.rs is dead).
    pub async fn activate(&mut self, playbook: PlaybookRef) -> Result<(), PlaybookError> {
        if playbook.stage != Stage::Active {
            return Err(PlaybookError::GateViolation(playbook.id));
        }
        todo!("spawn new child, warm it against one replay tick, swap, retire old")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlaybookOutput {
    Intent(Intent),
    /// Child missed the tick budget: kernel holds last safe command.
    Timeout,
    /// Child faulted: kernel restarts it; N faults → auto-demote + escalate.
    Fault(String),
}

#[derive(Debug, thiserror::Error)]
pub enum PlaybookError {
    #[error("gate violation: playbook {0} is not ACTIVE")]
    GateViolation(String),
    #[error("child process failed to spawn: {0}")]
    Spawn(String),
    #[error("protocol violation from child: {0}")]
    Protocol(String),
}
