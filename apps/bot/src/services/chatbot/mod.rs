mod prompt;
mod tools;
mod util;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use builders::{ActionRowBuilder, TextDisplayBuilder};
use rig_core::{
    client::CompletionClient as _,
    completion::Prompt,
    providers::gemini::{Client as GeminiClient, completion::GEMINI_2_5_FLASH},
};
use tokio::sync::Mutex;
use tracing::{info, warn};
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_model::{
    channel::message::{AllowedMentions, MessageFlags, component::Component},
    guild::Permissions,
    id::marker::ChannelMarker,
};
use twilight_model::{
    channel::{Message, message::MessageType},
    id::{Id, marker::GuildMarker},
};

use crate::{
    app::App,
    config::Config,
    services::chatbot::{
        prompt::build_system_prompt,
        tools::{FetchDocsTool, SourcePage},
        util::{now_secs, strip_self_mention, truncate_to_chars},
    },
};

/// The chatbot service for the support server.
pub struct ChatbotService {
    /// The Gemini client.
    client: GeminiClient,
    is_processing: AtomicBool,
    last_ratelimit_alert_at: Mutex<u64>,
}

impl ChatbotService {
    pub const COOLDOWN: u64 = 1_500;

    pub fn new() -> Self {
        Self {
            client: GeminiClient::new(&Config::get().gemini_key)
                .expect("Failed to create the chatbot client"),
            is_processing: AtomicBool::new(false),
            last_ratelimit_alert_at: Mutex::new(0),
        }
    }

    pub fn on_ready(&self) {
        info!("ChatbotService ready");
    }

    pub async fn on_message(&self, msg: &Message, guild_id: Id<GuildMarker>, app: Arc<App>) {
        if !self.should_handle(msg, &app) {
            return;
        }

        if self
            .is_processing
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        let _ = app.discord.create_typing_trigger(msg.channel_id).await;

        if let Err(e) = self.generate_and_reply(msg, &app).await {
            warn!("Chatbot error in guild {guild_id}: {e}");
        }

        tokio::time::sleep(Duration::from_millis(Self::COOLDOWN)).await;
        self.is_processing.store(false, Ordering::Release);
    }
}

impl ChatbotService {
    /// Checks the permissions, the channel, the author, etc.
    fn should_handle(&self, msg: &Message, app: &Arc<App>) -> bool {
        if msg.channel_id.get() != Config::get().chatbot_channel_id {
            return false;
        }
        if msg.author.bot || msg.webhook_id.is_some() {
            return false;
        }
        if !self.check_channel_permissions(app, msg.channel_id) {
            return false;
        }
        match msg.kind {
            MessageType::Regular => true,
            MessageType::Reply => msg
                .mentions
                .iter()
                .any(|u| u.id.cast() == app.application_id),
            _ => false,
        }
    }

    /// Checks if the bot can send the chatbot messages.
    fn check_channel_permissions(&self, app: &Arc<App>, channel_id: Id<ChannelMarker>) -> bool {
        match app
            .discord_cache
            .permissions()
            .in_channel(app.application_id.cast(), channel_id)
        {
            Ok(permissions) => permissions.contains(
                Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS,
            ),
            Err(e) => {
                // usually happens if the user, channel, or guild isn't in the cache
                warn!("Could not calculate permissions: {}", e);
                false
            }
        }
    }
}

