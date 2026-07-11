use crate::commands::prelude::*;

/// help user to find errors in his server
pub struct TroubleshootCommand;

use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::application::interaction::InteractionData;

use std::fs;
use serde_json::Value;

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

use anyhow::{Result, Context};

fn build_troubleshoot_menu(locale: &str, path: &str) -> Result<Reply> {
    let mut file_path = format!("apps/bot/locales/{locale}/troubleshoot.json");
    if !std::path::Path::new(&file_path).exists() {
        file_path = "apps/bot/locales/en/troubleshoot.json".to_string();
    }

    let file_content = fs::read_to_string(&file_path).context(format!("Failed to read {}", file_path))?;
    let mut current_node: Value = serde_json::from_str(&file_content).context("Failed to parse troubleshoot.json")?;

    for part in path.split('.') {
        current_node = current_node.get(part)
            .ok_or_else(|| anyhow::anyhow!("Key '{}' not found in troubleshoot.json", part))?
            .clone();
    }

    let title = current_node
        .get("title")
        .or_else(|| current_node.get("label"))
        .and_then(Value::as_str)
        .unwrap_or("Troubleshoot")
        .to_string();
        
    let message = current_node
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let mut container = ContainerBuilder::new()
        .accent_color(Palette::PRIMARY.int)
        .add_text_display(|d| d.content(format!("**{}**\n{}", title, message)));

    if let Some(options) = current_node.get("options").and_then(Value::as_object) {
        let mut select_options = Vec::new();
        for (key, option_node) in options {
            let label = option_node.get("label").and_then(Value::as_str).unwrap_or(key).to_string();
            
            let has_next = option_node.get("next").map(|v| !v.is_null()).unwrap_or(false);
            let next_path = if has_next {
                format!("{}.options.{}.next", path, key)
            } else {
                format!("{}.options.{}", path, key)
            };

            select_options.push((next_path, label));
        }

        container = container.add_action_row(|row| {
            row.add_select_menu(|menu| {
                let mut menu = menu.custom_id("troubleshoot-select");
                for (path_value, label) in select_options {
                    menu = menu.add_option(|opt| {
                        opt.label(label)
                            .value(path_value)
                    });
                }
                menu
            })
        });
    }

    Ok(Reply::new().components_v2(container.into()).ephemeral(true))
}

#[async_trait]
impl Command<App> for TroubleshootCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let locale = ctx.i18n_locale().to_string();
        println!("{}", locale);
        match build_troubleshoot_menu(&locale, "home") {
            Ok(reply) => {
                ctx.reply(reply).await?;
            }
            Err(err) => {
                ctx.say(format!("Error loading troubleshoot menu: {err}")).await?;
            }
        }
        Ok(())
    }

    async fn handle_component(&self, ctx: &CommandContext<'_>) -> CommandResult {
        if let Some(InteractionData::MessageComponent(data)) = ctx.interaction.data.as_ref() {
            let path = data.values.first().map(String::as_str).unwrap_or("home");
            
            let locale = ctx.i18n_locale().to_string();
            match build_troubleshoot_menu(&locale, path) {
                Ok(reply) => {
                    ctx.update_message(reply).await?;
                }
                Err(err) => {
                    ctx.reply(Reply::new().content(format!("Error loading troubleshoot menu: {err}")).ephemeral(true)).await?;
                }
            }
        }
        Ok(())
    }
}
