use std::sync::Arc;

use anyhow::Result;
use tracing::info;
use twilight_model::gateway::payload::incoming::Ready;

use crate::app::App;

pub async fn handle(_app: Arc<App>, ready: Ready) -> Result<()> {
    info!("Client ready as {} ({})", ready.user.name, ready.user.id);
    Ok(())
}
