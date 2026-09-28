//! Wizard stepper component.
//!
//! Step indicator showing all wizard steps with progress.

use leptos::prelude::*;
use crate::try_use_theme;
use super::super::types::{WizardStage, StepperOrientation};

/// Individual step item data.
#[derive(Debug, Clone)]
pub struct StepItem {
    /// Step number (1-based).
    pub number: usize,
    /// Step label.
    pub label: String,
    /// Whether this step is completed.
    pub completed: bool,
    /// Whether this step is current.
    pub current: bool,
    /// Whether this step has an error.
    pub has_error: bool,
}

impl StepItem {
    /// Creates a new step item.
    pub fn new(number: usize, label: impl Into<String>) -> Self {
        Self {
            number,
            label: label.into(),
            completed: false,
            current: false,
            has_error: false,
        }
    }

    /// Marks the step as completed.
    pub fn completed(mut self) -> Self {
        self.completed = true;
        self
    }

    /// Marks the step as current.
    pub fn current(mut self) -> Self {
        self.current = true;
        self
    }

    /// Marks the step as having an error.
    pub fn with_error(mut self) -> Self {
        self.has_error = true;
        self
    }
}

/// SVG icons for step status.
mod icons {
    use leptos::prelude::*;

    pub fn check_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="ant-wizard-icon">
                <polyline points="20 6 9 17 4 12"/>
            </svg>
        }
    }

    #[allow(dead_code)]
    pub fn error_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="ant-wizard-icon">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
        }
    }
}

/// Wizard stepper component.
///
/// Displays step indicators with completed/active/upcoming states.
#[component]
pub fn WizardStepper<T: WizardStage + 'static>(
    /// List of steps.
    #[prop(into)]
    steps: Signal<Vec<T>>,
    /// Current step index (0-based).
    #[prop(into)]
    current: Signal<usize>,
    /// Step names (optional, uses stage display_name if not provided).
    #[prop(optional, into)]
    step_names: Option<Signal<Vec<String>>>,
    /// Stepper orientation.
    #[prop(optional, into)]
    orientation: Option<StepperOrientation>,
    /// Allow clicking steps to navigate.
    #[prop(optional)]
    clickable: bool,
    /// Callback when a step is clicked.
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

    let orientation = orientation.unwrap_or_default();
    let prefix = format!("fx-wizard-{}-stepper", design_system);

    let container_class = move || {
        let mut classes = vec![
            prefix.clone(),
            format!("{}-{}", prefix, orientation.class_suffix()),
        ];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    view! {
        <div class=container_class role="navigation" aria-label="Wizard steps">
            <For
                each=move || {
                    let s = steps.get();
                    let names = step_names.map(|n| n.get()).unwrap_or_default();
                    s.into_iter().enumerate().map(|(idx, stage)| {
                        let name = names.get(idx).cloned()
                            .unwrap_or_else(|| stage.display_name().to_string());
                        (idx, stage, name)
                    }).collect::<Vec<_>>()
                }
                key=|(idx, _, _)| *idx
                children=move |(idx, _stage, name)| {
                    let prefix = format!("fx-wizard-{}-stepper", design_system);
                    let prefix_step = prefix.clone();
                    let prefix_indicator = prefix.clone();
                    let prefix_inner = prefix.clone();
                    let prefix_content = prefix.clone();
                    let prefix_connector = prefix.clone();

                    let is_completed = Signal::derive(move || idx < current.get());
                    let is_current = Signal::derive(move || idx == current.get());
                    let step_number = idx + 1;
                    let total = steps.get().len();

                    let step_class = move || {
                        let mut classes = vec![format!("{}-item", prefix_step)];
                        if is_completed.get() {
                            classes.push(format!("{}-item-completed", prefix_step));
                        }
                        if is_current.get() {
                            classes.push(format!("{}-item-active", prefix_step));
                        }
                        if clickable {
                            classes.push(format!("{}-item-clickable", prefix_step));
                        }
                        classes.join(" ")
                    };

                    let indicator_class = move || {
                        let mut classes = vec![format!("{}-indicator", prefix_indicator)];
                        if is_completed.get() {
                            classes.push(format!("{}-indicator-completed", prefix_indicator));
                        }
                        if is_current.get() {
                            classes.push(format!("{}-indicator-active", prefix_indicator));
                        }
                        classes.join(" ")
                    };

                    let handle_click = {
                        let on_click = on_step_click.clone();
                        move |_| {
                            if clickable {
                                if let Some(ref cb) = on_click {
                                    cb.run(idx);
                                }
                            }
                        }
                    };

                    view! {
                        <div
                            class=step_class
                            on:click=handle_click
                            role="listitem"
                            aria-current=move || if is_current.get() { Some("step") } else { None }
                        >
                            // Step indicator
                            <div class=indicator_class>
                                {move || {
                                    if is_completed.get() {
                                        view! { <span class=format!("{}-check", prefix_inner)>{icons::check_icon()}</span> }.into_any()
                                    } else {
                                        view! { <span class=format!("{}-number", prefix_inner)>{step_number}</span> }.into_any()
                                    }
                                }}
                            </div>

                            // Step content
                            <div class=format!("{}-content", prefix_content)>
                                <div class=format!("{}-title", prefix_content)>
                                    {name.clone()}
                                </div>
                            </div>

                            // Connector (except for last step)
                            {(idx < total - 1).then(|| {
                                let connector_class = move || {
                                    let mut classes = vec![format!("{}-connector", prefix_connector)];
                                    if is_completed.get() {
                                        classes.push(format!("{}-connector-completed", prefix_connector));
                                    }
                                    classes.join(" ")
                                };
                                view! { <div class=connector_class></div> }
                            })}
                        </div>
                    }
                }
            />
        </div>
    }
}
