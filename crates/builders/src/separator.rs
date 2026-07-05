use twilight_model::channel::message::component::{Separator, SeparatorSpacingSize};

/// Builder for action rows.
pub struct SeparatorBuilder {
    divider: Option<bool>,
    spacing: Option<SeparatorSpacingSize>,
}

impl SeparatorBuilder {
    pub fn new() -> Self {
        Self {
            divider: None,
            spacing: None,
        }
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = Some(divider);
        self
    }

    pub fn spacing(mut self, spacing: SeparatorSpacingSize) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn build(self) -> Separator {
        Separator {
            id: None,
            divider: self.divider,
            spacing: self.spacing,
        }
    }
}

impl Default for SeparatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
