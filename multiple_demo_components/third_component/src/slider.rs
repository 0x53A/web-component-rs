use egui::Color32;
use egui_web_component::EguiMount;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;

#[derive(WebComponent)]
#[web_component(name = "egui-slider")]
pub struct EguiSlider {
    element: Option<web_sys::HtmlElement>,
    mount: Option<EguiMount>,
}

impl EguiSlider {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            mount: None,
        }
    }
}

impl WebComponent for EguiSlider {
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
                Box::new(|cc| Ok(Box::new(SliderApp::new(cc)))),
            )
            .await;

            match result {
                Ok(mount) => {
                    EguiSlider::with_element(&element_copy, |comp| {
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

pub struct SliderApp {
    temperature: f32,
    volume: f32,
    brightness: f32,
}

impl SliderApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_web_component::install_fonts(&cc.egui_ctx);
        Self {
            temperature: 20.0,
            volume: 50.0,
            brightness: 75.0,
        }
    }
}

impl eframe::App for SliderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(0x11, 0x12, 0x14))
                    .inner_margin(15.0),
            )
            .show(ctx, |ui| {
                ui.heading("Interactive Sliders");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Temperature:");
                    ui.add(egui::Slider::new(&mut self.temperature, 0.0..=40.0).suffix("°C"));
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Volume:      ");
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).suffix("%"));
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Brightness:  ");
                    ui.add(egui::Slider::new(&mut self.brightness, 0.0..=100.0).suffix("%"));
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                // Visual feedback: signal-orange once things heat up, red once too hot.
                let temp_color = if self.temperature < 25.0 {
                    Color32::from_rgb(0x9a, 0x9a, 0xa0)
                } else if self.temperature < 32.0 {
                    Color32::from_rgb(0xff, 0x5a, 0x1f)
                } else {
                    Color32::from_rgb(0xe8, 0x43, 0x2b)
                };

                egui::Frame::default()
                    .fill(Color32::from_rgb(0x1b, 0x1c, 0x1f))
                    .stroke(egui::Stroke::new(1.0_f32, temp_color))
                    .inner_margin(10.0)
                    .corner_radius(0.0)
                    .show(ui, |ui| {
                        ui.label(format!("Current temperature: {:.1}°C", self.temperature));
                    });
            });
    }
}
