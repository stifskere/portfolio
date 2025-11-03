use yew::prelude::*;
use yew_icons::{Icon, IconId};
use translatable::translation;

use crate::utils::application::context::AppContext;

#[derive(Properties, PartialEq)]
pub struct SocialsBarProps {
    #[prop_or_default]
    pub class: String
}

#[function_component(SocialsBar)]
pub fn socials_bar(props: &SocialsBarProps) -> Html {
    let app_context = use_context::<AppContext>()
        .expect("App to not be broken.");

    html! {
        <nav class={classes!("socials-bar-container", &props.class)}>
            <div class="socials-div">
                <a href="https://github.com/stifskere" target="_blank">
                    <Icon icon_id={IconId::BootstrapGithub} />
                </a>
                <a href="mailto:esteve@memw.es" target="_blank">
                    <Icon icon_id={IconId::BootstrapEnvelopeAtFill} />
                </a>
                <a href="https://www.linkedin.com/in/memw" target="_blank">
                    <Icon icon_id={IconId::BootstrapLinkedin} />
                </a>
            </div>
            <div class="info-div">
                // Arrow
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="200 200 400 400">
                    <defs>
                        <marker id="arrow" markerWidth="6" markerHeight="6" viewBox="0 0 4.5 4.5"
                                refX="2.25" refY="2.25" orient="auto" markerUnits="strokeWidth">
                        <polygon points="0,4.5 1.5,2.25 0,0 4.5,2.25" fill="white"/>
                        </marker>
                    </defs>

                    <g stroke="white" fill="none" stroke-width="15" stroke-linecap="round" stroke-linejoin="round"
                        transform="matrix(-0.9986295,0.052336,-0.052336,-0.9986295,824.3862,819.5174)">
                        <path d="M282.3725 283.1612 Q525.3725 264.1612 517.3725 518.1612" marker-end="url(#arrow)"/>
                    </g>
                </svg>

                <p>
                    {
                        translation!(
                            (*app_context.language).clone(),
                            static components::socials_bar::check_out_text
                        )
                            .expect("Translation to exist.")
                    }
                </p>
            </div>
        </nav>
    }
}
