use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use tracing::info;
use twilight_http::Client;
use twilight_model::{
    application::command::{Command as DiscordCommand, CommandType},
    id::{Id, marker::ApplicationMarker},
};

use super::{Command, CommandMeta};

/// The global commands registry.
pub struct CommandRegistry<App: Send + Sync + 'static> {
    commands: HashMap<&'static str, Arc<dyn Command<App>>>,
}

impl<App: Send + Sync + 'static> CommandRegistry<App> {
    /// Build a registry.
    pub fn new(commands: Vec<Arc<dyn Command<App>>>) -> Self {
        let mut map = HashMap::new();

        for cmd in commands {
            map.insert(cmd.meta().name, cmd);
        }

        Self { commands: map }
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Command<App>>> {
        self.commands.get(name).cloned()
    }

    pub fn all(&self) -> impl Iterator<Item = Arc<dyn Command<App>>> + '_ {
        self.commands.values().cloned()
    }

    /// Register all slash commands (global + guilds).
    pub async fn register_global_commands(
        &self,
        http: &Client,
        application_id: Id<ApplicationMarker>,
    ) -> Result<()> {
        let mut global_cmds: Vec<DiscordCommand> = Vec::new();
        let mut guild_cmds: HashMap<u64, Vec<DiscordCommand>> = HashMap::new();

        for cmd in self.all() {
            let meta = cmd.meta();

            match &meta.guilds {
                None => global_cmds.push(build_discord_command(meta, application_id)),
                Some(guild_ids) => {
                    for id in guild_ids {
                        guild_cmds
                            .entry(*id)
                            .or_default()
                            .push(build_discord_command(meta, application_id));
                    }
                }
            }
        }

        let interaction_client = http.interaction(application_id);

        let global_count = global_cmds.len();
        info!("Registering {} global slash commands...", global_count);
        interaction_client.set_global_commands(&global_cmds).await?;
        info!("Registered {} global slash commands", global_count);

        for (guild_id, cmds) in guild_cmds {
            let id = Id::new(guild_id);
            let count = cmds.len();
            interaction_client.set_guild_commands(id, &cmds).await?;
            info!("Registered {} guild commands in guild {}", count, guild_id);
        }

        Ok(())
    }
}

fn build_discord_command(
    meta: &CommandMeta,
    application_id: Id<ApplicationMarker>,
) -> DiscordCommand {
    DiscordCommand {
        application_id: Some(application_id),
        contexts: Some(meta.contexts.to_vec()),
        default_member_permissions: None,
        #[allow(deprecated)]
        dm_permission: None,
        description: meta.description.to_string(),
        description_localizations: meta.description_localizations.clone(),
        guild_id: None,
        id: None,
        integration_types: Some(meta.installations.to_vec()),
        kind: CommandType::ChatInput,
        name: meta.name.to_string(),
        name_localizations: None,
        nsfw: None,
        options: meta.options.clone(),
        version: Id::new(1),
    }
}
