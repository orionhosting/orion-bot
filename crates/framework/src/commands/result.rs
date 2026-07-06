use std::fmt;

use anyhow::anyhow;

use crate::commands::context::OptionError;

/// The result of a command handler.
pub type CommandResult = Result<(), CommandError>;

/// The error type for CommandResult.
#[derive(Debug)]
pub enum CommandError {
    /// Any error.
    Anyhow(anyhow::Error),
    /// The user is missing a permission required to run the command.
    Permission(String),
    /// An error message.
    Message(String, bool),
    /// Command is restricted to the bot owner.
    NotOwner,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anyhow(err) => write!(f, "{:?}", err),
            Self::Permission(err) => write!(f, "Permission: {}", err),
            Self::Message(err, _) => write!(f, "Message: {}", err),
            Self::NotOwner => write!(f, "The user is not an owner"),
        }
    }
}

impl From<twilight_http::Error> for CommandError {
    fn from(e: twilight_http::Error) -> Self {
        CommandError::Anyhow(anyhow!(e))
    }
}

impl From<anyhow::Error> for CommandError {
    fn from(e: anyhow::Error) -> Self {
        CommandError::Anyhow(e)
    }
}

impl From<OptionError> for CommandError {
    fn from(e: OptionError) -> Self {
        CommandError::Anyhow(anyhow!(e))
    }
}
