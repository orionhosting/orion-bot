use twilight_model::channel::message::component::{Component, Container};

use crate::{
    SeparatorBuilder, action_row::ActionRowBuilder, media_gallery::MediaGalleryBuilder,
    section::SectionBuilder, text_display::TextDisplayBuilder,
};

/// Builder for containers.
pub struct ContainerBuilder {
    components: Vec<Component>,
    accent_color: Option<u32>,
    spoiler: bool,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            accent_color: None,
            spoiler: false,
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

    pub fn add_media_gallery<F>(mut self, f: F) -> Self
    where
        F: FnOnce(MediaGalleryBuilder) -> MediaGalleryBuilder,
    {
        self.components.push(Component::MediaGallery(
            f(MediaGalleryBuilder::new()).build(),
        ));
        self
    }

    pub fn add_section<F>(mut self, f: F) -> Self
    where
        F: FnOnce(SectionBuilder) -> SectionBuilder,
    {
        self.components
            .push(Component::Section(f(SectionBuilder::new()).build()));
        self
    }

    pub fn add_separator<F>(mut self, f: F) -> Self
    where
        F: FnOnce(SeparatorBuilder) -> SeparatorBuilder,
    {
        self.components
            .push(Component::Separator(f(SeparatorBuilder::new()).build()));
        self
    }

    pub fn add_action_row<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ActionRowBuilder) -> ActionRowBuilder,
    {
        self.components
            .push(Component::ActionRow(f(ActionRowBuilder::new()).build()));
        self
    }

    pub fn accent_color(mut self, color: u32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }

    pub fn build(self) -> Container {
        Container {
            id: None,
            accent_color: if self.accent_color.is_none() {
                None
            } else {
                Some(self.accent_color) // Some(Some(_))
            },
            components: self.components,
            spoiler: Some(self.spoiler),
        }
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ContainerBuilder> for Vec<Component> {
    fn from(builder: ContainerBuilder) -> Self {
        vec![Component::Container(builder.build())]
    }
}
