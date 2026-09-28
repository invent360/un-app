//! Spotlight tour guide component.

use leptos::prelude::*;
use super::types::{GuideStep, TourPlacement, ProgressStyle};
use super::guide_controls::GuideControls;
use crate::try_use_theme;

/// Spotlight tour guide for contextual UI walkthroughs.
///
/// Features:
/// - Highlights specific UI elements with spotlight effect
/// - Tooltip positioning (top, bottom, left, right)
/// - Click-through to highlighted element
/// - Scroll-to-target automation
/// - Backdrop dimming
/// - Keyboard navigation
///
/// # Example
///
/// ```ignore
/// let steps = vec![
///     GuideStep::new("1", "Welcome", "Click here to get started")
///         .target("#start-button")
///         .placement("bottom"),
///     GuideStep::new("2", "Settings", "Configure your preferences")
///         .target("#settings-menu")
///         .placement("left"),
/// ];
///
/// let active = RwSignal::new(0usize);
/// let running = RwSignal::new(true);
///
/// view! {
///     <GuideTour
///         steps=steps
///         active_step=active
///         running=running
///     />
/// }
/// ```
#[component]
pub fn GuideTour(
    /// The steps to display in the tour.
    #[prop(into)]
    steps: Vec<GuideStep>,
    /// Signal controlling the active step index (0-based).
    #[prop(into)]
    active_step: RwSignal<usize>,
    /// Signal controlling whether the tour is running.
    #[prop(into)]
    running: RwSignal<bool>,
    /// Default tooltip placement when step doesn't specify.
    #[prop(optional)]
    default_placement: Option<TourPlacement>,
    /// Progress indicator style.
    #[prop(optional)]
    progress_style: Option<ProgressStyle>,
    /// Spotlight padding around target element.
    #[prop(optional)]
    #[prop(default = 8)]
    spotlight_padding: i32,
    /// Spotlight border radius.
    #[prop(optional)]
    #[prop(default = 8)]
    spotlight_radius: i32,
    /// Allow clicking on spotlighted element.
    #[prop(optional)]
    #[prop(default = true)]
    allow_click_through: bool,
    /// Scroll target into view.
    #[prop(optional)]
    #[prop(default = true)]
    scroll_to_target: bool,
    /// Show skip button.
    #[prop(optional)]
    #[prop(default = true)]
    show_skip: bool,
    /// Callback when tour is completed.
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// Callback when tour is skipped.
    #[prop(optional, into)]
    on_skip: Option<Callback<()>>,
    /// Callback when step changes.
    #[prop(optional, into)]
    on_step_change: Option<Callback<usize>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));
    let default_placement = default_placement.unwrap_or_default();
    let progress_style = progress_style.unwrap_or(ProgressStyle::Dots);
    let total = steps.len();
    let steps = StoredValue::new(steps);

    // Build combined class
    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![format!("{}-tour", p)];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Navigation handlers
    let go_prev = {
        let on_step_change = on_step_change.clone();
        move |_| {
            let current = active_step.get();
            if current > 0 {
                let new_step = current - 1;
                active_step.set(new_step);
                if let Some(ref cb) = on_step_change {
                    cb.run(new_step);
                }
            }
        }
    };

    let go_next = {
        let on_step_change = on_step_change.clone();
        move |_| {
            let current = active_step.get();
            if current < total - 1 {
                let new_step = current + 1;
                active_step.set(new_step);
                if let Some(ref cb) = on_step_change {
                    cb.run(new_step);
                }
            }
        }
    };

    let handle_complete = {
        let on_complete = on_complete.clone();
        move |_| {
            if let Some(ref cb) = on_complete {
                cb.run(());
            }
            running.set(false);
        }
    };

    let handle_skip = {
        let on_skip = on_skip.clone();
        move |_| {
            if let Some(ref cb) = on_skip {
                cb.run(());
            }
            running.set(false);
        }
    };

    // Get current step
    let current_step = move || {
        let idx = active_step.get();
        steps.get_value().get(idx).cloned()
    };

    // Get placement for current step
    let get_placement = move || {
        current_step()
            .and_then(|s| s.placement.as_ref().map(|p| TourPlacement::from_str(p)))
            .unwrap_or(default_placement)
    };

    // Create signal for current step
    let current_signal = Signal::derive(move || active_step.get());

    view! {
        <Show when=move || running.get()>
            <div class=combined_class.clone()>
                // Backdrop overlay - click to close
                <div
                    class=format!("{}-tour-backdrop", prefix.get_value())
                    style="position: fixed; inset: 0; background: rgba(0,0,0,0.6); z-index: 10000; cursor: pointer;"
                    on:click=move |_| running.set(false)
                >
                    // SVG mask for spotlight effect
                    {
                        let p = prefix.get_value();
                        view! {
                            <svg class=format!("{}-tour-spotlight-mask", p) width="100%" height="100%">
                                <defs>
                                    <mask id="spotlight-mask">
                                        <rect x="0" y="0" width="100%" height="100%" fill="white" />
                                        // The cutout will be positioned via CSS/JS based on target
                                        <rect
                                            class=format!("{}-tour-spotlight-cutout", p)
                                            rx=spotlight_radius
                                            ry=spotlight_radius
                                            fill="black"
                                        />
                                    </mask>
                                </defs>
                                <rect
                                    x="0"
                                    y="0"
                                    width="100%"
                                    height="100%"
                                    fill="rgba(0, 0, 0, 0.5)"
                                    mask="url(#spotlight-mask)"
                                />
                            </svg>
                        }
                    }
                </div>

                // Tooltip for current step
                {move || {
                    let step = current_step();
                    let placement = get_placement();
                    let p = prefix.get_value();

                    step.map(|s| {
                        let title = s.title.clone();
                        let description = s.description.clone();
                        let target = s.target.clone();
                        let p2 = p.clone();

                        view! {
                            <div
                                class=move || format!(
                                    "{}-tour-tooltip {}",
                                    p2,
                                    placement.class(&p2)
                                )
                                style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); min-width: 480px; max-width: 600px; padding: 32px; background: #1f1f1f; border-radius: 16px; border: 1px solid #404040; box-shadow: 0 24px 80px rgba(0,0,0,0.6); z-index: 10001; animation: none; transition: none;"
                                data-target=target.clone()
                                on:click=|e| e.stop_propagation()
                            >
                                // Close button
                                {
                                    let p_close = p.clone();
                                    view! {
                                        <button
                                            type="button"
                                            class=format!("{}-tour-tooltip-close", p_close)
                                            style="position: absolute; top: 16px; right: 16px; width: 32px; height: 32px; background: rgba(255,255,255,0.1); border: none; color: rgba(255,255,255,0.7); cursor: pointer; font-size: 24px; line-height: 1; display: flex; align-items: center; justify-content: center; border-radius: 6px;"
                                            on:click=move |_| running.set(false)
                                            aria-label="Close tour"
                                        >
                                            "×"
                                        </button>
                                    }
                                }

                                // Arrow pointer (hidden for centered modal)
                                <div class=format!("{}-tour-tooltip-arrow", p) style="display: none;" />

                                // Content
                                <div
                                    class=format!("{}-tour-tooltip-content", p)
                                    style="margin-bottom: 24px;"
                                >
                                    <h4
                                        class=format!("{}-tour-tooltip-title", p)
                                        style="margin: 0 0 16px 0; font-size: 24px; font-weight: 600; color: #fff;"
                                    >
                                        {title}
                                    </h4>
                                    <p
                                        class=format!("{}-tour-tooltip-description", p)
                                        style="margin: 0; font-size: 16px; line-height: 1.6; color: rgba(255,255,255,0.75);"
                                    >
                                        {description}
                                    </p>
                                </div>

                                // Progress dots
                                {(progress_style != ProgressStyle::None && total > 1).then(|| {
                                    let p3 = prefix.get_value();
                                    view! {
                                        <div
                                            class=format!("{}-tour-tooltip-progress", p3)
                                            style="display: flex; justify-content: center; gap: 10px; margin-bottom: 24px;"
                                        >
                                            {(0..total).map(|i| {
                                                let p4 = p3.clone();
                                                let is_active = move || active_step.get() == i;
                                                view! {
                                                    <span
                                                        class=move || {
                                                            let mut c = format!("{}-tour-tooltip-dot", p4);
                                                            if is_active() { c.push_str(" active"); }
                                                            c
                                                        }
                                                        style=move || if is_active() {
                                                            "width: 12px; height: 12px; border-radius: 50%; background: #1677ff; transition: all 0.2s;"
                                                        } else {
                                                            "width: 12px; height: 12px; border-radius: 50%; background: #404040; transition: all 0.2s;"
                                                        }
                                                    />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }
                                })}

                                // Navigation controls: Back (left) | Skip (center, orange) | Next (right)
                                <div
                                    class=format!("{}-tour-tooltip-controls", p)
                                    style="display: flex; justify-content: space-between; align-items: center; gap: 16px;"
                                >
                                    {move || {
                                        let current = active_step.get();
                                        let is_first = current == 0;
                                        let is_last = current >= total - 1;
                                        let p5 = prefix.get_value();

                                        view! {
                                            // Back button (left) - show placeholder if first step
                                            <div style="flex: 1; display: flex; justify-content: flex-start;">
                                                {(!is_first).then(|| {
                                                    let go_prev = go_prev.clone();
                                                    let p7 = p5.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class=format!("{}-tour-tooltip-prev", p7)
                                                            style="padding: 12px 24px; background: #303030; border: none; border-radius: 8px; color: #fff; cursor: pointer; font-size: 14px; font-weight: 500;"
                                                            on:click=move |_| go_prev(())
                                                        >
                                                            "Back"
                                                        </button>
                                                    }
                                                })}
                                            </div>

                                            // Skip button (center, orange)
                                            <div style="flex: 1; display: flex; justify-content: center;">
                                                {show_skip.then(|| {
                                                    let handle_skip = handle_skip.clone();
                                                    let p6 = p5.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class=format!("{}-tour-tooltip-skip", p6)
                                                            style="padding: 12px 24px; background: transparent; border: 1px solid #fa8c16; border-radius: 8px; color: #fa8c16; cursor: pointer; font-size: 14px; font-weight: 500;"
                                                            on:click=move |_| handle_skip(())
                                                        >
                                                            "Skip"
                                                        </button>
                                                    }
                                                })}
                                            </div>

                                            // Next / Finish button (right)
                                            <div style="flex: 1; display: flex; justify-content: flex-end;">
                                                {if is_last {
                                                    let handle_complete = handle_complete.clone();
                                                    let p8 = p5.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class=format!("{}-tour-tooltip-complete", p8)
                                                            style="padding: 12px 24px; background: #52c41a; border: none; border-radius: 8px; color: #fff; cursor: pointer; font-size: 14px; font-weight: 500;"
                                                            on:click=move |_| handle_complete(())
                                                        >
                                                            "Finish"
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    let go_next = go_next.clone();
                                                    let p9 = p5.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class=format!("{}-tour-tooltip-next", p9)
                                                            style="padding: 12px 24px; background: #1677ff; border: none; border-radius: 8px; color: #fff; cursor: pointer; font-size: 14px; font-weight: 500;"
                                                            on:click=move |_| go_next(())
                                                        >
                                                            "Next"
                                                        </button>
                                                    }.into_any()
                                                }}
                                            </div>
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    })
                }}

                // Hidden script data for JavaScript positioning
                <script type="application/json" class=format!("{}-tour-data", prefix.get_value())>
                    {serde_json::to_string(&steps.get_value()).unwrap_or_default()}
                </script>
            </div>
        </Show>
    }
}

/// Beacon component for highlighting tour targets.
///
/// Place this near elements you want to highlight in the tour.
#[component]
pub fn TourBeacon(
    /// Whether the beacon is visible.
    #[prop(optional, into)]
    #[prop(default = MaybeSignal::Static(true))]
    visible: MaybeSignal<bool>,
    /// Beacon size in pixels.
    #[prop(optional)]
    #[prop(default = 24)]
    size: i32,
    /// Beacon color (CSS color value).
    #[prop(optional, into)]
    color: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));
    let color = color.unwrap_or_else(|| "var(--primary-color, #1890ff)".to_string());

    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![format!("{}-tour-beacon", p)];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let style = format!(
        "width: {}px; height: {}px; --beacon-color: {};",
        size, size, color
    );

    view! {
        <Show when=move || visible.get()>
            {
                let p = prefix.get_value();
                view! {
                    <span class=combined_class.clone() style=style.clone()>
                        <span class=format!("{}-tour-beacon-pulse", p) />
                        <span class=format!("{}-tour-beacon-dot", p) />
                    </span>
                }
            }
        </Show>
    }
}
