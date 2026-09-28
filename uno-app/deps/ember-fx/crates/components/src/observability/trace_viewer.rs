//! TraceViewer Leptos component.

use leptos::prelude::*;
use super::types::{Trace, Span, SpanId, SpanStatus};
use crate::try_use_theme;

/// TraceViewer component.
///
/// Waterfall visualization of distributed traces.
///
/// # Props
///
/// - `trace` - Trace data
/// - `selected_span` - Currently selected span
/// - `on_span_click` - Span click handler
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{TraceViewer, Trace, Span};
///
/// let trace = Trace::new("abc123", root_span);
///
/// view! {
///     <TraceViewer trace=Signal::derive(move || Some(trace.clone())) />
/// }
/// ```
#[component]
pub fn TraceViewer(
    /// Trace data.
    trace: Signal<Option<Trace>>,
    /// Currently selected span.
    #[prop(optional)]
    selected_span: Option<RwSignal<Option<SpanId>>>,
    /// Span click handler.
    #[prop(optional, into)]
    on_span_click: Option<Callback<Span>>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Internal selected span state if not provided
    let internal_selected = RwSignal::new(None::<SpanId>);
    let selected_span = selected_span.unwrap_or(internal_selected);
    let _on_span_click = on_span_click;

    // Build CSS classes
    let trace_prefix = format!("fx-trace-viewer-{}", design_system);

    let combined_class = {
        let mut parts = vec![trace_prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", trace_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Pre-compute class names
    let content_class = format!("{}-content", trace_prefix);
    let header_class = format!("{}-header", trace_prefix);
    let trace_id_class = format!("{}-trace-id", trace_prefix);
    let label_class = format!("{}-label", trace_prefix);
    let value_class = format!("{}-value", trace_prefix);
    let trace_info_class = format!("{}-trace-info", trace_prefix);
    let waterfall_class = format!("{}-waterfall", trace_prefix);
    let span_class = format!("{}-span", trace_prefix);
    let span_row_class = format!("{}-span-row", trace_prefix);
    let selected_class = format!("{}-selected", trace_prefix);
    let span_info_class = format!("{}-span-info", trace_prefix);
    let expand_icon_class = format!("{}-expand-icon", trace_prefix);
    let service_class = format!("{}-service", trace_prefix);
    let operation_class = format!("{}-operation", trace_prefix);
    let span_bar_container_class = format!("{}-span-bar-container", trace_prefix);
    let span_bar_class = format!("{}-span-bar", trace_prefix);
    let span_duration_class = format!("{}-span-duration", trace_prefix);
    let span_details_class = format!("{}-span-details", trace_prefix);
    let detail_header_class = format!("{}-detail-header", trace_prefix);
    let status_class = format!("{}-status", trace_prefix);
    let detail_body_class = format!("{}-detail-body", trace_prefix);
    let detail_row_class = format!("{}-detail-row", trace_prefix);
    let detail_label_class = format!("{}-detail-label", trace_prefix);
    let detail_value_class = format!("{}-detail-value", trace_prefix);
    let detail_section_class = format!("{}-detail-section", trace_prefix);
    let detail_section_title_class = format!("{}-detail-section-title", trace_prefix);
    let empty_class = format!("{}-empty", trace_prefix);
    let loading_overlay_class = format!("{}-loading-overlay", trace_prefix);
    let loading_spinner_class = format!("{}-loading-spinner", trace_prefix);

    // Format duration
    let format_duration = |ms: u64| {
        if ms < 1000 {
            format!("{}ms", ms)
        } else if ms < 60000 {
            format!("{:.1}s", ms as f64 / 1000.0)
        } else {
            format!("{:.1}m", ms as f64 / 60000.0)
        }
    };

    view! {
        <div class=combined_class>
            {move || {
                let content_class = content_class.clone();
                let header_class = header_class.clone();
                let trace_id_class = trace_id_class.clone();
                let label_class = label_class.clone();
                let value_class = value_class.clone();
                let trace_info_class = trace_info_class.clone();
                let waterfall_class = waterfall_class.clone();
                let span_class = span_class.clone();
                let span_row_class = span_row_class.clone();
                let selected_class = selected_class.clone();
                let span_info_class = span_info_class.clone();
                let expand_icon_class = expand_icon_class.clone();
                let service_class = service_class.clone();
                let operation_class = operation_class.clone();
                let span_bar_container_class = span_bar_container_class.clone();
                let span_bar_class = span_bar_class.clone();
                let span_duration_class = span_duration_class.clone();
                let span_details_class = span_details_class.clone();
                let detail_header_class = detail_header_class.clone();
                let status_class = status_class.clone();
                let detail_body_class = detail_body_class.clone();
                let detail_row_class = detail_row_class.clone();
                let detail_label_class = detail_label_class.clone();
                let detail_value_class = detail_value_class.clone();
                let detail_section_class = detail_section_class.clone();
                let detail_section_title_class = detail_section_title_class.clone();
                let empty_class = empty_class.clone();

                match trace.get() {
                    Some(t) => {
                        let trace_duration = t.duration_ms;
                        let trace_start = t.root_span.start_time;

                        // Flatten span tree to a list with depth
                        let flat_spans = flatten_spans(&t.root_span, 0, trace_start, trace_duration);
                        let trace_clone = t.clone();

                        view! {
                            <div class=content_class>
                                // Header
                                <div class=header_class>
                                    <div class=trace_id_class>
                                        <span class=label_class>"Trace: "</span>
                                        <span class=value_class>{t.trace_id.clone()}</span>
                                    </div>
                                    <div class=trace_info_class>
                                        <span>"Duration: "{format_duration(t.duration_ms)}</span>
                                        <span>" · "</span>
                                        <span>{t.service_count}" services"</span>
                                    </div>
                                </div>

                                // Waterfall
                                <div class=waterfall_class>
                                    {flat_spans.into_iter().map(|fs| {
                                        let span_id = fs.span_id.clone();
                                        let span_id_clone = span_id.clone();
                                        let span_class = span_class.clone();
                                        let span_row_class = span_row_class.clone();
                                        let selected_class = selected_class.clone();
                                        let span_info_class = span_info_class.clone();
                                        let expand_icon_class = expand_icon_class.clone();
                                        let service_class = service_class.clone();
                                        let operation_class = operation_class.clone();
                                        let span_bar_container_class = span_bar_container_class.clone();
                                        let span_bar_class = span_bar_class.clone();
                                        let span_duration_class = span_duration_class.clone();

                                        view! {
                                            <div
                                                class=span_class
                                                style=format!("--depth: {}", fs.depth)
                                            >
                                                <div
                                                    class=move || {
                                                        let is_selected = selected_span.get()
                                                            .map(|id| id == span_id)
                                                            .unwrap_or(false);
                                                        if is_selected {
                                                            format!("{} {}", span_row_class, selected_class)
                                                        } else {
                                                            span_row_class.clone()
                                                        }
                                                    }
                                                    on:click=move |_| {
                                                        selected_span.set(Some(span_id_clone.clone()));
                                                    }
                                                >
                                                    <div
                                                        class=span_info_class.clone()
                                                        style=format!("padding-left: {}px", fs.depth * 20)
                                                    >
                                                        {fs.has_children.then(|| view! {
                                                            <span class=expand_icon_class.clone()>"▼"</span>
                                                        })}
                                                        <span class=service_class.clone()>{fs.service.clone()}</span>
                                                        <span class=operation_class.clone()>{fs.operation.clone()}</span>
                                                    </div>

                                                    <div class=span_bar_container_class.clone()>
                                                        <div
                                                            class=format!("{} {}-status-{}", span_bar_class, span_bar_class, fs.status.as_suffix())
                                                            style=format!(
                                                                "left: {}%; width: {}%; background-color: {}",
                                                                fs.offset_pct,
                                                                fs.width_pct,
                                                                fs.status.as_color()
                                                            )
                                                        >
                                                            <span class=span_duration_class.clone()>
                                                                {format_duration(fs.duration_ms)}
                                                            </span>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>

                                // Selected span details
                                {move || {
                                    let span_details_class = span_details_class.clone();
                                    let detail_header_class = detail_header_class.clone();
                                    let operation_class = operation_class.clone();
                                    let status_class = status_class.clone();
                                    let detail_body_class = detail_body_class.clone();
                                    let detail_row_class = detail_row_class.clone();
                                    let detail_label_class = detail_label_class.clone();
                                    let detail_value_class = detail_value_class.clone();
                                    let detail_section_class = detail_section_class.clone();
                                    let detail_section_title_class = detail_section_title_class.clone();

                                    selected_span.get()
                                        .and_then(|id| find_span(&trace_clone.root_span, &id))
                                        .map(|span| {
                                            let attrs: Vec<_> = span.attributes.iter()
                                                .map(|(k, v)| (k.clone(), v.clone()))
                                                .collect();

                                            view! {
                                                <div class=span_details_class>
                                                    <div class=detail_header_class>
                                                        <span class=operation_class>
                                                            {span.operation.clone()}
                                                        </span>
                                                        <span
                                                            class=format!("{} {}-{}", status_class, status_class, span.status.as_suffix())
                                                            style=format!("color: {}", span.status.as_color())
                                                        >
                                                            {match span.status {
                                                                SpanStatus::Ok => "OK",
                                                                SpanStatus::Error => "ERROR",
                                                                SpanStatus::Timeout => "TIMEOUT",
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class=detail_body_class>
                                                        <div class=detail_row_class.clone()>
                                                            <span class=detail_label_class.clone()>"Service:"</span>
                                                            <span class=detail_value_class.clone()>{span.service.clone()}</span>
                                                        </div>
                                                        <div class=detail_row_class.clone()>
                                                            <span class=detail_label_class.clone()>"Duration:"</span>
                                                            <span class=detail_value_class.clone()>{format_duration(span.duration_ms)}</span>
                                                        </div>
                                                        <div class=detail_row_class.clone()>
                                                            <span class=detail_label_class.clone()>"Span ID:"</span>
                                                            <span class=detail_value_class.clone()>{span.span_id.clone()}</span>
                                                        </div>
                                                        {(!attrs.is_empty()).then(|| {
                                                            let detail_section_class = detail_section_class.clone();
                                                            let detail_section_title_class = detail_section_title_class.clone();
                                                            let detail_row_class = detail_row_class.clone();
                                                            let detail_label_class = detail_label_class.clone();
                                                            let detail_value_class = detail_value_class.clone();
                                                            view! {
                                                                <div class=detail_section_class>
                                                                    <span class=detail_section_title_class>"Attributes"</span>
                                                                    {attrs.into_iter().map(|(k, v)| {
                                                                        let drc = detail_row_class.clone();
                                                                        let dlc = detail_label_class.clone();
                                                                        let dvc = detail_value_class.clone();
                                                                        view! {
                                                                            <div class=drc>
                                                                                <span class=dlc>{format!("{}:", k)}</span>
                                                                                <span class=dvc>{v}</span>
                                                                            </div>
                                                                        }
                                                                    }).collect::<Vec<_>>()}
                                                                </div>
                                                            }
                                                        })}
                                                    </div>
                                                </div>
                                            }
                                        })
                                }}
                            </div>
                        }.into_any()
                    }
                    None => {
                        view! {
                            <div class=empty_class>
                                "No trace selected"
                            </div>
                        }.into_any()
                    }
                }
            }}

            // Loading overlay
            {loading.then(|| view! {
                <div class=loading_overlay_class>
                    <span class=loading_spinner_class></span>
                </div>
            })}
        </div>
    }
}

/// Flattened span for rendering.
#[derive(Clone)]
struct FlatSpan {
    span_id: String,
    operation: String,
    service: String,
    duration_ms: u64,
    status: SpanStatus,
    depth: usize,
    offset_pct: f64,
    width_pct: f64,
    has_children: bool,
}

/// Flatten span tree into a list with depth information.
fn flatten_spans(
    span: &Span,
    depth: usize,
    trace_start: i64,
    trace_duration: u64,
) -> Vec<FlatSpan> {
    let mut result = Vec::new();

    let offset_pct = if trace_duration > 0 {
        ((span.start_time - trace_start) as f64 / trace_duration as f64 * 100.0).max(0.0)
    } else {
        0.0
    };
    let width_pct = if trace_duration > 0 {
        (span.duration_ms as f64 / trace_duration as f64 * 100.0).max(0.5)
    } else {
        100.0
    };

    result.push(FlatSpan {
        span_id: span.span_id.clone(),
        operation: span.operation.clone(),
        service: span.service.clone(),
        duration_ms: span.duration_ms,
        status: span.status,
        depth,
        offset_pct,
        width_pct,
        has_children: !span.children.is_empty(),
    });

    for child in &span.children {
        result.extend(flatten_spans(child, depth + 1, trace_start, trace_duration));
    }

    result
}

/// Find a span by ID in the span tree.
fn find_span(span: &Span, id: &str) -> Option<Span> {
    if span.span_id == id {
        return Some(span.clone());
    }
    for child in &span.children {
        if let Some(found) = find_span(child, id) {
            return Some(found);
        }
    }
    None
}
