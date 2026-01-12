use egui::Color32;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[derive(WebComponent)]
#[web_component(name = "egui-component")]
pub struct EguiComponent {
    element: Option<web_sys::HtmlElement>,
    runner: Option<eframe::WebRunner>,
}

impl EguiComponent {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            runner: None,
        }
    }
}

impl WebComponent for EguiComponent {
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
                    Box::new(|cc| Ok(Box::new(EguiApp::new(cc)))),
                )
                .await;

            if let Err(e) = &result {
                web_sys::console::error_1(e);
            }

            if result.is_ok() {
                EguiComponent::with_element(&element_copy, |comp| {
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

pub struct EguiApp {
    counter: i32,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self { counter: 0 }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::GREEN)
                    .inner_margin(0.0),
            )
            .show(ctx, |ui| {
                ui.label(format!(
                    "Width: {} | Height: {}",
                    ui.available_width(),
                    ui.available_height()
                ));

                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::default()
                        .fill(Color32::LIGHT_BLUE)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.heading("Egui in Web Component!");
                            ui.add_space(10.0);
                            ui.label(format!("Counter: {}", self.counter));
                            ui.add_space(10.0);
                            if ui.button("Increment").clicked() {
                                self.counter += 1;
                            }
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);
                            ui.label("This egui app is running inside a web component!");
                        });
                });
            });
    }
}

