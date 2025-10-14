use yew::prelude::*;

use crate::sections::top::TopSection;
use crate::sections::showcase::ShowcaseSection;

#[function_component(App)]
pub fn app() -> Html {
    let scroller_ref = use_node_ref();

    html! {
        <main ref={scroller_ref.clone()}>
            <TopSection scroller_ref={scroller_ref.clone()} />
            <ShowcaseSection />
        </main>
    }
}
