//! GuideStepper component - wizard-like workflow with numbered steps.

use leptos::prelude::*;
use super::types::{GuideStep, StepperOrientation, StepperHeaderPosition, StepStatus, StepLayout, ControlsPosition, StepperHeaderLayout};
use super::guide_step::ImagePosition;
use super::markdown_text::MarkdownText;
use crate::try_use_theme;

/// Internal component for paginated step navigation.
/// Isolated to prevent reactive loop issues.
#[component]
fn PaginatedStepNav(
    /// All steps
    steps: StoredValue<Vec<GuideStep>>,
    /// Total step count
    total: usize,
    /// Max visible steps per page
    max_visible: usize,
    /// Active step signal
    active_step: RwSignal<usize>,
    /// CSS prefix
    prefix: StoredValue<String>,
    /// Whether steps are clickable
    clickable: bool,
    /// Linear mode
    linear: bool,
    /// Show titles
    show_titles: bool,
    /// Show separators
    show_separator: bool,
    /// Callback when step is clicked
    on_step_click: Callback<usize>,
) -> impl IntoView {
    // Pagination state - local to this component
    let current_page = RwSignal::new(0usize);
    let needs_pagination = total > max_visible;
    let total_pages = if needs_pagination { (total + max_visible - 1) / max_visible } else { 1 };
    let max_page = if total_pages > 0 { total_pages - 1 } else { 0 };

    // Auto-scroll to keep active step visible
    Effect::new(move |_| {
        if needs_pagination {
            let active = active_step.get();
            let page = current_page.get();
            let start = page * max_visible;
            let end = ((page + 1) * max_visible).min(total);
            if active < start || active >= end {
                let new_page = active / max_visible;
                current_page.set(new_page);
            }
        }
    });

    // Helper closures
    let is_step_active = move |index: usize| active_step.get() == index;
    let is_step_completed = move |index: usize| index < active_step.get();
    let is_step_disabled = move |index: usize| linear && index > active_step.get();

    let get_step_status = move |index: usize| -> StepStatus {
        if is_step_active(index) {
            StepStatus::Active
        } else if is_step_completed(index) {
            StepStatus::Completed
        } else {
            StepStatus::Pending
        }
    };

    // Pre-compute the step items as a static list to avoid reactive closure issues
    let p = prefix.get_value();
    let p_nav = p.clone();
    let p_list = p.clone();
    let p_prev = p.clone();
    let p_next = p.clone();

    view! {
        // Previous page arrow - always rendered, hidden via CSS class when not needed
        <button
            type="button"
            class=move || {
                let mut c = format!("{}-stepper-page-btn {}-stepper-page-prev", p_prev, p_prev);
                if !needs_pagination {
                    c.push_str(&format!(" {}-stepper-page-btn-hidden", p_prev));
                } else if current_page.get() == 0 {
                    c.push_str(&format!(" {}-stepper-page-btn-disabled", p_prev));
                }
                c
            }
            disabled=move || !needs_pagination || (current_page.get() == 0)
            on:click=move |_| {
                let page = current_page.get();
                if page > 0 {
                    current_page.set(page - 1);
                }
            }
            aria-label="Previous steps"
        >
            "‹"
        </button>

        // Step list - reactive based on current_page
        <nav class=format!("{}-stepper-nav", p_nav)>
            <ul class=format!("{}-stepper-nav-list", p_list)>
                {move || {
                    let page = current_page.get();
                    let start = if needs_pagination { page * max_visible } else { 0 };
                    let end = if needs_pagination { ((page + 1) * max_visible).min(total) } else { total };
                    let p = prefix.get_value();

                    (start..end).map(|i| {
                        let step = steps.get_value().get(i).cloned();
                        let is_last_visible = i == end - 1;

                        if let Some(step) = step {
                            let p_class = p.clone();
                            let p_action = p.clone();
                            let p_number = p.clone();
                            let p_check = p.clone();
                            let p_title = p.clone();
                            let p_sep = p.clone();
                            let step_title = step.title.clone();
                            let can_click = clickable && (!linear || i <= active_step.get_untracked());

                            view! {
                                <li
                                    class=move || {
                                        let mut classes = vec![format!("{}-stepper-item", p_class)];
                                        match get_step_status(i) {
                                            StepStatus::Active => classes.push(format!("{}-stepper-item-active", p_class)),
                                            StepStatus::Completed => classes.push(format!("{}-stepper-item-completed", p_class)),
                                            StepStatus::Pending => classes.push(format!("{}-stepper-item-pending", p_class)),
                                            StepStatus::Error => classes.push(format!("{}-stepper-item-error", p_class)),
                                        }
                                        if is_step_disabled(i) {
                                            classes.push(format!("{}-stepper-item-disabled", p_class));
                                        }
                                        classes.join(" ")
                                    }
                                    role="presentation"
                                    data-step=i
                                    aria-current=move || if is_step_active(i) { Some("step") } else { None }
                                >
                                    <button
                                        type="button"
                                        class=format!("{}-stepper-action", p_action)
                                        role="tab"
                                        tabindex=move || if is_step_disabled(i) { -1 } else { 0 }
                                        disabled=move || is_step_disabled(i)
                                        on:click=move |_| {
                                            if can_click {
                                                on_step_click.run(i);
                                            }
                                        }
                                        aria-selected=move || is_step_active(i)
                                    >
                                        <span class=format!("{}-stepper-number", p_number)>
                                            {move || {
                                                if is_step_completed(i) {
                                                    view! { <span class=format!("{}-stepper-check", p_check)>"✓"</span> }.into_any()
                                                } else {
                                                    view! { <span>{i + 1}</span> }.into_any()
                                                }
                                            }}
                                        </span>
                                        {show_titles.then(|| {
                                            view! {
                                                <span class=format!("{}-stepper-title", p_title)>
                                                    {step_title.clone()}
                                                </span>
                                            }
                                        })}
                                    </button>

                                    // Separator
                                    {(show_separator && !is_last_visible).then(|| {
                                        let p_sep2 = p_sep.clone();
                                        view! {
                                            <span
                                                class=move || {
                                                    let mut c = format!("{}-stepper-separator", p_sep2);
                                                    if is_step_completed(i) {
                                                        c.push_str(&format!(" {}-stepper-separator-completed", p_sep2));
                                                    }
                                                    c
                                                }
                                                aria-hidden="true"
                                            />
                                        }
                                    })}
                                </li>
                            }.into_any()
                        } else {
                            view! { <li></li> }.into_any()
                        }
                    }).collect_view()
                }}
            </ul>
        </nav>

        // Next page arrow - always rendered, hidden via CSS class when not needed
        <button
            type="button"
            class=move || {
                let mut c = format!("{}-stepper-page-btn {}-stepper-page-next", p_next, p_next);
                if !needs_pagination {
                    c.push_str(&format!(" {}-stepper-page-btn-hidden", p_next));
                } else if current_page.get() >= max_page {
                    c.push_str(&format!(" {}-stepper-page-btn-disabled", p_next));
                }
                c
            }
            disabled=move || !needs_pagination || (current_page.get() >= max_page)
            on:click=move |_| {
                let page = current_page.get();
                if page < max_page {
                    current_page.set(page + 1);
                }
            }
            aria-label="Next steps"
        >
            "›"
        </button>
    }
}

