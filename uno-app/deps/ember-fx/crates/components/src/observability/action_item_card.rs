//! ActionItemCard Leptos component.
//!
//! A card showing an action item with circular progress ring and count badge.

use leptos::prelude::*;
use crate::try_use_theme;

/// Status/color variants for action items.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActionItemStatus {
    #[default]
    Default,
    Warning,
    Success,
    Error,
    Info,
}

impl ActionItemStatus {
    /// Returns the primary CSS color.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Default => "#6b7280",
            Self::Warning => "#f59e0b",
            Self::Success => "#10b981",
            Self::Error => "#ef4444",
            Self::Info => "#3b82f6",
        }
    }

    /// Returns a lighter background color.
    pub fn as_bg_css(&self) -> &'static str {
        match self {
            Self::Default => "#f3f4f6",
            Self::Warning => "#fef3c7",
            Self::Success => "#d1fae5",
            Self::Error => "#fee2e2",
            Self::Info => "#dbeafe",
        }
    }

    /// Returns a border color.
    pub fn as_border_css(&self) -> &'static str {
        match self {
            Self::Default => "#e5e7eb",
            Self::Warning => "#fcd34d",
            Self::Success => "#6ee7b7",
            Self::Error => "#fca5a5",
            Self::Info => "#93c5fd",
        }
    }
}

/// Configuration for the action item card.
#[derive(Debug, Clone)]
pub struct ActionItemCardConfig {
    /// Card width.
    pub width: u32,
    /// Progress ring size.
    pub ring_size: u32,
    /// Progress ring stroke width.
    pub ring_stroke: f64,
    /// Show progress percentage in ring.
    pub show_progress_text: bool,
}

impl Default for ActionItemCardConfig {
    fn default() -> Self {
        Self {
            width: 160,
            ring_size: 40,
            ring_stroke: 4.0,
            show_progress_text: false,
        }
    }
}

/// ActionItemCard component.
///
/// Displays an action item with a circular progress ring, label, and count badge.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{ActionItemCard, ActionItemStatus};
///
/// view! {
///     <ActionItemCard
///         label="Hazard Assessment".to_string()
///         count=8
///         progress=Signal::derive(move || 0.75)
///         status=ActionItemStatus::Default
///     />
///     <ActionItemCard
///         label="Safety Training".to_string()
///         count=12
///         progress=Signal::derive(move || 0.5)
///         status=ActionItemStatus::Warning
///     />
///     <ActionItemCard
///         label="PPE Compliance".to_string()
///         count=5
///         progress=Signal::derive(move || 0.9)
///         status=ActionItemStatus::Success
///     />
/// }
/// ```
#[component]
pub fn ActionItemCard(
    /// Item label/title.
    #[prop(into)]
    label: String,
    /// Count value to display.
    #[prop(into)]
    count: Signal<u32>,
    /// Progress value (0.0 to 1.0).
    #[prop(into)]
    progress: Signal<f64>,
    /// Status/color variant.
    #[prop(optional)]
    status: Option<ActionItemStatus>,
    /// Configuration.
    #[prop(optional)]
    config: Option<ActionItemCardConfig>,
    /// Click callback.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let width = config.width;
    let ring_size = config.ring_size;
    let ring_stroke = config.ring_stroke;
    let show_progress_text = config.show_progress_text;

    let status = status.unwrap_or_default();

    let prefix = format!("fx-action-item-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Progress ring calculations
    let ring_radius = (ring_size as f64 / 2.0) - ring_stroke;
    let circumference = 2.0 * std::f64::consts::PI * ring_radius;

    let stroke_dasharray = move || {
        let pct = progress.get().clamp(0.0, 1.0);
        let filled = circumference * pct;
        format!("{:.1} {:.1}", filled, circumference - filled)
    };

    // Handle click
    let handle_click = move |_| {
        if let Some(ref cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <div
            class=combined_class
            style=format!(
                "width: {}px; padding: 12px; border-radius: 8px; background: {}; border: 1px solid {}; display: flex; align-items: center; gap: 10px; cursor: {};",
                width,
                status.as_bg_css(),
                status.as_border_css(),
                if on_click.is_some() { "pointer" } else { "default" }
            )
            on:click=handle_click
        >
            // Progress ring
            <div class=format!("{}-ring", prefix) style="flex-shrink: 0;">
                <svg
                    width=format!("{}", ring_size)
                    height=format!("{}", ring_size)
                    viewBox=format!("0 0 {} {}", ring_size, ring_size)
                >
                    // Background circle
                    <circle
                        cx=format!("{}", ring_size / 2)
                        cy=format!("{}", ring_size / 2)
                        r=format!("{:.1}", ring_radius)
                        fill="none"
                        stroke=status.as_border_css()
                        stroke-width=format!("{}", ring_stroke)
                        opacity="0.5"
                    />
                    // Progress arc
                    <circle
                        cx=format!("{}", ring_size / 2)
                        cy=format!("{}", ring_size / 2)
                        r=format!("{:.1}", ring_radius)
                        fill="none"
                        stroke=status.as_css()
                        stroke-width=format!("{}", ring_stroke)
                        stroke-dasharray=stroke_dasharray
                        stroke-linecap="round"
                        transform=format!("rotate(-90 {} {})", ring_size / 2, ring_size / 2)
                    />
                    // Progress text (optional)
                    {show_progress_text.then(|| {
                        view! {
                            <text
                                x=format!("{}", ring_size / 2)
                                y=format!("{}", ring_size / 2)
                                text-anchor="middle"
                                dominant-baseline="central"
                                fill=status.as_css()
                                font-size="10"
                                font-weight="500"
                            >
                                {move || format!("{:.0}%", progress.get() * 100.0)}
                            </text>
                        }
                    })}
                </svg>
            </div>

            // Label
            <div class=format!("{}-label", prefix) style="flex: 1; min-width: 0;">
                <span style=format!(
                    "font-size: 13px; font-weight: 500; color: {}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; display: block;",
                    status.as_css()
                )>
                    {label}
                </span>
            </div>

            // Count badge
            <div
                class=format!("{}-count", prefix)
                style=format!(
                    "font-size: 16px; font-weight: 600; color: {}; padding: 4px 8px; min-width: 32px; text-align: center;",
                    status.as_css()
                )
            >
                {move || format!("{:02}", count.get())}
            </div>
        </div>
    }
}

/// A group of action item cards.
#[component]
pub fn ActionItemList(
    /// Title for the action items section.
    #[prop(optional, into)]
    title: Option<String>,
    /// Children (ActionItemCard components).
    children: Children,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-action-list-{}", design_system);

    view! {
        <div class=format!("{} {}", prefix, class.unwrap_or_default())>
            // Title
            {title.map(|t| {
                view! {
                    <div class=format!("{}-title", prefix) style="font-size: 14px; font-weight: 600; margin-bottom: 12px; color: var(--fx-color-text, #374151);">
                        {t}
                    </div>
                }
            })}

            // Items
            <div class=format!("{}-items", prefix) style="display: flex; flex-direction: column; gap: 8px;">
                {children()}
            </div>
        </div>
    }
}
