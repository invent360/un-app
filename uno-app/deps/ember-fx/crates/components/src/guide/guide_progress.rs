//! Guide progress indicator component.

use leptos::prelude::*;
use super::types::{ProgressStyle, GuideStep};
use crate::try_use_theme;

/// Progress indicator for guide components.
///
/// Shows progress through guide steps using various styles.
#[component]
pub fn GuideProgress(
    /// Current step index (0-based).
    #[prop(into)]
    current: Signal<usize>,
    /// Total number of steps.
    #[prop(into)]
    total: usize,
    /// Progress indicator style.
    #[prop(optional, into)]
    style: Option<ProgressStyle>,
    /// Optional step data for step list style.
    #[prop(optional, into)]
    steps: Option<Vec<GuideStep>>,
    /// Callback when a step is clicked (for clickable progress).
    #[prop(optional, into)]
    on_step_click: Option<Callback<usize>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let style = style.unwrap_or_default();
    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));
    let style_class = style.class(&prefix.get_value());

    let combined_class = move || {
        let mut parts = vec![
            format!("{}-progress", prefix.get_value()),
            style_class.clone(),
        ];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let is_clickable = on_step_click.is_some();

    view! {
        <div class=combined_class>
            {match style {
                ProgressStyle::Dots => {
                    let p = prefix.get_value();
                    view! {
                        <div class=format!("{}-progress-dots", p)>
                            {(0..total).map(|i| {
                                let p2 = prefix.get_value();
                                let is_active = move || current.get() == i;
                                let is_completed = move || i < current.get();
                                let on_click = on_step_click.clone();

                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            let mut c = format!("{}-progress-dot", p2);
                                            if is_active() { c.push_str(" active"); }
                                            if is_completed() { c.push_str(" completed"); }
                                            c
                                        }
                                        disabled=!is_clickable
                                        on:click=move |_| {
                                            if let Some(ref cb) = on_click {
                                                cb.run(i);
                                            }
                                        }
                                        aria-label=format!("Go to step {}", i + 1)
                                        aria-current=move || if is_active() { "step" } else { "" }
                                    />
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                },

                ProgressStyle::Bar => {
                    let p = prefix.get_value();
                    view! {
                        <div class=format!("{}-progress-bar-container", p)>
                            <div
                                class=format!("{}-progress-bar", p)
                                style=move || format!(
                                    "width: {}%",
                                    if total > 0 { ((current.get() + 1) * 100) / total } else { 0 }
                                )
                                role="progressbar"
                                aria-valuenow=move || current.get() + 1
                                aria-valuemin="1"
                                aria-valuemax=total
                            />
                        </div>
                    }.into_any()
                },

                ProgressStyle::Numbers => {
                    let p = prefix.get_value();
                    view! {
                        <div class=format!("{}-progress-numbers", p)>
                            <span class=format!("{}-progress-current", p)>
                                {move || current.get() + 1}
                            </span>
                            <span class=format!("{}-progress-separator", p)>" / "</span>
                            <span class=format!("{}-progress-total", p)>
                                {total}
                            </span>
                        </div>
                    }.into_any()
                },

                ProgressStyle::Steps => {
                    let steps = steps.clone().unwrap_or_default();
                    let p = prefix.get_value();
                    view! {
                        <div class=format!("{}-progress-steps", p)>
                            {steps.into_iter().enumerate().map(|(i, step)| {
                                let p2 = prefix.get_value();
                                let is_active = move || current.get() == i;
                                let is_completed = move || i < current.get() || step.completed;
                                let on_click = on_step_click.clone();
                                let title = step.title.clone();

                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            let mut c = format!("{}-progress-step", p2);
                                            if is_active() { c.push_str(" active"); }
                                            if is_completed() { c.push_str(" completed"); }
                                            c
                                        }
                                        disabled=!is_clickable
                                        on:click=move |_| {
                                            if let Some(ref cb) = on_click {
                                                cb.run(i);
                                            }
                                        }
                                    >
                                        <span class=format!("{}-progress-step-indicator", p2)>
                                            {move || if is_completed() {
                                                view! { <span class="checkmark">"✓"</span> }.into_any()
                                            } else {
                                                view! { <span>{i + 1}</span> }.into_any()
                                            }}
                                        </span>
                                        <span class=format!("{}-progress-step-title", p2)>
                                            {title.clone()}
                                        </span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                },

                ProgressStyle::None => view! { <div /> }.into_any(),
            }}
        </div>
    }
}
