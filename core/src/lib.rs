//! boat-core: the Vessel Intelligence OS microkernel.
//!
//! Trusted computing base. Everything above the envelope is untrusted,
//! including all AI-authored code. See docs/05_KERNEL_ARCHITECTURE.md.
//!
//! Layers (authority decreases upward, speed decreases upward):
//!   L0 drivers   — hardware ⇄ bus events           (drivers/)
//!   L1 envelope  — the only actuator door          (envelope/  [AGENT READ-ONLY])
//!   L2 playbooks — AI-authored pure f(state)→intent (playbook/)
//!   L3 missions  — target-setting state machines    (missions, planned)
//!   L4 agents    — operator/engineer/analyst/auditor (agent_api/)

pub mod agent_api;
pub mod blackbox;
pub mod bus;
pub mod config;
pub mod drivers;
pub mod envelope;
pub mod kernel;
pub mod memory;
pub mod playbook;
pub mod state;

pub use bus::{Event, Lane};
pub use config::VesselProfile;
pub use state::VesselState;
