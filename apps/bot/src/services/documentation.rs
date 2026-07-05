use anyhow::Result;
use serde::Deserialize;
use std::time::Duration;
use tracing::info;

use moka::sync::Cache as MokaCache;

use crate::{config::Config, services::ServiceContext};

const SITEMAP_KEY: &str = "docs-sitemap";
const SITEMAP_TTL_SECS: u64 = 3600;

#[derive(Debug, Deserialize, Clone)]
pub struct DocPage {
    pub name: String,
    pub url: String,
    pub description: String,
    pub lang: String,
}

/// The documentation service.
///
/// Get data from the Orion Docs API.
pub struct DocumentationService {
    ctx: ServiceContext,
    sitemap: MokaCache<String, Vec<DocPage>>,
}

impl DocumentationService {
    pub(super) fn new(ctx: ServiceContext) -> Self {
        Self {
            ctx,
            sitemap: MokaCache::builder()
                .max_capacity(1)
                .time_to_live(Duration::from_secs(SITEMAP_TTL_SECS))
                .build(),
        }
    }

    /// Returns the cached docs sitemap.
    pub async fn get_documentation_sitemap(&self) -> Result<Vec<DocPage>> {
        if let Some(pages) = self.sitemap.get(SITEMAP_KEY) {
            return Ok(pages);
        }

        info!("Fetching documentation sitemap");

        let url = format!("{}/api/pages", Config::DOCS_URL);
        let pages: Vec<DocPage> = self.ctx.http.get(&url).send().await?.json().await?;

        self.sitemap.insert(SITEMAP_KEY.to_string(), pages.clone());
        Ok(pages)
    }
}
