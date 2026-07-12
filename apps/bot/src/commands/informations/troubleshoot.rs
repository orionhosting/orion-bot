use crate::commands::prelude::*;
use anyhow::Result;

/// Help users troubleshoot errors with their servers.
pub struct TroubleshootCommand;

command_meta! {
    META = CommandMeta::builder("troubleshoot", t!("commands.troubleshoot.description"))
        .description_localizations(localize_key(|locale| t!(locale, "commands.troubleshoot.description")))
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
impl Command<App> for TroubleshootCommand {
    fn meta(&self) -> &CommandMeta {
        &META
    }

    async fn handle_command(&self, ctx: &CommandContext<'_>) -> CommandResult {
        match build_troubleshoot_menu(&ctx.i18n_locale(), "home") {
            Ok(reply) => {
                ctx.reply(reply).await?;
            }
            Err(err) => {
                return Err(err.into());
            }
        }
        Ok(())
    }

    async fn handle_component(&self, ctx: &CommandContext<'_>) -> CommandResult {
        if let Some(InteractionData::MessageComponent(data)) = ctx.interaction.data.as_ref() {
            let path = data.values.first().map(String::as_str).unwrap_or("home");

            match build_troubleshoot_menu(&ctx.i18n_locale(), path) {
                Ok(reply) => {
                    ctx.update_message(reply).await?;
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
        Ok(())
    }
}

/// Get the parent path of an option.
fn parent_path(path: &str) -> Option<String> {
    if path == "home" {
        return None;
    }
    let segments: Vec<&str> = path.split(".options.").collect();
    (segments.len() > 1).then(|| segments[..segments.len() - 1].join(".options."))
}

#[t_ns(namespace = "commands.troubleshoot")]
fn build_troubleshoot_menu(locale: &Locale, path: &str) -> Result<Reply> {
    let Some(data) = get_node_data(locale, path) else {
        return Err(anyhow::anyhow!(
            "Path '{path}' could not be resolved in troubleshoot menu"
        ));
    };

    let mut container = ContainerBuilder::new()
        .accent_color(Palette::PRIMARY.int)
        .add_text_display(|d| d.content(format!("**{}**\n{}", data.title, data.message)));

    container = container.add_action_row(|row| {
        row.add_select_menu(|menu| {
            let mut menu = menu.custom_id("troubleshoot-select");

            for (path_value, label) in data.options {
                menu = menu.add_option(|opt| opt.label(label).value(path_value));
            }

            // Put the return button unless we are in home (no parent path)
            if let Some(return_path) = parent_path(path) {
                menu = menu.add_option(|opt| opt.label(t!(locale, "return")).value(return_path));
            }

            menu
        })
    });

    Ok(Reply::new().components_v2(container.into()).ephemeral(true))
}

/// Contains the localized strings.
#[derive(Default)]
struct NodeData {
    title: String,
    message: String,
    options: Vec<(&'static str, String)>,
}

impl NodeData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn options(mut self, options: Vec<(&'static str, String)>) -> Self {
        self.options = options;
        self
    }
}

#[t_ns(namespace = "commands.troubleshoot")]
fn get_node_data(locale: &Locale, path: &str) -> Option<NodeData> {
    // Route to submenus based on path prefix
    if path.starts_with("home.options.crash") {
        return get_crash_node_data(locale, path);
    }
    if path.starts_with("home.options.inaccessible") {
        return get_inaccessible_node_data(locale, path);
    }

    // Home menu
    match path {
        "home" => Some(
            NodeData::new()
                .title(t!(locale, "home.title", emoji = Emojis::LOGO))
                .message(t!(locale, "home.message",))
                .options(vec![
                    (
                        "home.options.crash.next",
                        t!(locale, "home.options.crash.label"),
                    ),
                    (
                        "home.options.inaccessible.next",
                        t!(locale, "home.options.inaccessible.label"),
                    ),
                ]),
        ),
        _ => None,
    }
}

#[t_ns(namespace = "commands.troubleshoot.home.options.crash.next")]
fn get_crash_node_data(locale: &Locale, path: &str) -> Option<NodeData> {
    match path.trim_start_matches("home.options.crash.next") {
        "" => Some(
            NodeData::new()
                .title(t!(locale, "title"))
                .message(t!(locale, "message"))
                .options(vec![
                    (
                        "home.options.crash.next.options.module_not_found",
                        t!(locale, "options.module_not_found.label"),
                    ),
                    (
                        "home.options.crash.next.options.port_in_use",
                        t!(locale, "options.port_in_use.label"),
                    ),
                    (
                        "home.options.crash.next.options.syntax_error",
                        t!(locale, "options.syntax_error.label"),
                    ),
                    (
                        "home.options.crash.next.options.insufficient_memory",
                        t!(locale, "options.insufficient_memory.label"),
                    ),
                    (
                        "home.options.crash.next.options.stay_starting_state",
                        t!(locale, "options.stay_starting_state.label"),
                    ),
                ]),
        ),
        ".options.module_not_found" => Some(
            NodeData::new()
                .title(format!(
                    "## {}",
                    t!(locale, "options.module_not_found.label")
                ))
                .message(t!(locale, "options.module_not_found.message")),
        ),
        ".options.port_in_use" => Some(
            NodeData::new()
                .title(format!("## {}", t!(locale, "options.port_in_use.label")))
                .message(t!(locale, "options.port_in_use.message")),
        ),
        ".options.syntax_error" => Some(
            NodeData::new()
                .title(format!("## {}", t!(locale, "options.syntax_error.label")))
                .message(t!(locale, "options.syntax_error.message")),
        ),
        ".options.insufficient_memory" => Some(
            NodeData::new()
                .title(format!(
                    "## {}",
                    t!(locale, "options.insufficient_memory.label")
                ))
                .message(t!(locale, "options.insufficient_memory.message")),
        ),
        ".options.stay_starting_state" => Some(
            NodeData::new()
                .title(format!(
                    "## {}",
                    t!(locale, "options.stay_starting_state.label")
                ))
                .message(t!(locale, "options.stay_starting_state.message")),
        ),
        _ => None,
    }
}

#[t_ns(namespace = "commands.troubleshoot.home.options.inaccessible")]
fn get_inaccessible_node_data(locale: &Locale, path: &str) -> Option<NodeData> {
    match path.trim_start_matches("home.options.inaccessible") {
        ".next" => Some(
            NodeData::new()
                .title(t!(locale, "next.title"))
                .message(t!(locale, "next.message"))
                .options(vec![
                    (
                        "home.options.inaccessible.next.options.502",
                        t!(locale, "next.options.502.label"),
                    ),
                    (
                        "home.options.inaccessible.next.options.404",
                        t!(locale, "next.options.404.label"),
                    ),
                    (
                        "home.options.inaccessible.next.options.domain_not_pointing",
                        t!(locale, "next.options.domain_not_pointing.label"),
                    ),
                ]),
        ),
        ".next.options.502" => Some(
            NodeData::new()
                .title(format!("## {}", t!(locale, "next.options.502.label")))
                .message(t!(locale, "next.options.502.message")),
        ),
        ".next.options.404" => Some(
            NodeData::new()
                .title(format!("## {}", t!(locale, "next.options.404.label")))
                .message(t!(locale, "next.options.404.message")),
        ),
        ".next.options.domain_not_pointing" => Some(
            NodeData::new()
                .title(format!(
                    "## {}",
                    t!(locale, "next.options.domain_not_pointing.label")
                ))
                .message(t!(locale, "next.options.domain_not_pointing.message",)),
        ),
        _ => None,
    }
}
