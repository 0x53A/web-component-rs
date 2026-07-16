use egui::Color32;
use egui_web_component::EguiMount;
use rust_web_component::WebComponent;
use rust_web_component_macro::WebComponent;

#[derive(WebComponent)]
#[web_component(name = "egui-component")]
pub struct EguiComponent {
    element: Option<web_sys::HtmlElement>,
    mount: Option<EguiMount>,
}

impl EguiComponent {
    fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            element: None,
            mount: None,
        }
    }
}

impl WebComponent for EguiComponent {
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
                Box::new(|cc| Ok(Box::new(EguiApp::new(cc)))),
            )
            .await;

            match result {
                Ok(mount) => {
                    EguiComponent::with_element(&element_copy, |comp| {
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

pub struct EguiApp {
    counter: i32,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_web_component::install_fonts(&cc.egui_ctx);
        Self { counter: 0 }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(0x11, 0x12, 0x14))
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
                        .fill(Color32::from_rgb(0x1b, 0x1c, 0x1f))
                        .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(0xff, 0x5a, 0x1f)))
                        .inner_margin(10.0)
                        .corner_radius(0.0)
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

