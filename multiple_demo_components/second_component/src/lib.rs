use wasm_bindgen::prelude::*;

mod colorpicker;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Second component module loaded".into());

    colorpicker::setup_colorpicker();
}
