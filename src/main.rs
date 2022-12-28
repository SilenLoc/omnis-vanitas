#![warn(clippy::all, rust_2018_idioms)]
#[cfg(target_arch = "wasm32")]
use omnis_vanitas::TemplateApp;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default()


    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}


