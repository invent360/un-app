//! Wizard container component.
//!
//! The main wizard UI component that brings together all subcomponents.

use leptos::prelude::*;
use crate::try_use_theme;
use crate::navigation::{Steps, StepItem, StepsDirection, StepStatus, StepsIconType, StepsLineWeight, StepsLabelPlacement, StepsResponsive};
use super::super::types::{WizardStage, WizardLayout, StepperOrientation};
use super::progress::WizardProgress;
use super::navigation::WizardNavigation;
use super::error::WizardErrorDisplay;

/// Wizard container component.
///
/// Complete wizard UI with stepper, progress, content, and navigation.
#[component]
pub fn WizardContainer<T: WizardStage + 'static>(
    /// Wizard title.
    #[prop(into)]
    title: String,
    /// Wizard description (optional).
    #[prop(optional, into)]
    description: Option<String>,
    /// List of steps.
    #[prop(into)]
    steps: Signal<Vec<T>>,
    /// Current step index (0-based).
    #[prop(into)]
    current: Signal<usize>,
    /// Error messages.
    #[prop(optional, into)]
    errors: Option<Signal<Vec<String>>>,
    /// Whether wizard is loading/submitting.
    #[prop(optional, into)]
    is_loading: Option<Signal<bool>>,
    /// Whether wizard has been submitted.
    #[prop(optional, into)]
    is_submitted: Option<Signal<bool>>,
    /// Layout variant.
    #[prop(optional, into)]
    layout: Option<WizardLayout>,
    /// Show progress bar.
    #[prop(optional)]
    show_progress: Option<bool>,
    /// Show header row (step name + counter + progress bar).
    #[prop(optional)]
    show_header: Option<bool>,
    /// Show step indicator.
    #[prop(optional)]
    show_steps: Option<bool>,
    /// Allow clicking steps to navigate.
    #[prop(optional)]
    clickable_steps: bool,
    /// Steps icon type (Default, Outlined, Dot).
    #[prop(optional)]
    steps_icon_type: Option<StepsIconType>,
    /// Steps line weight (Thin, Medium, Thick).
    #[prop(optional)]
    steps_line_weight: Option<StepsLineWeight>,
    /// Steps label placement (Horizontal, Vertical).
    #[prop(optional)]
    steps_label_placement: Option<StepsLabelPlacement>,
    /// Steps responsive behavior (Compress, Scroll, Hide).
    #[prop(optional)]
    steps_responsive: Option<StepsResponsive>,
    /// Custom step icon size in pixels.
    #[prop(optional)]
    steps_icon_size_px: Option<u32>,
    /// Custom step title font size in pixels.
    #[prop(optional)]
    steps_title_font_size_px: Option<u32>,
    /// Previous button click handler.
    #[prop(optional, into)]
    on_previous: Option<Callback<()>>,
    /// Next button click handler.
    #[prop(optional, into)]
    on_next: Option<Callback<()>>,
    /// Submit button click handler.
    #[prop(optional, into)]
    on_submit: Option<Callback<()>>,
    /// Step click handler (for clickable steps).
    #[prop(optional, into)]
    on_step_click: Option<Callback<usize>>,
    /// Wizard content (WizardStep children).
    children: ChildrenFn,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}", design_system);
    let layout = layout.unwrap_or_default();
    let show_progress = show_progress.unwrap_or(true);
    let show_header = show_header.unwrap_or(true);
    let show_steps = show_steps.unwrap_or(true);
    let errors = errors.unwrap_or_else(|| Signal::derive(Vec::new));
    let is_loading = is_loading.unwrap_or_else(|| Signal::derive(|| false));
    let is_submitted = is_submitted.unwrap_or_else(|| Signal::derive(|| false));
    let steps_icon_type = steps_icon_type.unwrap_or_default();
    let steps_line_weight = steps_line_weight.unwrap_or_default();
    let steps_label_placement = steps_label_placement.unwrap_or_default();
    let steps_responsive = steps_responsive.unwrap_or_default();
    // Use 0 to signal "use default" in Steps component
    let steps_icon_size = steps_icon_size_px.unwrap_or(0);
    let steps_title_font_size = steps_title_font_size_px.unwrap_or(0);

    // Derived signals
    let total_steps = Signal::derive({
        let steps = steps.clone();
        move || steps.get().len()
    });

    let progress_percentage = Signal::derive({
        let steps = steps.clone();
        let current = current.clone();
        move || {
            let total = steps.get().len();
            let curr = current.get() + 1;
            if total == 0 { 0 } else { ((curr as f32 / total as f32) * 100.0) as u32 }
        }
    });

    let can_go_back = Signal::derive({
        let current = current.clone();
        move || current.get() > 0
    });

    let can_go_forward = Signal::derive({
        let current = current.clone();
        let steps = steps.clone();
        move || current.get() < steps.get().len().saturating_sub(1)
    });

    let is_last_step = Signal::derive({
        let current = current.clone();
        let steps = steps.clone();
        move || current.get() == steps.get().len().saturating_sub(1)
    });

    let current_step_name = Signal::derive({
        let steps = steps.clone();
        let current = current.clone();
        move || {
            let idx = current.get();
            steps.get().get(idx).map(|s| s.display_name().to_string())
            .unwrap_or_default()
        }
    });

    // Container class based on layout
    let container_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        format!("{} {}-{} {}", prefix, prefix, layout.class_suffix(), class.unwrap_or_default())
    };

    // Stepper orientation based on layout
    let stepper_orientation = match layout {
        WizardLayout::Horizontal => StepperOrientation::Vertical,
        _ => StepperOrientation::Horizontal,
    };

    let has_description = description.is_some();

    match layout {
        WizardLayout::Horizontal => {
            // Pre-compute all class strings
            let layout_class = format!("{}-layout-horizontal", prefix);
            let sidebar_class = format!("{}-sidebar", prefix);
            let sidebar_title_class = format!("{}-sidebar-title", prefix);
            let sidebar_desc_class = format!("{}-sidebar-description", prefix);
            let main_class = format!("{}-main", prefix);
            let header_prefix = prefix.clone();
            let content_prefix = prefix.clone();

            view! {
                <div class=container_class>
                    <div class=layout_class>
                        // Sidebar
                        <aside class=sidebar_class>
                            <h2 class=sidebar_title_class>
                                {title.clone()}
                            </h2>
                            <Show when=move || has_description>
                                <p class=sidebar_desc_class.clone()>
                                    {description.clone().unwrap_or_default()}
                                </p>
                            </Show>

                            <Show when=move || show_steps>
                                <WizardStepperWithClickable
                                    steps=steps
                                    current=current
                                    orientation=stepper_orientation
                                    clickable=clickable_steps
                                    on_step_click=on_step_click.clone()
                                    icon_type=steps_icon_type
                                    line_weight=steps_line_weight
                                    label_placement=steps_label_placement
                                    responsive=steps_responsive
                                    icon_size_px=steps_icon_size
                                    title_font_size_px=steps_title_font_size
                                />
                            </Show>
                        </aside>

                        // Main content
                        <main class=main_class>
                            <Show when=move || show_header>
                                <WizardHeader
                                    current_step_name=current_step_name
                                    current_step=Signal::derive(move || current.get() + 1)
                                    total_steps=total_steps
                                    show_progress=show_progress
                                    progress_percentage=progress_percentage
                                    prefix=header_prefix.clone()
                                />
                            </Show>

                            <WizardErrorDisplay messages=errors />

                            <WizardContent prefix=content_prefix>
                                {children()}
                            </WizardContent>

                            <WizardNavigationWithCallbacks
                                can_go_back=can_go_back
                                can_go_forward=can_go_forward
                                is_last_step=is_last_step
                                is_loading=is_loading
                                is_submitted=is_submitted
                                on_previous=on_previous.clone()
                                on_next=on_next.clone()
                                on_submit=on_submit.clone()
                            />
                        </main>
                    </div>
                </div>
            }.into_any()
        }
        _ => {
            // Vertical layout - pre-compute all class strings
            let layout_class = format!("{}-layout-vertical", prefix);
            let header_full_class = format!("{}-header-full", prefix);
            let title_class = format!("{}-title", prefix);
            let desc_class = format!("{}-description", prefix);
            let stepper_container_class = format!("{}-stepper-container", prefix);
            let header_prefix = prefix.clone();
            let content_prefix = prefix.clone();

            view! {
                <div class=container_class>
                    <div class=layout_class>
                        <header class=header_full_class>
                            <h2 class=title_class>
                                {title.clone()}
                            </h2>
                            <Show when=move || has_description>
                                <p class=desc_class.clone()>
                                    {description.clone().unwrap_or_default()}
                                </p>
                            </Show>
                        </header>

                        <Show when=move || show_steps>
                            <div class=stepper_container_class.clone()>
                                <WizardStepperWithClickable
                                    steps=steps
                                    current=current
                                    orientation=StepperOrientation::Horizontal
                                    clickable=clickable_steps
                                    on_step_click=on_step_click.clone()
                                    icon_type=steps_icon_type
                                    line_weight=steps_line_weight
                                    label_placement=steps_label_placement
                                    responsive=steps_responsive
                                    icon_size_px=steps_icon_size
                                    title_font_size_px=steps_title_font_size
                                />
                            </div>
                        </Show>

                        <Show when=move || show_header>
                            <WizardHeader
                                current_step_name=current_step_name
                                current_step=Signal::derive(move || current.get() + 1)
                                total_steps=total_steps
                                show_progress=show_progress
                                progress_percentage=progress_percentage
                                prefix=header_prefix.clone()
                            />
                        </Show>

                        <WizardErrorDisplay messages=errors />

                        <WizardContent prefix=content_prefix>
                            {children()}
                        </WizardContent>

                        <WizardNavigationWithCallbacks
                            can_go_back=can_go_back
                            can_go_forward=can_go_forward
                            is_last_step=is_last_step
                            is_loading=is_loading
                            is_submitted=is_submitted
                            on_previous=on_previous.clone()
                            on_next=on_next.clone()
                            on_submit=on_submit.clone()
                        />
                    </div>
                </div>
            }.into_any()
        }
    }
}

