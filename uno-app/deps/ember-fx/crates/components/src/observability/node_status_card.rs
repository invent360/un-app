//! NodeStatusCard Leptos component.

use leptos::prelude::*;
use super::types::NodeState;
use crate::panel::StatCardSize;
use crate::try_use_theme;

/// NodeStatusCard component.
///
/// A StatCard variant with a status dot showing node state.
///
/// # Props
///
/// - `node_name` - Node identifier
/// - `status` - Current node status
/// - `uptime` - Uptime percentage
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{NodeStatusCard, NodeState};
///
/// view! {
///     <NodeStatusCard
///         node_name="ember-node-7"
///         status=Signal::derive(move || NodeState::Online)
///         uptime=0.9975
///     />
/// }
/// ```
#[component]
pub fn NodeStatusCard(
    /// Node name/identifier.
    #[prop(into)]
    node_name: String,
    /// Node status.
    #[prop(into)]
    status: Signal<NodeState>,
    /// Uptime percentage (0.0 to 1.0).
    #[prop(optional)]
    uptime: Option<f64>,
    /// Peer count.
    #[prop(optional)]
    peer_count: Option<u32>,
    /// Block height.
    #[prop(optional)]
    block_height: Option<u64>,
    /// Last sync time (Unix ms).
    #[prop(optional)]
    _last_sync: Option<i64>,
    /// Card size.
    #[prop(optional)]
    size: StatCardSize,
    /// Show details footer.
    #[prop(optional)]
    show_details: Option<bool>,
    /// On click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<String>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_details = show_details.unwrap_or(true);

    // Build CSS classes
    let card_prefix = format!("fx-statcard-{}", design_system);
    let status_prefix = format!("fx-node-status-card-{}", design_system);

    // Pre-compute class names
    let card_class = card_prefix.clone();
    let card_size_class = format!("{}-{}", card_prefix, size.as_str());
    let status_card_class = status_prefix.clone();
    let dot_class = format!("{}-dot", status_prefix);
    let title_class = format!("{}-title", card_prefix);
    let value_class = format!("{}-value", card_prefix);
    let footer_class = format!("{}-footer", card_prefix);
    let detail_class = format!("{}-detail", status_prefix);
    let detail_label_class = format!("{}-detail-label", status_prefix);
    let detail_value_class = format!("{}-detail-value", status_prefix);

    let combined_class = {
        let card_class = card_class.clone();
        let card_size_class = card_size_class.clone();
        let status_card_class = status_card_class.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![
                card_class.clone(),
                card_size_class.clone(),
                status_card_class.clone(),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    let dot_full_class = {
        let dot_class = dot_class.clone();
        move || {
            let s = status.get();
            format!("{} {}-{}", dot_class, dot_class, s.as_suffix())
        }
    };

    let node_name_click = node_name.clone();
    let click_handler = move |_| {
        if let Some(ref cb) = on_click {
            cb.run(node_name_click.clone());
        }
    };

    view! {
        <div
            class=combined_class
            on:click=click_handler
        >
            // Status dot
            <span
                class=dot_full_class
                style=move || format!("background: {};", status.get().as_color())
            />

            // Title (node name)
            <div class=title_class.clone()>{node_name.clone()}</div>

            // Status value
            <div class=value_class.clone()>
                {move || status.get().as_label()}
            </div>

            // Details footer
            {move || {
                if show_details {
                    Some(view! {
                        <div class=footer_class.clone()>
                            {uptime.map(|u| {
                                view! {
                                    <div class=detail_class.clone()>
                                        <span class=detail_label_class.clone()>"Uptime"</span>
                                        <span class=detail_value_class.clone()>
                                            {format!("{:.2}%", u * 100.0)}
                                        </span>
                                    </div>
                                }
                            })}
                            {peer_count.map(|p| {
                                view! {
                                    <div class=detail_class.clone()>
                                        <span class=detail_label_class.clone()>"Peers"</span>
                                        <span class=detail_value_class.clone()>
                                            {p.to_string()}
                                        </span>
                                    </div>
                                }
                            })}
                            {block_height.map(|b| {
                                view! {
                                    <div class=detail_class.clone()>
                                        <span class=detail_label_class.clone()>"Block"</span>
                                        <span class=detail_value_class.clone()>
                                            {format!("#{}", b)}
                                        </span>
                                    </div>
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
