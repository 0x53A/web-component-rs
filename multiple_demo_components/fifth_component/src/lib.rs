use wasm_bindgen::prelude::*;

mod shared_state;
mod textbox;
mod label;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Fifth component module loaded - inter-component communication enabled".into());

    // Register both components
    textbox::setup_textinput();
    label::setup_textdisplay();
}
