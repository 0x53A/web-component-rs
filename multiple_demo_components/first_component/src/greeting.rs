use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

#[derive(WebComponent)]
#[web_component(name = "greeting-card", observed_attributes = ["greeting"])]
pub struct GreetingCard {
    element: Option<web_sys::HtmlElement>,
    greeting: String,
}

impl GreetingCard {
    fn new() -> Self {
        Self {
            element: None,
            greeting: "Hello".to_string(),
        }
    }

    fn on_input(&mut self, text: String) {
        if let Some(el) = &self.element {
            if let Some(shadow) = el.shadow_root() {
                if let Ok(Some(output)) = shadow.query_selector(".output") {
                    let greeting_text = if text.is_empty() {
                        format!("{}, stranger!", self.greeting)
                    } else {
                        format!("{}, {}!", self.greeting, text)
                    };
                    output.set_text_content(Some(&greeting_text));
                }
            }
        }
    }
}

impl WebComponent for GreetingCard {
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
                :host {
                    display: block;
                    padding: 20px;
                    border: 1px solid #35363c;
                    border-left: 4px solid #ff5a1f;
                    background: #1b1c1f;
                    color: #ededed;
                    font-family: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
                }
                .input-group {
                    margin-bottom: 15px;
                }
                label {
                    color: #9a9aa0;
                }
                input {
                    padding: 10px;
                    border: 1px solid #35363c;
                    background: #111214;
                    color: #ededed;
                    font-size: 16px;
                    width: 200px;
                }
                input:focus {
                    outline: none;
                    border-color: #ff5a1f;
                }
                .output {
                    font-size: 24px;
                    font-weight: bold;
                    color: #ff5a1f;
                    padding: 10px;
                    background: #111214;
                    border: 1px solid #35363c;
                    text-align: center;
                }
            </style>
            <div class="input-group">
                <label>Your name: <input type="text" class="name-input" placeholder="Enter your name"></label>
            </div>
            <div class="output">Hello, stranger!</div>
        "#);

        let element_copy = element.clone();
        let oninput = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    let value = input.value();
                    GreetingCard::with_element(&element_copy, |c| c.on_input(value));
                }
            }
        }) as Box<dyn FnMut(_)>);

        shadow
            .query_selector(".name-input")
            .unwrap()
            .unwrap()
            .add_event_listener_with_callback("input", oninput.as_ref().unchecked_ref())
            .unwrap();
        oninput.forget();
    }

    fn attribute_changed(&mut self, name: &str, _old_value: Option<&str>, new_value: Option<&str>) {
        if name == "greeting" {
            if let Some(new_greeting) = new_value {
                self.greeting = new_greeting.to_string();
                // Update the display if there's current input
                if let Some(el) = &self.element {
                    if let Some(shadow) = el.shadow_root() {
                        if let Ok(Some(input)) = shadow.query_selector(".name-input") {
                            if let Ok(input_el) = input.dyn_into::<HtmlInputElement>() {
                                self.on_input(input_el.value());
                            }
                        }
                    }
                }
            }
        }
    }
}
