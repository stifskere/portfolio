use std::str::FromStr;

use gloo::utils::window;
use gloo::storage::{LocalStorage, Storage};
use translatable::Language;


static ALLOWED_LANGUAGES: &[Language; 4] = &[
    Language::EN,
    Language::ES,
    Language::CA,
    Language::RU
];

pub fn fallback_browser_language() -> &'static Language {
    &ALLOWED_LANGUAGES[0]
}

pub fn inferred_browser_language() -> Language {
    LocalStorage::get::<String>("language")
        .ok()
        .and_then(|language| Language::from_str(&language).ok())
        .take_if(|language| ALLOWED_LANGUAGES.contains(&language))
        .unwrap_or_else(|| {
            let language = window()
                .navigator()
                .language()
                .and_then(|lang| lang.split_once("-").map(|(lang, _)| lang.to_string()))
                .map(|lang| Language::from_str(lang.as_str()))
                .unwrap_or_else(|| Ok(fallback_browser_language().clone()))
                .unwrap_or_else(|_| fallback_browser_language().clone());

            set_browser_language(&language);

            language
        })
}

pub fn allowed_browser_languages() -> Vec<Language> {
    ALLOWED_LANGUAGES.to_vec()
}

pub fn set_browser_language(language: &Language) {
    if !ALLOWED_LANGUAGES.contains(language) {
        return;
    }

    // XXX: Remove unwraps.
    LocalStorage::set("language", language.to_string())
        .expect("LocalStorage to not be altered.");
}
