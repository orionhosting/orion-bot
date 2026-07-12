mod emojis;
mod palette;

use std::fmt;

pub use emojis::Emojis;
pub use palette::Palette;

use serde::Deserialize;

static CONFIG: std::sync::OnceLock<Config> = std::sync::OnceLock::new();

/// The bot .env configuration.
#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub orion_api_token: String,
    pub gemini_key: String,
    pub discord_token: String,
    pub support_guild_id: u64,
    pub status_channel_id: u64,
    pub chatbot_channel_id: u64,
    pub logs_webhook_id: u64,
    pub logs_webhook_token: String,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config").finish()
    }
}

impl Config {
    // IDs
    pub const OWNER_IDS: [u64; 2] = [619838036846575617, 755054105713704960];

    // Orion
    pub const DOMAIN: &'static str = "orionhost.xyz";
    pub const DOMAIN_URL: &'static str = "https://orionhost.xyz";
    pub const DASHBOARD_URL: &'static str = "https://orionhost.xyz/dashboard";
    pub const PANEL_URL: &'static str = "https://panel.orionhost.xyz";
    pub const ORION_GITHUB_URL: &'static str = "https://github.com/orionhosting";
    pub const BOT_GITHUB_URL: &'static str = "https://github.com/orionhosting/orion-bot";
    pub const CLI_GITHUB_URL: &'static str = "https://github.com/orionhosting/cli";
    pub const DOCS_URL: &'static str = "https://docs.orionhost.xyz";
    pub const STATUS_URL: &'static str = "https://status.orionhost.xyz";
    pub const API_URL: &'static str = "https://api.orionhost.xyz";
    pub const BOT_API_URL: &'static str = "https://bot.orionhost.xyz";

    // Discord links
    // pub const HELP_CODES_CHANNEL_ID: u64 = 1307513861695606834;
    // pub const TICKETS_CHANNEL_ID: u64 = 1307061230145507410;
    pub const SUPPORT_INVITE: &'static str = "https://discord.gg/gzYKugxq9a";
    pub const BOT_INVITE: &'static str = "https://discord.com/oauth2/authorize?client_id=1306868952793747546&scope=bot+applications.commands&permissions=8";
    pub const TICKETS_PANEL_URL: &'static str =
        "https://discord.com/channels/1306734190238371860/1307061230145507410";

    // Assets
    pub const BANNER_URL: &'static str = "https://media.discordapp.net/attachments/1480034193869246477/1480036557099503879/banner-large.png";
}

impl Config {
    pub fn init_from_env() -> &'static Self {
        let cfg = envy::from_env::<Self>().expect("Failed to parse .env");
        CONFIG.set(cfg).expect("Config already initialized");
        Self::get()
    }

    pub fn get() -> &'static Self {
        CONFIG.get().expect("Config not initialized before use")
    }

    /// Checks if the user is an owner.
    pub fn is_bot_owner(id: u64) -> bool {
        Config::OWNER_IDS.contains(&id)
    }
}