/// Helper component to handle optional on_step_click callback.
/// Uses the Steps component from the navigation module for a consistent UI.
#[component]
fn WizardStepperWithClickable<T: WizardStage + 'static>(
    steps: Signal<Vec<T>>,
    current: Signal<usize>,
    orientation: StepperOrientation,
    clickable: bool,
    on_step_click: Option<Callback<usize>>,
    #[prop(optional)]
    icon_type: Option<StepsIconType>,
    #[prop(optional)]
    line_weight: Option<StepsLineWeight>,
    #[prop(optional)]
    label_placement: Option<StepsLabelPlacement>,
    #[prop(optional)]
    responsive: Option<StepsResponsive>,
    #[prop(optional)]
    icon_size_px: u32,
    #[prop(optional)]
    title_font_size_px: u32,
) -> impl IntoView {
    // Convert StepperOrientation to StepsDirection
    let direction = match orientation {
        StepperOrientation::Horizontal => StepsDirection::Horizontal,
        StepperOrientation::Vertical => StepsDirection::Vertical,
    };

    // Build StepItem vec from WizardStage items
    // Don't pre-set status - let Steps component derive from current signal
    let step_items = Signal::derive({
        let steps = steps.clone();
        move || {
            steps.get()
                .iter()
                .map(|stage| StepItem::new(stage.display_name()))
                .collect::<Vec<_>>()
        }
    });

    let icon_type = icon_type.unwrap_or_default();
    let line_weight = line_weight.unwrap_or_default();
    let label_placement = label_placement.unwrap_or_default();
    let responsive = responsive.unwrap_or_default();

    if let Some(callback) = on_step_click {
        view! {
            <Steps
                items=step_items.get()
                current=current
                direction=direction
                clickable=clickable
                icon_type=icon_type
                line_weight=line_weight
                label_placement=label_placement
                responsive=responsive
                icon_size_px=icon_size_px
                title_font_size_px=title_font_size_px
                on_change=callback
            />
        }.into_any()
    } else {
        view! {
            <Steps
                items=step_items.get()
                current=current
                direction=direction
                clickable=clickable
                icon_type=icon_type
                line_weight=line_weight
                label_placement=label_placement
                responsive=responsive
                icon_size_px=icon_size_px
                title_font_size_px=title_font_size_px
            />
        }.into_any()
    }
}

