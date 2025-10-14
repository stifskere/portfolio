use gloo::events::EventListener;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct TopSectionProperties {
    pub scroller_ref: NodeRef
}

#[function_component(TopSection)]
pub fn top_section(props: &TopSectionProperties) -> Html {
    let show_chevron = use_state(|| true);

    {
        let scroller_element = props
            .scroller_ref
            .cast::<HtmlElement>()
            .expect("A main scroller element to exist.");

        let show_chevron = show_chevron.clone();

        use_effect(move || {
            let scroll_listener = EventListener::new(
                &scroller_element.clone(),
                "scroll",
                move |_| {
                    show_chevron.set(scroller_element.scroll_top() < 10);
                }
            );

            || {
                drop(scroll_listener);
            }
        })
    }

    html! {
        <section class="main-section top-section">
            <div class="top-section-titles">
                <h1>{"MEMW"}</h1>
                <h2>{"Full Stack Developer"}</h2>
            </div>
            if *show_chevron {
                <Icon
                    icon_id={IconId::LucideChevronDown}

                    class="top-section-down"
                />
            }
        </section>
    }
}
