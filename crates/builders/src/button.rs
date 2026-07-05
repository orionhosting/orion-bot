use twilight_model::{
    channel::message::{
        EmojiReactionType,
        component::{Button, ButtonStyle},
    },
    id::{
        Id,
        marker::{EmojiMarker, SkuMarker},
    },
};

/// Builder for buttons.
pub struct ButtonBuilder {
    style: ButtonStyle,
    custom_id: Option<String>,
    url: Option<String>,
    label: Option<String>,
    emoji: Option<EmojiReactionType>,
    disabled: bool,
    sku_id: Option<Id<SkuMarker>>,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            style: ButtonStyle::Primary,
            custom_id: None,
            url: None,
            label: None,
            emoji: None,
            disabled: false,
            sku_id: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn primary(mut self) -> Self {
        self.style = ButtonStyle::Primary;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.style = ButtonStyle::Secondary;
        self
    }

    pub fn success(mut self) -> Self {
        self.style = ButtonStyle::Success;
        self
    }

    pub fn danger(mut self) -> Self {
        self.style = ButtonStyle::Danger;
        self
    }

    pub fn link(mut self, url: impl Into<String>) -> Self {
        self.style = ButtonStyle::Link;
        self.url = Some(url.into());
        self
    }

    /// A premium button. Sets the SKU id and the style.
    pub fn premium(mut self, sku_id: Id<SkuMarker>) -> Self {
        self.style = ButtonStyle::Premium;
        self.sku_id = Some(sku_id);
        self
    }

    /// Sets the custom ID. (cannot be used for link and premium buttons).
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn unicode_emoji(mut self, emoji: impl Into<String>) -> Self {
        self.emoji = Some(EmojiReactionType::Unicode { name: emoji.into() });
        self
    }

    pub fn custom_emoji(mut self, id: Id<EmojiMarker>, animated: bool) -> Self {
        self.emoji = Some(EmojiReactionType::Custom {
            animated,
            id,
            name: None,
        });
        self
    }

    pub fn build(self) -> Button {
        Button {
            id: None,
            custom_id: self.custom_id,
            disabled: self.disabled,
            emoji: self.emoji,
            label: self.label,
            style: self.style,
            url: self.url,
            sku_id: self.sku_id,
        }
    }
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
