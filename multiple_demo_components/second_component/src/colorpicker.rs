use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

#[derive(WebComponent)]
#[web_component(name = "color-picker")]
pub struct ColorPicker {
    element: Option<web_sys::HtmlElement>,
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorPicker {
    fn new() -> Self {
        Self {
            element: None,
            red: 61,
            green: 180,
            blue: 90,
        }
    }

    fn update_color(&mut self) {
        if let Some(el) = &self.element {
            if let Some(shadow) = el.shadow_root() {
                let hex_color = format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue);
                let rgb_text = format!("RGB({}, {}, {})", self.red, self.green, self.blue);

                if let Ok(Some(preview)) = shadow.query_selector(".color-preview") {
                    preview
                        .dyn_ref::<web_sys::HtmlElement>()
                        .unwrap()
                        .style()
                        .set_property("background-color", &hex_color)
                        .unwrap();
                }

                if let Ok(Some(hex_text)) = shadow.query_selector(".hex-text") {
                    hex_text.set_text_content(Some(&hex_color));
                }

                if let Ok(Some(rgb_text_el)) = shadow.query_selector(".rgb-text") {
                    rgb_text_el.set_text_content(Some(&rgb_text));
                }
            }
        }
    }

    fn on_slider_change(&mut self, channel: &str, value: u8) {
        match channel {
            "red" => self.red = value,
            "green" => self.green = value,
            "blue" => self.blue = value,
            _ => {}
        }
        self.update_color();
    }
}

impl WebComponent for ColorPicker {
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
                    border-left: 4px solid #e8432b;
                    background: #1b1c1f;
                    color: #ededed;
                    font-family: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
                }
                .color-preview {
                    width: 100%;
                    height: 100px;
                    border: 1px solid #35363c;
                    margin-bottom: 15px;
                    background-color: #3db45a;
                }
                .slider-group {
                    margin: 10px 0;
                }
                .slider-group label {
                    display: inline-block;
                    width: 60px;
                    font-weight: bold;
                }
                input[type="range"] {
                    width: 200px;
                    margin: 0 10px;
                    accent-color: #e8432b;
                }
                .value-display {
                    display: inline-block;
                    width: 35px;
                    text-align: right;
                    color: #9a9aa0;
                }
                .color-info {
                    margin-top: 15px;
                    padding: 10px;
                    background: #111214;
                    border: 1px solid #35363c;
                    text-align: center;
                }
                .hex-text {
                    font-size: 20px;
                    font-weight: bold;
                    color: #ff5a1f;
                }
                .rgb-text {
                    font-size: 14px;
                    color: #9a9aa0;
                    margin-top: 5px;
                }
            </style>
            <div class="color-preview"></div>
            <div class="slider-group">
                <label style="color: #e8432b;">Red:</label>
                <input type="range" class="red-slider" min="0" max="255" value="61">
                <span class="value-display red-value">61</span>
            </div>
            <div class="slider-group">
                <label style="color: #ff5a1f;">Green:</label>
                <input type="range" class="green-slider" min="0" max="255" value="180">
                <span class="value-display green-value">180</span>
            </div>
            <div class="slider-group">
                <label style="color: #9a9aa0;">Blue:</label>
                <input type="range" class="blue-slider" min="0" max="255" value="90">
                <span class="value-display blue-value">90</span>
            </div>
            <div class="color-info">
                <div class="hex-text">#3DB45A</div>
                <div class="rgb-text">RGB(61, 180, 90)</div>
            </div>
        "#);

        // Set up event listeners for each slider
        for (channel, class) in [("red", "red"), ("green", "green"), ("blue", "blue")] {
            let element_copy = element.clone();
            let channel_copy = channel.to_string();
            let value_class = format!("{}-value", class);

            let oninput = Closure::wrap(Box::new(move |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        let value = input.value().parse::<u8>().unwrap_or(0);

                        // Update value display
                        if let Some(el) = element_copy.shadow_root() {
                            if let Ok(Some(value_el)) = el.query_selector(&format!(".{}", value_class)) {
                                value_el.set_text_content(Some(&value.to_string()));
                            }
                        }

                        let channel_ref = channel_copy.clone();
                        ColorPicker::with_element(&element_copy, move |c| {
                            c.on_slider_change(&channel_ref, value);
                        });
                    }
                }
            }) as Box<dyn FnMut(_)>);

            let slider_class = format!(".{}-slider", class);
            shadow
                .query_selector(&slider_class)
                .unwrap()
                .unwrap()
                .add_event_listener_with_callback("input", oninput.as_ref().unchecked_ref())
                .unwrap();
            oninput.forget();
        }

        self.update_color();
    }
}
