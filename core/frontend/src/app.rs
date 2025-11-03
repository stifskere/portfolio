use std::rc::Rc;

use yew::prelude::*;

use crate::components::language_selection::LanguageSelection;
use crate::components::socials_bar::SocialsBar;

use crate::sections::top::TopSection;
use crate::utils::application::context::{AppContext, InnerAppContext};
use crate::utils::language::inferred_browser_language;

#[function_component(App)]
pub fn app() -> Html {

    let app_context = Rc::new(InnerAppContext {
        language: use_state(inferred_browser_language)
    });

    html! {
        <ContextProvider<AppContext> context={app_context}>
            <main>
                <LanguageSelection class="language-selection" />
                <SocialsBar class="socials-bar" />
                <TopSection />
            </main>
        </ContextProvider<AppContext>>
    }
}
