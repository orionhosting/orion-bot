use twilight_model::channel::message::{AllowedMentions, Component, MessageFlags};

use crate::commands::prelude::*;

/// Send a panel.
pub struct PanelsCommand;

command_meta! {
    META = CommandMeta::builder("panels", "Send custom panels".into())
        .category("Private")
        .owner_only(true)
        .guilds([Config::get().support_guild_id])
        .options([
            CommandOptionBuilder::string("name", "The panel name")
                .required(true)
        ])
}

#[async_trait]
impl Command<App> for PanelsCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        let Some(channel) = &ctx.interaction.channel else {
            unreachable!();
        };

        ctx.defer_reply(true).await?;

        let components = match ctx.require_string_option("name")? {
            "links" => build_links_components(),
            "rules" => build_rules_components(),
            _ => {
                ctx.edit_reply(Reply::new().content("Invalid name!"))
                    .await?;
                return Ok(());
            }
        };

        ctx.http
            .create_message(channel.id)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .allowed_mentions(Some(&AllowedMentions::default()))
            .components(&components)
            .await?;

        ctx.edit_reply(Reply::new().content("Sent!")).await?;

        Ok(())
    }
}

fn build_links_components() -> Vec<Component> {
    let container = ContainerBuilder::new()
        .accent_color(Palette::PRIMARY.int)
        .add_text_display(|d| d.content(format!("## {} Orion - Links", Emojis::LOGO)))
        .add_media_gallery(|m| m.add_item_from_url(Config::BANNER_URL))
        .add_section(|s| {
            s.add_text_display(|d| {
                d.content("> Explore our hosting services or login in the dashboard.")
            })
            .set_button_accessory(|btn| {
                btn.link(Config::DOMAIN_URL)
                    .custom_emoji(Emojis::LOGO_EID, false)
                    .label("Website / Dashboard")
            })
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Small))
        .add_section(|s| {
            s.add_text_display(|d| d.content("> Use the panel to manage your servers."))
                .set_button_accessory(|btn| {
                    btn.link(Config::PANEL_URL)
                        .unicode_emoji("🔗")
                        .label("Panel")
                })
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Small))
        .add_section(|s| {
            s.add_text_display(|d| {
                d.content("> Read the documentation to learn how to use our hosting.")
            })
            .set_button_accessory(|btn| {
                btn.link(Config::DOCS_URL)
                    .unicode_emoji("📚")
                    .label("Documentation")
            })
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Small))
        .add_section(|s| {
            s.add_text_display(|d| d.content("> Get the status of our services in real-time."))
                .set_button_accessory(|btn| {
                    btn.link(Config::STATUS_URL)
                        .custom_emoji(Emojis::PROPERTY_EID, false)
                        .label("Status")
                })
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Small))
        .add_section(|s| {
            s.add_text_display(|d| d.content("> Need help? Open a ticket!"))
                .set_button_accessory(|btn| {
                    btn.link(Config::TICKETS_PANEL_URL)
                        .custom_emoji(Emojis::DISCORD_EID, false)
                        .label("Open a ticket")
                })
        });

    let legal = ContainerBuilder::new()
        .accent_color(Palette::PRIMARY.int)
        .add_text_display(|d| d.content(format!("## {} Legal Links", Emojis::LOGO)))
        .add_text_display(|d| {
            d.content(format!(
                "> Terms of Service: {}/terms\n> Privacy Policy: {}/privacy",
                Config::DOMAIN_URL,
                Config::DOMAIN_URL
            ))
        });

    vec![container.build().into(), legal.build().into()]
}

fn build_rules_components() -> Vec<Component> {
    let container = ContainerBuilder::new()
        .accent_color(Palette::PRIMARY.int)
        .add_text_display(|d| d.content(format!("## {} Orion - Rules", Emojis::LOGO)))
        .add_media_gallery(|m| m.add_item_from_url(Config::BANNER_URL))
        .add_text_display(|d| {
            d.content(
                "## Civilized behavior\n\
                Treat other members with respect, even if you disagree with them.\n\
                Hate speech, harassment and discriminatory language are strictly forbidden.\n\
                Please respect the decisions of moderators and administrators.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## Privacy\n\
                Private messages (PMs) may not be used for advertising or spamming.\n\
                Invitations to other servers or the addition of bots are strictly forbidden.\n\
                Members' personal information must not be shared without their consent.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## Security\n\
                Passwords and other sensitive information must not be shared.\n\
                Report any suspicious behavior or fraudulent activity to the moderators.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## Other\n\
                Links to malicious websites or explicit content are prohibited.\n\
                Excessive use of capital letters, spam or animated emotes is prohibited.\n\
                Members may not impersonate others.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## Civilized behavior\n\
                Treat other members with respect, even if you disagree with them.\n\
                Hate speech, harassment and discriminatory language are strictly forbidden.\n\
                Personal conflicts should be resolved privately or with the help of a moderator.\n\
                Please respect the decisions of moderators and administrators.",
            )
        });

    let row = ActionRowBuilder::new()
        .add_button(|btn| {
            btn.link(Config::DOMAIN_URL)
                .custom_emoji(Emojis::LOGO_EID, false)
                .label("Website")
        })
        .add_button(|btn| {
            btn.link(format!("{}/terms", Config::DOMAIN_URL))
                .custom_emoji(Emojis::LOGO_EID, false)
                .label("ToS")
        })
        .add_button(|btn| {
            btn.link(format!("{}/privacy", Config::DOMAIN_URL))
                .custom_emoji(Emojis::LOGO_EID, false)
                .label("Privacy Policy")
        });

    vec![container.build().into(), row.build().into()]
}
