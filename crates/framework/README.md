# Framework

A WIP framework for Rust bots.

## Commands usage example

commands/help.rs

```rs
/// Help command.
pub struct HelpCommand;

command_meta! {
    META = CommandMeta::builder("help", "The help command")
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

#[async_trait]
impl Command<App> for HelpCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        ctx.reply(Reply::new().content("Hello World"))
            .await?;

        Ok(())
    }
}
```

commands/mod.rs

```rs
pub fn all() -> Vec<Arc<dyn Command<App>>> {
    vec![
        Arc::new(help::HelpCommand),
    ]
}
```

main.rs

```rs
let discord = Arc::new(DiscordHttp::new("TOKEN"));
let application_id = "ID";
let app = Arc::new(App {
    discord,
    application_id,
});

let commands = Arc::new(CommandRegistry::new(commands::all()));

commands
    .register_global_commands(&app.discord, app.application_id)
    .await
    .context("Failed to register slash commands")?;
```

## TODO

- Add before/after hooks for commands
- More options
