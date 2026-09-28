//! Full-screen carousel overlay guide component.

use leptos::prelude::*;
use super::types::{GuideStep, GuideSize, GuideTransition, ProgressStyle};
use super::guide_step::{GuideStepView, ImagePosition};
use super::guide_progress::GuideProgress;
use super::guide_controls::GuideControls;
use crate::try_use_theme;

/// Full-screen carousel guide for immersive onboarding experiences.
///
/// Features:
/// - Modal overlay with backdrop
/// - Swipeable slides (touch support)
/// - Keyboard navigation (arrow keys, Escape)
/// - Progress indicators (dots, bar, numbers)
/// - Responsive layout (mobile-first)
///
/// # Example
///
/// ```ignore
/// let steps = vec![
///     GuideStep::new("1", "Welcome", "Get started with our app"),
///     GuideStep::new("2", "Setup", "Configure your preferences"),
/// ];
///
/// let active = RwSignal::new(0usize);
/// let visible = RwSignal::new(true);
///
/// view! {
///     <GuideCarousel
///         steps=steps
///         active_step=active
///         visible=visible
///         title="Getting Started"
///     />
/// }
/// ```
#[component]
pub fn GuideCarousel(
    /// The steps to display in the carousel.
    #[prop(into)]
    steps: Vec<GuideStep>,
    /// Signal controlling the active step index (0-based).
    #[prop(into)]
    active_step: RwSignal<usize>,
    /// Signal controlling visibility of the carousel.
    #[prop(into)]
    visible: RwSignal<bool>,
    /// Optional title displayed in the header.
    #[prop(optional, into)]
    title: Option<String>,
    /// Size variant.
    #[prop(optional)]
    size: Option<GuideSize>,
    /// Transition effect between steps.
    #[prop(optional)]
    transition: Option<GuideTransition>,
    /// Progress indicator style.
    #[prop(optional)]
    progress_style: Option<ProgressStyle>,
    /// Image position relative to content.
    #[prop(optional)]
    image_position: Option<ImagePosition>,
    /// Allow closing by clicking backdrop.
    #[prop(optional)]
    #[prop(default = true)]
    close_on_backdrop: bool,
    /// Allow closing with Escape key.
    #[prop(optional)]
    #[prop(default = true)]
    close_on_escape: bool,
    /// Show close button.
    #[prop(optional)]
    #[prop(default = true)]
    show_close: bool,
    /// Show skip button.
    #[prop(optional)]
    #[prop(default = true)]
    show_skip: bool,
    /// Callback when guide is completed.
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// Callback when guide is skipped.
    #[prop(optional, into)]
    on_skip: Option<Callback<()>>,
    /// Callback when guide is closed.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Callback when step changes.
    #[prop(optional, into)]
    on_step_change: Option<Callback<usize>>,
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
    let transition = transition.unwrap_or_default();
    let progress_style = progress_style.unwrap_or(ProgressStyle::Dots);
    let total = steps.len();
    let steps = StoredValue::new(steps);

    // Build combined class
    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![
            format!("{}-carousel", p),
            size.class(&p),
            transition.class(&p),
        ];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Navigation handlers
    let go_prev = {
        let on_step_change = on_step_change.clone();
        move |_| {
            let current = active_step.get();
            if current > 0 {
                let new_step = current - 1;
                active_step.set(new_step);
                if let Some(ref cb) = on_step_change {
                    cb.run(new_step);
                }
            }
        }
    };

    let go_next = {
        let on_step_change = on_step_change.clone();
        move |_| {
            let current = active_step.get();
            if current < total - 1 {
                let new_step = current + 1;
                active_step.set(new_step);
                if let Some(ref cb) = on_step_change {
                    cb.run(new_step);
                }
            }
        }
    };

    let handle_complete = {
        let on_complete = on_complete.clone();
        move |_| {
            if let Some(ref cb) = on_complete {
                cb.run(());
            }
            visible.set(false);
        }
    };

    let handle_skip = {
        let on_skip = on_skip.clone();
        move |_| {
            if let Some(ref cb) = on_skip {
                cb.run(());
            }
            visible.set(false);
        }
    };

    let handle_close = {
        let on_close = on_close.clone();
        move |_| {
            if let Some(ref cb) = on_close {
                cb.run(());
            }
            visible.set(false);
        }
    };

    let handle_backdrop_click = {
        let handle_close = handle_close.clone();
        move |_| {
            if close_on_backdrop {
                handle_close(());
            }
        }
    };

    // Create signal for current step
    let current_signal = Signal::derive(move || active_step.get());

    view! {
        <Show when=move || visible.get()>
            <div
                class=format!("{}-overlay", prefix.get_value())
                on:click=handle_backdrop_click.clone()
            >
                <div
                    class=combined_class.clone()
                    on:click=|e| e.stop_propagation()
                >
                    // Header
                    <div class=format!("{}-carousel-header", prefix.get_value())>
                        {title.clone().map(|t| {
                            let p = prefix.get_value();
                            view! {
                                <h2 class=format!("{}-carousel-title", p)>{t}</h2>
                            }
                        })}

                        {if show_close {
                            let handle_close = handle_close.clone();
                            let p = prefix.get_value();
                            view! {
                                <button
                                    type="button"
                                    class=format!("{}-carousel-close", p)
                                    on:click=move |_| handle_close(())
                                    aria-label="Close guide"
                                >
                                    "×"
                                </button>
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }}
                    </div>

                    // Progress indicator
                    <GuideProgress
                        current=current_signal
                        total=total
                        style=progress_style
                        steps=steps.get_value()
                    />

                    // Slides container
                    <div class=format!("{}-carousel-slides", prefix.get_value())>
                        <For
                            each=move || steps.get_value().into_iter().enumerate()
                            key=|(i, step)| format!("{}-{}", i, step.id.clone())
                            children=move |(i, step)| {
                                let p = prefix.get_value();
                                let is_active = move || active_step.get() == i;
                                view! {
                                    <div
                                        class=move || {
                                            let mut c = format!("{}-carousel-slide", p);
                                            if is_active() { c.push_str(" active"); }
                                            c
                                        }
                                        style=move || if is_active() { "display: block" } else { "display: none" }
                                    >
                                        <GuideStepView
                                            step=step.clone()
                                            is_active=is_active()
                                            image_position=image_position.unwrap_or_default()
                                        />
                                    </div>
                                }
                            }
                        />
                    </div>

                    // Controls
                    <GuideControls
                        current=current_signal
                        total=total
                        on_prev=Callback::new(go_prev.clone())
                        on_next=Callback::new(go_next.clone())
                        on_skip=Callback::new(handle_skip.clone())
                        on_complete=Callback::new(handle_complete.clone())
                        show_skip=show_skip
                    />
                </div>
            </div>
        </Show>
    }
}
