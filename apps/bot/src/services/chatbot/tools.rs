//! Tools for the chatbot.
//! Currently there is only one: `fetch_docs` to fetch the docs content.

use rig_core::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::Mutex;
use twilight_model::{
    channel::message::AllowedMentions,
    id::{Id, marker::MessageMarker},
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    app::App,
    config::{Config, Emojis, Palette},
};

#[derive(Deserialize)]
pub struct FetchDocsArgs {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct FetchDocsOutput {
    pub text: String,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct FetchDocsError(String);

pub struct FetchDocsTool {
    pub app: Arc<App>,
    pub channel_id: Id<twilight_model::id::marker::ChannelMarker>,
    pub message_id: Id<twilight_model::id::marker::MessageMarker>,
    pub source_page: Arc<Mutex<Option<(String, String, Id<MessageMarker>)>>>, // (Name, URL, ID)
    pub has_been_called: AtomicBool,
}

impl Tool for FetchDocsTool {
    const NAME: &'static str = "fetch_docs";

    type Error = FetchDocsError;
    type Args = FetchDocsArgs;
    type Output = FetchDocsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get the raw MDX of a page from the Orion documentation website."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": format!("The URL of the page to get. ('{}' followed by the path, like /fr/...)", Config::DOCS_URL)
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if self.has_been_called.swap(true, Ordering::Relaxed) {
            return Err(FetchDocsError(
                "You have already fetched documentation during this turn. Read the data you just retrieved to formulate your answer, do not call this tool again."
                .into()
            ));
        }

        let sitemap = self
            .app
            .services
            .documentation
            .get_documentation_sitemap()
            .await
            .map_err(|_| FetchDocsError("Could not fetch the documentation sitemap.".into()))?;

        let page = sitemap
            .into_iter()
            .find(|p| p.url == args.url)
            .ok_or_else(|| {
                FetchDocsError("Could not find the requested page in the documentation.".into())
            })?;

        // Send the 'reading' message
        let loading_msg = self
            .app
            .discord
            .create_message(self.channel_id)
            .allowed_mentions(Some(&AllowedMentions::default()))
            .reply(self.message_id)
            .embeds(&[EmbedBuilder::new()
                .color(Palette::GREEN.int)
                .description(format!(
                    "{} Reading the documentation...",
                    Emojis::LOADER_GREEN
                ))
                .build()])
            .await
            .map_err(|_| FetchDocsError("Could not send reading indicator message.".into()))?
            .model()
            .await
            .map_err(|_| FetchDocsError("Could not parse reading indicator response.".into()))?;

        // Lock the mutex
        *self.source_page.lock().await =
            Some((page.name.clone(), page.url.clone(), loading_msg.id));

        let mdx_url = format!("{}.mdx", page.url);
        let text = reqwest::get(&mdx_url)
            .await
            .map_err(|_| FetchDocsError("Could not reach the documentation server.".into()))?
            .text()
            .await
            .map_err(|_| FetchDocsError("Could not parse the documentation content.".into()))?;

        Ok(FetchDocsOutput { text })
    }
}
