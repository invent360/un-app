//! Guide navigation controls component.

use leptos::prelude::*;
use crate::try_use_theme;

/// Navigation controls for guide components.
///
/// Provides Previous, Next, Skip, and Complete buttons.
#[component]
pub fn GuideControls(
    /// Current step index (0-based).
    #[prop(into)]
    current: Signal<usize>,
    /// Total number of steps.
    #[prop(into)]
    total: usize,
    /// Callback when Previous is clicked.
    #[prop(optional, into)]
    on_prev: Option<Callback<()>>,
    /// Callback when Next is clicked.
    #[prop(optional, into)]
    on_next: Option<Callback<()>>,
    /// Callback when Skip is clicked.
    #[prop(optional, into)]
    on_skip: Option<Callback<()>>,
    /// Callback when Complete is clicked (last step).
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// Show skip button.
    #[prop(optional)]
    #[prop(default = true)]
    show_skip: bool,
    /// Previous button text.
    #[prop(optional, into)]
    prev_text: Option<String>,
    /// Next button text.
    #[prop(optional, into)]
    next_text: Option<String>,
    /// Skip button text.
    #[prop(optional, into)]
    skip_text: Option<String>,
    /// Complete button text.
    #[prop(optional, into)]
    complete_text: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));

    let prev_text = prev_text.unwrap_or_else(|| "Back".to_string());
    let next_text = next_text.unwrap_or_else(|| "Next".to_string());
    let skip_text = skip_text.unwrap_or_else(|| "Skip".to_string());
    let complete_text = complete_text.unwrap_or_else(|| "Done".to_string());

    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![format!("{}-controls", p)];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let is_first = move || current.get() == 0;
    let is_last = move || current.get() >= total.saturating_sub(1);

    view! {
        <div
            class=combined_class
            style="display: flex; justify-content: space-between; align-items: center; gap: 16px; padding: 16px 0;"
        >
            // Back button (left)
            <div style="flex: 1; display: flex; justify-content: flex-start;">
                {move || {
                    let on_prev = on_prev.clone();
                    let p = prefix.get_value();
                    if !is_first() {
                        view! {
                            <button
                                type="button"
                                class=format!("{}-controls-prev", p)
                                style="padding: 12px 24px; background: #303030; border: none; border-radius: 8px; color: #fff; cursor: pointer; font-size: 14px; font-weight: 500; display: flex; align-items: center; gap: 8px;"
                                on:click=move |_| {
                                    if let Some(ref cb) = on_prev {
                                        cb.run(());
                                    }
                                }
                            >
                                <span>"←"</span>
                                <span>{prev_text.clone()}</span>
                            </button>
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }
                }}
            </div>

            // Skip button (center, orange)
            <div style="flex: 1; display: flex; justify-content: center;">
                {move || {
                    let p = prefix.get_value();
                    if show_skip && on_skip.is_some() {
                        let on_skip = on_skip.clone();
                        view! {
                            <button
                                type="button"
                                class=format!("{}-controls-skip", p)
                                style="padding: 12px 24px; background: transparent; border: 1px solid #fa8c16; border-radius: 8px; color: #fa8c16; cursor: pointer; font-size: 14px; font-weight: 500;"
                                on:click=move |_| {
                                    if let Some(ref cb) = on_skip {
                                        cb.run(());
                                    }
                                }
                            >
                                {skip_text.clone()}
                            </button>
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }
                }}
            </div>

            // Next/Complete button (right)
            <div style="flex: 1; display: flex; justify-content: flex-end;">
                {move || {
                    let p = prefix.get_value();
                    if is_last() {
                        let on_complete = on_complete.clone();
                        view! {
                            <button
                                type="button"
                                class=format!("{}-controls-complete", p)
                                style="padding: 12px 24px; background: #52c41a; border: none; border-radius: 8px; color: #fff; cursor: pointer; font-size: 14px; font-weight: 500; display: flex; align-items: center; gap: 8px;"
                                on:click=move |_| {
                                    if let Some(ref cb) = on_complete {
                                        cb.run(());
                                    }
                                }
                            >
                                <span>{complete_text.clone()}</span>
                                <span>"✓"</span>
                            </button>
                        }.into_any()
                    } else {
                        let on_next = on_next.clone();
                        view! {
                            <button
                                type="button"
                                class=format!("{}-controls-next", p)
                                style="padding: 12px 24px; background: #1677ff; border: none; border-radius: 8px; color: #fff; cursor: pointer; font-size: 14px; font-weight: 500; display: flex; align-items: center; gap: 8px;"
                                on:click=move |_| {
                                    if let Some(ref cb) = on_next {
                                        cb.run(());
                                    }
                                }
                            >
                                <span>{next_text.clone()}</span>
                                <span>"→"</span>
                            </button>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
