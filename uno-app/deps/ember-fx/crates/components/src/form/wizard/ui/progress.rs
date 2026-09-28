//! Wizard progress components.
//!
//! Progress indicators for wizard completion.

use leptos::prelude::*;
use crate::try_use_theme;

/// Linear progress bar for wizard.
///
/// Shows completion percentage with smooth animation.
#[component]
pub fn WizardProgress(
    /// Progress percentage (0-100).
    #[prop(into)]
    percentage: Signal<u32>,
    /// Show percentage text inside bar.
    #[prop(optional)]
    show_percentage: bool,
    /// Progress bar height class.
    #[prop(optional, into)]
    height: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-progress", design_system);
    let prefix_container = prefix.clone();
    let prefix_text = prefix.clone();
    let height = height.unwrap_or_else(|| "h-2".to_string());

    let container_class = move || {
        let mut classes = vec![prefix_container.clone()];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    let track_class = format!("{}-track {}", prefix, height);
    let bar_class = format!("{}-bar", prefix);

    view! {
        <div class=container_class role="progressbar" aria-valuenow=move || percentage.get() aria-valuemin="0" aria-valuemax="100">
            <div class=track_class.clone()>
                <div
                    class=bar_class.clone()
                    style=move || format!("width: {}%", percentage.get())
                >
                    {show_percentage.then(move || {
                        let pct = percentage.get();
                        // Only show text if there's enough space
                        (pct > 10).then(|| {
                            view! {
                                <span class=format!("{}-text", prefix_text)>
                                    {format!("{}%", pct)}
                                </span>
                            }
                        })
                    })}
                </div>
            </div>
        </div>
    }
}

/// Circular progress indicator for wizard.
///
/// Shows progress as a ring indicator.
#[component]
pub fn WizardCircularProgress(
    /// Progress percentage (0-100).
    #[prop(into)]
    percentage: Signal<u32>,
    /// Size of the circle in pixels.
    #[prop(optional)]
    size: Option<u32>,
    /// Stroke width in pixels.
    #[prop(optional)]
    stroke_width: Option<u32>,
    /// Show percentage text in center.
    #[prop(optional)]
    show_percentage: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-circular-progress-{}", design_system);
    let prefix_container = prefix.clone();
    let prefix_svg = prefix.clone();
    let prefix_track = prefix.clone();
    let prefix_bar = prefix.clone();
    let prefix_text = prefix.clone();
    let size = size.unwrap_or(80);
    let stroke_width = stroke_width.unwrap_or(8);

    let radius = (size / 2) - (stroke_width / 2);
    let circumference = 2.0 * std::f32::consts::PI * radius as f32;

    let stroke_dashoffset = move || {
        let pct = percentage.get() as f32;
        circumference * (1.0 - pct / 100.0)
    };

    let container_class = move || {
        let mut classes = vec![prefix_container.clone()];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    view! {
        <div
            class=container_class
            style=format!("width: {}px; height: {}px; position: relative;", size, size)
        >
            <svg
                width=size
                height=size
                class=format!("{}-svg", prefix_svg)
                style="transform: rotate(-90deg);"
            >
                // Background circle
                <circle
                    cx=size / 2
                    cy=size / 2
                    r=radius
                    stroke-width=stroke_width
                    fill="none"
                    class=format!("{}-track", prefix_track)
                />
                // Progress circle
                <circle
                    cx=size / 2
                    cy=size / 2
                    r=radius
                    stroke-width=stroke_width
                    fill="none"
                    stroke-dasharray=circumference
                    stroke-dashoffset=stroke_dashoffset
                    stroke-linecap="round"
                    class=format!("{}-bar", prefix_bar)
                />
            </svg>

            {show_percentage.then(move || view! {
                <div
                    class=format!("{}-text", prefix_text)
                    style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center;"
                >
                    <span>
                        {move || format!("{}%", percentage.get())}
                    </span>
                </div>
            })}
        </div>
    }
}

/// Step counter text display.
///
/// Simple "Step X of Y" text indicator.
#[component]
pub fn WizardStepCounter(
    /// Current step (1-based).
    #[prop(into)]
    current: Signal<usize>,
    /// Total number of steps.
    #[prop(into)]
    total: Signal<usize>,
    /// Format template (use {current} and {total} placeholders).
    #[prop(optional, into)]
    format: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-step-counter", design_system);
    let format_template = format.unwrap_or_else(|| "Step {current} of {total}".to_string());

    let counter_class = move || {
        let mut classes = vec![prefix.clone()];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    view! {
        <div class=counter_class>
            {move || {
                format_template
                    .replace("{current}", &current.get().to_string())
                    .replace("{total}", &total.get().to_string())
            }}
        </div>
    }
}
