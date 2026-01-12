use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;

#[derive(WebComponent)]
#[web_component(name = "my-counter", observed_attributes = ["value"])]
pub struct MyCounter {
    count: i32,
    element: Option<web_sys::HtmlElement>,
}

impl MyCounter {
    fn new() -> Self {
        Self { count: 0, element: None }
    }

    fn on_click(&mut self) {
        self.count += 1;
        if let Some(el) = &self.element {
            if let Some(shadow) = el.shadow_root() {
                if let Ok(Some(count_el)) = shadow.query_selector(".count") {
                    count_el.set_text_content(Some(&format!("Count: {}", self.count)));
                }
            }
        }
    }
}

impl WebComponent for MyCounter {
    fn attach(&mut self, element: &web_sys::HtmlElement) {
        self.element = Some(element.clone());
    }

    fn connected(&mut self) {
        let element = self.element.as_ref().unwrap();
        
        let shadow = element
            .attach_shadow(&web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open))
            .unwrap();
        
        shadow.set_inner_html(r#"
            <style>
                :host { display: inline-block; padding: 10px; border: 1px solid #ccc; }
                button { padding: 5px 10px; cursor: pointer; }
            </style>
            <button class="increment">Click me!</button>
            <div class="count">Count: 0</div>
        "#);

        // Wire up click
        // Note: this leaks one closure per connected call - not an issue for most use cases
        let element_copy = element.clone();
        let onclick = Closure::wrap(Box::new(move || {
            MyCounter::with_element(&element_copy, |c| c.on_click());
        }) as Box<dyn FnMut()>);

        shadow
            .query_selector(".increment")
            .unwrap()
            .unwrap()
            .add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())
            .unwrap();
        onclick.forget();
    }
}