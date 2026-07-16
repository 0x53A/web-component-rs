use wasm_bindgen::prelude::*;

mod label;
mod shared_state;
mod textbox;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(
        &"Fifth component module loaded - inter-component communication enabled".into(),
    );

    // Register both components
    textbox::setup_textinput();
    label::setup_textdisplay();
}
