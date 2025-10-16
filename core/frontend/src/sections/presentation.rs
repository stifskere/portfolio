use yew::prelude::*;
use yew_icons::IconId;

use crate::components::text_scroller::TextScroller;
use crate::components::presentation_link::PresentationLink;

#[function_component(PresentationSection)]
pub fn presentation_section() -> Html {
    html! {
        <section class="main-section presentation-section">
            <TextScroller text="memw" class="presentation-top-scroller" />
            <article class="presentation-description">
                <div>
                    <h1>{"Hello There! :>"}</h1>
                    <p>{
                        r"
                            Lorem ipsum dolor sit amet consectetur adipiscing elit.
                            Quisque faucibus ex sapien vitae pellentesque sem placerat.
                            In id cursus mi pretium tellus duis convallis.
                            Tempus leo eu aenean sed diam urna tempor.
                            Pulvinar vivamus fringilla lacus nec metus bibendum egestas.
                            Iaculis massa nisl malesuada lacinia integer nunc posuere.
                            Ut hendrerit semper vel class aptent taciti sociosqu.
                            Ad litora torquent per conubia nostra inceptos himenaeos.
                        "
                    }</p>
                </div>
                <nav>
                    <PresentationLink
                        icon_id={IconId::LucideGithub}
                        text="GitHub"
                        description="Have you seen my projects!?"
                    />
                    <PresentationLink
                        icon_id={IconId::LucideLinkedin}
                        text="LinkedIn"
                        description="My professional development :<"
                    />
                    <PresentationLink
                        icon_id={IconId::LucideMail}
                        text="Email"
                        description="Want to do business? Email me!"
                    />
                    <PresentationLink
                        icon_id={IconId::LucidePhone}
                        text="WhatsApp"
                        description="Find email boring? Send me a WhatsApp!"
                    />
                    <PresentationLink
                        icon_id={IconId::LucideFolder}
                        text="Download CV"
                        description="Are you considering hiring me!?"
                    />
                </nav>
            </article>
        </section>
    }
}
