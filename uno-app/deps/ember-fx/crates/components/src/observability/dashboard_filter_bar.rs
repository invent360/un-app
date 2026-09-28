//! DashboardFilterBar Leptos component.
//!
//! A filter controls toolbar for dashboards with dropdowns and time picker.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::try_use_theme;

/// A filter option (value + display label).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterOption {
    /// The actual value.
    pub value: String,
    /// The display label.
    pub label: String,
}

impl FilterOption {
    /// Create a new filter option.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }

    /// Create a filter option where value equals label.
    pub fn simple(value: impl Into<String>) -> Self {
        let v: String = value.into();
        Self {
            value: v.clone(),
            label: v,
        }
    }
}

/// Configuration for available filter options.
#[derive(Debug, Clone, Default)]
pub struct FilterConfig {
    /// Available datasources.
    pub datasources: Vec<FilterOption>,
    /// Available regions.
    pub regions: Vec<FilterOption>,
    /// Available queues/resources.
    pub queues: Vec<FilterOption>,
    /// Available time ranges.
    pub time_ranges: Vec<FilterOption>,
}

impl FilterConfig {
    /// Create a new empty config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add datasource options.
    pub fn with_datasources(mut self, datasources: Vec<FilterOption>) -> Self {
        self.datasources = datasources;
        self
    }

    /// Add region options.
    pub fn with_regions(mut self, regions: Vec<FilterOption>) -> Self {
        self.regions = regions;
        self
    }

    /// Add queue options.
    pub fn with_queues(mut self, queues: Vec<FilterOption>) -> Self {
        self.queues = queues;
        self
    }

    /// Add time range options.
    pub fn with_time_ranges(mut self, time_ranges: Vec<FilterOption>) -> Self {
        self.time_ranges = time_ranges;
        self
    }

    /// Create a default time ranges config.
    pub fn default_time_ranges() -> Vec<FilterOption> {
        vec![
            FilterOption::new("15m", "Last 15 minutes"),
            FilterOption::new("1h", "Last 1 hour"),
            FilterOption::new("6h", "Last 6 hours"),
            FilterOption::new("24h", "Last 24 hours"),
            FilterOption::new("7d", "Last 7 days"),
            FilterOption::new("30d", "Last 30 days"),
        ]
    }
}

/// Current filter state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FilterState {
    /// Selected datasource value.
    pub datasource: String,
    /// Selected region value.
    pub region: String,
    /// Selected queue value.
    pub queue: String,
    /// Selected time range value.
    pub time_range: String,
}

impl FilterState {
    /// Create a new filter state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the datasource.
    pub fn with_datasource(mut self, datasource: impl Into<String>) -> Self {
        self.datasource = datasource.into();
        self
    }

    /// Set the region.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = region.into();
        self
    }

    /// Set the queue.
    pub fn with_queue(mut self, queue: impl Into<String>) -> Self {
        self.queue = queue.into();
        self
    }

    /// Set the time range.
    pub fn with_time_range(mut self, time_range: impl Into<String>) -> Self {
        self.time_range = time_range.into();
        self
    }
}

