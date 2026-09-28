//! Wizard navigation components.
//!
//! Navigation buttons for wizard steps.

use leptos::prelude::*;
use crate::try_use_theme;
use crate::button::{Button, ButtonVariant};
use super::super::types::NavigationAlignment;

/// Navigation button labels.
#[derive(Debug, Clone)]
pub struct NavigationLabels {
    /// Previous button label.
    pub previous: String,
    /// Next button label.
    pub next: String,
    /// Submit button label.
    pub submit: String,
    /// Cancel button label.
    pub cancel: String,
}

impl Default for NavigationLabels {
    fn default() -> Self {
        Self {
            previous: "Previous".to_string(),
            next: "Next".to_string(),
            submit: "Submit".to_string(),
            cancel: "Cancel".to_string(),
        }
    }
}

impl NavigationLabels {
    /// Creates new labels.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the previous label.
    pub fn with_previous(mut self, label: impl Into<String>) -> Self {
        self.previous = label.into();
        self
    }

    /// Sets the next label.
    pub fn with_next(mut self, label: impl Into<String>) -> Self {
        self.next = label.into();
        self
    }

    /// Sets the submit label.
    pub fn with_submit(mut self, label: impl Into<String>) -> Self {
        self.submit = label.into();
        self
    }

    /// Sets the cancel label.
    pub fn with_cancel(mut self, label: impl Into<String>) -> Self {
        self.cancel = label.into();
        self
    }
}

/// SVG icons for navigation buttons.
mod icons {
    use leptos::prelude::*;

    pub fn chevron_left() -> impl IntoView {
        view! {
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="15 18 9 12 15 6"/>
            </svg>
        }
    }

    pub fn chevron_right() -> impl IntoView {
        view! {
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"/>
            </svg>
        }
    }

    pub fn check() -> impl IntoView {
        view! {
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"/>
            </svg>
        }
    }
}

