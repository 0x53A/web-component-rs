use egui::Color32;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::shared_state;

#[derive(WebComponent)]
#[web_component(name = "text-display")]
pub struct TextDisplay {
    element: Option<web_sys::HtmlElement>,
    runner: Option<eframe::WebRunner>,
}

impl TextDisplay {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            runner: None,
        }
    }
}

impl WebComponent for TextDisplay {
    fn attach(&mut self, element: &web_sys::HtmlElement) {
        self.element = Some(element.clone());
    }

    fn connected(&mut self) {
        let element = self.element.as_ref().unwrap();

        let shadow = element
            .attach_shadow(&web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open))
            .expect("failed to attach shadow root");

        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .create_element("canvas")
            .expect("failed to create canvas")
            .unchecked_into::<HtmlCanvasElement>();

        let canvas_style = canvas.style();
        canvas_style.set_property("display", "block").unwrap();
        canvas_style.set_property("width", "100%").unwrap();
        canvas_style.set_property("height", "100%").unwrap();

        shadow.append_child(&canvas).unwrap();

        let runner = eframe::WebRunner::new();
        let element_copy = element.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let result = runner
                .start(
                    canvas,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Ok(Box::new(TextDisplayApp::new(cc)))),
                )
                .await;

            if let Err(e) = &result {
                web_sys::console::error_1(e);
            }

            if result.is_ok() {
                TextDisplay::with_element(&element_copy, |comp| {
                    comp.runner = Some(runner);
                });
            }
        });
    }

    fn disconnected(&mut self) {
        if let Some(runner) = self.runner.take() {
            runner.destroy();
        }
    }
}

pub struct TextDisplayApp {
    last_displayed_text: String,
}

impl TextDisplayApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        Self {
            last_displayed_text: String::new(),
        }
    }
}

impl eframe::App for TextDisplayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Read the shared text on every frame
        let current_text = shared_state::get_shared_text();

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(255, 250, 240))
                    .inner_margin(15.0),
            )
            .show(ctx, |ui| {
                ui.heading("Text Display Component");
                ui.add_space(10.0);

                ui.label("Synchronized text from input:");
                ui.add_space(5.0);

                // Display the shared text in a prominent way
                egui::Frame::default()
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(2.0, Color32::from_rgb(100, 149, 237)))
                    .inner_margin(15.0)
                    .corner_radius(5.0)
                    .show(ui, |ui| {
                        if current_text.is_empty() {
                            ui.label(
                                egui::RichText::new("(waiting for input...)")
                                    .italics()
                                    .color(Color32::GRAY)
                                    .size(18.0),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(&current_text)
                                    .color(Color32::from_rgb(0, 102, 204))
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
                                .color(Color32::GRAY),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("✓ Receiving updates")
                                .color(Color32::DARK_GREEN),
                        );
                    }
                });
            });

        // Request continuous updates to show changes immediately
        ctx.request_repaint();
    }
}
