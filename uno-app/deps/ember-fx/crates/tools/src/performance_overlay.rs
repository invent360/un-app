//! Performance overlay for monitoring render metrics.

use leptos::prelude::*;

/// Performance metrics overlay.
///
/// Displays FPS, memory usage, and render timing information.
///
/// # Example
///
/// ```ignore
/// use ember_fx_tools::PerformanceOverlay;
///
/// view! {
///     <PerformanceOverlay />
/// }
/// ```
#[component]
pub fn PerformanceOverlay(
    /// Whether the overlay is initially visible.
    #[prop(optional)]
    initial_visible: Option<bool>,
    /// Position of the overlay.
    #[prop(optional, into)]
    position: Option<String>,
) -> impl IntoView {
    let is_visible = RwSignal::new(initial_visible.unwrap_or(true));
    let fps = RwSignal::new(0u32);
    let frame_time = RwSignal::new(0.0f64);
    let _memory_mb = RwSignal::new(0.0f64);

    let position = position.unwrap_or_else(|| "top-left".to_string());
    let position_style = match position.as_str() {
        "top-right" => "top: 16px; right: 16px;",
        "bottom-left" => "bottom: 16px; left: 16px;",
        "bottom-right" => "bottom: 16px; right: 16px;",
        _ => "top: 16px; left: 16px;",
    };

    // FPS counter effect
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let frame_count = std::rc::Rc::new(std::cell::RefCell::new(0u32));
        let last_time = std::rc::Rc::new(std::cell::RefCell::new(0.0f64));

        let window = web_sys::window().expect("window");

        let f: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(f64)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let g = f.clone();

        let frame_count_clone = frame_count.clone();
        let last_time_clone = last_time.clone();

        *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
            let mut count = frame_count_clone.borrow_mut();
            let mut last = last_time_clone.borrow_mut();

            *count += 1;

            let elapsed = timestamp - *last;
            if elapsed >= 1000.0 {
                fps.set(*count);
                frame_time.set(elapsed / (*count as f64));
                *count = 0;
                *last = timestamp;
            }

            // Continue animation loop
            if let Some(ref closure) = *f.borrow() {
                let _ = web_sys::window()
                    .expect("window")
                    .request_animation_frame(closure.as_ref().unchecked_ref());
            }
        }));

        // Start the animation loop
        if let Some(ref closure) = *g.borrow() {
            let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
        }

        // Get memory info if available
        #[allow(unused_unsafe)]
        if let Some(performance) = window.performance() {
            // Note: memory API is Chrome-only and requires special access
            // This is a placeholder that shows 0
            memory_mb.set(0.0);
        }
    });

    view! {
        <div
            class="fx-performance-overlay"
            style=format!(
                "position: fixed; {}; z-index: 9999; font-family: monospace; font-size: 11px;",
                position_style
            )
        >
            <button
                type="button"
                style="padding: 4px 8px; background: #1a1a2e; color: #eee; border: 1px solid #333; border-radius: 4px; cursor: pointer; font-size: 10px;"
                on:click=move |_| is_visible.update(|v| *v = !*v)
            >
                {move || if is_visible.get() { "Hide" } else { "Perf" }}
            </button>

            <Show when=move || is_visible.get()>
                <div
                    style="margin-top: 4px; padding: 8px 12px; background: rgba(26, 26, 46, 0.95); border: 1px solid #333; border-radius: 4px; color: #eee;"
                >
                    <div style="display: flex; gap: 16px;">
                        <div>
                            <span style="color: #888;">"FPS: "</span>
                            <span style=move || {
                                let current_fps = fps.get();
                                format!(
                                    "color: {};",
                                    if current_fps >= 55 { "#4caf50" }
                                    else if current_fps >= 30 { "#ff9800" }
                                    else { "#f44336" }
                                )
                            }>
                                {move || fps.get()}
                            </span>
                        </div>
                        <div>
                            <span style="color: #888;">"Frame: "</span>
                            <span style="color: #4fc3f7;">
                                {move || format!("{:.1}ms", frame_time.get())}
                            </span>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
