//! Inline accordion guide component.

use leptos::prelude::*;
use super::types::{GuideStep, GuideSize, ProgressStyle};
use super::guide_step::GuideStepView;
use super::guide_progress::GuideProgress;
use crate::try_use_theme;

/// Inline accordion guide for embedded step-by-step content.
#[component]
pub fn GuideAccordion(
    /// The steps to display in the accordion.
    #[prop(into)]
    steps: Vec<GuideStep>,
    /// Signal controlling the active (expanded) step index. None = all collapsed.
    #[prop(into)]
    active_step: RwSignal<Option<usize>>,
    /// Completed step IDs.
    #[prop(into)]
    completed_steps: RwSignal<Vec<String>>,
    /// Optional title displayed above the accordion.
    #[prop(optional, into)]
    title: Option<String>,
    /// Optional description below the title.
    #[prop(optional, into)]
    description: Option<String>,
    /// Size variant.
    #[prop(optional)]
    size: Option<GuideSize>,
    /// Progress indicator style.
    #[prop(optional)]
    progress_style: Option<ProgressStyle>,
    /// Allow multiple sections to be expanded at once.
    #[prop(optional)]
    #[prop(default = false)]
    allow_multiple: bool,
    /// Require sequential completion (can't skip ahead).
    #[prop(optional)]
    #[prop(default = false)]
    require_sequential: bool,
    /// Show completion checkmarks.
    #[prop(optional)]
    #[prop(default = true)]
    show_completion: bool,
    /// Callback when a step is completed.
    #[prop(optional, into)]
    on_step_complete: Option<Callback<usize>>,
    /// Callback when all steps are completed.
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));
    let size = size.unwrap_or_default();
    let progress_style = progress_style.unwrap_or(ProgressStyle::Numbers);
    let total = steps.len();
    let steps = StoredValue::new(steps);

    // Build combined class
    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![
            format!("{}-accordion", p),
            size.class(&p),
        ];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Create current signal for progress
    let current_signal = Signal::derive(move || {
        active_step.get().unwrap_or(0)
    });

    // Check if step is completed
    let is_step_completed = move |step_id: &str| -> bool {
        completed_steps.get().contains(&step_id.to_string())
    };

    // Check how many steps are completed
    let completed_count = Signal::derive(move || {
        completed_steps.get().len()
    });

    view! {
        <div class=combined_class>
            // Header
            {title.clone().map(|t| {
                let p = prefix.get_value();
                view! {
                    <div class=format!("{}-accordion-header", p)>
                        <h2 class=format!("{}-accordion-title", p)>{t}</h2>
                        {description.clone().map(|d| view! {
                            <p class=format!("{}-accordion-description", p)>{d}</p>
                        })}
                    </div>
                }
            })}

            // Progress indicator
            {(progress_style != ProgressStyle::None).then(|| {
                let p = prefix.get_value();
                view! {
                    <div class=format!("{}-accordion-progress", p)>
                        <GuideProgress
                            current=current_signal
                            total=total
                            style=progress_style
                        />
                    </div>
                }
            })}

            // Accordion items
            <div class=format!("{}-accordion-items", prefix.get_value())>
                <For
                    each=move || steps.get_value().into_iter().enumerate()
                    key=|(i, step)| format!("{}-{}", i, step.id.clone())
                    children=move |(i, step)| {
                        let step_id = step.id.clone();
                        let step_title = step.title.clone();
                        let step_clone = step.clone();

                        // Create multiple clones of step_id for different closures
                        let step_id_for_completed = step_id.clone();
                        let step_id_for_indicator = step_id.clone();
                        let step_id_for_show = step_id.clone();
                        let step_id_for_mark = step_id.clone();

                        let is_expanded = move || active_step.get() == Some(i);

                        let can_access = move || {
                            if !require_sequential {
                                return true;
                            }
                            if i == 0 {
                                return true;
                            }
                            // Check if previous step is completed
                            let all_steps = steps.get_value();
                            if i > 0 && i <= all_steps.len() {
                                let prev_id = &all_steps[i - 1].id;
                                completed_steps.get().contains(prev_id)
                            } else {
                                false
                            }
                        };

                        let toggle_click = move |_| {
                            if !can_access() {
                                return;
                            }
                            let current = active_step.get();
                            if current == Some(i) {
                                active_step.set(None);
                            } else {
                                active_step.set(Some(i));
                            }
                        };

                        let mark_complete = {
                            let on_step_complete = on_step_complete.clone();
                            let on_complete = on_complete.clone();
                            move |_| {
                                // Add to completed
                                completed_steps.update(|c| {
                                    if !c.contains(&step_id_for_mark) {
                                        c.push(step_id_for_mark.clone());
                                    }
                                });

                                // Callback
                                if let Some(ref cb) = on_step_complete {
                                    cb.run(i);
                                }

                                // Check if all complete
                                if completed_steps.get().len() >= total {
                                    if let Some(ref cb) = on_complete {
                                        cb.run(());
                                    }
                                }

                                // Auto-advance
                                if i < total - 1 {
                                    active_step.set(Some(i + 1));
                                }
                            }
                        };

                        // Get prefix clones for each usage
                        let p_class = prefix.get_value();
                        let p_header = prefix.get_value();
                        let p_indicator = prefix.get_value();
                        let p_title = prefix.get_value();
                        let p_chevron = prefix.get_value();
                        let p_content = prefix.get_value();
                        let p_body = prefix.get_value();
                        let p_complete = prefix.get_value();

                        view! {
                            <div
                                class=move || {
                                    let mut c = format!("{}-accordion-item", p_class);
                                    if is_expanded() { c.push_str(" expanded"); }
                                    if completed_steps.get().contains(&step_id_for_completed) { c.push_str(" completed"); }
                                    if !can_access() { c.push_str(" disabled"); }
                                    c
                                }
                            >
                                <button
                                    type="button"
                                    class=format!("{}-accordion-item-header", p_header)
                                    on:click=toggle_click
                                    disabled=move || !can_access()
                                >
                                    <span class=format!("{}-accordion-item-indicator", p_indicator)>
                                        {move || if show_completion && completed_steps.get().contains(&step_id_for_indicator) {
                                            "✓".to_string()
                                        } else {
                                            (i + 1).to_string()
                                        }}
                                    </span>
                                    <span class=format!("{}-accordion-item-title", p_title)>
                                        {step_title.clone()}
                                    </span>
                                    <span class=move || {
                                        if is_expanded() {
                                            format!("{}-accordion-item-chevron rotated", p_chevron)
                                        } else {
                                            format!("{}-accordion-item-chevron", p_chevron)
                                        }
                                    }>
                                        "▼"
                                    </span>
                                </button>

                                <div
                                    class=format!("{}-accordion-item-content", p_content)
                                    style=move || if is_expanded() { "display: block" } else { "display: none" }
                                >
                                    <div class=format!("{}-accordion-item-body", p_body)>
                                        <p style="color: rgba(255,255,255,0.65); line-height: 1.6;">
                                            {step_clone.description.clone()}
                                        </p>

                                        <Show when=move || !completed_steps.get().contains(&step_id_for_show)>
                                            <button
                                                type="button"
                                                class=format!("{}-accordion-item-complete", p_complete)
                                                style="margin-top: 16px; padding: 8px 16px; background: #1677ff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                                on:click=mark_complete.clone()
                                            >
                                                {if i == total - 1 { "Complete Guide" } else { "Mark Complete" }}
                                            </button>
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
