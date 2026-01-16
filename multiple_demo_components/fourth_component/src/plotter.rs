use egui::{Color32, Pos2, Stroke};
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[derive(WebComponent)]
#[web_component(name = "egui-plotter")]
pub struct EguiPlotter {
    element: Option<web_sys::HtmlElement>,
    runner: Option<eframe::WebRunner>,
}

impl EguiPlotter {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            runner: None,
        }
    }
}

impl WebComponent for EguiPlotter {
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
                    Box::new(|cc| Ok(Box::new(PlotterApp::new(cc)))),
                )
                .await;

            if let Err(e) = &result {
                web_sys::console::error_1(e);
            }

            if result.is_ok() {
                EguiPlotter::with_element(&element_copy, |comp| {
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

pub struct PlotterApp {
    points: Vec<Pos2>,
    time: f32,
}

impl PlotterApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            points: Vec::new(),
            time: 0.0,
        }
    }
}

impl eframe::App for PlotterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.02;

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(20, 25, 35))
                    .inner_margin(10.0),
            )
            .show(ctx, |ui| {
                ui.heading("Wave Plotter");
                ui.add_space(5.0);

                // Drawing area
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width(), 150.0),
                    egui::Sense::hover(),
                );

                let rect = response.rect;
                painter.rect_filled(rect, 0.0, Color32::from_rgb(10, 15, 25));

                // Generate sine wave points
                let mut points = Vec::new();
                let width = rect.width();
                let center_y = rect.center().y;
                let amplitude = 50.0;

                for i in 0..100 {
                    let x = rect.left() + (i as f32 / 100.0) * width;
                    let phase = (i as f32 / 100.0) * 4.0 * std::f32::consts::PI + self.time;
                    let y = center_y + amplitude * phase.sin();
                    points.push(Pos2::new(x, y));
                }

                // Draw the wave
                for window in points.windows(2) {
                    painter.line_segment(
                        [window[0], window[1]],
                        Stroke::new(2.0, Color32::from_rgb(100, 200, 255)),
                    );
                }

                // Draw center line
                painter.line_segment(
                    [Pos2::new(rect.left(), center_y), Pos2::new(rect.right(), center_y)],
                    Stroke::new(1.0, Color32::from_rgb(80, 80, 80)),
                );

                ui.add_space(10.0);

                // Controls
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        self.time = 0.0;
                    }
                    ui.label(format!("Time: {:.2}s", self.time));
                });

                ui.label("A simple animated sine wave");
            });

        ctx.request_repaint();
    }
}
