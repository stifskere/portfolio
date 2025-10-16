use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct PresentationLinkProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub id: String,

    pub icon_id: IconId,
    pub text: String,
    pub description: String,

    #[prop_or_default]
    pub on_click: Callback<String>
}

#[function_component(PresentationLink)]
pub fn presentation_link(props: &PresentationLinkProps) -> Html {
    let on_click = {
        let id = format!("{}", props.id);
        let callback = props.on_click.clone();
        Callback::from(move |_| {
            callback.emit(id.clone());
        })
    };

    html! {
        <div
            class={classes!("presentation-link", &props.class)}
            id={props.id.clone()}
            onclick={on_click}
        >
            <Icon icon_id={props.icon_id} />
            <div>
                <p>{props.text.clone()}</p>
                <span>{props.description.clone()}</span>
            </div>
        </div>
    }
}

