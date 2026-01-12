use wasm_bindgen::prelude::*;

mod egui;
mod simple;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Web component module loaded".into());

    simple::setup_mycounter();
    egui::setup_eguicomponent();
}
