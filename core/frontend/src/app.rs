use yew::prelude::*;

use crate::components::socials_bar::SocialsBar;

use crate::sections::top::TopSection;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <SocialsBar />
            <TopSection />
        </main>
    }
}
