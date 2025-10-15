use yew::prelude::*;

use crate::sections::top::TopSection;
use crate::sections::showcase::ShowcaseSection;
use crate::sections::presentation::PresentationSection;

#[function_component(App)]
pub fn app() -> Html {
    let scroller_ref = use_node_ref();

    html! {
        <main ref={scroller_ref.clone()}>
            <TopSection scroller_ref={scroller_ref.clone()} />
            <PresentationSection />
            <ShowcaseSection />
        </main>
    }
}
