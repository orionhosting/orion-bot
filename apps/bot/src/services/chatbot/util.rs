use regex::Regex;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn strip_self_mention(text: &str) -> String {
    // Replace `[a](b)` by `a` if `a == b`
    static LINK_RE: OnceLock<Regex> = OnceLock::new();
    let link_re = LINK_RE.get_or_init(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap());

    let mut result = link_re
        .replace_all(text, |caps: &regex::Captures| {
            if caps[1] == caps[2] {
                caps[1].to_string() // Replace with just the text
            } else {
                caps[0].to_string() // Keep the whole markdown
            }
        })
        .into_owned();

    // Remove prefixes
    static PREFIX_RE: OnceLock<Regex> = OnceLock::new();
    let prefix_re = PREFIX_RE.get_or_init(|| {
        Regex::new(r"^(@[\w.]+:\s*|@Orion Hosting:\s*|Orion Hosting:\s*|Content:\s*)").unwrap()
    });

    result = prefix_re.replace(&result, "").into_owned();

    result.trim().to_owned()
}

pub(super) fn truncate_to_chars(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

pub(super) fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
