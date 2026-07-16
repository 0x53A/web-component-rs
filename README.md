# web-component-rs

Rust → WebAssembly custom elements, with an optional [`egui`](https://github.com/emilk/egui)/[`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe) backend for components that want an immediate-mode UI instead of hand-written DOM/CSS.

**Live demo:** https://0x53a.github.io/web-component-rs/

## How it works

A component is a plain Rust struct that implements `WebComponent` and derives `#[derive(WebComponent)]` with the tag name it should register:

```rust
#[derive(WebComponent)]
#[web_component(name = "my-counter", observed_attributes = ["value"])]
pub struct MyCounter { /* ... */ }

impl WebComponent for MyCounter {
    fn attach(&mut self, element: &web_sys::HtmlElement) { /* ... */ }
    fn connected(&mut self) { /* build the shadow DOM here */ }
    fn disconnected(&mut self) { /* ... */ }
    fn attribute_changed(&mut self, name: &str, old: Option<&str>, new: Option<&str>) { /* ... */ }
}
```

The derive macro generates a matching `HTMLElement` subclass in JS and wires its lifecycle callbacks (`connectedCallback`, `disconnectedCallback`, `attributeChangedCallback`, ...) back to the Rust struct, keyed by a per-instance id stored in the registry the macro also generates. Call `MyCounter::setup()` once (typically from `#[wasm_bindgen(start)]`) to register the custom element with the browser.

## Crate layout

- **`rust_web_component`** — the `WebComponent` trait and the low-level `eval_js`/instance-id plumbing the macro relies on.
- **`rust_web_component_macro`** — the `#[derive(WebComponent)]` proc macro.
- **`egui_web_component`** — `EguiMount`, shared shadow-root/canvas/`eframe::WebRunner` mounting logic for components whose UI is rendered with `egui` instead of raw DOM.
- **`demo_component`** — a small standalone demo with a raw-DOM counter and an egui component (not deployed; for local experimentation).
- **`multiple_demo_components/`** — five independent wasm crates (`first_component` … `fifth_component`) shown together on the live demo page, demonstrating: raw-DOM components, egui-backed components, multiple instances of the same component, and two components communicating purely through shared Rust state. See `multiple_demo_components/README.md`.

Each demo crate is meant to be a self-contained starting point — copy one, rename it, and build a real component from it. They intentionally don't share app-level code with each other; only the generic `rust_web_component`/`egui_web_component` plumbing is shared.

## Typography

Both the HTML pages and the egui-rendered components use [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono) (OFL-1.1), so the two rendering paths look like one system. The font files are checked into `/fonts` (and copied alongside each demo's HTML for self-hosting via `@font-face` — see `/fonts/OFL.txt` for the license) and embedded into the egui apps at compile time via `egui_web_component::install_fonts`. No external font server is involved.

## Build & run

Each wasm crate is built independently with `wasm-pack`:

```sh
cd demo_component  # or any crate under multiple_demo_components/
wasm-pack build --target web
```

Then serve the crate's directory (or `multiple_demo_components/` for `demo.html`) with any static file server, e.g.:

```sh
python3 -m http.server 8000
```

GitHub Pages deployment (`.github/workflows/pages.yml`) builds all five `multiple_demo_components/*` crates and publishes `multiple_demo_components/demo.html` as `index.html` on every push to `main`.
