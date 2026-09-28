//! EpochCountdownTimer Leptos component.

use leptos::prelude::*;
use crate::try_use_theme;

/// EpochCountdownTimer component.
///
/// Countdown display to next network epoch with progress indicator.
///
/// # Props
///
/// - `epoch_end_time` - Unix timestamp (ms) of epoch end
/// - `current_epoch` - Current epoch number
/// - `epoch_duration` - Total epoch duration in ms
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::EpochCountdownTimer;
///
/// view! {
///     <EpochCountdownTimer
///         epoch_end_time=Signal::derive(move || now + 3600000)
///         current_epoch=142
///         epoch_duration=86400000
///     />
/// }
/// ```
#[component]
pub fn EpochCountdownTimer(
    /// Epoch end time (Unix ms).
    #[prop(into)]
    epoch_end_time: Signal<i64>,
    /// Current epoch number.
    #[prop(optional)]
    current_epoch: Option<u64>,
    /// Epoch duration in ms (for progress calculation).
    #[prop(optional)]
    epoch_duration: Option<i64>,
    /// Show progress bar.
    #[prop(optional)]
    show_progress: Option<bool>,
    /// Compact mode.
    #[prop(optional)]
    compact: Option<bool>,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// On epoch complete callback.
    #[prop(optional, into)]
    _on_epoch_complete: Option<Callback<u64>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_progress = show_progress.unwrap_or(true);
    let compact = compact.unwrap_or(false);
    let label = label.unwrap_or_else(|| "Next Epoch".to_string());

    // Build CSS classes
    let prefix = format!("fx-epoch-countdown-{}", design_system);

    // Pre-compute class names
    let base_class = prefix.clone();
    let compact_class = format!("{}-compact", prefix);
    let header_class = format!("{}-header", prefix);
    let label_class = format!("{}-label", prefix);
    let epoch_class = format!("{}-epoch", prefix);
    let time_class = format!("{}-time", prefix);
    let progress_class = format!("{}-progress", prefix);
    let progress_bar_class = format!("{}-progress-bar", prefix);

    let combined_class = {
        let base_class = base_class.clone();
        let compact_class = compact_class.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![base_class.clone()];
            if compact {
                parts.push(compact_class.clone());
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Current time signal (would need to be updated via interval in real app)
    let current_time = RwSignal::new(js_sys::Date::now() as i64);

    // Calculate remaining time
    let remaining_ms = move || {
        let end = epoch_end_time.get();
        let now = current_time.get();
        (end - now).max(0)
    };

    // Format time as HH:MM:SS or D:HH:MM:SS
    let formatted_time = move || {
        let ms = remaining_ms();
        let total_seconds = ms / 1000;
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if days > 0 {
            format!("{}d {:02}:{:02}:{:02}", days, hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
    };

    // Calculate progress percentage
    let progress_percent = move || {
        if let Some(duration) = epoch_duration {
            if duration > 0 {
                let remaining = remaining_ms();
                let elapsed = duration - remaining;
                ((elapsed as f64 / duration as f64) * 100.0).clamp(0.0, 100.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    };

    view! {
        <div class=combined_class>
            // Header with label and epoch number
            <div class=header_class.clone()>
                <span class=label_class.clone()>{label.clone()}</span>
                {current_epoch.map(|epoch| {
                    view! {
                        <span class=epoch_class.clone()>
                            {format!("Epoch #{}", epoch)}
                        </span>
                    }
                })}
            </div>

            // Countdown time
            <div class=time_class.clone()>
                {formatted_time}
            </div>

            // Progress bar
            {move || {
                if show_progress && epoch_duration.is_some() {
                    Some(view! {
                        <div class=progress_class.clone()>
                            <div
                                class=progress_bar_class.clone()
                                style=move || format!("width: {}%;", progress_percent())
                            />
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
