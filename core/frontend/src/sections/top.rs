use gloo::{events::EventListener, utils::window};
use web_sys::{HtmlElement, ScrollBehavior, ScrollToOptions};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct TopSectionProperties {
    pub scroller_ref: NodeRef
}

#[function_component(TopSection)]
pub fn top_section(props: &TopSectionProperties) -> Html {
    let show_chevron = use_state(|| true);

    let scroller_element = props
        .scroller_ref
        .cast::<HtmlElement>()
        .expect("A main scroller element to exist.");

    {
        let scroller_element = scroller_element.clone();
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

    let on_click_chevron = {
        Callback::from(move |_| {
            let window_height = window()
                .inner_height()
                .ok()
                .map(|height| height.as_f64())
                .flatten();

            let Some(height) = window_height else {
                return;
            };

            let scroll_options = ScrollToOptions::new();
            scroll_options.set_top(height);
            scroll_options.set_behavior(ScrollBehavior::Smooth);

            scroller_element.scroll_to_with_scroll_to_options(&scroll_options);
        })
    };

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
                    onclick={on_click_chevron}
                />
            }
        </section>
    }
}
