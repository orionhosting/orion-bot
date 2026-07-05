use crate::commands::prelude::*;

/// Get the link for a documentation page.
pub struct DocsCommand;

command_meta! {
    META = CommandMeta::builder("docs", t!("commands.docs.description"))
        .description_localizations(localize_key(|locale| t!(locale, "commands.docs.description")))
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
        .options([
            CommandOptionBuilder::string("page", t!("commands.docs.options.page.description"))
                .description_localizations(localize_key(|locale| t!(locale, "commands.docs.options.page.description")))
                .autocomplete(true)
        ])
}

#[async_trait]
impl Command<App> for DocsCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    #[t_ns(namespace = "commands.docs")]
    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        // Get the option value, or the docs home page if none provided
        let page_choice = ctx
            .get_string_option("page")
            .map(String::from)
            .unwrap_or_else(|| {
                format!(
                    "{}/{}",
                    Config::DOCS_URL,
                    get_docs_locale(&ctx.i18n_locale())
                )
            });

        // Get the docs sitemap
        let sitemap = ctx
            .app
            .services
            .documentation
            .get_documentation_sitemap()
            .await?;

        match sitemap.iter().find(|p| p.url == page_choice) {
            Some(p) => {
                // page found, give the link
                ctx.reply(Reply::new().content(format!(
                    "**{} [{}]({}) **\n{}",
                    Emojis::PROPERTY,
                    p.name,
                    p.url,
                    p.description
                )))
                .await?;
            }
            None => {
                // page not found (invalid url)
                ctx.reply(
                    Reply::new()
                        .content(format!("{} {}", Emojis::WARN, t!(ctx, "err_invalid_page")))
                        .ephemeral(true),
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn handle_autocomplete(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let Some(focused) = ctx.get_focused_string().map(|s| s.to_lowercase()) else {
            // Invalid value
            return Ok(());
        };

        // Get the docs sitemap
        let sitemap = ctx
            .app
            .services
            .documentation
            .get_documentation_sitemap()
            .await?;

        let choices: Vec<CommandOptionChoice> = sitemap
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&focused)
                    && !(p.name.contains("/") && p.name.len() == 10) // changelogs
            })
            .take(25)
            .map(|p| CommandOptionChoice {
                name: format!("{} - {}", p.lang, p.name),
                name_localizations: None,
                value: CommandOptionChoiceValue::String(p.url.clone()),
            })
            .collect();

        ctx.respond(choices).await?;
        Ok(())
    }
}
