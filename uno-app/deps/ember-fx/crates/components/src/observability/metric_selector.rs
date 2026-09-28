//! MetricSelector component for PromQL-aware metric picking.
//!
//! Provides a searchable dropdown for selecting metrics with:
//! - Autocomplete filtering by name and description
//! - Metric type indicators (counter, gauge, histogram, summary)
//! - PromQL suggestions based on metric type
//! - Keyboard navigation support

use leptos::prelude::*;
use crate::try_use_theme;

/// Metric metadata for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricInfo {
    /// Metric name (e.g., "http_requests_total").
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Type of metric.
    pub metric_type: MetricType,
    /// Available labels.
    pub labels: Vec<String>,
}

impl MetricInfo {
    /// Create a new MetricInfo.
    pub fn new(name: impl Into<String>, description: impl Into<String>, metric_type: MetricType) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            metric_type,
            labels: Vec::new(),
        }
    }

    /// Add a label to this metric.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    /// Add multiple labels.
    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    /// Get PromQL suggestions for this metric.
    pub fn promql_suggestions(&self) -> Vec<String> {
        match self.metric_type {
            MetricType::Counter => vec![
                format!("rate({}[5m])", self.name),
                format!("increase({}[1h])", self.name),
                format!("sum(rate({}[5m]))", self.name),
                format!("sum by (instance) (rate({}[5m]))", self.name),
            ],
            MetricType::Gauge => vec![
                self.name.clone(),
                format!("avg({})", self.name),
                format!("max({})", self.name),
                format!("min({})", self.name),
                format!("avg by (instance) ({})", self.name),
            ],
            MetricType::Histogram => vec![
                format!("histogram_quantile(0.50, rate({}_bucket[5m]))", self.name),
                format!("histogram_quantile(0.95, rate({}_bucket[5m]))", self.name),
                format!("histogram_quantile(0.99, rate({}_bucket[5m]))", self.name),
                format!("rate({}_sum[5m]) / rate({}_count[5m])", self.name, self.name),
            ],
            MetricType::Summary => vec![
                format!("{}{{quantile=\"0.5\"}}", self.name),
                format!("{}{{quantile=\"0.95\"}}", self.name),
                format!("{}{{quantile=\"0.99\"}}", self.name),
            ],
        }
    }
}

/// Metric type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    /// Monotonically increasing counter.
    Counter,
    /// Value that can go up or down.
    Gauge,
    /// Bucketed distribution.
    Histogram,
    /// Pre-calculated quantiles.
    Summary,
}

impl MetricType {
    /// Get display label for the metric type.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge",
            Self::Histogram => "histogram",
            Self::Summary => "summary",
        }
    }

    /// Get a short abbreviation.
    pub fn as_abbrev(&self) -> &'static str {
        match self {
            Self::Counter => "CNT",
            Self::Gauge => "GAU",
            Self::Histogram => "HIS",
            Self::Summary => "SUM",
        }
    }

    /// Get CSS color class suffix.
    pub fn as_color_class(&self) -> &'static str {
        match self {
            Self::Counter => "primary",
            Self::Gauge => "success",
            Self::Histogram => "warning",
            Self::Summary => "info",
        }
    }
}

