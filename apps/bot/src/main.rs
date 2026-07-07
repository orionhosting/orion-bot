mod api;
mod app;
mod commands;
mod config;
mod context;
mod events;
mod localization;
mod logger;
mod remote_logger;
mod services;

use std::sync::Arc;

use anyhow::{Context as _, Result};
use framework::commands::CommandRegistry;
use mimalloc::MiMalloc;
use orion_api::OrionApiClient;
use tracing::{error, info};
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as DiscordHttp;
use twilight_model::gateway::{
    payload::outgoing::UpdatePresence,
    presence::{ActivityType, MinimalActivity, Status},
};

use crate::{
    api::start_api,
    app::App,
    config::Config,
    events::handle_event,
    remote_logger::RemoteLogger,
    services::{ServiceContext, ServiceManager},
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

rust_intl::load!(default = "en");

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = logger::init_logger();
    dotenvy::dotenv().unwrap();
    let config = Config::init_from_env();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // Http clients
    let discord = Arc::new(DiscordHttp::new(config.discord_token.clone()));

    let app_info = discord
        .current_user_application()
        .await
        .context("Failed to fetch application info")?
        .model()
        .await
        .context("Failed to deserialize application info")?;
    info!("Application: {} (id: {})", app_info.name, app_info.id);

    let http = Arc::new(
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .build()?,
    );
    let commands = Arc::new(CommandRegistry::new(commands::all()));
    let service_ctx = ServiceContext::new(app_info.id.get(), discord.clone(), http.clone());
    let orion_api = Arc::new(OrionApiClient::new(
        config.orion_api_token.clone(),
        http.clone(),
    ));
    let discord_cache = DefaultInMemoryCache::builder()
        .resource_types(
            ResourceType::MESSAGE
                | ResourceType::CHANNEL
                | ResourceType::ROLE
                | ResourceType::USER
                | ResourceType::MEMBER,
        )
        .message_cache_size(10)
        .build();

    let app = Arc::new(App::new(
        app_info.id,
        discord.clone(),
        discord_cache,
        commands,
        http.clone(),
        Arc::new(ServiceManager::new(service_ctx)),
        Arc::new(RemoteLogger::new(discord.clone()).expect("Failed to create remote logger")),
        orion_api,
    ));

    run_app(app).await
}

async fn run_app(app: Arc<App>) -> Result<()> {
    // Register commands

    app.commands
        .register_global_commands(&app.discord, app.application_id)
        .await
        .context("Failed to register slash commands")?;

    // Shard

    let intents = Intents::GUILDS
        | Intents::GUILD_MESSAGES
        | Intents::GUILD_MEMBERS
        | Intents::MESSAGE_CONTENT;

    let mut shard = Shard::new(ShardId::ONE, Config::get().discord_token.clone(), intents);

    // Ready

    app.services.on_ready();

    // API

    tokio::spawn({
        let app = app.clone();
        async move {
            start_api(app).await;
        }
    });

    // Event handler

    info!("Starting gateway connection");
    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let event = match item {
            Ok(event) => event,
            Err(source) => {
                error!(?source, "error receiving gateway event");
                continue;
            }
        };

        if matches!(event, Event::Ready(_)) {
            // Presence
            // TODO: this should be moved in ready.rs (but we need to Arc the shard)

            let activity = MinimalActivity {
                name: Config::DOMAIN.to_string(),
                kind: ActivityType::Playing,
                url: None,
            };
            let presence = UpdatePresence::new(vec![activity.into()], false, None, Status::Online)
                .expect("Invalid presence");

            shard.command(&presence);
        }

        app.discord_cache.update(&event);
        app.set_gateway_latency(shard.latency().average()).await;

        let app = app.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_event(app, event).await {
                error!("event handler error: {e:#}");
            }
        });
    }

    Ok(())
}
