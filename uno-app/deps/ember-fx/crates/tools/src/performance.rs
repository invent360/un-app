//! Performance measurement utilities for ember-fx.
//!
//! Provides tools for measuring CSS injection timing, theme switching latency,
//! and component render performance.
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_tools::performance::{PerfTimer, measure_css_injection, PerformanceReport};
//!
//! // Measure a specific operation
//! let timer = PerfTimer::start("theme-switch");
//! // ... perform operation ...
//! let duration = timer.stop();
//!
//! // Or use the measurement helpers
//! let css_timing = measure_css_injection();
//! ```

use leptos::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static PERF_METRICS: RefCell<HashMap<String, Vec<f64>>> = RefCell::new(HashMap::new());
}

/// Performance timer for measuring operation durations.
#[derive(Debug)]
pub struct PerfTimer {
    name: String,
    start_time: f64,
}

impl PerfTimer {
    /// Start a new performance timer.
    #[must_use]
    pub fn start(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start_time: now(),
        }
    }

    /// Stop the timer and return the duration in milliseconds.
    #[must_use]
    pub fn stop(self) -> f64 {
        let duration = now() - self.start_time;
        record_metric(&self.name, duration);
        duration
    }

    /// Stop the timer and log the result.
    pub fn stop_and_log(self) {
        let name = self.name.clone();
        let duration = self.stop();
        log::info!("[Perf] {}: {:.2}ms", name, duration);
    }
}

/// Get the current high-resolution timestamp in milliseconds.
#[must_use]
pub fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }
}

/// Record a performance metric.
pub fn record_metric(name: &str, value: f64) {
    PERF_METRICS.with(|metrics| {
        metrics
            .borrow_mut()
            .entry(name.to_string())
            .or_default()
            .push(value);
    });
}

/// Get all recorded metrics.
#[must_use]
pub fn get_metrics() -> HashMap<String, MetricStats> {
    PERF_METRICS.with(|metrics| {
        metrics
            .borrow()
            .iter()
            .map(|(name, values)| {
                (name.clone(), MetricStats::from_values(values))
            })
            .collect()
    })
}

/// Clear all recorded metrics.
pub fn clear_metrics() {
    PERF_METRICS.with(|metrics| {
        metrics.borrow_mut().clear();
    });
}

/// Statistics for a metric.
#[derive(Debug, Clone, Default)]
pub struct MetricStats {
    /// Number of samples.
    pub count: usize,
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
    /// Mean (average) value.
    pub mean: f64,
    /// Median value.
    pub median: f64,
    /// 95th percentile.
    pub p95: f64,
    /// Standard deviation.
    pub std_dev: f64,
}

impl MetricStats {
    /// Calculate statistics from a slice of values.
    #[must_use]
    pub fn from_values(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self::default();
        }

        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;

        let mut sorted: Vec<f64> = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let min = sorted.first().copied().unwrap_or(0.0);
        let max = sorted.last().copied().unwrap_or(0.0);

        let median = if count % 2 == 0 {
            let mid = count / 2;
            (sorted.get(mid - 1).copied().unwrap_or(0.0)
                + sorted.get(mid).copied().unwrap_or(0.0))
                / 2.0
        } else {
            sorted.get(count / 2).copied().unwrap_or(0.0)
        };

        let p95_index = ((count as f64) * 0.95).ceil() as usize;
        let p95 = sorted
            .get(p95_index.saturating_sub(1).min(count - 1))
            .copied()
            .unwrap_or(max);

        let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();

        Self {
            count,
            min,
            max,
            mean,
            median,
            p95,
            std_dev,
        }
    }
}

