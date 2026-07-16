use egui::Color32;
use egui_web_component::EguiMount;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;

use crate::shared_state;

#[derive(WebComponent)]
#[web_component(name = "text-display")]
pub struct TextDisplay {
    element: Option<web_sys::HtmlElement>,
    mount: Option<EguiMount>,
}

impl TextDisplay {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            mount: None,
        }
    }
}

impl WebComponent for TextDisplay {
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
                Box::new(|cc| Ok(Box::new(TextDisplayApp::new(cc)))),
            )
            .await;

            match result {
                Ok(mount) => {
                    TextDisplay::with_element(&element_copy, |comp| {
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

pub struct TextDisplayApp {
    last_displayed_text: String,
}

impl TextDisplayApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_web_component::install_fonts(&cc.egui_ctx);
        Self {
            last_displayed_text: String::new(),
        }
    }
}

impl eframe::App for TextDisplayApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Read the shared text on every frame
        let current_text = shared_state::get_shared_text();

        egui::Frame::default()
            .fill(Color32::from_rgb(0x11, 0x12, 0x14))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading("Text Display Component");
                ui.add_space(10.0);

                ui.label("Synchronized text from input:");
                ui.add_space(5.0);

                // Display the shared text in a prominent way
                egui::Frame::default()
                    .fill(Color32::from_rgb(0x1b, 0x1c, 0x1f))
                    .stroke(egui::Stroke::new(
                        2.0_f32,
                        Color32::from_rgb(0xff, 0x5a, 0x1f),
                    ))
                    .inner_margin(15.0)
                    .corner_radius(0.0)
                    .show(ui, |ui| {
                        if current_text.is_empty() {
                            ui.label(
                                egui::RichText::new("(waiting for input...)")
                                    .italics()
                                    .color(Color32::from_rgb(0x9a, 0x9a, 0xa0))
                                    .size(18.0),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(&current_text)
                                    .color(Color32::from_rgb(0xff, 0x5a, 0x1f))
                                    .size(20.0)
                                    .strong(),
                            );
                        }
                    });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                // Show a visual indicator when text changes
                if current_text != self.last_displayed_text {
                    self.last_displayed_text = current_text.clone();
                }

                ui.horizontal(|ui| {
                    ui.label("Status:");
                    if current_text.is_empty() {
                        ui.label(
                            egui::RichText::new("Waiting for input")
                                .color(Color32::from_rgb(0x9a, 0x9a, 0xa0)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("✓ Receiving updates")
                                .color(Color32::from_rgb(0xe8, 0x43, 0x2b)),
                        );
                    }
                });
            });

        // Request continuous updates to show changes immediately
        ui.ctx().request_repaint();
    }
}