/// Helper component to handle optional navigation callbacks.
#[component]
fn WizardNavigationWithCallbacks(
    can_go_back: Signal<bool>,
    can_go_forward: Signal<bool>,
    is_last_step: Signal<bool>,
    is_loading: Signal<bool>,
    is_submitted: Signal<bool>,
    on_previous: Option<Callback<()>>,
    on_next: Option<Callback<()>>,
    on_submit: Option<Callback<()>>,
) -> impl IntoView {
    match (on_previous, on_next, on_submit) {
        (Some(prev), Some(next), Some(submit)) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_previous=prev
                    on_next=next
                    on_submit=submit
                />
            }.into_any()
        }
        (Some(prev), Some(next), None) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_previous=prev
                    on_next=next
                />
            }.into_any()
        }
        (Some(prev), None, Some(submit)) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_previous=prev
                    on_submit=submit
                />
            }.into_any()
        }
        (None, Some(next), Some(submit)) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_next=next
                    on_submit=submit
                />
            }.into_any()
        }
        (Some(prev), None, None) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_previous=prev
                />
            }.into_any()
        }
        (None, Some(next), None) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_next=next
                />
            }.into_any()
        }
        (None, None, Some(submit)) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                    on_submit=submit
                />
            }.into_any()
        }
        (None, None, None) => {
            view! {
                <WizardNavigation
                    can_go_back=can_go_back
                    can_go_forward=can_go_forward
                    is_last_step=is_last_step
                    is_loading=is_loading
                    is_submitted=is_submitted
                />
            }.into_any()
        }
    }
}

