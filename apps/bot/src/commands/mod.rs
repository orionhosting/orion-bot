mod informations;
pub mod prelude;
mod private;

use std::sync::Arc;

use framework::commands::Command;

use crate::app::App;

/// All commands.
pub fn all() -> Vec<Arc<dyn Command<App>>> {
    vec![
        Arc::new(informations::ping::PingCommand),
        Arc::new(informations::help::HelpCommand),
        Arc::new(informations::account::AccountCommand),
        Arc::new(informations::docs::DocsCommand),
        Arc::new(informations::troubleshoot::TroubleshootCommand),
        Arc::new(private::config::ConfigCommand),
        Arc::new(private::user::UserCommand),
        Arc::new(private::panels::PanelsCommand),
    ]
}
