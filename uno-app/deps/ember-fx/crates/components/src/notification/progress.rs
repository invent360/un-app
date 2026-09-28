//! Progress Leptos component.
//!
//! Display progress of an operation with support for linear, circular,
//! and dashboard styles. Integrates with theme system for consistent colors.

use leptos::prelude::*;
use super::types::{ProgressType, ProgressStatus};
use crate::try_use_theme;

/// Helper function to get status color
fn status_color(status: ProgressStatus, stroke_color: &Option<String>) -> String {
    match status {
        ProgressStatus::Success => "var(--fx-color-success, #52c41a)".to_string(),
        ProgressStatus::Exception => "var(--fx-color-error, #ff4d4f)".to_string(),
        ProgressStatus::Active | ProgressStatus::Normal => {
            stroke_color.clone().unwrap_or_else(|| "var(--fx-color-primary, #1677ff)".to_string())
        }
    }
}

/// Progress component.
///
/// Display progress of an operation with various styles and statuses.
///
/// # Props
///
/// - `percent` - Current progress percentage (0-100)
/// - `progress_type` - Progress bar type (Line, Circle, Dashboard)
/// - `status` - Progress status (Normal, Success, Exception, Active)
/// - `show_info` - Whether to show percentage text
/// - `stroke_color` - Custom progress bar color
/// - `trail_color` - Color of unfilled portion
/// - `stroke_width` - Width of the progress bar
/// - `size` - Size for circle/dashboard type
/// - `steps` - Number of steps for segmented progress
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::Progress;
///
/// // Basic progress
/// view! {
///     <Progress percent=Signal::derive(|| 75.0) />
/// }
///
/// // Circle progress with status
/// view! {
///     <Progress
///         percent=Signal::derive(|| 100.0)
///         progress_type=ProgressType::Circle
///         status=ProgressStatus::Success
///     />
/// }
/// ```
#[component]
pub fn Progress(
    /// Current progress (0-100).
    #[prop(into)]
    percent: Signal<f64>,
    /// Progress bar type.
    #[prop(optional, into)]
    progress_type: Option<ProgressType>,
    /// Progress status.
    #[prop(optional, into)]
    status: Option<ProgressStatus>,
    /// Whether to show percentage.
    #[prop(optional)]
    show_info: Option<bool>,
    /// Custom progress color.
    #[prop(optional, into)]
    stroke_color: Option<String>,
    /// Trail/background color.
    #[prop(optional, into)]
    trail_color: Option<String>,
    /// Stroke width (px for line, percentage for circle).
    #[prop(optional)]
    stroke_width: Option<f64>,
    /// Size for circle type (px).
    #[prop(optional)]
    size: Option<u32>,
    /// Number of steps for segmented progress.
    #[prop(optional)]
    steps: Option<usize>,
    /// Format function for info text.
    #[prop(optional, into)]
    format: Option<Callback<f64, String>>,
    /// Success percent threshold (auto-success when reached).
    #[prop(optional)]
    success_percent: Option<f64>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let progress_type = progress_type.unwrap_or_default();
    let show_info = show_info.unwrap_or(true);
    let default_stroke_width = if progress_type == ProgressType::Line { 8.0 } else { 6.0 };
    let stroke_width = stroke_width.unwrap_or(default_stroke_width);
    let size = size.unwrap_or(120);

    // Build CSS classes
    let progress_prefix = format!("fx-progress-{}", design_system);
    let type_class = progress_type.class(&progress_prefix);

    // Derive effective status (auto-success at 100% or success_percent)
    let effective_status = move || {
        let p = percent.get();
        if let Some(s) = status {
            s
        } else if p >= success_percent.unwrap_or(100.0) {
            ProgressStatus::Success
        } else {
            ProgressStatus::Normal
        }
    };

    // Class names
    let outer_class = format!("{}-outer", progress_prefix);
    let inner_class = format!("{}-inner", progress_prefix);
    let bg_class = format!("{}-bg", progress_prefix);
    let text_class = format!("{}-text", progress_prefix);
    let circle_class = format!("{}-circle", progress_prefix);
    let steps_class = format!("{}-steps", progress_prefix);

    let progress_prefix_for_class = progress_prefix.clone();
    let class_clone = class.clone();

    let combined_class = move || {
        let mut parts = vec![progress_prefix_for_class.clone(), type_class.clone()];
        parts.push(effective_status().class(&progress_prefix_for_class));
        if show_info {
            parts.push(format!("{}-show-info", progress_prefix_for_class));
        }
        if let Some(ref custom) = class_clone {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Format percentage display
    let format_percent = move |p: f64| {
        if let Some(ref fmt) = format {
            fmt.run(p)
        } else {
            match effective_status() {
                ProgressStatus::Success => "✓".to_string(),
                ProgressStatus::Exception => "✕".to_string(),
                _ => format!("{}%", p.round() as i32),
            }
        }
    };

    let trail_bg = trail_color.clone()
        .unwrap_or_else(|| "var(--fx-color-fill-tertiary, rgba(0,0,0,0.04))".to_string());

    match progress_type {
        ProgressType::Line => {
            // Check if we should render steps
            if let Some(step_count) = steps {
                // Stepped/segmented progress
                let step_count = step_count.max(1);
                let progress_prefix = progress_prefix.clone();
                let trail_bg = trail_bg.clone();
                let stroke_color_for_steps = stroke_color.clone();

                view! {
                    <div class=combined_class>
                        <div class=steps_class>
                            {(0..step_count).map(|i| {
                                let progress_prefix = progress_prefix.clone();
                                let step_percent = (i as f64 + 1.0) / step_count as f64 * 100.0;
                                let trail_bg = trail_bg.clone();
                                let stroke_color = stroke_color_for_steps.clone();
                                view! {
                                    <div
                                        class=move || {
                                            let base = format!("{}-step", progress_prefix);
                                            if percent.get() >= step_percent {
                                                format!("{} {}-step-active", base, progress_prefix)
                                            } else {
                                                base
                                            }
                                        }
                                        style=move || {
                                            let color = if percent.get() >= step_percent {
                                                status_color(effective_status(), &stroke_color)
                                            } else {
                                                trail_bg.clone()
                                            };
                                            format!("background-color: {}; height: {}px;", color, stroke_width)
                                        }
                                    />
                                }
                            }).collect_view()}
                        </div>
                        {if show_info {
                            Some(view! {
                                <span class=text_class.clone()>
                                    {move || format_percent(percent.get())}
                                </span>
                            })
                        } else {
                            None
                        }}
                    </div>
                }.into_any()
            } else {
                // Standard linear progress
                let trail_bg_for_style = trail_bg.clone();
                let stroke_color_for_bar = stroke_color.clone();
                let stroke_color_for_text = stroke_color.clone();

                view! {
                    <div class=combined_class>
                        <div class=outer_class.clone()>
                            <div
                                class=inner_class.clone()
                                style=format!("background-color: {}; height: {}px; border-radius: {}px; overflow: hidden;", trail_bg_for_style, stroke_width, stroke_width / 2.0)
                            >
                                <div
                                    class=move || {
                                        let base = bg_class.clone();
                                        if effective_status() == ProgressStatus::Active {
                                            format!("{} {}-bg-active", base, progress_prefix)
                                        } else {
                                            base
                                        }
                                    }
                                    style=move || {
                                        let p = percent.get().clamp(0.0, 100.0);
                                        format!(
                                            "width: {}%; height: {}px; background-color: {}; border-radius: {}px; transition: width 0.3s ease;",
                                            p,
                                            stroke_width,
                                            status_color(effective_status(), &stroke_color_for_bar),
                                            stroke_width / 2.0
                                        )
                                    }
                                />
                            </div>
                        </div>
                        {if show_info {
                            Some(view! {
                                <span
                                    class=text_class.clone()
                                    style=move || {
                                        format!("color: {};", status_color(effective_status(), &stroke_color_for_text))
                                    }
                                >
                                    {move || format_percent(percent.get())}
                                </span>
                            })
                        } else {
                            None
                        }}
                    </div>
                }.into_any()
            }
        }
        ProgressType::Circle | ProgressType::Dashboard => {
            let radius = (size as f64 - stroke_width * 2.0) / 2.0;
            let circumference = 2.0 * std::f64::consts::PI * radius;
            let is_dashboard = progress_type == ProgressType::Dashboard;

            // Dashboard shows 75% of circle, starting from bottom
            let (total_arc, rotation) = if is_dashboard {
                (circumference * 0.75, 135.0) // 135 degrees = bottom-left start
            } else {
                (circumference, -90.0) // Start from top
            };

            let trail_bg_for_svg = trail_bg.clone();
            let center = size as f64 / 2.0;
            let stroke_color_for_circle = stroke_color.clone();
            let stroke_color_for_text = stroke_color.clone();

            view! {
                <div
                    class=combined_class
                    style=format!("width: {}px; height: {}px; display: inline-flex; align-items: center; justify-content: center; position: relative;", size, size)
                >
                    <svg
                        class=circle_class.clone()
                        viewBox=format!("0 0 {} {}", size, size)
                        style=format!("transform: rotate({}deg);", rotation)
                    >
                        // Trail circle (background)
                        <circle
                            cx=center
                            cy=center
                            r=radius
                            fill="none"
                            stroke=trail_bg_for_svg
                            stroke-width=stroke_width
                            stroke-linecap="round"
                            stroke-dasharray=if is_dashboard {
                                format!("{} {}", total_arc, circumference - total_arc)
                            } else {
                                String::new()
                            }
                        />
                        // Progress circle
                        <circle
                            cx=center
                            cy=center
                            r=radius
                            fill="none"
                            stroke=move || status_color(effective_status(), &stroke_color_for_circle)
                            stroke-width=stroke_width
                            stroke-linecap="round"
                            stroke-dasharray=move || {
                                let p = percent.get().clamp(0.0, 100.0);
                                let progress_arc = total_arc * (p / 100.0);
                                format!("{} {}", progress_arc, circumference)
                            }
                            style="transition: stroke-dasharray 0.3s ease, stroke 0.3s ease;"
                        />
                    </svg>
                    {if show_info {
                        Some(view! {
                            <span
                                class=text_class.clone()
                                style=move || {
                                    let font_size = (size as f64 * 0.2).max(12.0);
                                    format!(
                                        "position: absolute; font-size: {}px; color: {}; font-weight: 500;",
                                        font_size,
                                        status_color(effective_status(), &stroke_color_for_text)
                                    )
                                }
                            >
                                {move || format_percent(percent.get())}
                            </span>
                        })
                    } else {
                        None
                    }}
                </div>
            }.into_any()
        }
    }
}

/// Mini progress indicator for inline use.
#[component]
pub fn MiniProgress(
    /// Current progress (0-100).
    #[prop(into)]
    percent: Signal<f64>,
    /// Width of the progress bar.
    #[prop(optional)]
    width: Option<u32>,
    /// Height of the progress bar.
    #[prop(optional)]
    height: Option<u32>,
    /// Progress status.
    #[prop(optional, into)]
    status: Option<ProgressStatus>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let width = width.unwrap_or(100);
    let height = height.unwrap_or(4);

    let progress_prefix = format!("fx-progress-{}", design_system);

    let combined_class = {
        let mut parts = vec![progress_prefix.clone(), format!("{}-mini", progress_prefix)];
        if let Some(s) = status {
            parts.push(s.class(&progress_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let get_color = move || {
        match status {
            Some(ProgressStatus::Success) => "var(--fx-color-success, #52c41a)",
            Some(ProgressStatus::Exception) => "var(--fx-color-error, #ff4d4f)",
            _ => "var(--fx-color-primary, #1677ff)",
        }
    };

    view! {
        <div
            class=combined_class
            style=format!(
                "width: {}px; height: {}px; background: var(--fx-color-fill-tertiary, rgba(0,0,0,0.04)); border-radius: {}px; overflow: hidden;",
                width, height, height / 2
            )
        >
            <div
                style=move || {
                    let p = percent.get().clamp(0.0, 100.0);
                    format!(
                        "width: {}%; height: 100%; background: {}; border-radius: {}px; transition: width 0.3s ease;",
                        p, get_color(), height / 2
                    )
                }
            />
        </div>
    }
}
