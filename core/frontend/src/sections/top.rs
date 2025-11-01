use translatable::translation;
use yew::prelude::*;

use crate::utils::language::inferred_browser_language;

#[function_component(TopSection)]
pub fn top_section() -> Html {
    let browser_language = use_state(inferred_browser_language);

    // TODO: Implement smiling face.
    // TODO: Translate CV in three langauges.
    // TODO: Add rustc version footer.

    html! {
        <section class="top-section-container">
            <h1>{"Memw"}</h1>
            <p>
                {
                    translation!(
                        (*browser_language).clone(),
                        static sections::top::subtitle::first_p
                    )
                        .expect("Translation to exist.")
                }
                <a
                    class="text-link"
                    href="/assets/provisional-cv.pdf"
                >
                    {
                        translation!(
                            (*browser_language).clone(),
                            static sections::top::subtitle::cv_link
                        )
                            .expect("Translation to exist.")
                    }
                </a>
                {
                    translation!(
                        (*browser_language).clone(),
                        static sections::top::subtitle::last_p
                    )
                        .expect("Translation to exist.")
                }
            </p>

            <aside class="bottom-left-aside">
                <h2>
                    {
                        translation!(
                            (*browser_language).clone(),
                            static sections::top::instance_build,
                            build_time = env!("BUILD_TIME")
                        )
                            .expect("Translation to exist.")
                    }
                </h2>
                <p>{env!("RUSTC_VERSION")}</p>
            </aside>

            <aside class="bottom-right-aside">
                <h2>
                    {
                        translation!(
                            (*browser_language).clone(),
                            static sections::top::motivational_quote,
                            smiling_face = ";D"
                        )
                            .expect("Translation to exist.")
                    }
                </h2>
                <p>
                    {
                        translation!(
                            (*browser_language).clone(),
                            static sections::top::portfolio_development::first_p
                        )
                            .expect("Translation to exist.")
                    }
                    <a
                        class="text-link"
                        href="https://github.com/stifskere/portfolio"
                    >
                        {
                            translation!(
                                (*browser_language).clone(),
                                static sections::top::portfolio_development::link
                            )
                                .expect("Translation to exist.")
                        }
                    </a>
                </p>
            </aside>
        </section>
    }
}
