use std::sync::Arc;

use anyhow::Result;
use framework::commands::{CommandError, Reply};
use rust_intl::{t, t_ns};
use tracing::{error, info, warn};
use twilight_model::application::interaction::{Interaction, InteractionData, InteractionType};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    app::App,
    config::{Config, Emojis, Palette},
    context::CommandContext,
};

pub async fn handle(app: Arc<App>, interaction: Interaction) -> Result<()> {
    match interaction.kind {
        InteractionType::ApplicationCommand => handle_command(app, interaction).await,
        InteractionType::ApplicationCommandAutocomplete => {
            handle_autocomplete(app, interaction).await
        }
        _ => {}
    }

    Ok(())
}

async fn handle_command(app: Arc<App>, interaction: Interaction) {
    let Some(command_name) = interaction.data.as_ref().map(|d| match d {
        InteractionData::ApplicationCommand(d) => d.name.clone(),
        _ => String::new(),
    }) else {
        return;
    };

    info!(
        command = %command_name,
        user_id = ?interaction.author_id().map(|v| v.get()),
        guild_id = ?interaction.guild_id,
        "Slash command",
    );

    let cmd_ctx = CommandContext::new(
        app.clone(),
        app.discord.clone(),
        app.application_id,
        &interaction,
    );

    let command = match app.commands.get(&command_name) {
        Some(c) => c,
        None => {
            warn!("Unknown command: {command_name}");
            send_error_response(
                &cmd_ctx,
                &CommandError::Message(format!("Unknown command: `{command_name}`"), true),
            )
            .await;
            return;
        }
    };

    if command.meta().owner_only
        && !interaction
            .author_id()
            .is_some_and(|id| Config::is_bot_owner(id.get()))
    {
        send_error_response(&cmd_ctx, &CommandError::NotOwner).await;
        return;
    }

    let user = cmd_ctx.user();
    let logger = app.remote_logger.clone();
    let message = format!("`{} ({})` has used `/{}`", user.name, user.id, command_name);

    tokio::spawn(async move {
        let _ = logger.send_log(message).await;
    });

    if let Err(e) = command.handle_command(&cmd_ctx).await {
        error!(command = %command_name, error = %e, "Command execution failed");
        send_error_response(&cmd_ctx, &e).await;
    }
}

/// Handle an autocomplete interaction.
async fn handle_autocomplete(app: Arc<App>, interaction: Interaction) {
    let Some(command_name) = interaction.data.as_ref().map(|d| match d {
        InteractionData::ApplicationCommand(d) => d.name.clone(),
        _ => String::new(),
    }) else {
        return;
    };

    let command = match app.commands.get(&command_name) {
        Some(c) => c,
        None => return,
    };

    let cmd_ctx = CommandContext::new(
        app.clone(),
        app.discord.clone(),
        app.application_id,
        &interaction,
    );

    if let Err(e) = command.handle_autocomplete(&cmd_ctx).await {
        error!(command = %command_name, error = %e, "Autocomplete failed");
    }
}

/// Send an ephemeral error reply.
#[t_ns(namespace = "events.interactionCreate")]
async fn send_error_response(ctx: &CommandContext<'_>, err: &CommandError) {
    warn!("Command error: {:?}", err);

    // TODO: dont ignore result
    let _ = ctx
        .app
        .remote_logger
        .send_warning("A command error occurred, check the logs");

    let content = match err {
        CommandError::Permission(_msg) => "Missing Permissions!".into(),
        CommandError::NotOwner => t!("err_missing_permissions"),
        CommandError::Message(msg, _edit) => msg.clone(),
        CommandError::Anyhow(_) => t!("/common.errors.err_unknown"),
    };

    let content = format!("{} {content}", Emojis::WARN);

    let reply = Reply::new()
        .embed(
            EmbedBuilder::new()
                .color(Palette::RED.int)
                .description(content)
                .build(),
        )
        .ephemeral(true);

    let value = if ctx.has_sent_initial_response() {
        ctx.follow_up(reply).await.map(|_| ())
    } else {
        ctx.reply(reply).await.map(|_| ())
    };

    if let Err(e) = value {
        error!(error = %e, "Failed to send error response");
    }
}
