//! Shared plumbing for mounting an `eframe`/`egui` app inside a web component's shadow DOM.
//!
//! This is the part that's specific to *this* framework (attach a shadow root, create a
//! canvas, start an `eframe::WebRunner` on it) and would be identical for any egui-backed
//! component built on `rust_web_component`. App-specific configuration (visuals, logging,
//! `WebOptions`) stays with each component.

use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// Owns the `eframe::WebRunner` for a component instance.
pub struct EguiMount {
    runner: eframe::WebRunner,
}

impl EguiMount {
    /// Attaches a shadow root and canvas to `element`, then starts `app_creator` on it.
    ///
    /// Must be awaited from within `wasm_bindgen_futures::spawn_local`, since starting
    /// `eframe` is async. On success, the caller is expected to store the returned
    /// `EguiMount` on its component instance and later call [`EguiMount::disconnect`].
    pub async fn connect(
        element: &web_sys::HtmlElement,
        web_options: eframe::WebOptions,
        app_creator: eframe::AppCreator<'static>,
    ) -> Result<Self, wasm_bindgen::JsValue> {
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
        runner.start(canvas, web_options, app_creator).await?;
        Ok(Self { runner })
    }

    /// Tears down the running `eframe` app.
    pub fn disconnect(&self) {
        self.runner.destroy();
    }
}

/// Installs the JetBrains Mono font (bundled in `/fonts`, OFL-1.1 licensed — see
/// `/fonts/OFL.txt`) as the primary proportional and monospace font, so egui-rendered
/// components typographically match the surrounding page, which self-hosts the same font.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "jetbrains_mono".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../fonts/JetBrainsMono-Regular.ttf"
        ))),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "jetbrains_mono".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "jetbrains_mono".to_owned());
    ctx.set_fonts(fonts);
}
