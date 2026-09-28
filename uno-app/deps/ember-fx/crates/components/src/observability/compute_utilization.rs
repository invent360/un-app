//! ComputeUtilizationBar Leptos component.
//!
//! A progress bar showing compute/CPU core utilization.

use leptos::prelude::*;
use crate::try_use_theme;

/// ComputeUtilizationBar component.
///
/// Shows compute utilization with core count and task queue info.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::ComputeUtilizationBar;
///
/// view! {
///     <ComputeUtilizationBar
///         utilization=Signal::derive(move || 75.0)
///         core_count=8
///         active_tasks=12
///     />
/// }
/// ```
#[component]
pub fn ComputeUtilizationBar(
    /// Current utilization percentage (0-100).
    #[prop(into)]
    utilization: Signal<f64>,
    /// Number of CPU cores.
    #[prop(optional)]
    core_count: Option<u32>,
    /// Number of active tasks.
    #[prop(optional, into)]
    active_tasks: Option<Signal<u32>>,
    /// Queued tasks count.
    #[prop(optional, into)]
    queued_tasks: Option<Signal<u32>>,
    /// Show task info.
    #[prop(optional)]
    show_tasks: Option<bool>,
    /// Compact mode.
    #[prop(optional)]
    compact: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_tasks = show_tasks.unwrap_or(true);
    let compact = compact.unwrap_or(false);

    let prefix = format!("fx-compute-util-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if compact {
                parts.push(format!("{}-compact", prefix));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    let bar_color = move || {
        let val = utilization.get();
        if val >= 90.0 {
            "var(--fx-color-error, #ff4d4f)"
        } else if val >= 70.0 {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-success, #52c41a)"
        }
    };

    view! {
        <div class=combined_class>
            // Header
            <div class=format!("{}-header", prefix)>
                <span class=format!("{}-title", prefix)>
                    "⚡ Compute"
                    {core_count.map(|c| format!(" ({} cores)", c))}
                </span>
                <span
                    class=format!("{}-percent", prefix)
                    style=move || format!("color: {};", bar_color())
                >
                    {move || format!("{:.0}%", utilization.get())}
                </span>
            </div>

            // Progress bar
            <div class=format!("{}-bar-container", prefix)>
                <div
                    class=format!("{}-bar", prefix)
                    style=move || format!(
                        "width: {}%; background: {};",
                        utilization.get().clamp(0.0, 100.0),
                        bar_color()
                    )
                />
            </div>

            // Task info
            {move || {
                if show_tasks && (active_tasks.is_some() || queued_tasks.is_some()) {
                    Some(view! {
                        <div class=format!("{}-tasks", prefix)>
                            {active_tasks.map(|at| view! {
                                <span class=format!("{}-task-item", prefix)>
                                    <span class=format!("{}-task-label", prefix)>"Active:"</span>
                                    <span class=format!("{}-task-value", prefix)>{move || at.get()}</span>
                                </span>
                            })}
                            {queued_tasks.map(|qt| view! {
                                <span class=format!("{}-task-item", prefix)>
                                    <span class=format!("{}-task-label", prefix)>"Queued:"</span>
                                    <span class=format!("{}-task-value", prefix)>{move || qt.get()}</span>
                                </span>
                            })}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
