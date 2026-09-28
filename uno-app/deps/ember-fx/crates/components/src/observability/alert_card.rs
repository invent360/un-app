//! AlertCard Leptos component.

use leptos::prelude::*;
use super::types::{Alert, AlertId, Severity, AlertState};
use crate::try_use_theme;

/// AlertCard component.
///
/// Display alert status with severity indicator.
///
/// # Props
///
/// - `alert` - Alert data
/// - `on_acknowledge` - Acknowledge handler
/// - `on_silence` - Silence handler
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{AlertCard, Alert, Severity};
///
/// let alert = Alert::new("1", "High CPU", Severity::Critical, "CPU > 90%", now);
///
/// view! {
///     <AlertCard alert=Signal::derive(move || alert.clone()) />
/// }
/// ```
#[component]
pub fn AlertCard(
    /// Alert data.
    alert: Signal<Alert>,
    /// Acknowledge handler.
    #[prop(optional, into)]
    on_acknowledge: Option<Callback<AlertId>>,
    /// Silence handler.
    #[prop(optional, into)]
    on_silence: Option<Callback<AlertId>>,
    /// Show actions.
    #[prop(optional)]
    show_actions: Option<bool>,
    /// Compact mode.
    #[prop(optional)]
    compact: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_actions = show_actions.unwrap_or(true);
    let compact = compact.unwrap_or(false);

    // Build CSS classes
    let alert_prefix = format!("fx-alert-card-{}", design_system);

    // Pre-compute class names
    let base_class = alert_prefix.clone();
    let compact_class = format!("{}-compact", alert_prefix);
    let severity_indicator_class = format!("{}-severity-indicator", alert_prefix);
    let content_class = format!("{}-content", alert_prefix);
    let header_class = format!("{}-header", alert_prefix);
    let severity_badge_class = format!("{}-severity-badge", alert_prefix);
    let name_class = format!("{}-name", alert_prefix);
    let time_class = format!("{}-time", alert_prefix);
    let message_class = format!("{}-message", alert_prefix);
    let labels_class = format!("{}-labels", alert_prefix);
    let label_class = format!("{}-label", alert_prefix);
    let label_key_class = format!("{}-label-key", alert_prefix);
    let label_value_class = format!("{}-label-value", alert_prefix);
    let actions_class = format!("{}-actions", alert_prefix);
    let action_btn_class = format!("{}-action-btn", alert_prefix);
    let ack_btn_class = format!("{}-ack-btn", alert_prefix);
    let silence_btn_class = format!("{}-silence-btn", alert_prefix);
    let state_class = format!("{}-state", alert_prefix);
    let state_badge_class = format!("{}-state-badge", alert_prefix);

    let combined_class = {
        let base_class = base_class.clone();
        let compact_class = compact_class.clone();
        let class = class.clone();
        move || {
            let a = alert.get();
            let mut parts = vec![base_class.clone()];
            parts.push(format!("{}-{}", base_class, a.severity.as_suffix()));
            parts.push(format!("{}-{}", base_class, a.state.as_suffix()));
            if compact {
                parts.push(compact_class.clone());
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Format relative time
    let format_relative_time = |fired_at: i64| {
        let now = js_sys::Date::now() as i64;
        let diff_ms = now - fired_at;
        let diff_secs = diff_ms / 1000;
        let diff_mins = diff_secs / 60;
        let diff_hours = diff_mins / 60;
        let diff_days = diff_hours / 24;

        if diff_days > 0 {
            format!("{}d ago", diff_days)
        } else if diff_hours > 0 {
            format!("{}h ago", diff_hours)
        } else if diff_mins > 0 {
            format!("{}m ago", diff_mins)
        } else {
            "just now".to_string()
        }
    };

    view! {
        <div class=combined_class>
            // Severity indicator
            <div
                class=severity_indicator_class
                style=move || format!("background-color: {}", alert.get().severity.as_color())
            ></div>

            // Content
            <div class=content_class>
                // Header row
                <div class=header_class>
                    // Severity badge
                    <span
                        class=severity_badge_class
                        style=move || format!("color: {}", alert.get().severity.as_color())
                    >
                        {move || alert.get().severity.as_icon()}
                        " "
                        {move || alert.get().severity.as_label()}
                    </span>

                    // Alert name
                    <span class=name_class>
                        {move || alert.get().name.clone()}
                    </span>

                    // Time
                    <span class=time_class>
                        {move || format_relative_time(alert.get().fired_at)}
                    </span>
                </div>

                // Message
                {(!compact).then(|| view! {
                    <div class=message_class.clone()>
                        {move || alert.get().message.clone()}
                    </div>
                })}

                // Labels
                {move || {
                    let labels_class = labels_class.clone();
                    let label_class = label_class.clone();
                    let label_key_class = label_key_class.clone();
                    let label_value_class = label_value_class.clone();
                    let a = alert.get();
                    (!compact && !a.labels.is_empty()).then(move || {
                        let labels: Vec<_> = a.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                        view! {
                            <div class=labels_class>
                                {labels.into_iter().map(|(k, v)| {
                                    let lc = label_class.clone();
                                    let lkc = label_key_class.clone();
                                    let lvc = label_value_class.clone();
                                    view! {
                                        <span class=lc>
                                            <span class=lkc>{k}"="</span>
                                            <span class=lvc>{v}</span>
                                        </span>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    })
                }}

                // Actions
                {show_actions.then(|| {
                    let on_ack = on_acknowledge.clone();
                    let on_sil = on_silence.clone();
                    let actions_class = actions_class.clone();
                    let action_btn_class = action_btn_class.clone();
                    let ack_btn_class = ack_btn_class.clone();
                    let silence_btn_class = silence_btn_class.clone();

                    view! {
                        <div class=actions_class>
                            {on_ack.map(|callback| {
                                let alert_id = move || alert.get().id.clone();
                                let btn_class = format!("{} {}", action_btn_class, ack_btn_class);
                                view! {
                                    <button
                                        class=btn_class
                                        on:click=move |_| {
                                            callback.run(alert_id());
                                        }
                                    >
                                        "Ack"
                                    </button>
                                }
                            })}
                            {on_sil.map(|callback| {
                                let alert_id = move || alert.get().id.clone();
                                let btn_class = format!("{} {}", action_btn_class, silence_btn_class);
                                view! {
                                    <button
                                        class=btn_class
                                        on:click=move |_| {
                                            callback.run(alert_id());
                                        }
                                    >
                                        "Silence"
                                    </button>
                                }
                            })}
                        </div>
                    }
                })}
            </div>

            // State indicator
            <div class=state_class>
                <span class=move || format!("{} {}-{}", state_badge_class, state_badge_class, alert.get().state.as_suffix())>
                    {move || alert.get().state.as_label()}
                </span>
            </div>
        </div>
    }
}

/// AlertList component.
///
/// Display a list of alerts.
#[component]
pub fn AlertList(
    /// Alerts to display.
    alerts: Signal<Vec<Alert>>,
    /// Show only firing alerts.
    #[prop(optional)]
    firing_only: Option<bool>,
    /// Acknowledge handler.
    #[prop(optional, into)]
    on_acknowledge: Option<Callback<AlertId>>,
    /// Silence handler.
    #[prop(optional, into)]
    on_silence: Option<Callback<AlertId>>,
    /// Maximum alerts to show.
    #[prop(optional)]
    max_alerts: Option<usize>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let firing_only = firing_only.unwrap_or(false);
    let max_alerts = max_alerts.unwrap_or(10);

    // Build CSS classes
    let list_prefix = format!("fx-alert-list-{}", design_system);

    // Pre-compute class names
    let empty_class = format!("{}-empty", list_prefix);
    let items_class = format!("{}-items", list_prefix);
    let loading_overlay_class = format!("{}-loading-overlay", list_prefix);
    let loading_spinner_class = format!("{}-loading-spinner", list_prefix);

    let combined_class = {
        let mut parts = vec![list_prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", list_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Filter and sort alerts
    let filtered_alerts = move || {
        let mut list: Vec<_> = alerts.get()
            .into_iter()
            .filter(|a| !firing_only || a.state == AlertState::Firing)
            .collect();

        // Sort by severity (critical first) then by time (newest first)
        list.sort_by(|a, b| {
            b.severity.cmp(&a.severity)
                .then_with(|| b.fired_at.cmp(&a.fired_at))
        });

        list.into_iter().take(max_alerts).collect::<Vec<_>>()
    };

    view! {
        <div class=combined_class>
            {move || {
                let empty_class = empty_class.clone();
                let items_class = items_class.clone();
                let list = filtered_alerts();
                if list.is_empty() {
                    view! {
                        <div class=empty_class>
                            "No alerts"
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class=items_class>
                            {list.into_iter().map(|alert| {
                                let alert_signal = Signal::derive(move || alert.clone());
                                // Conditionally pass callbacks to avoid Option<Callback> trait issues
                                match (on_acknowledge.clone(), on_silence.clone()) {
                                    (Some(ack), Some(sil)) => view! {
                                        <AlertCard alert=alert_signal on_acknowledge=ack on_silence=sil compact=true />
                                    }.into_any(),
                                    (Some(ack), None) => view! {
                                        <AlertCard alert=alert_signal on_acknowledge=ack compact=true />
                                    }.into_any(),
                                    (None, Some(sil)) => view! {
                                        <AlertCard alert=alert_signal on_silence=sil compact=true />
                                    }.into_any(),
                                    (None, None) => view! {
                                        <AlertCard alert=alert_signal compact=true />
                                    }.into_any(),
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }}

            // Loading overlay
            {loading.then(|| view! {
                <div class=loading_overlay_class>
                    <span class=loading_spinner_class></span>
                </div>
            })}
        </div>
    }
}