/// MetricSelector component.
///
/// A searchable dropdown for selecting Prometheus/OTEL metrics with
/// PromQL-aware autocomplete and suggestions.
///
/// # Example
///
/// ```ignore
/// let metrics = vec![
///     MetricInfo::new("http_requests_total", "Total HTTP requests", MetricType::Counter)
///         .labels(vec!["method".into(), "status".into()]),
///     MetricInfo::new("process_cpu_seconds", "CPU time in seconds", MetricType::Counter),
/// ];
///
/// let selected = RwSignal::new(None);
///
/// view! {
///     <MetricSelector
///         metrics=Signal::derive(move || metrics.clone())
///         selected=selected
///         on_select=|metric| log!("Selected: {}", metric.name)
///     />
/// }
/// ```
#[component]
pub fn MetricSelector(
    /// Available metrics to select from.
    metrics: Signal<Vec<MetricInfo>>,
    /// Currently selected metric name.
    selected: RwSignal<Option<String>>,
    /// Callback when a metric is selected.
    #[prop(optional, into)]
    on_select: Option<Callback<MetricInfo>>,
    /// Callback when a PromQL suggestion is selected.
    #[prop(optional, into)]
    on_promql_select: Option<Callback<String>>,
    /// Placeholder text for the search input.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Whether to show PromQL suggestions.
    #[prop(optional)]
    show_suggestions: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-metric-selector-{}", design_system);
    let search_input = RwSignal::new(String::new());
    let show_dropdown = RwSignal::new(false);
    let focused_index = RwSignal::new(0usize);
    let show_promql = RwSignal::new(false);
    let selected_metric = RwSignal::new(None::<MetricInfo>);

    let placeholder = placeholder.unwrap_or_else(|| "Search metrics...".to_string());
    let show_suggestions = show_suggestions.unwrap_or(true);

    // Filter metrics based on search
    let filtered_metrics = move || {
        let query = search_input.get().to_lowercase();
        if query.is_empty() {
            metrics.get()
        } else {
            metrics.get()
                .into_iter()
                .filter(|m| {
                    m.name.to_lowercase().contains(&query)
                    || m.description.to_lowercase().contains(&query)
                })
                .collect()
        }
    };

    // Handle metric selection
    let select_metric = move |metric: MetricInfo| {
        selected.set(Some(metric.name.clone()));
        search_input.set(metric.name.clone());
        selected_metric.set(Some(metric.clone()));
        show_dropdown.set(false);

        if show_suggestions {
            show_promql.set(true);
        }

        if let Some(ref callback) = on_select {
            callback.run(metric);
        }
    };

    // Handle PromQL selection
    let select_promql = move |query: String| {
        show_promql.set(false);

        if let Some(ref callback) = on_promql_select {
            callback.run(query);
        }
    };

    let prefix_outer = prefix.clone();
    let prefix_input = prefix.clone();
    let prefix_badge = prefix.clone();
    let prefix_dropdown = prefix.clone();
    let prefix_promql = prefix.clone();

    view! {
        <div
            class=format!("{} {}", prefix_outer, class.clone().unwrap_or_default())
            style="position: relative;"
        >
            // Search input
            <div
                class=format!("{}-input-wrapper", prefix_input)
                style="display: flex; align-items: center; gap: 8px;"
            >
                <input
                    type="text"
                    class=format!("{}-input", prefix_input)
                    placeholder=placeholder.clone()
                    prop:value=search_input
                    style="flex: 1; padding: 8px 12px; border: 1px solid #434343; border-radius: 6px; \
                           background: #1f1f1f; color: #ffffff; font-size: 14px; outline: none;"
                    on:input=move |ev| {
                        search_input.set(event_target_value(&ev));
                        show_dropdown.set(true);
                        show_promql.set(false);
                        focused_index.set(0);
                    }
                    on:focus=move |_| {
                        show_dropdown.set(true);
                        show_promql.set(false);
                    }
                    on:blur=move |_| {
                        // Delay to allow click on dropdown items
                        set_timeout(
                            move || {
                                show_dropdown.set(false);
                            },
                            std::time::Duration::from_millis(200)
                        );
                    }
                    on:keydown=move |ev| {
                        let key = ev.key();
                        let filtered = filtered_metrics();
                        match key.as_str() {
                            "ArrowDown" => {
                                ev.prevent_default();
                                focused_index.update(|i| *i = (*i + 1).min(filtered.len().saturating_sub(1)));
                            }
                            "ArrowUp" => {
                                ev.prevent_default();
                                focused_index.update(|i| *i = i.saturating_sub(1));
                            }
                            "Enter" => {
                                ev.prevent_default();
                                if let Some(metric) = filtered.get(focused_index.get()) {
                                    select_metric(metric.clone());
                                }
                            }
                            "Escape" => {
                                show_dropdown.set(false);
                                show_promql.set(false);
                            }
                            _ => {}
                        }
                    }
                />
                // Type indicator badge
                {move || selected_metric.get().map(|m| {
                    let badge_color = match m.metric_type {
                        MetricType::Counter => "#1890ff",
                        MetricType::Gauge => "#52c41a",
                        MetricType::Histogram => "#faad14",
                        MetricType::Summary => "#13c2c2",
                    };
                    view! {
                        <span style=format!(
                            "padding: 2px 8px; border-radius: 4px; font-size: 12px; font-weight: 600; \
                             background: {}; color: white;",
                            badge_color
                        )>
                            {m.metric_type.as_abbrev()}
                        </span>
                    }
                })}
            </div>

            // Metrics dropdown
            {move || show_dropdown.get().then(|| {
                let filtered = filtered_metrics();
                let prefix = prefix_dropdown.clone();

                if filtered.is_empty() {
                    view! {
                        <div
                            class=format!("{}-dropdown", prefix)
                            style="position: absolute; top: 100%; left: 0; right: 0; margin-top: 4px; \
                                   background: #262626; border: 1px solid #434343; border-radius: 6px; \
                                   max-height: 300px; overflow-y: auto; z-index: 1000;"
                        >
                            <div style="padding: 16px; text-align: center; color: #8c8c8c;">
                                "No metrics found"
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div
                            class=format!("{}-dropdown", prefix)
                            style="position: absolute; top: 100%; left: 0; right: 0; margin-top: 4px; \
                                   background: #262626; border: 1px solid #434343; border-radius: 6px; \
                                   max-height: 300px; overflow-y: auto; z-index: 1000;"
                        >
                            {filtered.into_iter().enumerate().map(|(i, metric)| {
                                let is_focused = move || focused_index.get() == i;
                                let metric_clone = metric.clone();
                                let name = metric.name.clone();
                                let desc = metric.description.clone();
                                let mtype = metric.metric_type;
                                let label_count = metric.labels.len();
                                let _prefix_item = prefix.clone();

                                let type_color = match mtype {
                                    MetricType::Counter => "#1890ff",
                                    MetricType::Gauge => "#52c41a",
                                    MetricType::Histogram => "#faad14",
                                    MetricType::Summary => "#13c2c2",
                                };

                                view! {
                                    <div
                                        style=move || format!(
                                            "padding: 10px 12px; cursor: pointer; border-bottom: 1px solid #333; {}",
                                            if is_focused() { "background: #363636;" } else { "" }
                                        )
                                        on:mouseenter=move |_| focused_index.set(i)
                                        on:click={
                                            let metric = metric_clone.clone();
                                            move |_| select_metric(metric.clone())
                                        }
                                    >
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
                                            <span style="font-weight: 500; color: #ffffff; font-family: monospace;">{name}</span>
                                            <span style=format!(
                                                "padding: 2px 6px; border-radius: 3px; font-size: 10px; \
                                                 background: {}22; color: {}; border: 1px solid {}44;",
                                                type_color, type_color, type_color
                                            )>
                                                {mtype.as_label()}
                                            </span>
                                        </div>
                                        <div style="font-size: 12px; color: #8c8c8c;">{desc}</div>
                                        {(label_count > 0).then(|| {
                                            view! {
                                                <div style="font-size: 11px; color: #595959; margin-top: 4px;">
                                                    {label_count} " labels"
                                                </div>
                                            }
                                        })}
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            })}

            // PromQL suggestions
            {move || {
                let should_show = show_suggestions && show_promql.get();
                should_show.then(|| {
                    selected_metric.get().map(|metric| {
                        let suggestions = metric.promql_suggestions();

                        view! {
                            <div style="margin-top: 12px; background: #1f1f1f; border: 1px solid #434343; \
                                        border-radius: 6px; overflow: hidden;">
                                <div style="padding: 8px 12px; background: #262626; font-weight: 600; \
                                            font-size: 12px; color: #bfbfbf; border-bottom: 1px solid #434343;">
                                    "PromQL Suggestions"
                                </div>
                                {suggestions.into_iter().enumerate().map(|(idx, query)| {
                                    let query_clone = query.clone();
                                    let hovered = RwSignal::new(false);

                                    view! {
                                        <div
                                            style=move || format!(
                                                "padding: 8px 12px; cursor: pointer; border-bottom: 1px solid #333; \
                                                 transition: background 0.15s; {}",
                                                if hovered.get() { "background: #363636;" } else { "" }
                                            )
                                            on:mouseenter=move |_| hovered.set(true)
                                            on:mouseleave=move |_| hovered.set(false)
                                            on:click={
                                                let q = query_clone.clone();
                                                move |_| select_promql(q.clone())
                                            }
                                        >
                                            <code style="font-size: 13px; color: #52c41a;">{query}</code>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    })
                }).flatten()
            }}
        </div>
    }
}