/// DashboardFilterBar component.
///
/// A toolbar with filter dropdowns for datasource, region, queue, and time range.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     DashboardFilterBar, FilterConfig, FilterOption, FilterState,
/// };
///
/// let config = FilterConfig::new()
///     .with_datasources(vec![
///         FilterOption::new("cloudwatch", "my-cloudwatch-datasource"),
///     ])
///     .with_regions(vec![
///         FilterOption::simple("default"),
///         FilterOption::simple("us-east-1"),
///     ])
///     .with_queues(vec![
///         FilterOption::simple("my-queue"),
///         FilterOption::simple("other-queue"),
///     ])
///     .with_time_ranges(FilterConfig::default_time_ranges());
///
/// let state = RwSignal::new(FilterState::new()
///     .with_datasource("cloudwatch")
///     .with_region("default")
///     .with_queue("my-queue")
///     .with_time_range("24h"));
///
/// view! {
///     <DashboardFilterBar
///         config=config
///         state=state
///     />
/// }
/// ```
#[component]
pub fn DashboardFilterBar(
    /// Filter configuration.
    #[prop(into)]
    config: FilterConfig,
    /// Current filter state.
    #[prop(into)]
    state: RwSignal<FilterState>,
    /// Callback when refresh is clicked.
    #[prop(optional, into)]
    on_refresh: Option<Callback<()>>,
    /// Callback when any filter changes.
    #[prop(optional, into)]
    on_change: Option<Callback<FilterState>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-filter-bar-{}", design_system);

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

    // Clone config for use in closures
    let datasources = config.datasources.clone();
    let regions = config.regions.clone();
    let queues = config.queues.clone();
    let time_ranges = config.time_ranges.clone();

    // Handle datasource change
    let handle_datasource = {
        let on_change = on_change.clone();
        move |ev: web_sys::Event| {
            let target = ev.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.unchecked_into();
            let value = select.value();
            state.update(|s| s.datasource = value);
            if let Some(ref cb) = on_change {
                cb.run(state.get());
            }
        }
    };

    // Handle region change
    let handle_region = {
        let on_change = on_change.clone();
        move |ev: web_sys::Event| {
            let target = ev.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.unchecked_into();
            let value = select.value();
            state.update(|s| s.region = value);
            if let Some(ref cb) = on_change {
                cb.run(state.get());
            }
        }
    };

    // Handle queue change
    let handle_queue = {
        let on_change = on_change.clone();
        move |ev: web_sys::Event| {
            let target = ev.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.unchecked_into();
            let value = select.value();
            state.update(|s| s.queue = value);
            if let Some(ref cb) = on_change {
                cb.run(state.get());
            }
        }
    };

    // Handle time range change
    let handle_time_range = {
        let on_change = on_change.clone();
        move |ev: web_sys::Event| {
            let target = ev.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.unchecked_into();
            let value = select.value();
            state.update(|s| s.time_range = value);
            if let Some(ref cb) = on_change {
                cb.run(state.get());
            }
        }
    };

    // Handle refresh click
    let handle_refresh = move |_| {
        if let Some(ref cb) = on_refresh {
            cb.run(());
        }
    };

    // Shared select styles
    let select_style = "
        background: var(--fx-color-bg-elevated, #1f1f1f);
        border: 1px solid var(--fx-color-border, #303030);
        border-radius: 4px;
        color: var(--fx-color-text, #fff);
        padding: 6px 12px;
        font-size: 13px;
        cursor: pointer;
        outline: none;
    ";

    let button_style = "
        background: var(--fx-color-bg-elevated, #1f1f1f);
        border: 1px solid var(--fx-color-border, #303030);
        border-radius: 4px;
        color: var(--fx-color-text, #fff);
        padding: 6px 12px;
        font-size: 13px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 4px;
    ";

    view! {
        <div
            class=combined_class
            style="display: flex; align-items: center; gap: 12px; padding: 8px 12px; background: var(--fx-color-bg-container, #141414); border-bottom: 1px solid var(--fx-color-border, #303030);"
        >
            // Datasource filter
            {(!datasources.is_empty()).then(|| {
                let datasources = datasources.clone();
                view! {
                    <div class=format!("{}-filter", prefix) style="display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 12px; color: var(--fx-color-text-secondary, #a0a0a0);">"Datasource"</span>
                        <select
                            style=select_style
                            on:change=handle_datasource.clone()
                            prop:value=move || state.get().datasource
                        >
                            {datasources.iter().map(|opt| {
                                let value = opt.value.clone();
                                let label = opt.label.clone();
                                view! {
                                    <option value=value>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </div>
                }
            })}

            // Region filter
            {(!regions.is_empty()).then(|| {
                let regions = regions.clone();
                view! {
                    <div class=format!("{}-filter", prefix) style="display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 12px; color: var(--fx-color-text-secondary, #a0a0a0);">"Region"</span>
                        <select
                            style=select_style
                            on:change=handle_region.clone()
                            prop:value=move || state.get().region
                        >
                            {regions.iter().map(|opt| {
                                let value = opt.value.clone();
                                let label = opt.label.clone();
                                view! {
                                    <option value=value>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </div>
                }
            })}

            // Queue filter
            {(!queues.is_empty()).then(|| {
                let queues = queues.clone();
                view! {
                    <div class=format!("{}-filter", prefix) style="display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 12px; color: var(--fx-color-text-secondary, #a0a0a0);">"Queue name"</span>
                        <select
                            style=select_style
                            on:change=handle_queue.clone()
                            prop:value=move || state.get().queue
                        >
                            {queues.iter().map(|opt| {
                                let value = opt.value.clone();
                                let label = opt.label.clone();
                                view! {
                                    <option value=value>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </div>
                }
            })}

            // Spacer
            <div style="flex: 1;"></div>

            // Time range picker
            {(!time_ranges.is_empty()).then(|| {
                let time_ranges = time_ranges.clone();
                view! {
                    <div class=format!("{}-time-picker", prefix) style="display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 14px;">"\u{1F552}"</span>
                        <select
                            style=select_style
                            on:change=handle_time_range.clone()
                            prop:value=move || state.get().time_range
                        >
                            {time_ranges.iter().map(|opt| {
                                let value = opt.value.clone();
                                let label = opt.label.clone();
                                view! {
                                    <option value=value>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </div>
                }
            })}

            // Zoom out button
            <button
                style=button_style
                title="Zoom out"
            >
                "\u{1F50D}"
            </button>

            // Refresh button
            <button
                style=button_style
                on:click=handle_refresh
                title="Refresh"
            >
                "\u{21BB} Refresh"
            </button>
        </div>
    }
}
