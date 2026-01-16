use wasm_bindgen::prelude::*;

mod slider;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Third component module loaded".into());

    slider::setup_eguislider();
}
