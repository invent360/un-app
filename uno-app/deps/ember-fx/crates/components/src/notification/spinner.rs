//! Spinner/Loading Leptos components.
//!
//! Includes:
//! - `Spinner` - Ant Design style dot spinner
//! - `Loading` - Simple border-based spinner
//! - `PrimeProgressSpinner` - PrimeReact SVG spinner with dash/color animations
//! - `LoadingOverlay` - Full-page centered loading overlay

use leptos::prelude::*;
use super::types::SpinnerSize;
use crate::try_use_theme;

/// Spinner component.
///
/// Loading indicator for async operations.
///
/// # Props
///
/// - `spinning` - Whether the spinner is visible/active
/// - `size` - Spinner size (Small, Default, Large)
/// - `tip` - Loading tip text
/// - `delay` - Delay before showing spinner (ms)
/// - `indicator` - Custom spinner indicator
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::Spinner;
///
/// view! {
///     <Spinner spinning=true tip="Loading...">
///         <div>Content to show when not loading</div>
///     </Spinner>
/// }
/// ```
#[component]
pub fn Spinner(
    /// Whether the spinner is active.
    #[prop(optional, into)]
    spinning: Option<Signal<bool>>,
    /// Spinner size.
    #[prop(optional, into)]
    size: Option<SpinnerSize>,
    /// Loading tip text.
    #[prop(optional, into)]
    tip: Option<String>,
    /// Custom spinner indicator.
    #[prop(optional, into)]
    indicator: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Content to wrap (will be blurred when loading).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let spinning = spinning.unwrap_or_else(|| Signal::derive(|| true));

    // Build CSS classes
    let spin_prefix = format!("fx-spin-{}", design_system);
    let size_class = size.class(&spin_prefix);

    // Class names (before closure captures)
    let dot_class = format!("{}-dot", spin_prefix);
    let dot_item_class = format!("{}-dot-item", spin_prefix);
    let text_class = format!("{}-text", spin_prefix);
    let container_class = format!("{}-container", spin_prefix);
    let blur_class = format!("{}-blur", spin_prefix);
    let nested_class = format!("{}-nested-loading", spin_prefix);

    // Clone for closure
    let spin_prefix_for_class = spin_prefix.clone();

    let combined_class = move || {
        let mut parts = vec![spin_prefix_for_class.clone()];
        if size != SpinnerSize::Default {
            parts.push(size_class.clone());
        }
        if spinning.get() {
            parts.push(format!("{}-spinning", spin_prefix_for_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Helper to create default spinner dots
    let dot_class_for_default = dot_class.clone();
    let dot_item_class_for_default = dot_item_class.clone();
    let make_default_indicator = move || {
        view! {
            <span class=dot_class_for_default.clone()>
                <i class=dot_item_class_for_default.clone()></i>
                <i class=dot_item_class_for_default.clone()></i>
                <i class=dot_item_class_for_default.clone()></i>
                <i class=dot_item_class_for_default.clone()></i>
            </span>
        }.into_any()
    };

    if children.is_some() {
        // Nested spinner with content
        let make_indicator = make_default_indicator.clone();
        view! {
            <div class=move || {
                let mut cls = vec![nested_class.clone()];
                if spinning.get() {
                    cls.push(format!("{}-spinning", nested_class));
                }
                cls.join(" ")
            }>
                <Show when=move || spinning.get()>
                    <div class=combined_class.clone()>
                        {if let Some(ref ind) = indicator {
                            view! { <span class=dot_class.clone()>{ind.clone()}</span> }.into_any()
                        } else {
                            make_indicator()
                        }}
                        {tip.clone().map(|t| view! {
                            <div class=text_class.clone()>{t}</div>
                        })}
                    </div>
                </Show>
                <div class=move || {
                    let mut cls = vec![container_class.clone()];
                    if spinning.get() {
                        cls.push(blur_class.clone());
                    }
                    cls.join(" ")
                }>
                    {children.map(|c| c())}
                </div>
            </div>
        }.into_any()
    } else {
        // Standalone spinner
        view! {
            <div class=combined_class>
                {if let Some(ref ind) = indicator {
                    view! { <span class=dot_class.clone()>{ind.clone()}</span> }.into_any()
                } else {
                    make_default_indicator()
                }}
                {tip.clone().map(|t| view! {
                    <div class=text_class.clone()>{t}</div>
                })}
            </div>
        }.into_any()
    }
}

/// Simple loading spinner (no wrapper).
#[component]
pub fn Loading(
    /// Spinner size.
    #[prop(optional, into)]
    size: Option<SpinnerSize>,
    /// Custom color.
    #[prop(optional, into)]
    color: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let size = size.unwrap_or_default();

    // Build CSS classes
    let loading_prefix = format!("fx-loading-{}", design_system);
    let size_class = size.class(&loading_prefix);

    let combined_class = {
        let mut parts = vec![loading_prefix.clone()];
        if size != SpinnerSize::Default {
            parts.push(size_class);
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let style = color.map(|c| format!("border-color: {} transparent transparent transparent;", c));

    view! {
        <div class=combined_class style=style>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
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
/// - `animation_duration` - Duration of rotation animation (default: "2s")
/// - `single_color` - If true, uses primary color only (no color cycling)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::PrimeProgressSpinner;
///
/// view! {
///     <PrimeProgressSpinner size=SpinnerSize::Large />
/// }
/// ```
#[component]
pub fn PrimeProgressSpinner(
    /// Spinner size.
    #[prop(optional, into)]
    size: Option<SpinnerSize>,
    /// Stroke width of the circle.
    #[prop(into, default = "2".to_string())]
    stroke_width: String,
    /// Animation duration for rotation.
    #[prop(into, default = "2s".to_string())]
    animation_duration: String,
    /// Use single color instead of color cycling.
    #[prop(optional)]
    single_color: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("prime");

    let size = size.unwrap_or_default();

    // Build CSS classes
    let spin_prefix = format!("fx-spin-{}", design_system);
    let size_class = size.class(&spin_prefix);

    let svg_class = format!("{}-svg", spin_prefix);
    let circle_class = if single_color {
        format!("{}-circle {}-circle-single", spin_prefix, spin_prefix)
    } else {
        format!("{}-circle", spin_prefix)
    };

    let combined_class = {
        let mut parts = vec![spin_prefix.clone()];
        parts.push(size_class);
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Get dimensions based on size
    let (width, height) = match size {
        SpinnerSize::Small => (50, 50),
        SpinnerSize::Default => (100, 100),
        SpinnerSize::Large => (150, 150),
    };

    view! {
        <div
            class=combined_class
            style=format!("width: {}px; height: {}px;", width, height)
            role="progressbar"
            aria-busy="true"
        >
            <svg
                class=svg_class
                viewBox="25 25 50 50"
                style=format!("animation-duration: {};", animation_duration)
            >
                <circle
                    class=circle_class
                    cx="50"
                    cy="50"
                    r="20"
                    fill="none"
                    stroke-width=stroke_width
                    stroke-miterlimit="10"
                />
            </svg>
        </div>
    }
}

/// Full-page loading overlay with centered spinner.
///
/// Displays a dark semi-transparent overlay with a centered progress spinner
/// and optional loading message.
///
/// # Props
///
/// - `visible` - Signal controlling overlay visibility
/// - `message` - Optional loading message text
/// - `spinner_size` - Size of the spinner (default: Large)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::LoadingOverlay;
///
/// let loading = RwSignal::new(false);
///
/// view! {
///     <LoadingOverlay
///         visible=Signal::derive(move || loading.get())
///         message=Some("Saving...".to_string())
///     />
///     <button on:click=move |_| loading.set(true)>
///         "Start Loading"
///     </button>
/// }
/// ```
#[component]
pub fn LoadingOverlay(
    /// Whether the overlay is visible.
    #[prop(into)]
    visible: Signal<bool>,
    /// Optional loading message.
    #[prop(optional, into)]
    message: Option<String>,
    /// Spinner size (default: Large).
    #[prop(optional, into)]
    spinner_size: Option<SpinnerSize>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("prime");

    let size = spinner_size.unwrap_or(SpinnerSize::Large);

    let overlay_class = format!("fx-loading-overlay-{}", design_system);
    let content_class = format!("fx-loading-overlay-{}-content", design_system);
    let message_class = format!("fx-loading-overlay-{}-message", design_system);

    view! {
        <Show when=move || visible.get()>
            <div class=overlay_class.clone()>
                <div class=content_class.clone()>
                    <PrimeProgressSpinner size=size />
                    {message.clone().map(|msg| view! {
                        <p class=message_class.clone()>{msg}</p>
                    })}
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
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("prime");

    let spinner_class = format!("fx-inline-spinner-{}", design_system);
    let icon_class = format!("fx-inline-spinner-{}-icon", design_system);
    let text_class = format!("fx-inline-spinner-{}-text", design_system);

    let combined_class = {
        let mut parts = vec![spinner_class];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <span class=combined_class>
            <span
                class=icon_class
                style=format!("width: {}px; height: {}px;", size, size)
            />
            {message.map(|msg| view! {
                <span class=text_class>{msg}</span>
            })}
        </span>
    }
}
