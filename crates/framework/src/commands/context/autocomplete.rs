use twilight_model::application::{
    command::CommandOptionType,
    interaction::{
        InteractionData,
        application_command::{CommandDataOption, CommandOptionValue},
    },
};

use super::CommandContext;

/// An autocomplete focused value.
pub enum FocusedValue<'a> {
    String(&'a str),
    Integer(i64),
    Number(f64),
}

impl<'a, App> CommandContext<'a, App> {
    /// The raw options for an autocomplete interaction.
    fn autocomplete_options(&self) -> &[CommandDataOption] {
        match &self.interaction.data {
            Some(InteractionData::ApplicationCommand(data)) => &data.options,
            _ => &[],
        }
    }

    fn resolved_autocomplete_options(&self) -> &[CommandDataOption] {
        let top = self.autocomplete_options();
        match top.first().map(|o| &o.value) {
            Some(CommandOptionValue::SubCommand(opts)) => opts,
            Some(CommandOptionValue::SubCommandGroup(group)) => {
                match group.first().map(|o| &o.value) {
                    Some(CommandOptionValue::SubCommand(opts)) => opts,
                    _ => top,
                }
            }
            _ => top,
        }
    }

    /// Get the autocomplete focused value.
    pub fn get_focused_option(&self) -> Option<(&str, FocusedValue<'_>)> {
        self.resolved_autocomplete_options()
            .iter()
            .find_map(|o| match &o.value {
                CommandOptionValue::Focused(raw, kind) => {
                    let value = match kind {
                        CommandOptionType::Integer => {
                            FocusedValue::Integer(raw.parse().unwrap_or_default())
                        }
                        CommandOptionType::Number => {
                            FocusedValue::Number(raw.parse().unwrap_or_default())
                        }
                        _ => FocusedValue::String(raw.as_str()),
                    };
                    Some((o.name.as_str(), value))
                }
                _ => None,
            })
    }

    /// Shorthand for `get_focused_option` if the option is a string.
    pub fn get_focused_string(&self) -> Option<&str> {
        self.get_focused_option().and_then(|(_, v)| match v {
            FocusedValue::String(s) => Some(s),
            _ => None,
        })
    }
}
