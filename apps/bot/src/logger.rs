use std::panic;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes the logger and panic hook.
///
/// The WorkerGuard must be kept alive otherwise not logs will be written in the files.
pub fn init_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        // Console
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        // File
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking_appender))
        .init();

    // Route panics to tracing
    panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();

        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic payload type".to_string()
        };

        let location = panic_info
            .location()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "Unknown location".to_string());
        tracing::error!(%location, message = %message, "A fatal panic occurred");
    }));

    guard
}
