//! RoutingTableStatus Leptos component.
//!
//! Shows DHT routing table health and bucket status.

use leptos::prelude::*;
use crate::try_use_theme;

/// Routing bucket info.
#[derive(Debug, Clone, Default)]
pub struct RoutingBucket {
    /// Bucket index (distance).
    pub index: u8,
    /// Number of peers in bucket.
    pub peer_count: u32,
    /// Maximum bucket size.
    pub max_size: u32,
    /// Is bucket full?
    pub is_full: bool,
    /// Last refresh timestamp.
    pub last_refresh: Option<u64>,
}

impl RoutingBucket {
    /// Create a new routing bucket.
    pub fn new(index: u8, peer_count: u32, max_size: u32) -> Self {
        Self {
            index,
            peer_count,
            max_size,
            is_full: peer_count >= max_size,
            last_refresh: None,
        }
    }

    /// Set last refresh timestamp.
    pub fn last_refresh(mut self, timestamp: u64) -> Self {
        self.last_refresh = Some(timestamp);
        self
    }

    /// Get fill percentage.
    pub fn fill_percentage(&self) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            (self.peer_count as f64 / self.max_size as f64) * 100.0
        }
    }
}

/// RoutingTableStatus component.
///
/// Displays DHT routing table status with bucket visualization.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{RoutingTableStatus, RoutingBucket};
///
/// let buckets = (0..20).map(|i| {
///     RoutingBucket::new(i, (20 - i as u32) % 20, 20)
/// }).collect::<Vec<_>>();
///
/// view! {
///     <RoutingTableStatus
///         buckets=Signal::derive(move || buckets.clone())
///         total_peers=Signal::derive(move || 156)
///     />
/// }
/// ```
#[component]
pub fn RoutingTableStatus(
    /// Routing buckets.
    #[prop(into)]
    buckets: Signal<Vec<RoutingBucket>>,
    /// Total peers in routing table.
    #[prop(into)]
    total_peers: Signal<u32>,
    /// Target peer count (optional).
    #[prop(optional, into)]
    target_peers: Option<u32>,
    /// Last table refresh.
    #[prop(optional, into)]
    last_refresh: Option<Signal<String>>,
    /// Card title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Show bucket details.
    #[prop(optional)]
    show_buckets: Option<bool>,
    /// Compact visualization.
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

    let title = title.unwrap_or_else(|| "Routing Table".to_string());
    let target = target_peers.unwrap_or(200);
    let show_buckets = show_buckets.unwrap_or(true);
    let compact = compact.unwrap_or(false);

    let prefix = format!("fx-routing-table-{}", design_system);

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

    // Table health based on peer count
    let health_color = move || {
        let peers = total_peers.get();
        let pct = (peers as f64 / target as f64) * 100.0;
        if pct >= 80.0 {
            "var(--fx-color-success, #52c41a)"
        } else if pct >= 50.0 {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-error, #ff4d4f)"
        }
    };

    // Bucket fill color
    let bucket_color = |fill: f64| -> &'static str {
        if fill >= 80.0 {
            "var(--fx-color-success, #52c41a)"
        } else if fill >= 50.0 {
            "var(--fx-color-info, #1890ff)"
        } else if fill > 0.0 {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-border, #303030)"
        }
    };

    // Clone prefix for closures
    let buckets_prefix = prefix.clone();
    let stats_prefix = prefix.clone();

    view! {
        <div class=combined_class>
            // Header
            <div class=format!("{}-header", prefix)>
                <span class=format!("{}-icon", prefix)>"🗺️"</span>
                <span class=format!("{}-title", prefix)>{title}</span>
            </div>

            // Peer count summary
            <div class=format!("{}-summary", prefix)>
                <div class=format!("{}-peer-count", prefix)>
                    <span
                        class=format!("{}-count-value", prefix)
                        style=move || format!("color: {};", health_color())
                    >
                        {move || total_peers.get()}
                    </span>
                    <span class=format!("{}-count-label", prefix)>
                        {format!(" / {} peers", target)}
                    </span>
                </div>

                // Progress bar
                <div class=format!("{}-progress-container", prefix)>
                    <div
                        class=format!("{}-progress-bar", prefix)
                        style=move || {
                            let pct = ((total_peers.get() as f64 / target as f64) * 100.0).min(100.0);
                            format!("width: {}%; background: {};", pct, health_color())
                        }
                    />
                </div>

                {last_refresh.map(|lr| {
                    view! {
                        <div class=format!("{}-refresh", prefix)>
                            <span class=format!("{}-refresh-label", prefix)>"Last refresh: "</span>
                            <span class=format!("{}-refresh-value", prefix)>{move || lr.get()}</span>
                        </div>
                    }
                })}
            </div>

            // Bucket visualization
            {move || {
                let p = buckets_prefix.clone();
                if show_buckets {
                    Some(view! {
                        <div class=format!("{}-buckets", p)>
                            <div class=format!("{}-buckets-label", p)>"Bucket Fill Status"</div>
                            <div class=format!("{}-buckets-grid", p)>
                                {buckets.get().into_iter().map(|bucket| {
                                    let bp = p.clone();
                                    let fill = bucket.fill_percentage();
                                    let color = bucket_color(fill);
                                    let height = if compact {
                                        format!("{}%", fill.max(5.0))
                                    } else {
                                        format!("{}%", fill.max(10.0))
                                    };

                                    view! {
                                        <div
                                            class=format!("{}-bucket", bp)
                                            title=format!(
                                                "Bucket {}: {}/{} peers ({:.0}%)",
                                                bucket.index,
                                                bucket.peer_count,
                                                bucket.max_size,
                                                fill
                                            )
                                        >
                                            <div
                                                class=format!("{}-bucket-fill", bp)
                                                style=format!(
                                                    "height: {}; background: {};",
                                                    height, color
                                                )
                                            />
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                            <div class=format!("{}-buckets-legend", p)>
                                <span>"0"</span>
                                <span>"Distance"</span>
                                <span>{move || buckets.get().len().saturating_sub(1)}</span>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}

            // Bucket stats
            {move || {
                let p = stats_prefix.clone();
                if !compact {
                    let b = buckets.get();
                    let full_count = b.iter().filter(|b| b.is_full).count();
                    let empty_count = b.iter().filter(|b| b.peer_count == 0).count();
                    let total_buckets = b.len();

                    Some(view! {
                        <div class=format!("{}-stats", p)>
                            <div class=format!("{}-stat", p)>
                                <span class=format!("{}-stat-value", p)>{total_buckets}</span>
                                <span class=format!("{}-stat-label", p)>"Buckets"</span>
                            </div>
                            <div class=format!("{}-stat", p)>
                                <span class=format!("{}-stat-value {}-stat-full", p, p)>
                                    {full_count}
                                </span>
                                <span class=format!("{}-stat-label", p)>"Full"</span>
                            </div>
                            <div class=format!("{}-stat", p)>
                                <span class=format!("{}-stat-value {}-stat-empty", p, p)>
                                    {empty_count}
                                </span>
                                <span class=format!("{}-stat-label", p)>"Empty"</span>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
