use egui::Color32;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::shared_state;

#[derive(WebComponent)]
#[web_component(name = "text-input")]
pub struct TextInput {
    element: Option<web_sys::HtmlElement>,
    runner: Option<eframe::WebRunner>,
}

impl TextInput {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            runner: None,
        }
    }
}

impl WebComponent for TextInput {
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
                    Box::new(|cc| Ok(Box::new(TextInputApp::new(cc)))),
                )
                .await;

            if let Err(e) = &result {
                web_sys::console::error_1(e);
            }

            if result.is_ok() {
                TextInput::with_element(&element_copy, |comp| {
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

pub struct TextInputApp {
    text: String,
}

impl TextInputApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
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
                    .fill(Color32::from_rgb(240, 248, 255))
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
