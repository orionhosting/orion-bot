use twilight_model::channel::message::component::{ActionRow, Component};

use crate::button::ButtonBuilder;

/// Builder for action rows.
pub struct ActionRowBuilder {
    components: Vec<Component>,
}

impl ActionRowBuilder {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_button<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ButtonBuilder) -> ButtonBuilder,
    {
        self.components
            .push(Component::Button(f(ButtonBuilder::new()).build()));
        self
    }

    pub fn build(self) -> ActionRow {
        ActionRow {
            id: None,
            components: self.components,
        }
    }
}

impl Default for ActionRowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
