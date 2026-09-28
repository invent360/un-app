//! NetworkHealthBanner Leptos component.

use leptos::prelude::*;
use super::types::NetworkStatus;
use crate::try_use_theme;

/// NetworkHealthBanner component.
///
/// Top-of-page banner showing network-wide health status.
///
/// # Props
///
/// - `status` - Network health status
/// - `message` - Optional custom message
/// - `show_details` - Show additional details (peer count, latency)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{NetworkHealthBanner, NetworkStatus};
///
/// view! {
///     <NetworkHealthBanner
///         status=Signal::derive(move || NetworkStatus::Healthy)
///         message="All systems operational"
///     />
/// }
/// ```
#[component]
pub fn NetworkHealthBanner(
    /// Network health status.
    #[prop(into)]
    status: Signal<NetworkStatus>,
    /// Custom status message.
    #[prop(optional, into)]
    message: Option<String>,
    /// Show additional network details.
    #[prop(optional)]
    show_details: Option<bool>,
    /// Peer count (shown if show_details is true).
    #[prop(optional, into)]
    peer_count: Option<Signal<u32>>,
    /// Average latency in ms (shown if show_details is true).
    #[prop(optional, into)]
    avg_latency: Option<Signal<u32>>,
    /// On click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_details = show_details.unwrap_or(false);

    // Build CSS classes
    let prefix = format!("fx-network-banner-{}", design_system);

    // Pre-compute class names
    let base_class = prefix.clone();
    let status_class = format!("{}-status", prefix);
    let icon_class = format!("{}-icon", prefix);
    let label_class = format!("{}-label", prefix);
    let message_class = format!("{}-message", prefix);
    let details_class = format!("{}-details", prefix);
    let detail_item_class = format!("{}-detail-item", prefix);

    let combined_class = {
        let base_class = base_class.clone();
        let class = class.clone();
        move || {
            let s = status.get();
            let mut parts = vec![
                base_class.clone(),
                format!("{}-{}", base_class, s.as_suffix()),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Default message based on status
    let display_message = {
        let message = message.clone();
        move || {
            if let Some(ref msg) = message {
                msg.clone()
            } else {
                match status.get() {
                    NetworkStatus::Healthy => "Network is healthy".to_string(),
                    NetworkStatus::Degraded => "Network is experiencing issues".to_string(),
                    NetworkStatus::Critical => "Network has critical issues".to_string(),
                    NetworkStatus::Offline => "Network is offline".to_string(),
                    NetworkStatus::Connecting => "Connecting to network...".to_string(),
                }
            }
        }
    };

    let click_handler = move |_| {
        if let Some(ref cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <div
            class=combined_class
            on:click=click_handler
        >
            // Status section
            <div class=status_class.clone()>
                <span
                    class=icon_class.clone()
                    style=move || format!("color: {};", status.get().as_color())
                >
                    {move || status.get().as_icon()}
                </span>
                <span class=label_class.clone()>
                    {move || status.get().as_label()}
                </span>
                <span class=message_class.clone()>
                    {display_message}
                </span>
            </div>

            // Details section
            {move || {
                let details_class = details_class.clone();
                let detail_item_class_1 = detail_item_class.clone();
                let detail_item_class_2 = detail_item_class.clone();

                if show_details {
                    Some(view! {
                        <div class=details_class>
                            {peer_count.map(|pc| {
                                let class = detail_item_class_1.clone();
                                view! {
                                    <span class=class>
                                        {move || format!("{} peers", pc.get())}
                                    </span>
                                }
                            })}
                            {avg_latency.map(|lat| {
                                let class = detail_item_class_2.clone();
                                view! {
                                    <span class=class>
                                        {move || format!("{}ms avg", lat.get())}
                                    </span>
                                }
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
