use twilight_model::channel::message::{AllowedMentions, Component, Embed, MessageFlags};

/// A reply helper.
///
/// `allowed_mentions` defaults to "none" (no pings).
pub struct Reply {
    pub(crate) content: Option<String>,
    pub(crate) embeds: Vec<Embed>,
    pub(crate) components: Vec<Component>,
    pub(crate) flags: MessageFlags,
    pub(crate) allowed_mentions: Option<AllowedMentions>,
}

impl Reply {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: Vec::new(),
            components: Vec::new(),
            flags: MessageFlags::empty(),
            allowed_mentions: Some(AllowedMentions::default()),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    /// Same as `components`, but also add the IS_COMPONENTS_V2 flag.
    pub fn components_v2(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self.flags |= MessageFlags::IS_COMPONENTS_V2;
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        if ephemeral {
            self.flags |= MessageFlags::EPHEMERAL;
        } else {
            self.flags &= !MessageFlags::EPHEMERAL;
        }
        self
    }

    /// Override the allowed mentions.
    pub fn allowed_mentions(mut self, mentions: AllowedMentions) -> Self {
        self.allowed_mentions = Some(mentions);
        self
    }
}

impl Default for Reply {
    fn default() -> Self {
        Self::new()
    }
}
