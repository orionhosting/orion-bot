use std::collections::HashMap;

use crate::Locale;

/// Convert a locale to a Discord locale.
///
/// See https://docs.discord.com/developers/reference#locales
pub fn get_discord_locale(locale: &Locale) -> &'static str {
    match *locale {
        Locale::En => "en-US",
        Locale::Fr => "fr",
        Locale::Es => "es-ES",
        Locale::De => "de",
        Locale::Lt => "lt",
    }
}

/// Convert a Discord locale to a locale.
///
/// See https://docs.discord.com/developers/reference#locales
pub fn from_discord_locale(locale: &str) -> Locale {
    match locale {
        "en-US" | "en-GB" => Locale::En,
        "fr" => Locale::Fr,
        "es-ES" | "es-419" => Locale::Es,
        "de" => Locale::De,
        "lt" => Locale::Lt,
        _ => Locale::DEFAULT,
    }
}

/// Convert a locale to a Orion Docs locale.
pub fn get_docs_locale(locale: &Locale) -> &'static str {
    match *locale {
        Locale::Fr => "fr",
        _ => "en",
    }
}

/// Get all translations for a key.
pub fn localize_key<F: Fn(Locale) -> String>(t: F) -> HashMap<String, String> {
    let mut localizations = HashMap::new();
    for locale in Locale::ALL {
        localizations.insert(get_discord_locale(locale).to_string(), t(*locale));
    }
    localizations
}
