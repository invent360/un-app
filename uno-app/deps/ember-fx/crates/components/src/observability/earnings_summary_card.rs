//! EarningsSummaryCard Leptos component.

use leptos::prelude::*;
use super::types::EarningsCategory;
use crate::panel::{StatCardSize, TrendFlag};
use crate::try_use_theme;

/// EarningsSummaryCard component.
///
/// A StatCard variant showing token earnings with breakdown.
///
/// # Props
///
/// - `total` - Total earnings value
/// - `token_symbol` - Token symbol (e.g., "EMB")
/// - `breakdown` - Earnings breakdown by category
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{EarningsSummaryCard, EarningsCategory};
///
/// view! {
///     <EarningsSummaryCard
///         total=Signal::derive(move || 1234.56)
///         token_symbol="EMB"
///         breakdown=vec![
///             EarningsCategory::new("Storage", 800.0),
///             EarningsCategory::new("Relay", 434.56),
///         ]
///     />
/// }
/// ```
#[component]
pub fn EarningsSummaryCard(
    /// Total earnings value.
    #[prop(into)]
    total: Signal<f64>,
    /// Token symbol.
    #[prop(optional, into)]
    token_symbol: Option<String>,
    /// Fiat value (optional).
    #[prop(optional, into)]
    fiat_value: Option<Signal<f64>>,
    /// Fiat currency symbol.
    #[prop(optional, into)]
    fiat_symbol: Option<String>,
    /// Earnings breakdown by category.
    #[prop(optional)]
    breakdown: Option<Vec<EarningsCategory>>,
    /// Time period label.
    #[prop(optional, into)]
    period: Option<String>,
    /// Trend compared to previous period.
    #[prop(optional)]
    trend: Option<TrendFlag>,
    /// Trend value.
    #[prop(optional, into)]
    trend_value: Option<String>,
    /// Card size.
    #[prop(optional)]
    size: StatCardSize,
    /// Show breakdown in footer.
    #[prop(optional)]
    show_breakdown: Option<bool>,
    /// Use icons instead of dots for categories.
    #[prop(optional)]
    use_icons: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let token_symbol = token_symbol.unwrap_or_else(|| "EMB".to_string());
    let fiat_symbol = fiat_symbol.unwrap_or_else(|| "$".to_string());
    let show_breakdown = show_breakdown.unwrap_or(true);
    let use_icons = use_icons.unwrap_or(false);

    // Build CSS classes
    let card_prefix = format!("fx-statcard-{}", design_system);
    let earnings_prefix = format!("fx-earnings-card-{}", design_system);

    // Pre-compute class names
    let card_class = card_prefix.clone();
    let card_size_class = format!("{}-{}", card_prefix, size.as_str());
    let earnings_class = earnings_prefix.clone();
    let title_class = format!("{}-title", card_prefix);
    let total_class = format!("{}-total", earnings_prefix);
    let amount_class = format!("{}-amount", earnings_prefix);
    let symbol_class = format!("{}-symbol", earnings_prefix);
    let fiat_class = format!("{}-fiat", earnings_prefix);
    let trend_class = format!("{}-trend", earnings_prefix);
    let breakdown_class = format!("{}-breakdown", earnings_prefix);
    let category_class = format!("{}-category", earnings_prefix);
    let category_name_class = format!("{}-category-name", earnings_prefix);
    let category_dot_class = format!("{}-category-dot", earnings_prefix);
    let category_icon_class = format!("{}-category-icon", earnings_prefix);
    let category_value_class = format!("{}-category-value", earnings_prefix);

    let combined_class = {
        let card_class = card_class.clone();
        let card_size_class = card_size_class.clone();
        let earnings_class = earnings_class.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![
                card_class.clone(),
                card_size_class.clone(),
                earnings_class.clone(),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Format total with commas (manual implementation since Rust doesn't support {:,})
    let formatted_total = move || {
        let val = total.get();
        if val >= 1000.0 {
            // Manual comma formatting
            let integer_part = val.trunc() as i64;
            let decimal_part = ((val.fract() * 100.0).round() as i64).abs();
            let int_str = integer_part.to_string();
            let with_commas: String = int_str
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    if i > 0 && i % 3 == 0 { format!("{},", c) } else { c.to_string() }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            format!("{}.{:02}", with_commas, decimal_part)
        } else {
            format!("{:.4}", val)
        }
    };

    // Default category colors
    let category_colors = [
        "var(--fx-color-orange, #fa8c16)",
        "var(--fx-color-cyan, #13c2c2)",
        "var(--fx-color-purple, #722ed1)",
        "var(--fx-color-geekblue, #2f54eb)",
        "var(--fx-color-green, #52c41a)",
        "var(--fx-color-magenta, #eb2f96)",
    ];

    view! {
        <div class=combined_class>
            // Title with period
            <div class=title_class.clone()>
                {period.clone().unwrap_or_else(|| "Total Earnings".to_string())}
            </div>

            // Total amount
            <div class=total_class.clone()>
                <span class=amount_class.clone()>{formatted_total}</span>
                <span class=symbol_class.clone()>{token_symbol.clone()}</span>
            </div>

            // Fiat equivalent
            {move || {
                fiat_value.map(|fv| {
                    let fiat_sym = fiat_symbol.clone();
                    view! {
                        <div class=fiat_class.clone()>
                            {move || format!("≈ {}{:.2}", fiat_sym, fv.get())}
                        </div>
                    }
                })
            }}

            // Trend indicator
            {trend.map(|t| {
                let trend_val = trend_value.clone().unwrap_or_default();
                let trend_color = match t {
                    TrendFlag::Up => "var(--fx-color-success, #52c41a)",
                    TrendFlag::Down => "var(--fx-color-error, #ff4d4f)",
                    TrendFlag::Flat => "var(--fx-color-text-secondary, #a6a6a6)",
                };
                let trend_icon = match t {
                    TrendFlag::Up => "↑",
                    TrendFlag::Down => "↓",
                    TrendFlag::Flat => "→",
                };
                view! {
                    <div
                        class=trend_class.clone()
                        style=format!("color: {};", trend_color)
                    >
                        <span>{trend_icon}</span>
                        <span>{trend_val}</span>
                    </div>
                }
            })}

            // Breakdown
            {move || {
                if show_breakdown {
                    breakdown.clone().map(|cats| {
                        let category_icon_class = category_icon_class.clone();
                        let category_dot_class = category_dot_class.clone();
                        view! {
                            <div class=breakdown_class.clone()>
                                {cats.iter().enumerate().map(|(i, cat)| {
                                    let color = cat.color.clone()
                                        .unwrap_or_else(|| category_colors[i % category_colors.len()].to_string());
                                    let has_icon = use_icons && cat.icon.is_some();
                                    let icon = cat.icon.clone();
                                    let category_icon_class = category_icon_class.clone();
                                    let category_dot_class = category_dot_class.clone();
                                    view! {
                                        <div class=category_class.clone()>
                                            <span class=category_name_class.clone()>
                                                {if has_icon {
                                                    view! {
                                                        <span
                                                            class=category_icon_class
                                                            style=format!("color: {};", color)
                                                        >
                                                            {icon.unwrap_or_default()}
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <span
                                                            class=category_dot_class
                                                            style=format!("background: {};", color)
                                                        />
                                                    }.into_any()
                                                }}
                                                {cat.name.clone()}
                                            </span>
                                            <span class=category_value_class.clone()>
                                                {format!("{:.2}", cat.amount)}
                                            </span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
