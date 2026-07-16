use egui::Color32;
use egui_web_component::EguiMount;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;

use crate::shared_state;

#[derive(WebComponent)]
#[web_component(name = "text-input")]
pub struct TextInput {
    element: Option<web_sys::HtmlElement>,
    mount: Option<EguiMount>,
}

impl TextInput {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            mount: None,
        }
    }
}

impl WebComponent for TextInput {
    fn attach(&mut self, element: &web_sys::HtmlElement) {
        self.element = Some(element.clone());
    }

    fn connected(&mut self) {
        let element = self.element.as_ref().unwrap().clone();
        let element_copy = element.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let result = EguiMount::connect(
                &element,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(TextInputApp::new(cc)))),
            )
            .await;

            match result {
                Ok(mount) => {
                    TextInput::with_element(&element_copy, |comp| {
                        comp.mount = Some(mount);
                    });
                }
                Err(e) => web_sys::console::error_1(&e),
            }
        });
    }

    fn disconnected(&mut self) {
        if let Some(mount) = self.mount.take() {
            mount.disconnect();
        }
    }
}

pub struct TextInputApp {
    text: String,
}

impl TextInputApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_web_component::install_fonts(&cc.egui_ctx);
        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for TextInputApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(0x11, 0x12, 0x14))
                    .inner_margin(15.0),
            )
            .show(ctx, |ui| {
                ui.heading("Text Input Component");
                ui.add_space(10.0);

                ui.label("Type something here:");
                ui.add_space(5.0);

                let response = ui.text_edit_singleline(&mut self.text);

                // Update shared state whenever the text changes
                if response.changed() {
                    shared_state::set_shared_text(self.text.clone());
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Character count:");
                    ui.label(format!("{}", self.text.len()));
                });

                if ui.button("Clear").clicked() {
                    self.text.clear();
                    shared_state::set_shared_text(self.text.clone());
                }

                ui.add_space(5.0);
                ui.label("The text display component below will update automatically!");
            });
    }
}
