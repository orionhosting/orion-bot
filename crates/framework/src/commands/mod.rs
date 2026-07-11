mod context;
mod meta;
mod registry;
mod reply;
mod result;

use async_trait::async_trait;

pub use context::*;
pub use meta::*;
pub use registry::*;
pub use reply::*;
pub use result::*;

/// Every command implements this trait.
///
/// `App` is your app, which will be available in every handler with `ctx.app`.
#[async_trait]
pub trait Command<App: Send + Sync + 'static>: Send + Sync {
    /// Returns the command metadata.
    fn meta(&self) -> &CommandMeta;

    /// Handle a chat input command.
    async fn handle_command(&self, ctx: &CommandContext<'_, App>) -> CommandResult;

    /// Handle autocomplete.
    async fn handle_autocomplete(&self, ctx: &CommandContext<'_, App>) -> CommandResult {
        let _ = ctx;
        Ok(())
    }

    /// Handle a component interaction.
    async fn handle_component(&self, ctx: &CommandContext<'_, App>) -> CommandResult {
        let _ = ctx;
        Ok(())
    }
}
