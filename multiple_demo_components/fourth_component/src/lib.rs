use wasm_bindgen::prelude::*;

mod plotter;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Fourth component module loaded".into());

    plotter::setup_eguiplotter();
}
