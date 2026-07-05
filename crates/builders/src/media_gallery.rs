use twilight_model::channel::message::component::{
    MediaGallery, MediaGalleryItem, UnfurledMediaItem,
};

/// Builder for media gallery items.
pub struct MediaGalleryItemBuilder {
    url: String,
    description: Option<String>,
    spoiler: bool,
}

impl MediaGalleryItemBuilder {
    fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: None,
            spoiler: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }

    fn build(self) -> MediaGalleryItem {
        MediaGalleryItem {
            media: UnfurledMediaItem {
                url: self.url,
                proxy_url: None,
                height: None,
                width: None,
                content_type: None,
            },
            description: self.description,
            spoiler: Some(self.spoiler),
        }
    }
}

/// Builder for media galleries.
pub struct MediaGalleryBuilder {
    items: Vec<MediaGalleryItem>,
}

impl MediaGalleryBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Shorthand for [`MediaGalleryBuilder::add_item`].
    pub fn add_item_from_url(mut self, url: impl Into<String>) -> Self {
        self.items.push(MediaGalleryItemBuilder::new(url).build());
        self
    }

    pub fn add_item<F>(mut self, url: impl Into<String>, f: F) -> Self
    where
        F: FnOnce(MediaGalleryItemBuilder) -> MediaGalleryItemBuilder,
    {
        self.items
            .push(f(MediaGalleryItemBuilder::new(url)).build());
        self
    }

    pub fn build(self) -> MediaGallery {
        MediaGallery {
            id: None,
            items: self.items,
        }
    }
}

impl Default for MediaGalleryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
