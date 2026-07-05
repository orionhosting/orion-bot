pub mod chatbot;
pub mod documentation;
pub mod status;

use std::sync::Arc;

use chatbot::ChatbotService;
use documentation::DocumentationService;
use status::StatusService;
use twilight_http::Client as DiscordHttp;

#[derive(Clone)]
pub struct ServiceContext {
    pub application_id: u64,
    pub discord: Arc<DiscordHttp>,
    pub http: Arc<reqwest::Client>,
}

impl ServiceContext {
    pub fn new(application_id: u64, discord: Arc<DiscordHttp>, http: Arc<reqwest::Client>) -> Self {
        Self {
            application_id,
            discord,
            http,
        }
    }
}

pub struct ServiceManager {
    pub status: StatusService,
    pub chatbot: ChatbotService,
    pub documentation: DocumentationService,
}

impl ServiceManager {
    pub fn new(ctx: ServiceContext) -> Self {
        Self {
            status: StatusService::new(ctx.clone()),
            chatbot: ChatbotService::new(),
            documentation: DocumentationService::new(ctx),
        }
    }

    /// Called once the bot is ready.
    pub fn on_ready(&self) {
        self.status.on_ready();
        self.chatbot.on_ready();
    }
}
