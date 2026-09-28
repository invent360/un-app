//! FederationStatusPanel Leptos component.
//!
//! Shows federation/multi-chain connectivity status.

use leptos::prelude::*;
use crate::try_use_theme;

/// Federation chain status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChainStatus {
    /// Connected and synced.
    #[default]
    Connected,
    /// Connecting.
    Connecting,
    /// Syncing blocks.
    Syncing,
    /// Disconnected.
    Disconnected,
    /// Error state.
    Error,
}

impl ChainStatus {
    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Connected => "Connected",
            Self::Connecting => "Connecting",
            Self::Syncing => "Syncing",
            Self::Disconnected => "Disconnected",
            Self::Error => "Error",
        }
    }

    /// Returns the icon.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Connected => "✓",
            Self::Connecting => "◌",
            Self::Syncing => "↻",
            Self::Disconnected => "✗",
            Self::Error => "⚠",
        }
    }

    /// Returns the color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Connected => "var(--fx-color-success, #52c41a)",
            Self::Connecting => "var(--fx-color-info, #1890ff)",
            Self::Syncing => "var(--fx-color-warning, #faad14)",
            Self::Disconnected => "var(--fx-color-text-secondary, #8c8c8c)",
            Self::Error => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Connecting => "connecting",
            Self::Syncing => "syncing",
            Self::Disconnected => "disconnected",
            Self::Error => "error",
        }
    }
}

/// A federated chain entry.
#[derive(Debug, Clone)]
pub struct FederatedChain {
    /// Chain ID.
    pub id: String,
    /// Chain name.
    pub name: String,
    /// Chain status.
    pub status: ChainStatus,
    /// Block height.
    pub block_height: Option<u64>,
    /// Latency in ms.
    pub latency_ms: Option<u32>,
    /// Chain icon (emoji or URL).
    pub icon: Option<String>,
}

impl FederatedChain {
    /// Create a new federated chain.
    pub fn new(id: impl Into<String>, name: impl Into<String>, status: ChainStatus) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            status,
            block_height: None,
            latency_ms: None,
            icon: None,
        }
    }

    /// Set block height.
    pub fn block_height(mut self, height: u64) -> Self {
        self.block_height = Some(height);
        self
    }

    /// Set latency.
    pub fn latency(mut self, ms: u32) -> Self {
        self.latency_ms = Some(ms);
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// FederationStatusPanel component.
///
/// Displays status of multiple federated chains.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{FederationStatusPanel, FederatedChain, ChainStatus};
///
/// let chains = vec![
///     FederatedChain::new("eth", "Ethereum", ChainStatus::Connected)
///         .block_height(18_500_000)
///         .latency(45)
///         .icon("⟠"),
///     FederatedChain::new("dot", "Polkadot", ChainStatus::Syncing)
///         .block_height(17_200_000)
///         .icon("●"),
/// ];
///
/// view! {
///     <FederationStatusPanel
///         chains=Signal::derive(move || chains.clone())
///     />
/// }
/// ```
#[component]
pub fn FederationStatusPanel(
    /// List of federated chains.
    #[prop(into)]
    chains: Signal<Vec<FederatedChain>>,
    /// Panel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Show expanded details.
    #[prop(optional)]
    expanded: Option<bool>,
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

    let title = title.unwrap_or_else(|| "Federation Status".to_string());
    let expanded = expanded.unwrap_or(true);
    let compact = compact.unwrap_or(false);

    let prefix = format!("fx-federation-{}", design_system);

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

    // Connection summary
    let summary = move || {
        let c = chains.get();
        let total = c.len();
        let connected = c.iter().filter(|ch| ch.status == ChainStatus::Connected).count();
        (connected, total)
    };

    // Clone prefix for chains closure
    let chains_prefix = prefix.clone();

    view! {
        <div class=combined_class>
            // Header
            <div class=format!("{}-header", prefix)>
                <span class=format!("{}-icon", prefix)>"🔗"</span>
                <span class=format!("{}-title", prefix)>{title}</span>
                <span class=format!("{}-summary", prefix)>
                    {move || {
                        let (connected, total) = summary();
                        format!("{}/{} connected", connected, total)
                    }}
                </span>
            </div>

            // Chain list
            <div class=format!("{}-chains", prefix)>
                {move || {
                    let inner_prefix = chains_prefix.clone();
                    chains.get().into_iter().map(|chain| {
                        let p = inner_prefix.clone();
                        let status_class = format!(
                            "{}-chain-status {}-chain-status-{}",
                            p, p, chain.status.as_suffix()
                        );
                        let p2 = p.clone();
                        let p3 = p.clone();
                        let p4 = p.clone();

                        view! {
                            <div class=format!("{}-chain", p)>
                                // Icon/Name
                                <div class=format!("{}-chain-info", p)>
                                    {chain.icon.clone().map(|i| {
                                        view! {
                                            <span class=format!("{}-chain-icon", p2)>{i}</span>
                                        }
                                    })}
                                    <span class=format!("{}-chain-name", p)>
                                        {chain.name.clone()}
                                    </span>
                                </div>

                                // Status
                                <div class=status_class style=format!("color: {};", chain.status.as_color())>
                                    <span class=format!("{}-status-icon", p)>
                                        {chain.status.as_icon()}
                                    </span>
                                    <span class=format!("{}-status-label", p)>
                                        {chain.status.as_label()}
                                    </span>
                                </div>

                                // Details (if expanded)
                                {if expanded && !compact {
                                    Some(view! {
                                        <div class=format!("{}-chain-details", p3)>
                                            {chain.block_height.map(|h| {
                                                view! {
                                                    <span class=format!("{}-detail", p4)>
                                                        {"Block: "}{format_block_height(h)}
                                                    </span>
                                                }
                                            })}
                                            {chain.latency_ms.map(|l| {
                                                let latency_color = if l < 100 {
                                                    "var(--fx-color-success, #52c41a)"
                                                } else if l < 300 {
                                                    "var(--fx-color-warning, #faad14)"
                                                } else {
                                                    "var(--fx-color-error, #ff4d4f)"
                                                };
                                                view! {
                                                    <span
                                                        class=format!("{}-detail {}-latency", p4, p4)
                                                        style=format!("color: {};", latency_color)
                                                    >
                                                        {format!("{}ms", l)}
                                                    </span>
                                                }
                                            })}
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

/// Format block height with commas.
fn format_block_height(height: u64) -> String {
    let s = height.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
