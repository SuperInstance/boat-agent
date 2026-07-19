//! Entry point. Headless kernel; the Tauri shell is a separate crate that
//! links this lib and is just an adapter (docs/11_MIGRATION_MAP.md).

use boat_core::{config::VesselProfile, kernel::Kernel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let profile_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "vessel.toml".to_string());

    // One file describes the whole boat. Docs: schemas/vessel-profile.schema.json
    let profile = VesselProfile::load(&profile_path)?;

    let mut kernel = Kernel::bootstrap(profile).await?;

    // Runs the fixed-tick loop until shutdown or watchdog-safe halt.
    // With feature `replay`, this drives the loop from a recorded corpus
    // instead of live drivers — the universal test harness (docs/07).
    kernel.run().await
}
