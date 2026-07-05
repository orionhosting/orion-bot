use crate::commands::prelude::*;
use orion_api::{CreateCreditTransactionBody, CreditTransactionType, OrionError, PatchStateBody};

/// Config command.
pub struct ConfigCommand;

command_meta! {
    META = CommandMeta::builder("config", "Configure Orion".into())
        .category("Private")
        .owner_only(true)
        .guilds([Config::get().support_guild_id])
        .options([
            CommandOptionBuilder::subcommand("get-available-servers", "Get the available servers count"),
            CommandOptionBuilder::subcommand("set-available-servers", "Set the available servers count")
                .options([
                    CommandOptionBuilder::integer("amount", "The quantity")
                        .min_value_int(1)
                        .max_value_int(10000)
                        .required(true)
                ]),
            CommandOptionBuilder::subcommand("credits-add", "Add credits")
                .options([
                    CommandOptionBuilder::user("user", "The user")
                        .required(true),
                    CommandOptionBuilder::integer("type", "The type")
                        .add_choice_int("The user won a giveaway", CreditTransactionType::Giveaway as i64)
                        .add_choice_int("Other", CreditTransactionType::Custom as i64)
                        .required(true),
                    CommandOptionBuilder::integer("amount", "The quantity")
                        .min_value_int(1)
                        .max_value_int(10000)
                        .required(true),
                    CommandOptionBuilder::string("reason", "Only needed when the type is custom")
                        .max_length(500)
                ])
       ])
}

#[async_trait]
impl Command<App> for ConfigCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        ctx.defer_reply(true).await?;

        match ctx.get_subcommand() {
            Some("get-available-servers") => {
                let state = ctx
                    .app
                    .orion_api
                    .get_state()
                    .await
                    .map_err(|e| CommandError::Anyhow(e.into()))?;

                ctx.edit_reply(Reply::new().content(format!(
                    "Available free servers: {}",
                    state.available_free_servers
                )))
                .await?;
            }
            Some("set-available-servers") => {
                let amount = ctx.require_integer_option("amount")?;

                ctx.app
                    .orion_api
                    .patch_state(PatchStateBody {
                        available_free_servers: Some(amount as u32),
                        maintenance_mode: None,
                    })
                    .await
                    .map_err(|e| CommandError::Anyhow(e.into()))?;

                ctx.edit_reply(
                    Reply::new().content(format!("Amount updated. New value: {}", amount)),
                )
                .await?;
            }
            Some("credits-add") => {
                let user = ctx.require_user_option("user")?;
                let kind = ctx.require_integer_option("type")?;
                let amount = ctx.require_integer_option("amount")?;
                let reason = ctx.get_string_option("reason").map(|v| v.trim());

                let kind = CreditTransactionType::try_from(kind as u8)
                    .unwrap_or(CreditTransactionType::Custom);

                match kind {
                    CreditTransactionType::Custom => {
                        if reason.is_none() {
                            ctx.edit_reply(Reply::new().content(
                                "You need to specify the 'reason'. It will be shown to the user.",
                            ))
                            .await?;
                        }
                    }
                    _ => {
                        if reason.is_some() {
                            ctx.edit_reply(Reply::new().content(
                                "You cannot specify a reason when the type is not custom.",
                            ))
                            .await?;
                        }
                    }
                }

                let result = ctx
                    .app
                    .orion_api
                    .create_credit_transaction(
                        user.get().to_string(),
                        CreateCreditTransactionBody {
                            kind,
                            amount,
                            reason: reason.map(String::from),
                        },
                    )
                    .await;

                match result {
                    Ok(result) => {
                        ctx.edit_reply(Reply::new().content(format!(
                            "Credits added, the user has now {} credits. Transaction ID: {}",
                            result.user_credits, result.transaction_id
                        )))
                        .await?;
                    }
                    Err(OrionError::Api { status, message }) => {
                        ctx.edit_reply(Reply::new().content(format!(
                            "Failed to add credits: {} (status: {})",
                            message.unwrap_or("no message".into()),
                            status
                        )))
                        .await?;
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!(e).into());
                    }
                };
            }
            _ => {}
        };

        Ok(())
    }
}