impl ChatbotService {
    async fn generate_and_reply(&self, msg: &Message, app: &Arc<App>) -> anyhow::Result<()> {
        // History

        let mut history_lines = Vec::new();
        let mut history_length = 0;

        if let Some(message_ids) = app.discord_cache.channel_messages(msg.channel_id) {
            // newest messages first
            for &m_id in message_ids.iter().rev() {
                if m_id == msg.id {
                    continue;
                }

                // get the message from the cache
                if let Some(cached_msg) = app.discord_cache.message(m_id) {
                    let content = cached_msg.content();

                    if content.is_empty() {
                        continue;
                    }

                    // The history is limited to 1500 characters.
                    // However, we include at least one message (history_length > 0)
                    if history_length > 0 && (history_length + content.len() >= 1500) {
                        break;
                    }

                    // get the author username from the user cache
                    let username = app
                        .discord_cache
                        .user(cached_msg.author())
                        .map(|u| u.name.clone())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let line = format!("@{}: {}\n", username, content);

                    history_lines.push(line);
                    history_length += content.len();

                    // Limit to 10 messages maximum
                    if history_lines.len() >= 10 {
                        break;
                    }
                }
            }
        }

        let history_text = history_lines.join("");

        // Docs Sitemap

        let docs_list = match app.services.documentation.get_documentation_sitemap().await {
            Ok(pages) => pages
                .iter()
                .filter(|p| p.lang == "fr")
                .map(|p| {
                    format!(
                        "- [{}]({}): {}",
                        p.name,
                        p.url.trim_start_matches(Config::DOCS_URL),
                        p.description
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n"),
            Err(_) => String::new(),
        };

        let system_prompt = build_system_prompt(&docs_list, &history_text);
        let source_page_data = Arc::new(Mutex::new(None));

        let tool = FetchDocsTool {
            app: app.clone(),
            channel_id: msg.channel_id,
            message_id: msg.id,
            source_page: source_page_data.clone(),
            has_been_called: AtomicBool::new(false),
        };

        let agent = self
            .client
            .agent(GEMINI_2_5_FLASH)
            .preamble(&system_prompt)
            .tool(tool)
            .build();

        let user_text = format!("@{}: {}", msg.author.name, msg.content);

        let response = match agent.prompt(user_text.as_str()).await {
            Ok(r) => r,
            Err(e) => return self.handle_api_error(e, msg, app).await,
        };

        let text = strip_self_mention(&response);
        let source_info = source_page_data.lock().await.clone();

        self.send_reply(msg, &text, app, source_info).await;
        Ok(())
    }

    async fn handle_api_error(
        &self,
        err: rig_core::completion::PromptError,
        msg: &Message,
        app: &Arc<App>,
    ) -> anyhow::Result<()> {
        let err_str = err.to_string();

        if err_str.contains("503") {
            let _ = app
                .discord
                .create_reaction(
                    msg.channel_id,
                    msg.id,
                    &RequestReactionType::Unicode { name: "⏰" },
                )
                .await;
        } else if err_str.contains("429") {
            let now = now_secs();
            let mut last = self.last_ratelimit_alert_at.lock().await;
            if now - *last > 60 {
                *last = now;
                drop(last);
                self.send_reply(msg, "*The chatbot has reached its message limit. Please wait a moment or contact support instead.*", app, None).await;
            }
        } else {
            return Err(anyhow::anyhow!("Chatbot API error: {err_str}"));
        }

        Ok(())
    }

    async fn send_reply(
        &self,
        msg: &Message,
        text: &str,
        app: &Arc<App>,
        source_page: Option<SourcePage>,
    ) {
        let mut components: Vec<Component> = Vec::new();
        let mut target_message_id = None;

        // If the chatbot has read the docs, include the source page and extract the message ID to edit
        if let Some(SourcePage {
            name,
            url,
            loading_msg_id,
        }) = source_page
        {
            target_message_id = Some(loading_msg_id);
            components.push(
                ActionRowBuilder::new()
                    .add_button(|b| {
                        b.link(url)
                            .unicode_emoji("🔗")
                            .label(format!("Source - {}", name))
                    })
                    .build()
                    .into(),
            );
        }

        // components and content based on character length
        let (content, final_components, flags) = if text.chars().count() > 2000 {
            let truncated = truncate_to_chars(text, 4000);
            let mut all_components = components;
            all_components.insert(
                0,
                TextDisplayBuilder::new().content(truncated).build().into(),
            );
            (None, all_components, Some(MessageFlags::IS_COMPONENTS_V2))
        } else {
            (Some(text), components, None)
        };
        let mentions = AllowedMentions::default();

        // If a message was sent by the tool, edit. Otherwise, create a new reply
        let result = match target_message_id {
            Some(loading_id) => {
                let mut updater = app.discord.update_message(msg.channel_id, loading_id);

                // Clear the original loading embed
                updater = updater.allowed_mentions(Some(&mentions));
                updater = updater.embeds(Some(&[]));
                updater = updater.components(Some(&final_components));

                if let Some(cnt) = content {
                    updater = updater.content(Some(cnt));
                }
                if let Some(flg) = flags {
                    updater = updater.flags(flg);
                }

                updater.await
            }
            None => {
                let mut builder = app
                    .discord
                    .create_message(msg.channel_id)
                    .reply(msg.id)
                    .components(&final_components)
                    .allowed_mentions(Some(&mentions));

                if let Some(cnt) = content {
                    builder = builder.content(cnt);
                }
                if let Some(flg) = flags {
                    builder = builder.flags(flg);
                }

                builder.await
            }
        };

        if let Err(e) = result {
            warn!("Failed to send/update chatbot reply: {e}");
        }
    }
}
