//! Progress Spinner components for uno-admin.
//!
//! Provides PrimeReact-style animated SVG spinners for loading states.
//!
//! Components:
//! - `ProgressSpinner` - Main SVG spinner with animated dash/color cycle
//! - `LoadingOverlay` - Full-page centered overlay with spinner
//! - `InlineSpinner` - Small spinner for buttons and inline use

use leptos::prelude::*;

/// Spinner size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    Tiny,
    Small,
    #[default]
    Default,
    Large,
}

impl SpinnerSize {
    /// Get the pixel dimensions for this size.
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            SpinnerSize::Tiny => (20, 20),
            SpinnerSize::Small => (50, 50),
            SpinnerSize::Default => (100, 100),
            SpinnerSize::Large => (150, 150),
        }
    }

    /// Get the CSS class suffix for this size.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            SpinnerSize::Tiny => "xs",
            SpinnerSize::Small => "sm",
            SpinnerSize::Default => "default",
            SpinnerSize::Large => "lg",
        }
    }
}

/// PrimeReact-style animated SVG progress spinner.
///
/// Features a rotating SVG with animated dash pattern and color cycling
/// through red, blue, green, and orange.
///
/// # Props
///
/// - `size` - Spinner size (Small=50px, Default=100px, Large=150px)
/// - `stroke_width` - Width of the circle stroke (default: "2")
/// - `single_color` - If true, uses primary color only (no color cycling)
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// view! {
///     <ProgressSpinner size=SpinnerSize::Large />
/// }
/// ```
#[component]
pub fn ProgressSpinner(
    /// Spinner size.
    #[prop(optional)]
    size: Option<SpinnerSize>,
    /// Stroke width of the circle.
    #[prop(into, default = "2".to_string())]
    stroke_width: String,
    /// Use single color instead of color cycling.
    #[prop(optional)]
    single_color: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let size = size.unwrap_or_default();
    let (width, height) = size.dimensions();

    let circle_class = if single_color {
        "p-progress-spinner-circle p-progress-spinner-circle-single"
    } else {
        "p-progress-spinner-circle"
    };

    let combined_class = {
        let mut parts = vec![
            "p-progress-spinner".to_string(),
            format!("p-progress-spinner-{}", size.class_suffix()),
        ];
        if let Some(custom) = class {
            parts.push(custom);
        }
        parts.join(" ")
    };

    // Use larger stroke width for better visibility
    let stroke = if stroke_width == "2" { "4".to_string() } else { stroke_width };

    view! {
        <div
            class=combined_class
            style=format!("width: {}px; height: {}px;", width, height)
            role="progressbar"
            aria-busy="true"
        >
            <svg
                class="p-progress-spinner-svg"
                viewBox="25 25 50 50"
                style="animation: p-progress-spinner-rotate 2s linear infinite;"
            >
                <circle
                    class=circle_class
                    cx="50"
                    cy="50"
                    r="20"
                    fill="none"
                    stroke-width=stroke
                    stroke-miterlimit="10"
                    style="stroke: #3b82f6; stroke-linecap: round; stroke-dasharray: 89, 200; animation: p-progress-spinner-dash 1.5s ease-in-out infinite, p-progress-spinner-color 6s ease-in-out infinite;"
                />
            </svg>
        </div>
    }
}

/// Full-page loading overlay with centered spinner.
///
/// Displays a dark semi-transparent overlay with a centered progress spinner
/// and optional loading message. Perfect for blocking operations like save,
/// publish, or delete.
///
/// # Props
///
/// - `visible` - Signal controlling overlay visibility
/// - `message` - Reactive loading message (Signal<Option<String>>)
/// - `spinner_size` - Size of the spinner (default: Large)
///
/// # Example
///
/// ```ignore
/// let loading = RwSignal::new(false);
/// let loading_message = RwSignal::new(None::<String>);
///
/// view! {
///     <LoadingOverlay
///         visible=Signal::derive(move || loading.get())
///         message=Signal::derive(move || loading_message.get())
///     />
///     <button on:click=move |_| {
///         loading.set(true);
///         loading_message.set(Some("Saving...".to_string()));
///     }>
///         "Save"
///     </button>
/// }
/// ```
#[component]
pub fn LoadingOverlay(
    /// Whether the overlay is visible.
    #[prop(into)]
    visible: Signal<bool>,
    /// Reactive loading message.
    #[prop(into)]
    message: Signal<Option<String>>,
    /// Spinner size (default: Large).
    #[prop(optional)]
    spinner_size: Option<SpinnerSize>,
) -> impl IntoView {
    let size = spinner_size.unwrap_or(SpinnerSize::Large);

    view! {
        <Show when=move || visible.get()>
            <div class="p-loading-overlay">
                <div class="p-loading-overlay-content">
                    <ProgressSpinner size=size />
                    {move || {
                        message.get().map(|msg| view! {
                            <p class="p-loading-overlay-message">{msg}</p>
                        })
                    }}
                </div>
            </div>
        </Show>
    }
}

/// Inline spinner for buttons and small UI elements.
///
/// A compact spinner that can be placed inline with text,
/// typically used inside buttons during loading states.
///
/// # Example
///
/// ```ignore
/// view! {
///     <button disabled=loading.get()>
///         {move || if loading.get() {
///             view! { <InlineSpinner message=Some("Saving...".to_string()) /> }.into_any()
///         } else {
///             view! { "Save" }.into_any()
///         }}
///     </button>
/// }
/// ```
#[component]
pub fn InlineSpinner(
    /// Size in pixels (default: 16).
    #[prop(default = 16)]
    size: u32,
    /// Optional message next to spinner.
    #[prop(optional, into)]
    message: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let combined_class = {
        let mut parts = vec!["p-inline-spinner".to_string()];
        if let Some(custom) = class {
            parts.push(custom);
        }
        parts.join(" ")
    };

    view! {
        <span class=combined_class>
            <span
                class="p-inline-spinner-icon"
                style=format!("width: {}px; height: {}px;", size, size)
            />
            {message.map(|msg| view! {
                <span class="p-inline-spinner-text">{msg}</span>
            })}
        </span>
    }
}
