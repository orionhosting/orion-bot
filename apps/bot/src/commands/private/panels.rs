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
                "## 1. Respectful behavior\n\
                Treat everyone with respect, even when opinions differ.\n\
                Hate speech, harassment, discrimination, and personal attacks are not allowed.\n\
                Avoid unnecessary drama, excessive controversial discussions, or conflicts brought in from other communities.\n\
                Handle disagreements calmly, and contact a moderator if needed.\n\
                Respect moderator and administrator decisions.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## 2. Chat guidelines\n\
                English and French are the community's official languages.\n\
                Do your best to use either language so everyone can participate and moderators can understand conversations.\n\
                Keep messages relevant and avoid flooding the chat, excessive capital letters, or repeated animated emotes.\n\
                Advertising, unsolicited promotions, and NSFW content are prohibited.\n\
                Do not post malicious links or other harmful content.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## 3. Usernames and identity\n\
                Choose a display name that is readable, easy to mention, and reasonably sized.\n\
                Do not impersonate other members, staff, or other individuals.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## 4. Privacy and security\n\
                Never share passwords, credentials, or other sensitive information.\n\
                Do not disclose another person's private information without their permission.\n\
                Do not use private messages to spam, advertise, or send unsolicited server invitations.\n\
                Unauthorized server invitations and bot additions are prohibited.\n\
                Report suspicious behavior, scams, or security concerns to the moderation team.",
            )
        })
        .add_separator(|s| s.divider(true).spacing(SeparatorSpacingSize::Large))
        .add_text_display(|d| {
            d.content(
                "## 5. Moderation\n\
                Follow these guidelines across the community, including private messages where applicable.\n\
                Moderators may take action when necessary to keep the community safe and welcoming.",
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
