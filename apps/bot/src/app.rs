use std::sync::Arc;

use framework::commands::CommandRegistry;
use orion_api::OrionApiClient;
use tokio::{sync::RwLock, time::Duration};
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client as DiscordHttp;
use twilight_model::id::{Id, marker::ApplicationMarker};

use crate::services::ServiceManager;

/// The shared bot state.
pub struct App {
    /// The bot application ID.
    pub application_id: Id<ApplicationMarker>,
    /// The Discord HTTP client.
    pub discord: Arc<DiscordHttp>,
    /// The Discord cache.
    pub discord_cache: InMemoryCache,
    pub commands: Arc<CommandRegistry<App>>,

    /// HTTP client.
    pub http: Arc<reqwest::Client>,
    /// Orion API client.
    pub orion_api: Arc<OrionApiClient>,
    pub services: Arc<ServiceManager>,

    gateway_latency: RwLock<Option<Duration>>,
}

impl App {
    pub fn new(
        application_id: Id<ApplicationMarker>,
        discord: Arc<DiscordHttp>,
        discord_cache: InMemoryCache,
        commands: Arc<CommandRegistry<App>>,
        http: Arc<reqwest::Client>,
        services: Arc<ServiceManager>,
        orion_api: Arc<OrionApiClient>,
    ) -> Self {
        Self {
            application_id,
            discord,
            discord_cache,
            commands,
            http,
            orion_api,
            services,
            gateway_latency: RwLock::new(None),
        }
    }

    pub async fn set_gateway_latency(&self, latency: Option<Duration>) {
        *self.gateway_latency.write().await = latency;
    }

    /// Average gateway latency.
    pub async fn gateway_latency(&self) -> Option<Duration> {
        *self.gateway_latency.read().await
    }
}
