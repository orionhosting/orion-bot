use std::time::Instant;

use crate::commands::prelude::*;

/// Check the latencies of the bot and the Orion services.
pub struct PingCommand;

command_meta! {
    META = CommandMeta::builder("ping", t!("commands.ping.description"))
        .description_localizations(localize_key(|locale| t!(locale, "commands.ping.description")))
        .category("Informations")
        .installations(&[
            ApplicationIntegrationType::GuildInstall,
            ApplicationIntegrationType::UserInstall,
        ])
        .contexts(&[
            InteractionContextType::Guild,
            InteractionContextType::BotDm,
            InteractionContextType::PrivateChannel,
        ])
}

#[async_trait]
impl Command<App> for PingCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    #[t_ns(namespace = "commands.ping")]
    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let now = Instant::now();
        ctx.reply(
            Reply::new().components_v2(
                ContainerBuilder::new()
                    .add_text_display(|d| {
                        d.content(format!("{} {}", Emojis::LOADER_GREEN, t!(ctx, "pinging")))
                    })
                    .into(),
            ),
        )
        .await?;

        // discord latencies
        let discord_ping = now.elapsed().as_millis();
        let discord_ws_ping = ctx.app.gateway_latency().await;

        // services latencies
        let domain_ping = ctx.app.services.status.ping_url(Config::DOMAIN_URL).await;
        let panel_ping = ctx.app.services.status.ping_url(Config::PANEL_URL).await;
        let docs_ping = ctx.app.services.status.ping_url(Config::DOCS_URL).await;
        let api_ping = ctx.app.services.status.ping_url(Config::API_URL).await;
        let fr1_ping = ctx
            .app
            .services
            .status
            .ping_url(&format!("http://fr1.{}:8080", Config::DOMAIN))
            .await;

        // container
        let offline_label = t!(ctx, "/common.offline").to_lowercase();
        let fmt = |p: Option<u64>| {
            p.map(|ms| format!("{ms}ms"))
                .unwrap_or_else(|| offline_label.clone())
        };
        let icon = |p: Option<u64>| {
            if p.is_some() {
                Emojis::ONLINE
            } else {
                Emojis::DND
            }
        };

        let container = ContainerBuilder::new()
            .add_text_display(|d| {
                d.content(format!(
                    "## {}  {}",
                    Emojis::LOGO,
                    t!(ctx, "services.title")
                ))
            })
            .add_text_display(|d| {
                d.content(format!(
                    "> {} Orion Website: `{}`\n\
                     > {} Orion Panel: `{}`\n\
                     > {} Orion Docs: `{}`\n\
                     > {} Orion API: `{}`\n\
                     > {} Node fr1: `{}`",
                    icon(domain_ping),
                    fmt(domain_ping),
                    icon(panel_ping),
                    fmt(panel_ping),
                    icon(docs_ping),
                    fmt(docs_ping),
                    icon(api_ping),
                    fmt(api_ping),
                    icon(fr1_ping),
                    fmt(fr1_ping),
                ))
            })
            .add_text_display(|d| d.content(t!(ctx, "services.more")))
            .add_action_row(|row| {
                row.add_button(|b| {
                    b.link(Config::STATUS_URL)
                        .custom_emoji(Emojis::PROPERTY_EID, false)
                        .label(t!(ctx, "/common.status_page"))
                })
            })
            .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
            .add_text_display(|d| {
                d.content(format!("## {}  {}", Emojis::DISCORD, t!(ctx, "bot.title")))
            })
            .add_text_display(|d| {
                let gateway_ms = discord_ws_ping
                    .map(|d| d.as_millis().to_string())
                    .unwrap_or("-1".into());
                d.content(format!(
                    "> Discord Gateway: `{gateway_ms}ms`\n> Discord API: `{discord_ping}ms`"
                ))
            });

        ctx.edit_reply(Reply::new().components_v2(container.into()))
            .await?;

        Ok(())
    }
}