/// Wizard navigation buttons component.
///
/// Provides Previous, Next, and Submit buttons with proper state handling.
#[component]
pub fn WizardNavigation(
    /// Whether we can go back.
    #[prop(into)]
    can_go_back: Signal<bool>,
    /// Whether we can go forward.
    #[prop(into)]
    can_go_forward: Signal<bool>,
    /// Whether this is the last step.
    #[prop(into)]
    is_last_step: Signal<bool>,
    /// Whether the wizard is loading/submitting.
    #[prop(optional, into)]
    is_loading: Option<Signal<bool>>,
    /// Whether the wizard has been submitted.
    #[prop(optional, into)]
    is_submitted: Option<Signal<bool>>,
    /// Button labels.
    #[prop(optional)]
    labels: Option<NavigationLabels>,
    /// Button alignment.
    #[prop(optional, into)]
    alignment: Option<NavigationAlignment>,
    /// Show cancel button.
    #[prop(optional)]
    show_cancel: bool,
    /// Previous button click handler.
    #[prop(optional, into)]
    on_previous: Option<Callback<()>>,
    /// Next button click handler.
    #[prop(optional, into)]
    on_next: Option<Callback<()>>,
    /// Submit button click handler.
    #[prop(optional, into)]
    on_submit: Option<Callback<()>>,
    /// Cancel button click handler.
    #[prop(optional, into)]
    on_cancel: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-navigation", design_system);
    let prefix_container = prefix.clone();
    let prefix_left = prefix.clone();
    let prefix_right = prefix;

    let labels = labels.unwrap_or_default();
    let alignment = alignment.unwrap_or_default();
    let is_loading = is_loading.unwrap_or_else(|| Signal::derive(|| false));
    let is_submitted = is_submitted.unwrap_or_else(|| Signal::derive(|| false));

    let container_class = move || {
        let mut classes = vec![
            prefix_container.clone(),
            format!("{}-{}", prefix_container, alignment.class_suffix()),
        ];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    let is_disabled = Signal::derive(move || is_loading.get() || is_submitted.get());

    let prev_label = labels.previous.clone();
    let next_label = labels.next.clone();
    let submit_label = labels.submit.clone();
    let cancel_label = labels.cancel.clone();

    view! {
        <div class=container_class>
            // Left side buttons (Cancel + Previous)
            <div class=format!("{}-left", prefix_left)>
                {show_cancel.then({
                    let cancel_label = cancel_label.clone();
                    let on_cancel = on_cancel.clone();
                    move || {
                        let cancel_label = cancel_label.clone();
                        let on_cancel = on_cancel.clone();
                        view! {
                            <Button
                                variant=ButtonVariant::Outline
                                disabled=is_disabled.get()
                                on_click=move |_| {
                                    if let Some(ref cb) = on_cancel {
                                        cb.run(());
                                    }
                                }
                            >
                                {cancel_label.clone()}
                            </Button>
                        }
                    }
                })}

                {move || {
                    let prev_label = prev_label.clone();
                    let on_previous = on_previous.clone();
                    can_go_back.get().then(move || {
                        let on_previous = on_previous.clone();
                        view! {
                            <Button
                                variant=ButtonVariant::Outline
                                disabled=is_disabled.get()
                                on_click=move |_| {
                                    if let Some(ref cb) = on_previous {
                                        cb.run(());
                                    }
                                }
                            >
                                {icons::chevron_left()}
                                <span>{prev_label.clone()}</span>
                            </Button>
                        }
                    })
                }}
            </div>

            // Right side buttons (Next/Submit)
            <div class=format!("{}-right", prefix_right)>
                {move || {
                    let next_label = next_label.clone();
                    let submit_label = submit_label.clone();
                    let on_next = on_next.clone();
                    let on_submit = on_submit.clone();

                    if is_last_step.get() {
                        let on_submit = on_submit.clone();
                        let submit_label = submit_label.clone();
                        view! {
                            <Button
                                variant=ButtonVariant::Primary
                                disabled=is_disabled.get() || is_submitted.get()
                                loading=is_loading.get()
                                on_click=move |_| {
                                    if let Some(ref cb) = on_submit {
                                        cb.run(());
                                    }
                                }
                            >
                                {move || {
                                    if is_submitted.get() {
                                        view! {
                                            {icons::check()}
                                            <span>"Submitted"</span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <span>{submit_label.clone()}</span>
                                        }.into_any()
                                    }
                                }}
                            </Button>
                        }.into_any()
                    } else if can_go_forward.get() {
                        let on_next = on_next.clone();
                        let next_label = next_label.clone();
                        view! {
                            <Button
                                variant=ButtonVariant::Primary
                                disabled=is_disabled.get()
                                on_click=move |_| {
                                    if let Some(ref cb) = on_next {
                                        cb.run(());
                                    }
                                }
                            >
                                <span>{next_label.clone()}</span>
                                {icons::chevron_right()}
                            </Button>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

/// Compact navigation for mobile/small screens.
///
/// Uses icon-only buttons or minimal text.
#[component]
pub fn WizardCompactNavigation(
    /// Whether we can go back.
    #[prop(into)]
    can_go_back: Signal<bool>,
    /// Whether we can go forward.
    #[prop(into)]
    can_go_forward: Signal<bool>,
    /// Whether this is the last step.
    #[prop(into)]
    is_last_step: Signal<bool>,
    /// Current step name.
    #[prop(into)]
    current_step_name: Signal<String>,
    /// Current step (1-based).
    #[prop(into)]
    current_step: Signal<usize>,
    /// Total steps.
    #[prop(into)]
    total_steps: Signal<usize>,
    /// Whether the wizard is loading/submitting.
    #[prop(optional, into)]
    is_loading: Option<Signal<bool>>,
    /// Previous button click handler.
    #[prop(optional, into)]
    on_previous: Option<Callback<()>>,
    /// Next button click handler.
    #[prop(optional, into)]
    on_next: Option<Callback<()>>,
    /// Submit button click handler.
    #[prop(optional, into)]
    on_submit: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-compact-nav", design_system);
    let prefix_container = prefix.clone();
    let prefix_info = prefix.clone();
    let prefix_title = prefix.clone();
    let prefix_counter = prefix;

    let is_loading = is_loading.unwrap_or_else(|| Signal::derive(|| false));

    let container_class = move || {
        let mut classes = vec![prefix_container.clone()];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    view! {
        <div class=container_class>
            // Previous button
            {move || {
                let on_previous = on_previous.clone();
                can_go_back.get().then(move || {
                    let on_previous = on_previous.clone();
                    view! {
                        <Button
                            variant=ButtonVariant::Outline
                            icon_only=true
                            aria_label="Previous step".to_string()
                            on_click=move |_| {
                                if let Some(ref cb) = on_previous {
                                    cb.run(());
                                }
                            }
                        >
                            {icons::chevron_left()}
                        </Button>
                    }
                })
            }}

            // Center info
            <div class=format!("{}-info", prefix_info)>
                <div class=format!("{}-title", prefix_title)>
                    {move || current_step_name.get()}
                </div>
                <div class=format!("{}-counter", prefix_counter)>
                    {move || format!("{} of {}", current_step.get(), total_steps.get())}
                </div>
            </div>

            // Next/Submit button
            {move || {
                let on_next = on_next.clone();
                let on_submit = on_submit.clone();

                if is_last_step.get() {
                    let on_submit = on_submit.clone();
                    view! {
                        <Button
                            variant=ButtonVariant::Primary
                            icon_only=true
                            loading=is_loading.get()
                            aria_label="Submit".to_string()
                            on_click=move |_| {
                                if let Some(ref cb) = on_submit {
                                    cb.run(());
                                }
                            }
                        >
                            {icons::check()}
                        </Button>
                    }.into_any()
                } else if can_go_forward.get() {
                    let on_next = on_next.clone();
                    view! {
                        <Button
                            variant=ButtonVariant::Primary
                            icon_only=true
                            aria_label="Next step".to_string()
                            on_click=move |_| {
                                if let Some(ref cb) = on_next {
                                    cb.run(());
                                }
                            }
                        >
                            {icons::chevron_right()}
                        </Button>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}
