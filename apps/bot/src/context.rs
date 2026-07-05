use crate::{Locale, LocaleProvider, app::App, localization::from_discord_locale};

pub type CommandContext<'a> = framework::commands::CommandContext<'a, App>;

impl<'a> LocaleProvider for &CommandContext<'a> {
    fn i18n_locale(&self) -> crate::Locale {
        match &self.interaction.locale {
            Some(v) => from_discord_locale(v),
            None => Locale::DEFAULT,
        }
    }
}
