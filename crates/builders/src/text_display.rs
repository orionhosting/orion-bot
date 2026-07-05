use twilight_model::channel::message::component::TextDisplay;

/// Builder for text displays.
///
/// # Panics
///
/// [`TextDisplayBuilder::build`] will panic if no content was set.
pub struct TextDisplayBuilder {
    content: Option<String>,
}

impl TextDisplayBuilder {
    pub fn new() -> Self {
        Self { content: None }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn build(self) -> TextDisplay {
        TextDisplay {
            id: None,
            content: self.content.expect("Text display requires content"),
        }
    }
}

impl Default for TextDisplayBuilder {
    fn default() -> Self {
        Self::new()
    }
}
