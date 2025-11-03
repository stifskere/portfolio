use yew::prelude::*;

use crate::utils::language::{allowed_browser_languages, fallback_browser_language};
use crate::utils::application::context::AppContext;

#[derive(Properties, PartialEq)]
pub struct LanguageSelectionProps {
    #[prop_or_default]
    pub class: String
}

#[function_component(LanguageSelection)]
pub fn language_selection(props: &LanguageSelectionProps) -> Html {
    let app_context = use_context::<AppContext>()
        .expect("App to not be broken.");

    let on_click = {
        let app_context = app_context.clone();

        Callback::from(move |_| {
            let allowed_languages = allowed_browser_languages();
            let language = allowed_languages
                .iter()
                .enumerate()
                .find(|(_, lang)| *app_context.language == **lang)
                .map(|(i, _)| &allowed_languages[(i + 1) % allowed_languages.len()])
                .unwrap_or_else(|| fallback_browser_language())
                .clone();

            app_context.set_language(language);
        })
    };

    html! {
        <aside class={classes!("language-selection-container", &props.class)}>
            <button onclick={on_click}>
                {app_context.language.to_string()}
            </button>
        </aside>
    }
}