/// Internal wizard header component.
#[component]
fn WizardHeader(
    current_step_name: Signal<String>,
    current_step: Signal<usize>,
    total_steps: Signal<usize>,
    show_progress: bool,
    progress_percentage: Signal<u32>,
    prefix: String,
) -> impl IntoView {
    let header_class = format!("{}-header", prefix);
    let info_class = format!("{}-header-info", prefix);
    let name_class = format!("{}-current-step-name", prefix);
    let counter_class = format!("{}-step-counter", prefix);

    view! {
        <div class=header_class>
            <div class=info_class>
                <span class=name_class>
                    {move || current_step_name.get()}
                </span>
                <span class=counter_class>
                    {move || format!("Step {} of {}", current_step.get(), total_steps.get())}
                </span>
            </div>

            <Show when=move || show_progress>
                <WizardProgress
                    percentage=progress_percentage
                    show_percentage=false
                />
            </Show>
        </div>
    }
}

/// Wizard content wrapper component.
#[component]
pub fn WizardContent(
    prefix: String,
    children: Children,
) -> impl IntoView {
    let content_class = format!("{}-content", prefix);
    let inner_class = format!("{}-content-inner", prefix);

    view! {
        <div class=content_class>
            <div class=inner_class>
                {children()}
            </div>
        </div>
    }
}

/// Individual wizard step content wrapper.
///
/// Use this to wrap content for each step.
#[component]
pub fn WizardStep<T: WizardStage>(
    /// The stage this content is for.
    #[prop(into)]
    stage: T,
    /// Step content.
    children: ChildrenFn,
    /// Additional classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-step", design_system);
    let step_class = format!("{} {}", prefix, class.unwrap_or_default());

    view! {
        <div
            class=step_class
            data-stage=stage.display_name()
        >
            {children()}
        </div>
    }
}
