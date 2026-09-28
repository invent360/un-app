//! SloHeatmapPanel Leptos component.
//!
//! A multi-column SLO compliance indicator with heatmap bands.

use leptos::prelude::*;
use crate::try_use_theme;

/// SLO status level based on score thresholds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SloStatus {
    /// Excellent (green) - above good threshold.
    #[default]
    Excellent,
    /// Good (light green) - above warning threshold.
    Good,
    /// Warning (yellow) - above critical threshold.
    Warning,
    /// Critical (red) - below critical threshold.
    Critical,
}

impl SloStatus {
    /// Determine status from score and thresholds.
    pub fn from_score(score: f64, good: f64, warn: f64, critical: f64) -> Self {
        if score >= good {
            Self::Excellent
        } else if score >= warn {
            Self::Good
        } else if score >= critical {
            Self::Warning
        } else {
            Self::Critical
        }
    }

    /// Returns the CSS color for this status.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Excellent => "var(--fx-color-success, #52c41a)",
            Self::Good => "var(--fx-color-success-light, #73d13d)",
            Self::Warning => "var(--fx-color-warning, #faad14)",
            Self::Critical => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Excellent => "excellent",
            Self::Good => "good",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}

/// Configuration for an SLO column.
#[derive(Debug, Clone)]
pub struct SloColumn {
    /// Column name/label.
    pub name: String,
    /// Current score (0-100).
    pub score: f64,
    /// Threshold for "good" status (default: 95).
    pub threshold_good: f64,
    /// Threshold for "warning" status (default: 90).
    pub threshold_warn: f64,
    /// Threshold for "critical" status (default: 80).
    pub threshold_critical: f64,
}

impl SloColumn {
    /// Create a new SLO column with default thresholds.
    pub fn new(name: impl Into<String>, score: f64) -> Self {
        Self {
            name: name.into(),
            score,
            threshold_good: 95.0,
            threshold_warn: 90.0,
            threshold_critical: 80.0,
        }
    }

    /// Create with custom thresholds.
    pub fn with_thresholds(
        name: impl Into<String>,
        score: f64,
        good: f64,
        warn: f64,
        critical: f64,
    ) -> Self {
        Self {
            name: name.into(),
            score,
            threshold_good: good,
            threshold_warn: warn,
            threshold_critical: critical,
        }
    }

    /// Get the status for this column.
    pub fn status(&self) -> SloStatus {
        SloStatus::from_score(
            self.score,
            self.threshold_good,
            self.threshold_warn,
            self.threshold_critical,
        )
    }
}

/// Number of heatmap rows to display.
const HEATMAP_ROWS: usize = 20;

/// SloHeatmapPanel component.
///
/// A panel showing SLO compliance across multiple services with heatmap visualization.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{SloHeatmapPanel, SloColumn};
///
/// let columns = vec![
///     SloColumn::new("Front End", 85.8),
///     SloColumn::new("Backend", 87.0),
///     SloColumn::new("Database", 92.4),
/// ];
///
/// view! {
///     <SloHeatmapPanel
///         columns=Signal::derive(move || columns.clone())
///         title="SLO Latency Indicator".to_string()
///     />
/// }
/// ```
#[component]
pub fn SloHeatmapPanel(
    /// SLO columns to display.
    #[prop(into)]
    columns: Signal<Vec<SloColumn>>,
    /// Panel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Show heatmap bands.
    #[prop(optional)]
    show_heatmap: Option<bool>,
    /// Show progress bar indicator.
    #[prop(optional)]
    show_progress: Option<bool>,
    /// Panel height.
    #[prop(optional)]
    height: Option<u32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_heatmap = show_heatmap.unwrap_or(true);
    let show_progress = show_progress.unwrap_or(true);
    let height = height.unwrap_or(300);

    let prefix = format!("fx-slo-heatmap-{}", design_system);

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

    // Generate heatmap row colors (gradient from green to red)
    let heatmap_colors: Vec<&'static str> = vec![
        "#52c41a", // Green (top - good)
        "#52c41a",
        "#52c41a",
        "#52c41a",
        "#73d13d",
        "#73d13d",
        "#95de64",
        "#95de64",
        "#b7eb8f",
        "#b7eb8f",
        "#ffe58f", // Yellow (middle)
        "#ffd666",
        "#ffc53d",
        "#faad14",
        "#fa8c16",
        "#fa541c", // Orange
        "#f5222d",
        "#ff4d4f", // Red (bottom - bad)
        "#ff4d4f",
        "#ff4d4f",
    ];

    let prefix_for_columns = prefix.clone();

    view! {
        <div
            class=combined_class
            style=format!("height: {}px;", height)
        >
            // Title
            {title.clone().map(|t| {
                let prefix = prefix.clone();
                view! {
                    <div class=format!("{}-title", prefix)>
                        {t}
                    </div>
                }
            })}

            // Columns container
            <div class=format!("{}-columns", prefix_for_columns)>
                {move || {
                    let cols = columns.get();
                    cols.iter().map(|col| {
                        let status = col.status();
                        let score = col.score;
                        let name = col.name.clone();
                        let prefix = prefix.clone();
                        let heatmap_colors = heatmap_colors.clone();

                        // Calculate which row the progress indicator should be on
                        // 0 = top (100%), HEATMAP_ROWS-1 = bottom (0%)
                        let progress_row = ((100.0 - score.clamp(0.0, 100.0)) / 100.0 * (HEATMAP_ROWS - 1) as f64) as usize;

                        view! {
                            <div class=format!("{}-column", prefix)>
                                // Score value
                                <div
                                    class=format!("{}-score", prefix)
                                    style=format!("color: {};", status.as_color())
                                >
                                    {format!("{:.1}", score)}
                                </div>

                                // Heatmap bands
                                {show_heatmap.then(|| {
                                    let prefix = prefix.clone();
                                    view! {
                                        <div class=format!("{}-heatmap", prefix)>
                                            {heatmap_colors.iter().enumerate().map(|(i, &color)| {
                                                let is_progress_row = show_progress && i == progress_row;
                                                let row_class = if is_progress_row {
                                                    format!("{}-row {}-row-active", prefix, prefix)
                                                } else {
                                                    format!("{}-row", prefix)
                                                };

                                                view! {
                                                    <div
                                                        class=row_class
                                                        style=format!("background-color: {};", color)
                                                    >
                                                        {is_progress_row.then(|| {
                                                            view! {
                                                                <div
                                                                    class=format!("{}-progress-indicator", prefix)
                                                                    style=format!(
                                                                        "background-color: {}; width: {}%;",
                                                                        status.as_color(),
                                                                        score
                                                                    )
                                                                />
                                                            }
                                                        })}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }
                                })}

                                // Column name
                                <div class=format!("{}-name", prefix)>
                                    {name}
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
