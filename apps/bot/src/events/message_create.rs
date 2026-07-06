use std::sync::Arc;

use anyhow::Result;
use twilight_model::channel::Message;

use crate::app::App;

pub async fn handle(app: Arc<App>, message: Message) -> Result<()> {
    if message.author.bot || message.webhook_id.is_some() {
        return Ok(());
    }

    // Chatbot
    if let Some(guild_id) = message.guild_id {
        app.services
            .chatbot
            .on_message(&message, guild_id, app.clone())
            .await;
    }

    Ok(())
}
