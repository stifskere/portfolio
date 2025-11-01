use tracing::Level as TracingLevel;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer as ts_layer;
use tracing_subscriber::{prelude::*, registry as ts_registry};
use tracing_web::MakeWebConsoleWriter;
use yew::Renderer;

use app::App;

mod app;
mod sections;
mod components;
mod utils;

fn main() {
    let fmt_layer = ts_layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", TracingLevel::DEBUG)
                .with_default(TracingLevel::TRACE),
        );

    ts_registry()
        .with(fmt_layer)
        .init();

    Renderer::<App>::new()
        .render();
}
