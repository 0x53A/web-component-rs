use std::sync::Mutex;

/// Shared state for inter-component communication
/// This allows different web components to communicate purely through Rust
static SHARED_TEXT: Mutex<String> = Mutex::new(String::new());

/// Update the shared text (called by the textbox component)
pub fn set_shared_text(text: String) {
    if let Ok(mut shared) = SHARED_TEXT.lock() {
        *shared = text;
        web_sys::console::log_1(&format!("Shared state updated: {}", shared).into());
    }
}

/// Get the current shared text (called by the label component)
pub fn get_shared_text() -> String {
    SHARED_TEXT.lock().map(|s| s.clone()).unwrap_or_default()
}
