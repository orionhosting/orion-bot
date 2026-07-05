use twilight_model::channel::message::component::{
    Component, Section, Thumbnail, UnfurledMediaItem,
};

use crate::button::ButtonBuilder;
use crate::text_display::TextDisplayBuilder;

/// Builder for sections.
///
/// # Panics
///
/// [`SectionBuilder::build`] will panic if no accessory was set.
pub struct SectionBuilder {
    components: Vec<Component>,
    accessory: Option<Component>,
}

impl SectionBuilder {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            accessory: None,
        }
    }

    pub fn add_text_display<F>(mut self, f: F) -> Self
    where
        F: FnOnce(TextDisplayBuilder) -> TextDisplayBuilder,
    {
        self.components
            .push(Component::TextDisplay(f(TextDisplayBuilder::new()).build()));
        self
    }

    /// Set a thumbnail as the accessory.
    pub fn set_thumbnail_accessory(mut self, url: impl Into<String>) -> Self {
        self.accessory = Some(Component::Thumbnail(Thumbnail {
            id: None,
            media: UnfurledMediaItem {
                url: url.into(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None,
            },
            description: None,
            spoiler: Some(false),
        }));
        self
    }

    /// Set a button as the accessory.
    pub fn set_button_accessory<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ButtonBuilder) -> ButtonBuilder,
    {
        self.accessory = Some(Component::Button(f(ButtonBuilder::new()).build()));
        self
    }

    pub fn build(self) -> Section {
        Section {
            id: None,
            components: self.components,
            accessory: Box::new(
                self.accessory
                    .expect("Section requires an accessory"),
            ),
        }
    }
}

impl Default for SectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
