//! SystemHealthCard Leptos component.
//!
//! A card displaying system health with circular gauge, issues count, and service status icons.

use leptos::prelude::*;
use crate::try_use_theme;

/// Health rating level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemHealthRating {
    /// Critical (0-49) - Red.
    Critical,
    /// Poor (50-69) - Red.
    Poor,
    /// Fair (70-89) - Yellow/Amber.
    Fair,
    /// Good (90-99) - Green.
    #[default]
    Good,
    /// Excellent (100) - Green.
    Excellent,
}

impl SystemHealthRating {
    /// Determine rating from score.
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=49 => Self::Critical,
            50..=69 => Self::Poor,
            70..=89 => Self::Fair,
            90..=99 => Self::Good,
            _ => Self::Excellent,
        }
    }

    /// Get the display label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::Poor => "POOR",
            Self::Fair => "FAIR",
            Self::Good => "GOOD",
            Self::Excellent => "EXCELLENT",
        }
    }

    /// Get the color for this rating.
    pub fn color(&self) -> &'static str {
        match self {
            Self::Critical | Self::Poor => "#dc2626",
            Self::Fair => "#d97706",
            Self::Good | Self::Excellent => "#65a30d",
        }
    }

    /// Get the badge background color.
    pub fn badge_bg(&self) -> &'static str {
        match self {
            Self::Critical | Self::Poor => "#fef2f2",
            Self::Fair => "#fffbeb",
            Self::Good | Self::Excellent => "#f0fdf4",
        }
    }
}

/// Service status indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ServiceStatus {
    /// Service is healthy (green check).
    #[default]
    Healthy,
    /// Service has issues (shows delta number).
    Issues(i32),
    /// Service status unknown/neutral (dash).
    Unknown,
}

impl ServiceStatus {
    /// Get the indicator color.
    pub fn color(&self) -> &'static str {
        match self {
            Self::Healthy => "#22c55e",
            Self::Issues(n) if *n < 0 => "#dc2626",
            Self::Issues(_) => "#f59e0b",
            Self::Unknown => "#9ca3af",
        }
    }
}

/// A service with icon and status.
#[derive(Debug, Clone)]
pub struct ServiceIndicator {
    /// Service name.
    pub name: String,
    /// Icon type.
    pub icon: ServiceIcon,
    /// Service status.
    pub status: ServiceStatus,
}

impl ServiceIndicator {
    /// Create a new service indicator.
    pub fn new(name: impl Into<String>, icon: ServiceIcon, status: ServiceStatus) -> Self {
        Self {
            name: name.into(),
            icon,
            status,
        }
    }

    /// Create a healthy service.
    pub fn healthy(name: impl Into<String>, icon: ServiceIcon) -> Self {
        Self::new(name, icon, ServiceStatus::Healthy)
    }

    /// Create a service with issues.
    pub fn issues(name: impl Into<String>, icon: ServiceIcon, delta: i32) -> Self {
        Self::new(name, icon, ServiceStatus::Issues(delta))
    }

    /// Create an unknown status service.
    pub fn unknown(name: impl Into<String>, icon: ServiceIcon) -> Self {
        Self::new(name, icon, ServiceStatus::Unknown)
    }
}

/// Service icon types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ServiceIcon {
    /// Document/file icon.
    #[default]
    Document,
    /// Settings/gear icon.
    Settings,
    /// Cloud icon.
    Cloud,
    /// Database icon.
    Database,
    /// Shield/security icon.
    Shield,
    /// Server icon.
    Server,
    /// Network icon.
    Network,
}

/// Configuration for the system health card.
#[derive(Debug, Clone)]
pub struct SystemHealthCardConfig {
    /// Gauge size (diameter).
    pub gauge_size: u32,
    /// Show checkbox.
    pub show_checkbox: bool,
    /// Show action button.
    pub show_action: bool,
}

