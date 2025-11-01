use yew::prelude::*;

use crate::sections::top::TopSection;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <TopSection />
        </main>
    }
}
