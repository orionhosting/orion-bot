use crate::commands::prelude::*;
use orion_api::OrionError;

/// View your Orion account.
pub struct AccountCommand;

command_meta! {
    META = CommandMeta::builder("account", t!("commands.account.description"))
        .description_localizations(localize_key(|locale| t!(locale, "commands.account.description")))
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
impl Command<App> for AccountCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    #[t_ns(namespace = "commands.account")]
    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let user = ctx.app.orion_api.get_user(ctx.user_id().to_string()).await;

        match user {
            Ok(user) => {
                // The user has an account, build the container

                let container = ContainerBuilder::new()
                    .accent_color(Palette::PRIMARY.int)
                    .add_text_display(|d| {
                        d.content(format!(
                            "## {} Orion - {}",
                            Emojis::LOGO,
                            t!(ctx, "card.title.your_account")
                        ))
                    })
                    .add_text_display(|d| {
                        d.content(t!(ctx, "card.content.credits", amount = user.credits))
                    })
                    .add_action_row(|row| {
                        row.add_button(|b| {
                            b.link(Config::DASHBOARD_URL)
                                .label(t!(ctx, "/common.dashboard"))
                        })
                        .add_button(|b| b.link(Config::PANEL_URL).label(t!(ctx, "/common.panel")))
                    });

                ctx.reply(Reply::new().components_v2(container.into()))
                    .await?;
            }
            Err(OrionError::Api { status: 404, .. }) => {
                // 404 means the user does not have an account yet

                let container = ContainerBuilder::new()
                    .accent_color(Palette::PRIMARY.int)
                    .add_text_display(|d| d.content(format!("## {} Orion", Emojis::LOGO)))
                    .add_text_display(|d| d.content(t!(ctx, "no_account.content.guide")))
                    .add_action_row(|row| {
                        row.add_button(|b| {
                            b.link(Config::DASHBOARD_URL)
                                .label(t!(ctx, "no_account.buttons.create_account"))
                        })
                    });

                ctx.reply(Reply::new().components_v2(container.into()))
                    .await?;
            }
            Err(e) => {
                return Err(anyhow::anyhow!(e).into());
            }
        }

        Ok(())
    }
}
