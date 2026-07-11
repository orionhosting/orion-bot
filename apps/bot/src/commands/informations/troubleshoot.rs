use crate::commands::prelude::*;

/// help user to find errors in his server
pub struct TroubleshootCommand;

command_meta! {
    META = CommandMeta::builder("troubleshoot", t!("commands.troubleshoot.description"))
        .description_localizations(localize_key(|locale| t!(locale, "commands.troubleshoot.description")))
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
impl Command<App> for TroubleshootCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    #[t_ns(namespace = "commands.troubleshoot")]
    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        ctx.reply(
            Reply::new().components_v2(
                ContainerBuilder::new()
                    .add_text_display(|d| {
                        d.content(format!("oeoeoe"))
                    })
                    .into(),
            ),
        )
        .await?;

        Ok(())
    }
}
