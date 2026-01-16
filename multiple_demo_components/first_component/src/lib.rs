use wasm_bindgen::prelude::*;

mod greeting;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"First component module loaded".into());

    greeting::setup_greetingcard();
}
