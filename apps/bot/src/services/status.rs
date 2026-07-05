use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use builders::ContainerBuilder;
use tracing::{info, warn};
use twilight_model::{
    channel::message::{
        MessageFlags,
        component::{Component, Container, SeparatorSpacingSize},
    },
    id::{Id, marker::ChannelMarker},
};

use crate::{
    config::{Config, Emojis, Palette},
    services::ServiceContext,
};

const UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// The status service.
///
/// - Ping services
/// - Auto-update the status message in the status channel
pub struct StatusService {
    ctx: ServiceContext,
}

impl StatusService {
    pub(super) fn new(ctx: ServiceContext) -> Self {
        Self { ctx }
    }

    pub(super) fn on_ready(&self) {
        info!("StatusService ready");

        let svc = StatusService::new(self.ctx.clone());
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(UPDATE_INTERVAL);
            interval.tick().await;
            loop {
                interval.tick().await;
                if let Err(e) = svc.tick().await {
                    warn!("Status tick error: {e}");
                }
            }
        });
    }

    /// Ping a URL and return latency in ms.
    pub async fn ping_url(&self, url: &str) -> Option<u64> {
        let start = Instant::now();
        let result =
            tokio::time::timeout(Duration::from_secs(5), self.ctx.http.head(url).send()).await;

        match result {
            Ok(Ok(_)) => Some(start.elapsed().as_millis() as u64),
            Ok(Err(e)) => {
                // "Connection refused" / "other side closed" still counts as reachable
                let msg = e.to_string();
                let reachable = msg.contains("connection refused")
                    || msg.contains("other side closed")
                    || msg.contains("Connection reset");
                reachable.then(|| start.elapsed().as_millis() as u64)
            }
            Err(_) => None, // timeout
        }
    }

    /// This function is called in an interval.
    ///
    /// Edit the status message in the status channel, or
    /// create it if it doesn't exist.
    async fn tick(&self) -> Result<()> {
        let fr1_url = format!("http://fr1.{}:8080", Config::DOMAIN);
        let (domain_ping, panel_ping, docs_ping, bot_api_ping, api_ping, fr1_ping) = tokio::join!(
            self.ping_url(Config::DOMAIN_URL),
            self.ping_url(Config::PANEL_URL),
            self.ping_url(Config::DOCS_URL),
            self.ping_url(Config::BOT_API_URL),
            self.ping_url(Config::API_URL),
            self.ping_url(&fr1_url),
        );

        let components = vec![Component::Container(build_status_container(
            domain_ping,
            panel_ping,
            docs_ping,
            bot_api_ping,
            api_ping,
            fr1_ping,
        ))];

        let channel_id: Id<ChannelMarker> = Id::new(Config::get().status_channel_id);

        // Find the previous status message (it's the only message in the
        // channel with the V2 flag set) and edit it
        let messages = self
            .ctx
            .discord
            .channel_messages(channel_id)
            .limit(15)
            .await?
            .models()
            .await?;

        let existing = messages.into_iter().find(|m| {
            m.author.id.get() == self.ctx.application_id
                && m.flags
                    .unwrap_or_else(MessageFlags::empty)
                    .contains(MessageFlags::IS_COMPONENTS_V2)
        });

        match existing {
            Some(msg) => {
                self.ctx
                    .discord
                    .update_message(channel_id, msg.id)
                    .components(Some(&components))
                    .await?;
            }
            None => {
                self.ctx
                    .discord
                    .create_message(channel_id)
                    .components(&components)
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .await?;
            }
        }

        Ok(())
    }
}

fn build_status_container(
    domain: Option<u64>,
    panel: Option<u64>,
    docs: Option<u64>,
    bot_api: Option<u64>,
    api: Option<u64>,
    fr1: Option<u64>,
) -> Container {
    let fmt = |ms: Option<u64>| {
        ms.map(|ms| format!("{ms}ms"))
            .unwrap_or_else(|| "offline".into())
    };
    let icon = |ms: Option<u64>| {
        if ms.is_some() {
            Emojis::ONLINE
        } else {
            Emojis::DND
        }
    };

    let next_update = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + UPDATE_INTERVAL.as_secs();

    ContainerBuilder::new()
        .accent_color(Palette::PRIMARY.int)
        .add_text_display(|d| d.content(format!("## {} Services Status", Emojis::LOGO)))
        .add_media_gallery(|m| m.add_item_from_url(Config::BANNER_URL))
        .add_text_display(|d| {
            d.content(format!(
                "> {} Orion Website: `{}`\n\
                 > {} Orion Panel: `{}`\n\
                 > {} Orion Docs: `{}`\n\
                 > {} Orion Bot: `{}`\n\
                 > {} Orion API: `{}`\n\
                 > {} Node fr1: `{}`",
                icon(domain),
                fmt(domain),
                icon(panel),
                fmt(panel),
                icon(docs),
                fmt(docs),
                icon(bot_api),
                fmt(bot_api),
                icon(api),
                fmt(api),
                icon(fr1),
                fmt(fr1),
            ))
        })
        .add_text_display(|d| d.content(format!("> Next update <t:{next_update}:R>")))
        .add_separator(|s| s.divider(false).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| d.content("Use the status page for more details about our services."))
        .add_action_row(|row| {
            row.add_button(|b| {
                b.link(Config::STATUS_URL)
                    .label("Status Page")
                    .custom_emoji(Emojis::PROPERTY_EID, false)
            })
        })
        .build()
}
