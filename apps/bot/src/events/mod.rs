pub mod interaction_create;
pub mod message_create;
pub mod ready;

use std::sync::Arc;

use anyhow::Result;
use twilight_gateway::Event;

use crate::{app::App, events};

/// Handle a Discord gateway event.
pub async fn handle_event(app: Arc<App>, event: Event) -> Result<()> {
    match event {
        Event::Ready(ready) => events::ready::handle(app, ready).await,
        Event::InteractionCreate(interaction) => {
            events::interaction_create::handle(app, interaction.0).await
        }
        Event::MessageCreate(message) => events::message_create::handle(app, message.0).await,
        _ => Ok(()),
    }
}
