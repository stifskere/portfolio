use palette::{rgb::channels::Rgba, Srgb};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextScrollerProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub id: Option<String>,

    pub text: String,
    #[prop_or(Srgb::new(0, 0, 0))]
    pub text_color: Srgb<u8>,
    #[prop_or(Srgb::new(255, 255, 255))]
    pub background_color: Srgb<u8>
}

#[function_component(TextScroller)]
pub fn text_scroller(props: &TextScrollerProps) -> Html {
    html! {
        <div
            class={classes!("text-scroller", &props.class)}
            id={props.id.clone()}
            style={format!(
                "--text-color: #{:08x}; --background-color: #{:08x};",
                props.text_color.into_u32::<Rgba>(),
                props.background_color.into_u32::<Rgba>()
            )}
        >
            { for (1..=10).collect::<Vec<_>>().iter().map(|key|
                html! {
                    <span key={*key}>{&props.text}</span>
                }
            ) }
        </div>
    }
}
