use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionChoiceValue},
        interaction::InteractionContextType,
    },
    oauth::ApplicationIntegrationType,
};

/// Macro to lazily define your command metadata statically.
/// The value will be lazy locked.
#[macro_export]
macro_rules! command_meta {
    ($vis:vis $name:ident = $builder:expr) => {
        $vis static $name: std::sync::LazyLock<CommandMeta> =
            std::sync::LazyLock::new(|| $builder.build());
    };
}

/// Static metadata for a command.
#[derive(Debug, Clone)]
pub struct CommandMeta {
    pub name: &'static str,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
    pub category: &'static str,
    pub installations: &'static [ApplicationIntegrationType],
    pub contexts: &'static [InteractionContextType],
    pub guilds: Option<Vec<u64>>,
    pub owner_only: bool,
    pub options: Vec<CommandOption>,
}

/// Builder for [`CommandMeta`].
/// ```ignore
/// command_meta! {
///     META = CommandMeta::builder("ping", "Pong!")
///         .category(categories::Informations);
/// }
/// ```
///
/// Defaults:
/// - Only available in guilds
/// - "general" category
pub struct CommandMetaBuilder {
    name: &'static str,
    description: String,
    description_localizations: Option<HashMap<String, String>>,
    category: &'static str,
    installations: &'static [ApplicationIntegrationType],
    contexts: &'static [InteractionContextType],
    guilds: Option<Vec<u64>>,
    owner_only: bool,
    options: Vec<CommandOption>,
}

impl CommandMeta {
    pub fn builder(name: &'static str, description: String) -> CommandMetaBuilder {
        CommandMetaBuilder {
            name,
            description,
            description_localizations: None,
            category: "general",
            installations: &[ApplicationIntegrationType::GuildInstall],
            contexts: &[InteractionContextType::Guild],
            guilds: None,
            owner_only: false,
            options: Vec::new(),
        }
    }
}

impl CommandMetaBuilder {
    pub fn category(mut self, category: &'static str) -> Self {
        self.category = category;
        self
    }

    pub fn description_localizations(mut self, descriptions: HashMap<String, String>) -> Self {
        self.description_localizations = Some(descriptions);
        self
    }

    pub fn installations(mut self, installations: &'static [ApplicationIntegrationType]) -> Self {
        self.installations = installations;
        self
    }

    pub fn contexts(mut self, contexts: &'static [InteractionContextType]) -> Self {
        self.contexts = contexts;
        self
    }

    pub fn guilds(mut self, guilds: impl Into<Vec<u64>>) -> Self {
        self.guilds = Some(guilds.into());
        self
    }

    pub fn owner_only(mut self, owner_only: bool) -> Self {
        self.owner_only = owner_only;
        self
    }

    pub fn options<I, T>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<CommandOption>,
    {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }

    pub fn build(self) -> CommandMeta {
        CommandMeta {
            name: self.name,
            description: self.description,
            description_localizations: self.description_localizations,
            category: self.category,
            installations: self.installations,
            contexts: self.contexts,
            guilds: self.guilds,
            owner_only: self.owner_only,
            options: self.options,
        }
    }
}

impl From<CommandOptionBuilder> for CommandOption {
    fn from(builder: CommandOptionBuilder) -> Self {
        builder.build()
    }
}

// TODO: move this to another file

use std::collections::HashMap;
use twilight_model::{
    application::command::{
        CommandOptionChoice, CommandOptionType, CommandOptionValue as OptionBound,
    },
    channel::ChannelType,
};

/// A command option builder.
pub struct CommandOptionBuilder {
    inner: CommandOption,
}

impl CommandOptionBuilder {
    fn new(
        kind: CommandOptionType,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            inner: CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: description.into(),
                description_localizations: None,
                kind,
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: name.into(),
                name_localizations: None,
                options: None,
                required: None,
            },
        }
    }

    pub fn string(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::String, name, description)
    }

    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Integer, name, description)
    }

    pub fn number(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Number, name, description)
    }

    pub fn boolean(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Boolean, name, description)
    }

    pub fn user(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::User, name, description)
    }

    pub fn channel(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Channel, name, description)
    }

    pub fn role(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Role, name, description)
    }

    pub fn mentionable(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Mentionable, name, description)
    }

    pub fn attachment(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Attachment, name, description)
    }

    pub fn subcommand(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::SubCommand, name, description)
    }

    pub fn subcommand_group(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(CommandOptionType::SubCommandGroup, name, description)
    }

    pub fn required(mut self, required: bool) -> Self {
        self.inner.required = Some(required);
        self
    }

    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.inner.autocomplete = Some(autocomplete);
        self
    }

    pub fn min_length(mut self, min: u16) -> Self {
        self.inner.min_length = Some(min);
        self
    }

    pub fn max_length(mut self, max: u16) -> Self {
        self.inner.max_length = Some(max);
        self
    }

    pub fn min_value_int(mut self, min: i64) -> Self {
        self.inner.min_value = Some(OptionBound::Integer(min));
        self
    }

    pub fn max_value_int(mut self, max: i64) -> Self {
        self.inner.max_value = Some(OptionBound::Integer(max));
        self
    }

    pub fn min_value_number(mut self, min: f64) -> Self {
        self.inner.min_value = Some(OptionBound::Number(min));
        self
    }

    pub fn max_value_number(mut self, max: f64) -> Self {
        self.inner.max_value = Some(OptionBound::Number(max));
        self
    }

    pub fn channel_types(mut self, types: Vec<ChannelType>) -> Self {
        self.inner.channel_types = Some(types);
        self
    }

    /// Add a string choice.
    pub fn add_choice_string(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner
            .choices
            .get_or_insert_with(Vec::new)
            .push(CommandOptionChoice {
                name: name.into(),
                name_localizations: None,
                value: CommandOptionChoiceValue::String(value.into()),
            });
        self
    }

    /// Add an integer choice.
    pub fn add_choice_int(mut self, name: impl Into<String>, value: i64) -> Self {
        self.inner
            .choices
            .get_or_insert_with(Vec::new)
            .push(CommandOptionChoice {
                name: name.into(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Integer(value),
            });
        self
    }

    /// Add an option.
    pub fn option(mut self, option: impl Into<CommandOption>) -> Self {
        self.inner
            .options
            .get_or_insert_with(Vec::new)
            .push(option.into());
        self
    }

    /// Add options.
    pub fn options<I, T>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<CommandOption>,
    {
        self.inner.options = Some(options.into_iter().map(Into::into).collect());
        self
    }

    pub fn name_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.inner.name_localizations = Some(map);
        self
    }

    pub fn description_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.inner.description_localizations = Some(map);
        self
    }

    pub fn build(self) -> CommandOption {
        self.inner
    }
}
