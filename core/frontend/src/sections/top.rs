use yew::prelude::*;

#[function_component(TopSection)]
pub fn top_section() -> Html {
    html! {
        <section class="top-section-container">
            <h1>{"Memw"}</h1>
            <p>
                {"This page is coming soon, "}
                <a
                    class="text-link"
                    href="/assets/provisional-cv.pdf"
                >
                    {"you can download my CV"}
                </a>
                {" anyways."}
            </p>

            <aside class="bottom-left-aside">
                <h2>{format!("This instance was built in {}", env!("BUILD_TIME"))}</h2>
            </aside>

            <aside class="bottom-right-aside">
                <h2>{"Expect Great Things ;D"}</h2>
                <p>{"Stay tunned about the "}
                    <a
                        class="text-link"
                        href="https://github.com/stifskere/portfolio"
                    >
                        {"portfolio development."}
                    </a>
                </p>
            </aside>
        </section>
    }
}
