//! Safe area provider for notched devices.

use leptos::prelude::*;

/// Safe area insets for notched devices.
#[derive(Clone, Copy, Debug, Default)]
pub struct SafeAreaInsets {
    /// Top inset in pixels.
    pub top: f64,
    /// Right inset in pixels.
    pub right: f64,
    /// Bottom inset in pixels.
    pub bottom: f64,
    /// Left inset in pixels.
    pub left: f64,
}

impl SafeAreaInsets {
    /// Create new safe area insets.
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self { top, right, bottom, left }
    }

    /// Check if there are any non-zero insets.
    pub fn has_insets(&self) -> bool {
        self.top > 0.0 || self.right > 0.0 || self.bottom > 0.0 || self.left > 0.0
    }
}

/// Safe area provider component.
///
/// Provides safe area insets to child components through context.
/// Automatically detects safe area insets from CSS environment variables.
///
/// # Example
///
/// ```ignore
/// use ember_fx_mobile::{SafeAreaProvider, use_safe_area};
///
/// view! {
///     <SafeAreaProvider>
///         <MyApp />
///     </SafeAreaProvider>
/// }
///
/// // In child component:
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let insets = use_safe_area();
///
///     view! {
///         <div style=format!("padding-top: {}px;", insets.top)>
///             "Content with safe area padding"
///         </div>
///     }
/// }
/// ```
#[component]
pub fn SafeAreaProvider(
    /// Child content.
    children: Children,
) -> impl IntoView {
    let insets = RwSignal::new(SafeAreaInsets::default());

    // Detect safe area insets on mount
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(root) = document.document_element() {
                        if let Ok(Some(style)) = window.get_computed_style(&root) {
                            let get_inset = |prop: &str| -> f64 {
                                style.get_property_value(prop)
                                    .ok()
                                    .and_then(|v| v.trim().trim_end_matches("px").parse().ok())
                                    .unwrap_or(0.0)
                            };

                            // Try to read env() values through computed styles
                            // This requires CSS variables set on :root
                            let top = get_inset("--sat");
                            let right = get_inset("--sar");
                            let bottom = get_inset("--sab");
                            let left = get_inset("--sal");

                            insets.set(SafeAreaInsets::new(top, right, bottom, left));
                        }
                    }
                }
            }
        }
    });

    provide_context(insets);

    view! {
        <div
            class="fx-safe-area-provider"
            style="--sat: env(safe-area-inset-top, 0px); --sar: env(safe-area-inset-right, 0px); --sab: env(safe-area-inset-bottom, 0px); --sal: env(safe-area-inset-left, 0px);"
        >
            {children()}
        </div>
    }
}

/// Get safe area insets from context.
///
/// Returns default (zero) insets if `SafeAreaProvider` is not in the component tree.
#[must_use]
pub fn use_safe_area() -> SafeAreaInsets {
    use_context::<RwSignal<SafeAreaInsets>>()
        .map(|s| s.get())
        .unwrap_or_default()
}
