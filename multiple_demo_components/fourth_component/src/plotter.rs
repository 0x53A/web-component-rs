use egui::{Color32, Pos2, Stroke};
use egui_web_component::EguiMount;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;

#[derive(WebComponent)]
#[web_component(name = "egui-plotter")]
pub struct EguiPlotter {
    element: Option<web_sys::HtmlElement>,
    mount: Option<EguiMount>,
}

impl EguiPlotter {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            mount: None,
        }
    }
}

impl WebComponent for EguiPlotter {
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
                Box::new(|cc| Ok(Box::new(PlotterApp::new(cc)))),
            )
            .await;

            match result {
                Ok(mount) => {
                    EguiPlotter::with_element(&element_copy, |comp| {
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

pub struct PlotterApp {
    time: f32,
}

impl PlotterApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_web_component::install_fonts(&cc.egui_ctx);
        Self { time: 0.0 }
    }
}

impl eframe::App for PlotterApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.time += 0.02;

        egui::Frame::default()
            .fill(Color32::from_rgb(0x11, 0x12, 0x14))
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.heading("Wave Plotter");
                ui.add_space(5.0);

                // Drawing area
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width(), 150.0),
                    egui::Sense::hover(),
                );

                let rect = response.rect;
                painter.rect_filled(rect, 0.0, Color32::from_rgb(0x1b, 0x1c, 0x1f));

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
                        Stroke::new(2.0_f32, Color32::from_rgb(0xff, 0x5a, 0x1f)),
                    );
                }

                // Draw center line
                painter.line_segment(
                    [
                        Pos2::new(rect.left(), center_y),
                        Pos2::new(rect.right(), center_y),
                    ],
                    Stroke::new(1.0_f32, Color32::from_rgb(0x35, 0x36, 0x3c)),
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

        ui.ctx().request_repaint();
    }
}
