mod autocomplete;
mod options;

pub use autocomplete::*;
pub use options::*;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use anyhow::{Context as _, Result, anyhow};
use twilight_http::{Client, client::InteractionClient};
use twilight_model::{
    application::{command::CommandOptionChoice, interaction::Interaction},
    channel::{Message, message::MessageFlags},
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{
        Id,
        marker::{ApplicationMarker, GuildMarker},
    },
    user::User,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::commands::Reply;

/// A command context, with many helpers.
pub struct CommandContext<'a, App> {
    pub app: Arc<App>,
    pub http: Arc<Client>,
    pub application_id: Id<ApplicationMarker>,
    pub interaction: &'a Interaction,
    /// Store whether the initial interaction response (ack/defer/reply)
    /// has already been sent.
    has_sent_initial_response: AtomicBool,
}

impl<'a, App> CommandContext<'a, App> {
    pub fn new(
        app: Arc<App>,
        http: Arc<Client>,
        application_id: Id<ApplicationMarker>,
        interaction: &'a Interaction,
    ) -> Self {
        Self {
            app,
            http,
            application_id,
            interaction,
            has_sent_initial_response: AtomicBool::new(false),
        }
    }

    pub fn interaction_client(&self) -> InteractionClient<'_> {
        self.http.interaction(self.application_id)
    }

    pub fn user(&self) -> &User {
        self.interaction
            .author()
            .expect("interaction always has an author")
    }

    pub fn user_id(&self) -> u64 {
        self.user().id.get()
    }

    pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.interaction.guild_id
    }
}

impl<'a, App> CommandContext<'a, App> {
    /// Defer the reply.
    pub async fn defer_reply(&self, ephemeral: bool) -> Result<()> {
        if self.has_sent_initial_response.swap(true, Ordering::SeqCst) {
            anyhow::bail!("initial response already sent, cannot defer_reply");
        }

        let mut flags = MessageFlags::empty();
        if ephemeral {
            flags |= MessageFlags::EPHEMERAL;
        }

        let data = InteractionResponseDataBuilder::new().flags(flags).build();

        let response = InteractionResponse {
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
            data: Some(data),
        };

        self.interaction_client()
            .create_response(self.interaction.id, &self.interaction.token, &response)
            .await
            .context("failed to defer interaction")?;

        Ok(())
    }

    /// Send the initial interaction response.
    pub async fn reply(&self, reply: Reply) -> Result<()> {
        if self.has_sent_initial_response.swap(true, Ordering::SeqCst) {
            anyhow::bail!("initial response already sent; cannot reply");
        }

        let mut builder = InteractionResponseDataBuilder::new()
            .flags(reply.flags)
            .allowed_mentions(reply.allowed_mentions.unwrap_or_default());

        if let Some(content) = reply.content {
            builder = builder.content(content);
        }
        if !reply.embeds.is_empty() {
            builder = builder.embeds(reply.embeds);
        }
        if !reply.components.is_empty() {
            builder = builder.components(reply.components);
        }

        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(builder.build()),
        };

        self.interaction_client()
            .create_response(self.interaction.id, &self.interaction.token, &response)
            .await
            .context("failed to send interaction response")?;

        Ok(())
    }

    /// Send the initial interaction response and return the created `Message`.
    pub async fn reply_with_response(&self, reply: Reply) -> Result<Message> {
        self.reply(reply).await?;

        let message = self
            .interaction_client()
            .response(&self.interaction.token)
            .await
            .context("failed to fetch initial response (reply_with_response)")?
            .model()
            .await
            .context("failed to deserialize initial response (reply_with_response)")?;

        Ok(message)
    }

    /// Send a followup message.
    pub async fn follow_up(&self, reply: Reply) -> Result<Message> {
        let client = self.interaction_client();
        let mut req = client
            .create_followup(&self.interaction.token)
            .flags(reply.flags);

        if let Some(content) = reply.content.as_deref() {
            req = req.content(content);
        }
        if !reply.embeds.is_empty() {
            req = req.embeds(&reply.embeds);
        }
        if !reply.components.is_empty() {
            req = req.components(&reply.components);
        }

        let message = req
            .await
            .context("failed to send followup")?
            .model()
            .await
            .context("failed to deserialize followup message")?;

        Ok(message)
    }

    /// Shorthand for a text-only reply. Allowed mentions are none by default.
    pub async fn say(&self, content: impl Into<String>) -> Result<()> {
        self.reply(Reply::new().content(content)).await
    }

    /// Edit the original response.
    pub async fn edit_reply(&self, reply: Reply) -> Result<()> {
        let client = self.interaction_client();
        let mut req = client
            .update_response(&self.interaction.token)
            .flags(reply.flags);

        req = req.content(reply.content.as_deref());

        let embeds = (!reply.embeds.is_empty()).then_some(reply.embeds.as_slice());
        req = req.embeds(embeds);

        let components = (!reply.components.is_empty()).then_some(reply.components.as_slice());
        req = req.components(components);

        req.await.context("failed to edit interaction response")?;

        Ok(())
    }

    /// Delete the original response.
    pub async fn delete(&self) -> Result<()> {
        self.interaction_client()
            .delete_response(&self.interaction.token)
            .await
            .context("failed to delete interaction response")?;
        Ok(())
    }

    /// Respond to an autocomplete interaction.
    pub async fn respond(&self, choices: Vec<CommandOptionChoice>) -> Result<()> {
        let data = InteractionResponseDataBuilder::new()
            .choices(choices)
            .build();

        let response = InteractionResponse {
            kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
            data: Some(data),
        };

        self.interaction_client()
            .create_response(self.interaction.id, &self.interaction.token, &response)
            .await
            .map_err(|e| anyhow!(e))
            .context("failed to send autocomplete response")?;

        Ok(())
    }
}
