use crate::commands::prelude::*;
use orion_api::OrionError;

/// View a user account.
pub struct UserCommand;

command_meta! {
    META = CommandMeta::builder("user", "Get a user's Orion account info".into())
        .category("Private")
        .owner_only(true)
        .guilds([Config::get().support_guild_id])
        .options([
            CommandOptionBuilder::user("user", "The user to look up")
                .required(true)
        ])
}

#[async_trait]
impl Command<App> for UserCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let user_id = ctx.require_user_option("user")?;

        ctx.defer_reply(true).await?;

        // get user from Orion API
        let user = ctx.app.orion_api.get_user(user_id.get().to_string()).await;

        match user {
            Ok(user) => {
                // The user has an account, build the container

                let container = ContainerBuilder::new()
                    .accent_color(Palette::PRIMARY.int)
                    .add_text_display(|d| d.content(format!("## {} Orion Account", Emojis::LOGO)))
                    .add_text_display(|d| {
                        d.content(format!(
                            "\n> Discord ID: `{}`\
                            \n> Orion ID: `{}`\
                            \n> Panel ID: `{}`",
                            user.discord_id, user.id, user.panel_id
                        ))
                    })
                    .add_text_display(|d| {
                        d.content(format!(
                            "\n> Username: `{}`\
                            \n> Credits: `{}`",
                            user.username, user.credits,
                        ))
                    })
                    .add_text_display(|d| {
                        d.content(format!(
                            "\n> Referral code: `{}`\
                            \n> Referral usage: `{}`\
                            \n> Referral reward: `{}` credits",
                            user.referral_code, user.referral_usage, user.referral_gains,
                        ))
                    })
                    .add_text_display(|d| {
                        d.content(format!(
                            "\n> Last dashboard login: <t:{}:R>",
                            user.last_login_at / 1000,
                        ))
                    });

                ctx.edit_reply(Reply::new().components_v2(container.into()))
                    .await?;
            }
            Err(OrionError::Api { status, .. }) if status == 404 => {
                // 404 means the user does not have an account

                ctx.edit_reply(Reply::new().content("This user does not have an Orion account"))
                    .await?;
            }
            Err(e) => {
                return Err(anyhow::anyhow!(e).into());
            }
        }

        Ok(())
    }
}