/// GuideStepper - A wizard-like workflow component with numbered progression.
///
/// Supports both horizontal and vertical orientations, with mobile-first
/// responsive design that automatically adapts to screen size.
///
/// # Features
///
/// - Horizontal and vertical layouts
/// - Linear mode (sequential) or free navigation
/// - Customizable header position (top, right, bottom, left)
/// - Step completion tracking
/// - Responsive design (mobile-first)
/// - Keyboard navigation
/// - Theme-aware styling
///
/// # Example
///
/// ```ignore
/// let steps = vec![
///     GuideStep::new("1", "Account", "Create your account"),
///     GuideStep::new("2", "Profile", "Complete your profile"),
///     GuideStep::new("3", "Settings", "Configure settings"),
/// ];
///
/// let active = RwSignal::new(0usize);
///
/// view! {
///     <GuideStepper
///         steps=steps
///         active_step=active
///         orientation=StepperOrientation::Horizontal
///     />
/// }
/// ```
#[component]
pub fn GuideStepper(
    /// The steps to display.
    #[prop(into)]
    steps: Vec<GuideStep>,
    /// Signal controlling the active step index (0-based).
    #[prop(into)]
    active_step: RwSignal<usize>,
    /// Stepper orientation.
    #[prop(optional)]
    orientation: Option<StepperOrientation>,
    /// Header position relative to step number (horizontal only).
    #[prop(optional)]
    header_position: Option<StepperHeaderPosition>,
    /// Linear mode - steps must be completed in order.
    #[prop(optional)]
    #[prop(default = false)]
    linear: bool,
    /// Show separator lines between steps.
    #[prop(optional)]
    #[prop(default = true)]
    show_separator: bool,
    /// Show navigation controls (Back/Next buttons).
    #[prop(optional)]
    #[prop(default = true)]
    show_controls: bool,
    /// Allow clicking on step headers to navigate (disabled in linear mode).
    #[prop(optional)]
    #[prop(default = true)]
    clickable: bool,
    /// Show step titles in navigation (set to false for number-only display).
    #[prop(optional)]
    #[prop(default = true)]
    show_titles: bool,
    /// Image position in content panel (left or right).
    #[prop(optional)]
    image_position: Option<ImagePosition>,
    /// Layout mode for step content (side-by-side or stacked).
    #[prop(optional)]
    layout: Option<StepLayout>,
    /// Position of navigation controls (Bottom or TopRight).
    #[prop(optional)]
    controls_position: Option<ControlsPosition>,
    /// Header layout mode (Standard or Split). Split mode places steps left, controls right.
    #[prop(optional)]
    header_layout: Option<StepperHeaderLayout>,
    /// Maximum number of visible steps before pagination kicks in. Default: 10.
    #[prop(optional)]
    #[prop(default = 10)]
    max_visible_steps: usize,
    /// Callback when step changes.
    #[prop(optional, into)]
    on_step_change: Option<Callback<usize>>,
    /// Callback when all steps are completed.
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// Custom content renderer for each step.
    #[prop(optional)]
    children: Option<Children>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));
    let orientation = orientation.unwrap_or_default();
    let header_position = header_position.unwrap_or_default();
    let total = steps.len();
    let steps = StoredValue::new(steps);

    let layout = layout.unwrap_or_default();
    let controls_position = controls_position.unwrap_or_default();
    let header_layout = header_layout.unwrap_or_default();

    // Build combined class
    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![
            format!("{}-stepper", p),
            orientation.class(&p),
            layout.class(&p),
            controls_position.class(&p),
        ];
        if orientation == StepperOrientation::Horizontal {
            parts.push(header_position.class(&p));
            parts.push(header_layout.class(&p));
        }
        if linear {
            parts.push(format!("{}-stepper-linear", p));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Check if step is active
    let is_step_active = move |index: usize| active_step.get() == index;

    // Check if step is completed (before active)
    let is_step_completed = move |index: usize| index < active_step.get();

    // Check if step is disabled (in linear mode, future steps)
    let is_step_disabled = move |index: usize| linear && index > active_step.get();

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
        let on_complete = on_complete.clone();
        move |_| {
            let current = active_step.get();
            if current < total - 1 {
                let new_step = current + 1;
                active_step.set(new_step);
                if let Some(ref cb) = on_step_change {
                    cb.run(new_step);
                }
            } else {
                // Last step - trigger complete
                if let Some(ref cb) = on_complete {
                    cb.run(());
                }
            }
        }
    };

    let go_to_step = {
        let on_step_change = on_step_change.clone();
        move |index: usize| {
            if !linear || index <= active_step.get() {
                active_step.set(index);
                if let Some(ref cb) = on_step_change {
                    cb.run(index);
                }
            }
        }
    };

    // Get step status
    let get_step_status = move |index: usize| -> StepStatus {
        if is_step_active(index) {
            StepStatus::Active
        } else if is_step_completed(index) {
            StepStatus::Completed
        } else {
            StepStatus::Pending
        }
    };

    // Create signal for current step
    let current_signal = Signal::derive(move || active_step.get());

    match orientation {
        StepperOrientation::Horizontal => {
            // Clone handlers for use in closures
            let go_prev_for_header = go_prev.clone();
            let go_next_for_header = go_next.clone();

            // Helper to render step item
            let render_step_item = move |i: usize, step: &GuideStep, is_last_visible: bool| {
                let p = prefix.get_value();
                let p_class = p.clone();
                let p_action = p.clone();
                let p_number = p.clone();
                let p_check = p.clone();
                let p_title = p.clone();
                let p_sep = p.clone();
                let step_title = step.title.clone();
                let go_to = go_to_step.clone();
                let can_click = clickable && (!linear || i <= active_step.get());

                view! {
                    <li
                        class=move || {
                            let mut classes = vec![format!("{}-stepper-item", p_class)];
                            match get_step_status(i) {
                                StepStatus::Active => classes.push(format!("{}-stepper-item-active", p_class)),
                                StepStatus::Completed => classes.push(format!("{}-stepper-item-completed", p_class)),
                                StepStatus::Pending => classes.push(format!("{}-stepper-item-pending", p_class)),
                                StepStatus::Error => classes.push(format!("{}-stepper-item-error", p_class)),
                            }
                            if is_step_disabled(i) {
                                classes.push(format!("{}-stepper-item-disabled", p_class));
                            }
                            classes.join(" ")
                        }
                        role="presentation"
                        data-step=i
                        aria-current=move || if is_step_active(i) { Some("step") } else { None }
                    >
                        <button
                            type="button"
                            class=format!("{}-stepper-action", p_action)
                            role="tab"
                            tabindex=move || if is_step_disabled(i) { -1 } else { 0 }
                            disabled=move || is_step_disabled(i)
                            on:click=move |_| {
                                if can_click {
                                    go_to(i);
                                }
                            }
                            aria-selected=move || is_step_active(i)
                        >
                            <span class=format!("{}-stepper-number", p_number)>
                                {move || {
                                    if is_step_completed(i) {
                                        view! { <span class=format!("{}-stepper-check", p_check)>"✓"</span> }.into_any()
                                    } else {
                                        view! { <span>{i + 1}</span> }.into_any()
                                    }
                                }}
                            </span>
                            {show_titles.then(|| {
                                view! {
                                    <span class=format!("{}-stepper-title", p_title)>
                                        {step_title.clone()}
                                    </span>
                                }
                            })}
                        </button>

                        // Separator (not on last visible item)
                        {(show_separator && !is_last_visible).then(|| {
                            let p_sep2 = p_sep.clone();
                            view! {
                                <span
                                    class=move || {
                                        let mut c = format!("{}-stepper-separator", p_sep2);
                                        if is_step_completed(i) {
                                            c.push_str(&format!(" {}-stepper-separator-completed", p_sep2));
                                        }
                                        c
                                    }
                                    aria-hidden="true"
                                />
                            }
                        })}
                    </li>
                }
            };

            // Decide which header to render based on layout
            if header_layout == StepperHeaderLayout::Split {
                // Split layout: steps left with pagination, controls right
                view! {
                    <div
                        class=combined_class.clone()
                        role="tablist"
                        aria-orientation="horizontal"
                    >
                        // Unified split header row
                        <div class=format!("{}-stepper-header-split", prefix.get_value())>
                            // Left section: paginated step navigation
                            <div class=format!("{}-stepper-header-left", prefix.get_value())>
                                <PaginatedStepNav
                                    steps=steps
                                    total=total
                                    max_visible=max_visible_steps
                                    active_step=active_step
                                    prefix=prefix
                                    clickable=clickable
                                    linear=linear
                                    show_titles=show_titles
                                    show_separator=show_separator
                                    on_step_click=Callback::new(move |i| go_to_step(i))
                                />
                            </div>

                            // Right section: Back/Next controls
                            {show_controls.then(|| {
                                let p = prefix.get_value();
                                let p_for_back = p.clone();
                                let p_for_next = p.clone();
                                let go_prev = go_prev_for_header.clone();
                                let go_next = go_next_for_header.clone();

                                view! {
                                    <div class=format!("{}-stepper-header-right", p)>
                                        // Back button
                                        {move || {
                                            let is_first = active_step.get() == 0;
                                            let p2 = p_for_back.clone();
                                            let go_prev = go_prev.clone();
                                            (!is_first).then(|| {
                                                view! {
                                                    <button
                                                        type="button"
                                                        class=format!("{}-stepper-btn {}-stepper-btn-prev", p2, p2)
                                                        on:click=move |_| go_prev(())
                                                    >
                                                        <span class=format!("{}-stepper-btn-icon", p2)>"←"</span>
                                                        <span>"Back"</span>
                                                    </button>
                                                }
                                            })
                                        }}

                                        // Next/Complete button
                                        {move || {
                                            let current = active_step.get();
                                            let is_last = current >= total - 1;
                                            let p3 = p_for_next.clone();
                                            let go_next = go_next.clone();

                                            if is_last {
                                                view! {
                                                    <button
                                                        type="button"
                                                        class=format!("{}-stepper-btn {}-stepper-btn-complete", p3, p3)
                                                        on:click=move |_| go_next(())
                                                    >
                                                        <span>"Complete"</span>
                                                        <span class=format!("{}-stepper-btn-icon", p3)>"✓"</span>
                                                    </button>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <button
                                                        type="button"
                                                        class=format!("{}-stepper-btn {}-stepper-btn-next", p3, p3)
                                                        on:click=move |_| go_next(())
                                                    >
                                                        <span>"Next"</span>
                                                        <span class=format!("{}-stepper-btn-icon", p3)>"→"</span>
                                                    </button>
                                                }.into_any()
                                            }
                                        }}
                                    </div>
                                }
                            })}
                        </div>

                        // Content panel (no controls - they're in the header now)
                        <div class=format!("{}-stepper-panels", prefix.get_value())>
                            {move || {
                                let current = active_step.get();
                                let p = prefix.get_value();

                                steps.get_value().get(current).map(|step| {
                                    let title = step.title.clone();
                                    let title_for_indicator = title.clone();
                                    let description = step.description.clone();
                                    let image = step.image.clone();
                                    let p2 = p.clone();
                                    let p3 = p.clone();

                                    view! {
                                        <div
                                            class=format!("{}-stepper-panel", p)
                                            role="tabpanel"
                                            data-step=current
                                        >
                                            <div class=format!("{}-stepper-content", p3)>
                                                <div class=format!("{}-stepper-step-indicator", p3)>
                                                    {format!("Step {}: {}", current + 1, title_for_indicator)}
                                                </div>

                                                <div class=format!("{}-stepper-content-body", p2)>
                                                    {image.map(|img_url| {
                                                        let p_media = p2.clone();
                                                        view! {
                                                            <div class=format!("{}-stepper-media", p_media)>
                                                                <img
                                                                    src=img_url
                                                                    alt=title.clone()
                                                                    class=format!("{}-stepper-image", p_media)
                                                                    loading="lazy"
                                                                />
                                                            </div>
                                                        }
                                                    })}

                                                    <div class=format!("{}-stepper-description-box", p3)>
                                                        <MarkdownText
                                                            text=description
                                                            class=format!("{}-stepper-content-description", p3)
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })
                            }}
                        </div>
                    </div>
                }.into_any()
            } else {
                // Standard layout: existing behavior
                view! {
                    <div
                        class=combined_class.clone()
                        role="tablist"
                        aria-orientation="horizontal"
                    >
                        // Step headers (navigation)
                        <nav class=format!("{}-stepper-nav", prefix.get_value())>
                            <ul class=format!("{}-stepper-nav-list", prefix.get_value())>
                                {steps.get_value().into_iter().enumerate().map(|(i, step)| {
                                    let is_last = i == total - 1;
                                    render_step_item(i, &step, is_last)
                                }).collect_view()}
                            </ul>
                        </nav>

                        // Content panel
                        <div class=format!("{}-stepper-panels", prefix.get_value())>
                            {move || {
                                let current = active_step.get();
                                let p = prefix.get_value();
                                let is_first = current == 0;
                                let is_last = current >= total - 1;
                                let go_prev = go_prev.clone();
                                let go_next = go_next.clone();

                                steps.get_value().get(current).map(|step| {
                                    let title = step.title.clone();
                                    let title_for_indicator = title.clone();
                                    let description = step.description.clone();
                                    let image = step.image.clone();
                                    let p2 = p.clone();
                                    let p3 = p.clone();
                                    let p4 = p.clone();
                                    let go_prev = go_prev.clone();
                                    let go_next = go_next.clone();

                                    view! {
                                        <div
                                            class=format!("{}-stepper-panel", p)
                                            role="tabpanel"
                                            data-step=current
                                        >
                                            // Content container holds everything
                                            <div class=format!("{}-stepper-content", p3)>
                                                // Step indicator shows "Step x: Title"
                                                <div class=format!("{}-stepper-step-indicator", p3)>
                                                    {format!("Step {}: {}", current + 1, title_for_indicator)}
                                                </div>

                                                // Body container: image (left) + description (right)
                                                <div class=format!("{}-stepper-content-body", p2)>
                                                    // Image section (if present) - now inside content
                                                    {image.map(|img_url| {
                                                        let p_media = p2.clone();
                                                        view! {
                                                            <div class=format!("{}-stepper-media", p_media)>
                                                                <img
                                                                    src=img_url
                                                                    alt=title.clone()
                                                                    class=format!("{}-stepper-image", p_media)
                                                                    loading="lazy"
                                                                />
                                                            </div>
                                                        }
                                                    })}

                                                    // Description box (right of image)
                                                    <div class=format!("{}-stepper-description-box", p3)>
                                                        <MarkdownText
                                                            text=description
                                                            class=format!("{}-stepper-content-description", p3)
                                                        />
                                                    </div>
                                                </div>

                                                // Navigation controls inside content
                                                {show_controls.then(|| {
                                                    let p5 = p4.clone();
                                                    let go_prev = go_prev.clone();
                                                    let go_next = go_next.clone();

                                                    view! {
                                                        <div class=format!("{}-stepper-controls", p5)>
                                                            // Back button
                                                            <div class=format!("{}-stepper-controls-left", p5)>
                                                                {(!is_first).then(|| {
                                                                    let go_prev = go_prev.clone();
                                                                    let p6 = p5.clone();
                                                                    view! {
                                                                        <button
                                                                            type="button"
                                                                            class=format!("{}-stepper-btn {}-stepper-btn-prev", p6, p6)
                                                                            on:click=move |_| go_prev(())
                                                                        >
                                                                            <span class=format!("{}-stepper-btn-icon", p6)>"←"</span>
                                                                            <span>"Back"</span>
                                                                        </button>
                                                                    }
                                                                })}
                                                            </div>

                                                            // Next/Complete button
                                                            <div class=format!("{}-stepper-controls-right", p5)>
                                                                {
                                                                    let go_next = go_next.clone();
                                                                    let p7 = p5.clone();
                                                                    if is_last {
                                                                        view! {
                                                                            <button
                                                                                type="button"
                                                                                class=format!("{}-stepper-btn {}-stepper-btn-complete", p7, p7)
                                                                                on:click=move |_| go_next(())
                                                                            >
                                                                                <span>"Complete"</span>
                                                                                <span class=format!("{}-stepper-btn-icon", p7)>"✓"</span>
                                                                            </button>
                                                                        }.into_any()
                                                                    } else {
                                                                        view! {
                                                                            <button
                                                                                type="button"
                                                                                class=format!("{}-stepper-btn {}-stepper-btn-next", p7, p7)
                                                                                on:click=move |_| go_next(())
                                                                            >
                                                                                <span>"Next"</span>
                                                                                <span class=format!("{}-stepper-btn-icon", p7)>"→"</span>
                                                                            </button>
                                                                        }.into_any()
                                                                    }
                                                                }
                                                            </div>
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        </div>
                                    }
                                })
                            }}
                        </div>
                    </div>
                }.into_any()
            }
        }

        StepperOrientation::Vertical => {
            view! {
                <div
                    class=combined_class.clone()
                    role="tablist"
                    aria-orientation="vertical"
                >
                    {steps.get_value().into_iter().enumerate().map(|(i, step)| {
                        let p = prefix.get_value();
                        let p_panel = p.clone();
                        let p_header = p.clone();
                        let p_action = p.clone();
                        let p_number = p.clone();
                        let p_check = p.clone();
                        let p_title = p.clone();
                        let p_toggle = p.clone();
                        let p_sep = p.clone();
                        let p_content = p.clone();
                        let p_content2 = p.clone();
                        let p_desc = p.clone();
                        let p_controls = p.clone();
                        let step_title = step.title.clone();
                        let step_description = step.description.clone();
                        let go_to = go_to_step.clone();
                        let go_prev = go_prev.clone();
                        let go_next = go_next.clone();
                        let can_click = clickable && (!linear || i <= active_step.get());

                        view! {
                            <div
                                class=move || {
                                    let mut classes = vec![format!("{}-stepper-panel-vertical", p_panel)];
                                    match get_step_status(i) {
                                        StepStatus::Active => classes.push(format!("{}-stepper-panel-active", p_panel)),
                                        StepStatus::Completed => classes.push(format!("{}-stepper-panel-completed", p_panel)),
                                        StepStatus::Pending => classes.push(format!("{}-stepper-panel-pending", p_panel)),
                                        StepStatus::Error => classes.push(format!("{}-stepper-panel-error", p_panel)),
                                    }
                                    classes.join(" ")
                                }
                                data-step=i
                                aria-current=move || if is_step_active(i) { Some("step") } else { None }
                            >
                                // Header
                                <div class=format!("{}-stepper-header-vertical", p_header)>
                                    <button
                                        type="button"
                                        class=format!("{}-stepper-action", p_action)
                                        role="tab"
                                        tabindex=move || if is_step_disabled(i) { -1 } else { 0 }
                                        disabled=move || is_step_disabled(i)
                                        on:click=move |_| {
                                            if can_click {
                                                go_to(i);
                                            }
                                        }
                                        aria-selected=move || is_step_active(i)
                                        aria-expanded=move || is_step_active(i)
                                    >
                                        <span
                                            class=move || {
                                                let mut c = format!("{}-stepper-number", p_number);
                                                if is_step_active(i) {
                                                    c.push_str(&format!(" {}-stepper-number-active", p_number));
                                                } else if is_step_completed(i) {
                                                    c.push_str(&format!(" {}-stepper-number-completed", p_number));
                                                }
                                                c
                                            }
                                        >
                                            {move || {
                                                if is_step_completed(i) {
                                                    view! { <span class=format!("{}-stepper-check", p_check)>"✓"</span> }.into_any()
                                                } else {
                                                    view! { <span>{i + 1}</span> }.into_any()
                                                }
                                            }}
                                        </span>
                                        <span class=format!("{}-stepper-title", p_title)>
                                            {step_title.clone()}
                                        </span>
                                    </button>
                                </div>

                                // Toggleable content (only visible when active)
                                <div
                                    class=move || {
                                        let mut c = format!("{}-stepper-toggleable", p_toggle);
                                        if is_step_active(i) {
                                            c.push_str(&format!(" {}-stepper-toggleable-open", p_toggle));
                                        }
                                        c
                                    }
                                >
                                    // Vertical separator (connector line)
                                    {(show_separator && i < total - 1).then(|| {
                                        let p_sep2 = p_sep.clone();
                                        view! {
                                            <div
                                                class=move || {
                                                    let mut c = format!("{}-stepper-separator-vertical", p_sep2);
                                                    if is_step_completed(i) {
                                                        c.push_str(&format!(" {}-stepper-separator-completed", p_sep2));
                                                    }
                                                    c
                                                }
                                                aria-hidden="true"
                                            />
                                        }
                                    })}

                                    // Content
                                    <Show when=move || is_step_active(i)>
                                        <div class=format!("{}-stepper-content-vertical", p_content)>
                                            <div class=format!("{}-stepper-content", p_content2)>
                                                <MarkdownText
                                                    text=step_description.clone()
                                                    class=format!("{}-stepper-content-description", p_desc)
                                                />
                                            </div>

                                            // Controls inside content
                                            {show_controls.then(|| {
                                                let p2 = p_controls.clone();
                                                let is_first = i == 0;
                                                let is_last = i >= total - 1;
                                                let go_prev = go_prev.clone();
                                                let go_next = go_next.clone();

                                                view! {
                                                    <div class=format!("{}-stepper-controls-vertical", p2)>
                                                        // Back button
                                                        {(!is_first).then(|| {
                                                            let p3 = p2.clone();
                                                            let go_prev = go_prev.clone();
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class=format!("{}-stepper-btn {}-stepper-btn-prev", p3, p3)
                                                                    on:click=move |_| go_prev(())
                                                                >
                                                                    <span class=format!("{}-stepper-btn-icon", p3)>"←"</span>
                                                                    <span>"Back"</span>
                                                                </button>
                                                            }
                                                        })}

                                                        // Next/Complete button
                                                        {
                                                            let p4 = p2.clone();
                                                            let go_next = go_next.clone();
                                                            if is_last {
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        class=format!("{}-stepper-btn {}-stepper-btn-complete", p4, p4)
                                                                        on:click=move |_| go_next(())
                                                                    >
                                                                        <span>"Complete"</span>
                                                                        <span class=format!("{}-stepper-btn-icon", p4)>"✓"</span>
                                                                    </button>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        class=format!("{}-stepper-btn {}-stepper-btn-next", p4, p4)
                                                                        on:click=move |_| go_next(())
                                                                    >
                                                                        <span>"Next"</span>
                                                                        <span class=format!("{}-stepper-btn-icon", p4)>"→"</span>
                                                                    </button>
                                                                }.into_any()
                                                            }
                                                        }
                                                    </div>
                                                }
                                            })}
                                        </div>
                                    </Show>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            }.into_any()
        }
    }
}
