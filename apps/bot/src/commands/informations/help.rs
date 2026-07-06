use crate::commands::prelude::*;

/// Help command.
pub struct HelpCommand;

command_meta! {
    META = CommandMeta::builder("help", t!("commands.help.description"))
        .description_localizations(localize_key(|locale| t!(locale, "commands.help.description")))
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
impl Command<App> for HelpCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    #[t_ns(namespace = "commands.help")]
    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let container = ContainerBuilder::new()
            .accent_color(Palette::PRIMARY.int)
            .add_text_display(|d| {
                d.content(format!("## {} Orion - {}", Emojis::LOGO, t!(ctx, "title")))
            })
            .add_media_gallery(|m| m.add_item_from_url(Config::BANNER_URL))
            .add_text_display(|d| {
                d.content(format!(
                    "### {}\n> {}",
                    t!(ctx, "hosting.title"),
                    t!(ctx, "hosting.description")
                ))
            })
            .add_action_row(|row| {
                row.add_button(|btn| {
                    btn.link(Config::DASHBOARD_URL)
                        .custom_emoji(Emojis::LOGO_EID, false)
                        .label(t!(ctx, "hosting.buttons.create_free_server"))
                })
            })
            .add_separator(|s| s.divider(false).spacing(SeparatorSpacingSize::Small))
            .add_text_display(|d| {
                d.content(format!(
                    "### {}\n> {}: {}\n> {}: {}\n> {}: {}\n> {}: {}\n> {} : {}",
                    t!(ctx, "services.title"),
                    t!(ctx, "services.website"),
                    Config::DOMAIN_URL,
                    t!(ctx, "/common.dashboard"),
                    Config::DASHBOARD_URL,
                    t!(ctx, "/common.panel"),
                    Config::PANEL_URL,
                    t!(ctx, "/common.documentation"),
                    Config::DOCS_URL,
                    t!(ctx, "/common.status"),
                    Config::STATUS_URL,
                ))
            })
            .add_separator(|s| s.divider(false).spacing(SeparatorSpacingSize::Small))
            .add_text_display(|d| {
                d.content(format!(
                    "### GitHubs\n> Orion Hosting: {}\n> Orion Bot: {}\n> Orion CLI: {}",
                    Config::ORION_GITHUB_URL,
                    Config::BOT_GITHUB_URL,
                    Config::CLI_GITHUB_URL,
                ))
            })
            .add_action_row(|row| {
                row.add_button(|btn| {
                    btn.link(Config::SUPPORT_INVITE)
                        .custom_emoji(Emojis::DISCORD_EID, false)
                        .label(t!(ctx, "services.buttons.join_support"))
                })
                .add_button(|btn| {
                    btn.link(Config::BOT_INVITE)
                        .custom_emoji(Emojis::LOGO_EID, false)
                        .label(t!(ctx, "services.buttons.add_bot"))
                })
            });

        ctx.reply(Reply::new().components_v2(container.into()))
            .await?;

        Ok(())
    }
}
