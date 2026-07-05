use std::{error, fmt};

use twilight_model::{
    application::interaction::{
        InteractionData,
        application_command::{CommandDataOption, CommandOptionValue},
    },
    id::{
        Id,
        marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker},
    },
};

use super::CommandContext;

#[derive(Debug)]
pub enum OptionError {
    Missing(&'static str),
    WrongType(&'static str),
}

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(name) => write!(f, "missing required option `{name}`"),
            Self::WrongType(name) => write!(f, "option `{name}` had an unexpected type"),
        }
    }
}

impl error::Error for OptionError {}

macro_rules! option_getters {
    ($get:ident, $require:ident, $variant:ident, $ty:ty) => {
        pub fn $get(&self, name: &str) -> Option<$ty> {
            self.resolved_options()
                .iter()
                .find(|o| o.name == name)
                .and_then(|o| match &o.value {
                    CommandOptionValue::$variant(v) => Some(v.clone()),
                    _ => None,
                })
        }

        pub fn $require(&self, name: &'static str) -> Result<$ty, OptionError> {
            self.$get(name).ok_or(OptionError::Missing(name))
        }
    };
}

impl<'a, App> CommandContext<'a, App> {
    pub fn options(&self) -> &[CommandDataOption] {
        match &self.interaction.data {
            Some(InteractionData::ApplicationCommand(data)) => &data.options,
            _ => &[],
        }
    }

    /// Name of the invoked subcommand.
    pub fn get_subcommand(&self) -> Option<&str> {
        match self.options().first().map(|o| (o.name.as_str(), &o.value)) {
            Some((name, CommandOptionValue::SubCommand(_))) => Some(name),
            Some((_, CommandOptionValue::SubCommandGroup(group))) => {
                group.first().map(|o| o.name.as_str())
            }
            _ => None,
        }
    }

    /// Name of the invoked subcommand group.
    pub fn get_subcommand_group(&self) -> Option<&str> {
        match self.options().first().map(|o| (o.name.as_str(), &o.value)) {
            Some((name, CommandOptionValue::SubCommandGroup(_))) => Some(name),
            _ => None,
        }
    }

    fn resolved_options(&self) -> &[CommandDataOption] {
        let top = self.options();
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

    pub fn get_string_option(&self, name: &str) -> Option<&str> {
        self.resolved_options()
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| match &o.value {
                CommandOptionValue::String(s) => Some(s.as_str()),
                _ => None,
            })
    }

    pub fn require_string_option(&self, name: &'static str) -> Result<&str, OptionError> {
        self.get_string_option(name)
            .ok_or(OptionError::Missing(name))
    }

    option_getters!(get_integer_option, require_integer_option, Integer, i64);
    option_getters!(get_number_option, require_number_option, Number, f64);
    option_getters!(get_boolean_option, require_boolean_option, Boolean, bool);
    option_getters!(get_user_option, require_user_option, User, Id<UserMarker>);
    option_getters!(
        get_channel_option,
        require_channel_option,
        Channel,
        Id<ChannelMarker>
    );
    option_getters!(get_role_option, require_role_option, Role, Id<RoleMarker>);
    option_getters!(
        get_mentionable_option,
        require_mentionable_option,
        Mentionable,
        Id<GenericMarker>
    );
    option_getters!(
        get_attachment_option,
        require_attachment_option,
        Attachment,
        Id<AttachmentMarker>
    );
}