impl Default for SystemHealthCardConfig {
    fn default() -> Self {
        Self {
            gauge_size: 120,
            show_checkbox: true,
            show_action: true,
        }
    }
}

/// Render service icon SVG.
fn render_service_icon(icon: ServiceIcon) -> impl IntoView {
    match icon {
        ServiceIcon::Document => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6zM6 4h7v5h5v11H6V4z"/>
                <path d="M8 12h8v2H8zM8 16h5v2H8z"/>
            </svg>
        }.into_any(),
        ServiceIcon::Settings => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <path d="M12 15.5A3.5 3.5 0 1112 8.5a3.5 3.5 0 010 7zm7.43-2.53l1.63.95-.97 1.68-1.79-.6a6.48 6.48 0 01-1.3.75l-.38 1.75h-1.95l-.37-1.75a6.48 6.48 0 01-1.3-.75l-1.8.6-.96-1.68 1.63-.95a5.85 5.85 0 010-1.5l-1.63-.95.97-1.68 1.79.6c.38-.3.82-.55 1.3-.75l.38-1.75h1.95l.37 1.75c.48.2.92.45 1.3.75l1.8-.6.96 1.68-1.63.95c.08.5.08 1 0 1.5z"/>
            </svg>
        }.into_any(),
        ServiceIcon::Cloud => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <path d="M19.35 10.04A7.49 7.49 0 0012 4C9.11 4 6.6 5.64 5.35 8.04A5.994 5.994 0 000 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z"/>
            </svg>
        }.into_any(),
        ServiceIcon::Database => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <path d="M12 3C7.58 3 4 4.79 4 7v10c0 2.21 3.58 4 8 4s8-1.79 8-4V7c0-2.21-3.58-4-8-4zm0 2c3.87 0 6 1.5 6 2s-2.13 2-6 2-6-1.5-6-2 2.13-2 6-2zm6 12c0 .5-2.13 2-6 2s-6-1.5-6-2v-2.23c1.61.78 3.72 1.23 6 1.23s4.39-.45 6-1.23V17zm0-5c0 .5-2.13 2-6 2s-6-1.5-6-2V9.77C7.61 10.55 9.72 11 12 11s4.39-.45 6-1.23V12z"/>
            </svg>
        }.into_any(),
        ServiceIcon::Shield => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
            </svg>
        }.into_any(),
        ServiceIcon::Server => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <path d="M4 1h16a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2V3a2 2 0 012-2zm0 8h16a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4a2 2 0 012-2zm0 8h16a2 2 0 012 2v2a2 2 0 01-2 2H4a2 2 0 01-2-2v-2a2 2 0 012-2zM6 4.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3z"/>
            </svg>
        }.into_any(),
        ServiceIcon::Network => view! {
            <svg width="24" height="24" viewBox="0 0 24 24" fill="#3b82f6">
                <circle cx="12" cy="12" r="3"/>
                <circle cx="4" cy="12" r="2"/>
                <circle cx="20" cy="12" r="2"/>
                <circle cx="12" cy="4" r="2"/>
                <circle cx="12" cy="20" r="2"/>
                <path d="M9 12H6m9 0h3M12 9V6m0 9v3" stroke="#3b82f6" stroke-width="1.5" fill="none"/>
            </svg>
        }.into_any(),
    }
}

