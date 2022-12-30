#![warn(clippy::all, rust_2018_idioms)]
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use omnis_vanitas::TemplateApp;
use tokio::runtime::Runtime;

// When compiling natively:
//[cfg(not(target_arch = "wasm32"))]
fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "omni vanitas",
        native_options,
        Box::new(|_| Box::new(omnis_vanitas::TemplateApp::new())),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });
    // Make sure panics are logged using `console.error`.

    use eframe::{Theme, WebGlContextOption};
    use futures::executor;
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions {
        follow_system_theme: false,
        default_theme: Theme::Dark,

        webgl_context_option: WebGlContextOption::BestFirst,
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "omni_vanitas_canvas", // hardcode it
            web_options,
            Box::new(|_| Box::new(TemplateApp::new())),
        )
        .await
        .expect("failed to start eframe");
    });
}
