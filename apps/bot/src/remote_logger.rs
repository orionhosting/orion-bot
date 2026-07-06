use std::sync::Arc;

use tracing::info;
use twilight_http::Client as HttpClient;
use twilight_model::{
    channel::message::Embed,
    id::{Id, marker::WebhookMarker},
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::config::{Config, Palette};

#[derive(Debug)]
pub enum LogMessage {
    Text(String),
    Embeds(Vec<Embed>),
}

pub struct RemoteLogger {
    http: Arc<HttpClient>,
    webhook_id: Id<WebhookMarker>,
    webhook_token: String,
}

impl RemoteLogger {
    pub fn new(http: Arc<HttpClient>) -> anyhow::Result<Self> {
        let cfg = Config::get();

        Ok(Self {
            http,
            webhook_id: Id::new(cfg.logs_webhook_id),
            webhook_token: cfg.logs_webhook_token.clone(),
        })
    }

    pub async fn send_log(&self, message: impl Into<String>) -> anyhow::Result<()> {
        let content = format!("`[LOG]` {}", message.into());
        self.execute_webhook(LogMessage::Text(content)).await
    }

    pub async fn send_warning(&self, warning: &str) -> anyhow::Result<()> {
        let embed = EmbedBuilder::new()
            .color(Palette::ORANGE.int)
            .title("Warning")
            .description(truncate(warning, 4096))
            .build();

        self.execute_webhook(LogMessage::Embeds(vec![embed])).await
    }

    async fn execute_webhook(&self, msg: LogMessage) -> anyhow::Result<()> {
        if cfg!(debug_assertions) {
            //Skip sending if not in production
            info!(?msg);
            return Ok(());
        }

        let mut builder = self
            .http
            .execute_webhook(self.webhook_id, &self.webhook_token);

        match msg {
            LogMessage::Text(content) => {
                let content = truncate(&content, 2000);
                builder = builder.content(&content);
                builder.await?
            }
            LogMessage::Embeds(embeds) => {
                builder = builder.embeds(&embeds);
                builder.await?
            }
        };

        Ok(())
    }
}

fn truncate(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}
