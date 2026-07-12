use twilight_model::channel::message::component::{SelectMenu, SelectMenuOption, SelectMenuType};

/// Builder for string select menu options.
pub struct StringSelectMenuOptionBuilder {
    label: String,
    value: String,
    description: Option<String>,
    default: bool,
}

impl StringSelectMenuOptionBuilder {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            value: String::new(),
            description: None,
            default: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    pub fn build(self) -> SelectMenuOption {
        SelectMenuOption {
            default: self.default,
            description: self.description,
            emoji: None,
            label: self.label,
            value: self.value,
        }
    }
}

impl Default for StringSelectMenuOptionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for string select menus.
pub struct StringSelectMenuBuilder {
    custom_id: String,
    options: Vec<SelectMenuOption>,
    placeholder: Option<String>,
    min_values: Option<u8>,
    max_values: Option<u8>,
    disabled: bool,
}

impl StringSelectMenuBuilder {
    pub fn new() -> Self {
        Self {
            custom_id: String::new(),
            options: Vec::new(),
            placeholder: None,
            min_values: None,
            max_values: None,
            disabled: false,
        }
    }

    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = custom_id.into();
        self
    }

    pub fn add_option<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StringSelectMenuOptionBuilder) -> StringSelectMenuOptionBuilder,
    {
        self.options
            .push(f(StringSelectMenuOptionBuilder::new()).build());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> SelectMenu {
        SelectMenu {
            channel_types: None,
            custom_id: self.custom_id,
            default_values: None,
            disabled: self.disabled,
            kind: SelectMenuType::Text,
            max_values: self.max_values,
            min_values: self.min_values,
            options: Some(self.options),
            placeholder: self.placeholder,
            id: None,
            required: None,
        }
    }
}

impl Default for StringSelectMenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}