/// Measure CSS injection timing.
///
/// Returns the time taken to inject CSS into the document.
#[must_use]
pub fn measure_css_injection(css: &str, style_id: &str) -> f64 {
    let timer = PerfTimer::start("css-injection");

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(style) = document.create_element("style") {
                    style.set_id(style_id);
                    style.set_text_content(Some(css));

                    if let Some(head) = document.head() {
                        let _ = head.append_child(&style);
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (css, style_id);
    }

    timer.stop()
}

/// Measure theme switching latency.
///
/// Measures the time to update the theme attribute and trigger CSS recalculation.
#[must_use]
pub fn measure_theme_switch(new_theme: &str) -> f64 {
    let timer = PerfTimer::start("theme-switch");

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("data-theme", new_theme);

                    // Force style recalculation
                    let _ = window.get_computed_style(&root);
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = new_theme;
    }

    timer.stop()
}

/// Performance report component.
///
/// Displays collected performance metrics in a debug panel.
#[component]
pub fn PerformanceReport(
    /// Whether to show the report.
    #[prop(optional)]
    visible: Option<bool>,
) -> impl IntoView {
    let visible = visible.unwrap_or(true);
    let metrics = RwSignal::new(get_metrics());

    // Refresh metrics periodically
    let refresh = move || {
        metrics.set(get_metrics());
    };

    view! {
        <Show when=move || visible fallback=|| ()>
            <div class="fx-perf-report" style="
                position: fixed;
                bottom: 10px;
                right: 10px;
                background: rgba(0, 0, 0, 0.9);
                color: #0f0;
                font-family: monospace;
                font-size: 12px;
                padding: 12px;
                border-radius: 4px;
                max-width: 400px;
                max-height: 300px;
                overflow: auto;
                z-index: 10000;
            ">
                <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                    <strong>"Performance Metrics"</strong>
                    <button
                        style="background: #333; color: #0f0; border: 1px solid #0f0; cursor: pointer; padding: 2px 8px;"
                        on:click=move |_| refresh()
                    >
                        "Refresh"
                    </button>
                </div>
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="border-bottom: 1px solid #333;">
                            <th style="text-align: left; padding: 4px;">"Metric"</th>
                            <th style="text-align: right; padding: 4px;">"Mean"</th>
                            <th style="text-align: right; padding: 4px;">"P95"</th>
                            <th style="text-align: right; padding: 4px;">"Count"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            metrics.get().iter().map(|(name, stats)| {
                                let name = name.clone();
                                view! {
                                    <tr style="border-bottom: 1px solid #222;">
                                        <td style="padding: 4px;">{name}</td>
                                        <td style="text-align: right; padding: 4px;">
                                            {format!("{:.2}ms", stats.mean)}
                                        </td>
                                        <td style="text-align: right; padding: 4px;">
                                            {format!("{:.2}ms", stats.p95)}
                                        </td>
                                        <td style="text-align: right; padding: 4px;">
                                            {stats.count}
                                        </td>
                                    </tr>
                                }
                            }).collect_view()
                        }}
                    </tbody>
                </table>
                <div style="margin-top: 8px; font-size: 10px; color: #666;">
                    "Press Refresh to update metrics"
                </div>
            </div>
        </Show>
    }
}

/// Run a benchmark and return statistics.
///
/// Executes the provided function `iterations` times and collects timing data.
pub fn benchmark<F>(name: &str, iterations: usize, mut f: F) -> MetricStats
where
    F: FnMut(),
{
    let mut timings = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = now();
        f();
        timings.push(now() - start);
    }

    let stats = MetricStats::from_values(&timings);

    log::info!(
        "[Benchmark] {}: mean={:.2}ms, p95={:.2}ms, n={}",
        name,
        stats.mean,
        stats.p95,
        iterations
    );

    // Also record for the report
    for timing in &timings {
        record_metric(name, *timing);
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_stats() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = MetricStats::from_values(&values);

        assert_eq!(stats.count, 5);
        assert!((stats.mean - 3.0).abs() < 0.001);
        assert!((stats.median - 3.0).abs() < 0.001);
        assert!((stats.min - 1.0).abs() < 0.001);
        assert!((stats.max - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_perf_timer() {
        let timer = PerfTimer::start("test");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let duration = timer.stop();
        assert!(duration >= 10.0);
    }
}
