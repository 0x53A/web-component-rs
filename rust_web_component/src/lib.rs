use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

pub fn next_id() -> u32 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn eval(code: &str) -> JsValue;
}

pub fn eval_js(code: &str) {
    eval(code);
}

pub trait WebComponent {
    /// Called from JS constructor. Store the element reference here.
    fn attach(&mut self, element: &web_sys::HtmlElement);
    
    /// Called when element is inserted into DOM
    fn connected(&mut self) {}
    
    /// Called when element is removed from DOM
    fn disconnected(&mut self) {}
    
    /// Called when element is moved to a new document
    fn adopted(&mut self) {}
    
    /// Called when an observed attribute changes
    fn attribute_changed(&mut self, _name: &str, _old: Option<&str>, _new: Option<&str>) {}
}