/// SystemHealthCard component.
///
/// A card showing system health score, issues count, and service status indicators.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     SystemHealthCard, ServiceIndicator, ServiceIcon, ServiceStatus
/// };
///
/// let services = vec![
///     ServiceIndicator::healthy("Documents", ServiceIcon::Document),
///     ServiceIndicator::healthy("Settings", ServiceIcon::Settings),
///     ServiceIndicator::issues("Cloud", ServiceIcon::Cloud, -30),
///     ServiceIndicator::healthy("Database", ServiceIcon::Database),
///     ServiceIndicator::healthy("Security", ServiceIcon::Shield),
/// ];
///
/// view! {
///     <SystemHealthCard
///         name="Manufacturing_Dev".to_string()
///         subtitle="PowerStore 9000 | RV429L63".to_string()
///         health_score=Signal::derive(|| 70)
///         issues=Signal::derive(|| 1)
///         services=Signal::derive(move || services.clone())
///     />
/// }
/// ```
#[component]
pub fn SystemHealthCard(
    /// System name.
    #[prop(into)]
    name: String,
    /// Subtitle (e.g., device model | serial).
    #[prop(optional, into)]
    subtitle: Option<String>,
    /// Health score (0-100).
    #[prop(into)]
    health_score: Signal<u32>,
    /// Number of issues.
    #[prop(into, default = Signal::derive(|| 0))]
    issues: Signal<u32>,
    /// Service indicators.
    #[prop(into, default = Signal::derive(|| vec![]))]
    services: Signal<Vec<ServiceIndicator>>,
    /// Checkbox selected state.
    #[prop(optional, into)]
    selected: Option<RwSignal<bool>>,
    /// Action click handler.
    #[prop(optional, into)]
    on_action: Option<Callback<()>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<SystemHealthCardConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let gauge_size = config.gauge_size;
    let show_checkbox = config.show_checkbox;
    let show_action = config.show_action;

    let prefix = format!("fx-system-health-{}", design_system);

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

    // Calculate rating
    let rating = move || SystemHealthRating::from_score(health_score.get());

    // SVG gauge calculations
    let stroke_width = 8.0;
    let radius = (gauge_size as f64 / 2.0) - stroke_width;
    let circumference = 2.0 * std::f64::consts::PI * radius;

    let stroke_dashoffset = move || {
        let score = health_score.get().min(100) as f64;
        circumference * (1.0 - score / 100.0)
    };

    // Handle checkbox toggle
    let handle_checkbox = move |_| {
        if let Some(sel) = selected {
            sel.update(|v| *v = !*v);
        }
    };

    // Handle action click
    let handle_action = move |_| {
        if let Some(ref cb) = on_action {
            cb.run(());
        }
    };

    view! {
        <div
            class=combined_class
            style="background: #fff; border: 1px solid #e5e7eb; border-radius: 8px; padding: 16px; min-width: 320px;"
        >
            // Header
            <div style="display: flex; align-items: flex-start; gap: 12px; margin-bottom: 16px; padding-bottom: 12px; border-bottom: 1px solid #f3f4f6;">
                // Checkbox
                {show_checkbox.then(|| {
                    view! {
                        <div
                            style="cursor: pointer; margin-top: 4px;"
                            on:click=handle_checkbox
                        >
                            <svg width="18" height="18" viewBox="0 0 18 18">
                                {move || {
                                    let is_selected = selected.map(|s| s.get()).unwrap_or(false);
                                    if is_selected {
                                        view! {
                                            <rect x="1" y="1" width="16" height="16" rx="3" fill="#3b82f6" stroke="#3b82f6"/>
                                            <path d="M4 9l3 3 7-7" stroke="#fff" stroke-width="2" fill="none"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <rect x="1" y="1" width="16" height="16" rx="3" fill="none" stroke="#d1d5db" stroke-width="1.5"/>
                                        }.into_any()
                                    }
                                }}
                            </svg>
                        </div>
                    }
                })}

                // Score badge
                <div style=move || format!(
                    "display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: 50%; background: {}; color: {}; font-weight: 700; font-size: 12px;",
                    rating().badge_bg(), rating().color()
                )>
                    {move || health_score.get()}
                </div>

                // Name and subtitle
                <div style="flex: 1;">
                    <div style="color: #3b82f6; font-size: 18px; font-weight: 600;">
                        {name.clone()}
                    </div>
                    {subtitle.clone().map(|sub| {
                        view! {
                            <div style="color: #6b7280; font-size: 13px; margin-top: 2px;">
                                {sub}
                            </div>
                        }
                    })}
                </div>

                // Action button
                {show_action.then(|| {
                    view! {
                        <button
                            style="background: none; border: none; cursor: pointer; padding: 4px; color: #9ca3af;"
                            on:click=handle_action
                        >
                            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5">
                                <path d="M4 4l12 12M16 4L4 16"/>
                            </svg>
                        </button>
                    }
                })}
            </div>

            // Content
            <div style="display: flex; gap: 24px;">
                // Health Score gauge
                <div style="flex-shrink: 0;">
                    <div style="color: #6b7280; font-size: 12px; font-weight: 500; margin-bottom: 8px;">
                        "Health Score"
                    </div>
                    <div style=format!("position: relative; width: {}px; height: {}px;", gauge_size, gauge_size)>
                        <svg
                            width=format!("{}", gauge_size)
                            height=format!("{}", gauge_size)
                            viewBox=format!("0 0 {} {}", gauge_size, gauge_size)
                            style="transform: rotate(-90deg);"
                        >
                            // Background circle
                            <circle
                                cx=format!("{}", gauge_size / 2)
                                cy=format!("{}", gauge_size / 2)
                                r=format!("{:.1}", radius)
                                fill="none"
                                stroke="#e5e7eb"
                                stroke-width=format!("{}", stroke_width)
                            />
                            // Progress circle
                            <circle
                                cx=format!("{}", gauge_size / 2)
                                cy=format!("{}", gauge_size / 2)
                                r=format!("{:.1}", radius)
                                fill="none"
                                stroke=move || rating().color()
                                stroke-width=format!("{}", stroke_width)
                                stroke-linecap="round"
                                stroke-dasharray=format!("{:.1}", circumference)
                                stroke-dashoffset=move || format!("{:.1}", stroke_dashoffset())
                                style="transition: stroke-dashoffset 0.5s ease-out;"
                            />
                        </svg>
                        // Center text
                        <div style=format!(
                            "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center;"
                        )>
                            <div style=move || format!(
                                "font-size: 32px; font-weight: 700; color: {}; line-height: 1;",
                                rating().color()
                            )>
                                {move || health_score.get()}
                            </div>
                            <div style=move || format!(
                                "font-size: 11px; font-weight: 600; color: {}; margin-top: 2px;",
                                rating().color()
                            )>
                                {move || rating().label()}
                            </div>
                        </div>
                    </div>
                </div>

                // Issues and Services
                <div style="flex: 1;">
                    // Issues count
                    <div style="margin-bottom: 16px;">
                        <div style="color: #6b7280; font-size: 12px; font-weight: 500; margin-bottom: 4px;">
                            "Issues"
                        </div>
                        <div style="font-size: 32px; font-weight: 300; color: #374151;">
                            {move || issues.get()}
                        </div>
                    </div>

                    // Service icons
                    <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                        {move || {
                            services.get().into_iter().map(|service| {
                                let status_indicator = match service.status {
                                    ServiceStatus::Healthy => view! {
                                        <svg width="14" height="14" viewBox="0 0 14 14">
                                            <path d="M2 7l3 3 7-7" stroke="#22c55e" stroke-width="2" fill="none"/>
                                        </svg>
                                    }.into_any(),
                                    ServiceStatus::Issues(n) => view! {
                                        <span style=format!(
                                            "font-size: 11px; font-weight: 600; color: {};",
                                            if n < 0 { "#dc2626" } else { "#f59e0b" }
                                        )>
                                            {n}
                                        </span>
                                    }.into_any(),
                                    ServiceStatus::Unknown => view! {
                                        <span style="color: #9ca3af; font-size: 14px; font-weight: 600;">"—"</span>
                                    }.into_any(),
                                };

                                view! {
                                    <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
                                        {render_service_icon(service.icon)}
                                        {status_indicator}
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
