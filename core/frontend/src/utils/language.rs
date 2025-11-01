use std::str::FromStr;

use gloo::utils::window;
use translatable::Language;


static ALLOWED_LANGUAGES: &[Language; 3] = &[
    Language::EN,
    Language::ES,
    Language::CA
];

pub fn inferred_browser_language() -> Language {
    let fallback = ALLOWED_LANGUAGES[0].clone(); // always exists.

    window()
        .navigator()
        .language()
        .and_then(|lang| lang.split_once("-").map(|(lang, _)| lang.to_string()))
        .map(|lang| Language::from_str(lang.as_str()))
        .unwrap_or(Ok(fallback.clone()))
        .unwrap_or(fallback)
}
