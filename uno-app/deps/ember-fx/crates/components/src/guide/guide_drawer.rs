//! Side panel drawer guide component.

use leptos::prelude::*;
use super::types::{GuideStep, GuideSize, DrawerPosition, ProgressStyle};
use super::guide_step::{GuideStepView, ImagePosition};
use super::guide_progress::GuideProgress;
use super::guide_controls::GuideControls;
use crate::try_use_theme;

/// Side panel stepper guide for non-blocking walkthroughs.
///
/// Features:
/// - Slides in from left, right, or bottom
/// - Vertical step navigation
/// - Non-blocking (user can interact with background)
/// - Collapsible/minimizable
/// - Responsive (full-screen on mobile)
///
/// # Example
///
/// ```ignore
/// let steps = vec![
///     GuideStep::new("1", "First Step", "Description here"),
///     GuideStep::new("2", "Second Step", "Another description"),
/// ];
///
/// let active = RwSignal::new(0usize);
/// let open = RwSignal::new(true);
///
/// view! {
///     <GuideDrawer
///         steps=steps
///         active_step=active
///         open=open
///         position=DrawerPosition::Right
///     />
/// }
/// ```
#[component]
pub fn GuideDrawer(
    /// The steps to display in the drawer.
    #[prop(into)]
    steps: Vec<GuideStep>,
    /// Signal controlling the active step index (0-based).
    #[prop(into)]
    active_step: RwSignal<usize>,
    /// Signal controlling whether drawer is open.
    #[prop(into)]
    open: RwSignal<bool>,
    /// Optional title displayed in the header.
    #[prop(optional, into)]
    title: Option<String>,
    /// Position of the drawer.
    #[prop(optional)]
    position: Option<DrawerPosition>,
    /// Size variant.
    #[prop(optional)]
    size: Option<GuideSize>,
    /// Progress indicator style.
    #[prop(optional)]
    progress_style: Option<ProgressStyle>,
    /// Show backdrop overlay.
    #[prop(optional)]
    #[prop(default = false)]
    show_backdrop: bool,
    /// Allow minimizing to a floating button.
    #[prop(optional)]
    #[prop(default = true)]
    allow_minimize: bool,
    /// Show step list in sidebar.
    #[prop(optional)]
    #[prop(default = true)]
    show_step_list: bool,
    /// Callback when guide is completed.
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// Callback when guide is skipped.
    #[prop(optional, into)]
    on_skip: Option<Callback<()>>,
    /// Callback when drawer is closed.
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
    let position = position.unwrap_or_default();
    let size = size.unwrap_or_default();
    let progress_style = progress_style.unwrap_or(ProgressStyle::Steps);
    let total = steps.len();
    let steps = StoredValue::new(steps);

    // Minimized state
    let minimized = RwSignal::new(false);

    // Store custom class for use in reactive closure
    let custom_class = StoredValue::new(class.clone());

    // Build combined class as a derived signal
    let combined_class = Signal::derive(move || {
        let p = prefix.get_value();
        let mut parts = vec![
            format!("{}-drawer", p),
            position.class(&p),
            size.class(&p),
        ];
        if minimized.get() {
            parts.push("minimized".to_string());
        }
        if !open.get() {
            parts.push("closed".to_string());
        }
        if let Some(ref custom) = custom_class.get_value() {
            parts.push(custom.clone());
        }
        parts.join(" ")
    });

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

    let go_to_step = {
        let on_step_change = on_step_change.clone();
        move |step: usize| {
            active_step.set(step);
            if let Some(ref cb) = on_step_change {
                cb.run(step);
            }
        }
    };

    let handle_complete = {
        let on_complete = on_complete.clone();
        move |_| {
            if let Some(ref cb) = on_complete {
                cb.run(());
            }
            open.set(false);
        }
    };

    let handle_skip = {
        let on_skip = on_skip.clone();
        move |_| {
            if let Some(ref cb) = on_skip {
                cb.run(());
            }
            open.set(false);
        }
    };

    let handle_close = {
        let on_close = on_close.clone();
        move |_| {
            if let Some(ref cb) = on_close {
                cb.run(());
            }
            open.set(false);
        }
    };

    let toggle_minimize = move |_| {
        minimized.update(|m| *m = !*m);
    };

    // Create signal for current step
    let current_signal = Signal::derive(move || active_step.get());

    view! {
        // Minimized floating button
        <Show when=move || minimized.get() && allow_minimize>
            {
                let p = prefix.get_value();
                view! {
                    <button
                        type="button"
                        class=format!("{}-drawer-minimized-btn", p)
                        on:click=move |_| minimized.set(false)
                        aria-label="Expand guide"
                    >
                        <span class=format!("{}-drawer-minimized-icon", p)>"?"</span>
                        <span class=format!("{}-drawer-minimized-progress", p)>
                            {move || format!("{}/{}", active_step.get() + 1, total)}
                        </span>
                    </button>
                }
            }
        </Show>

        // Backdrop (optional)
        <Show when=move || show_backdrop && open.get() && !minimized.get()>
            <div
                class=format!("{}-drawer-backdrop", prefix.get_value())
                on:click=handle_close.clone()
            />
        </Show>

        // Main drawer
        <Show when=move || open.get() && !minimized.get()>
            <div class=combined_class>
                // Header
                <div class=format!("{}-drawer-header", prefix.get_value())>
                    {title.clone().map(|t| {
                        let p = prefix.get_value();
                        view! {
                            <h2 class=format!("{}-drawer-title", p)>{t}</h2>
                        }
                    })}

                    <div class=format!("{}-drawer-header-actions", prefix.get_value())>
                        {if allow_minimize {
                            let p = prefix.get_value();
                            view! {
                                <button
                                    type="button"
                                    class=format!("{}-drawer-minimize", p)
                                    on:click=toggle_minimize
                                    aria-label="Minimize guide"
                                >
                                    "−"
                                </button>
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }}

                        {
                            let p = prefix.get_value();
                            let close_handler = handle_close.clone();
                            view! {
                                <button
                                    type="button"
                                    class=format!("{}-drawer-close", p)
                                    on:click=close_handler
                                    aria-label="Close guide"
                                >
                                    "×"
                                </button>
                            }
                        }
                    </div>
                </div>

                // Content area with step list
                <div class=format!("{}-drawer-body", prefix.get_value())>
                    // Step list sidebar
                    {if show_step_list {
                        let p = prefix.get_value();
                        view! {
                            <div class=format!("{}-drawer-step-list", p)>
                                <GuideProgress
                                    current=current_signal
                                    total=total
                                    style=progress_style
                                    steps=steps.get_value()
                                    on_step_click=Callback::new(go_to_step.clone())
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }}

                    // Current step content
                    <div class=format!("{}-drawer-content", prefix.get_value())>
                        <For
                            each=move || steps.get_value().into_iter().enumerate()
                            key=|(i, step)| format!("{}-{}", i, step.id.clone())
                            children=move |(i, step)| {
                                let is_active = move || active_step.get() == i;
                                view! {
                                    <Show when=is_active>
                                        <GuideStepView
                                            step=step.clone()
                                            is_active=true
                                            image_position=ImagePosition::Top
                                        />
                                    </Show>
                                }
                            }
                        />
                    </div>
                </div>

                // Footer controls
                <div class=format!("{}-drawer-footer", prefix.get_value())>
                    <GuideControls
                        current=current_signal
                        total=total
                        on_prev=Callback::new(go_prev.clone())
                        on_next=Callback::new(go_next.clone())
                        on_skip=Callback::new(handle_skip.clone())
                        on_complete=Callback::new(handle_complete.clone())
                    />
                </div>
            </div>
        </Show>
    }
}
