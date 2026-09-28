//! Steps Leptos component.
//!
//! Step indicator for multi-step processes with support for horizontal,
//! vertical, and navigation layouts. Integrates with theme system.

use leptos::prelude::*;
use super::types::{StepsDirection, StepsType, StepStatus, StepItem, StepsSize, StepsLabelPlacement, StepsIconType, StepsLineWeight, StepsResponsive};
use crate::try_use_theme;

/// SVG icons for step statuses.
mod icons {
    use leptos::prelude::*;

    pub fn check_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" style="width: 14px; height: 14px;">
                <polyline points="20 6 9 17 4 12"/>
            </svg>
        }
    }

    pub fn close_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="width: 14px; height: 14px;">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
        }
    }
}

/// Get icon styles based on status and icon type.
fn get_icon_styles(status: StepStatus, icon_type: StepsIconType) -> (&'static str, &'static str, &'static str, u8, &'static str) {
    // Returns (background, border_color, text_color, border_width, box_shadow)
    match icon_type {
        StepsIconType::Default => {
            match status {
                StepStatus::Finish | StepStatus::Process => (
                    "var(--fx-color-primary, #1677ff)",
                    "var(--fx-color-primary, #1677ff)",
                    "#fff",
                    0,
                    "none",
                ),
                StepStatus::Error => (
                    "var(--fx-color-error, #ff4d4f)",
                    "var(--fx-color-error, #ff4d4f)",
                    "#fff",
                    0,
                    "none",
                ),
                StepStatus::Wait => (
                    "rgba(255,255,255,0.08)",
                    "rgba(255,255,255,0.25)",
                    "rgba(255,255,255,0.45)",
                    1,
                    "none",
                ),
            }
        },
        StepsIconType::Outlined => {
            match status {
                StepStatus::Finish => (
                    "var(--fx-color-primary, #1677ff)",
                    "var(--fx-color-primary, #1677ff)",
                    "#fff",
                    0,
                    "none",
                ),
                StepStatus::Process => (
                    "transparent",
                    "var(--fx-color-primary, #1677ff)",
                    "var(--fx-color-primary, #1677ff)",
                    2,
                    "0 0 0 4px rgba(22, 119, 255, 0.15)",
                ),
                StepStatus::Error => (
                    "transparent",
                    "var(--fx-color-error, #ff4d4f)",
                    "var(--fx-color-error, #ff4d4f)",
                    2,
                    "none",
                ),
                StepStatus::Wait => (
                    "rgba(255,255,255,0.08)",
                    "rgba(255,255,255,0.25)",
                    "rgba(255,255,255,0.45)",
                    1,
                    "none",
                ),
            }
        },
        StepsIconType::Dot => {
            match status {
                StepStatus::Finish => (
                    "var(--fx-color-primary, #1677ff)",
                    "var(--fx-color-primary, #1677ff)",
                    "#fff",
                    0,
                    "none",
                ),
                StepStatus::Process => (
                    "transparent",
                    "var(--fx-color-primary, #1677ff)",
                    "var(--fx-color-primary, #1677ff)",
                    2,
                    "0 0 0 4px rgba(22, 119, 255, 0.15)",
                ),
                StepStatus::Error => (
                    "transparent",
                    "var(--fx-color-error, #ff4d4f)",
                    "var(--fx-color-error, #ff4d4f)",
                    2,
                    "none",
                ),
                StepStatus::Wait => (
                    "rgba(255,255,255,0.08)",
                    "rgba(255,255,255,0.25)",
                    "rgba(255,255,255,0.45)",
                    1,
                    "none",
                ),
            }
        },
    }
}

/// Steps component.
#[component]
pub fn Steps(
    /// Current step index (0-indexed).
    #[prop(into)]
    current: Signal<usize>,
    /// Step items.
    #[prop(into)]
    items: Vec<StepItem>,
    /// Steps direction.
    #[prop(optional, into)]
    direction: Option<StepsDirection>,
    /// Steps type/style.
    #[prop(optional, into)]
    steps_type: Option<StepsType>,
    /// Status of current step.
    #[prop(optional, into)]
    status: Option<StepStatus>,
    /// Steps size.
    #[prop(optional, into)]
    size: Option<StepsSize>,
    /// Label placement.
    #[prop(optional, into)]
    label_placement: Option<StepsLabelPlacement>,
    /// Icon type/style variant.
    #[prop(optional, into)]
    icon_type: Option<StepsIconType>,
    /// Connector line weight.
    #[prop(optional, into)]
    line_weight: Option<StepsLineWeight>,
    /// Mobile responsive behavior.
    #[prop(optional, into)]
    responsive: Option<StepsResponsive>,
    /// Custom icon size in pixels (overrides size preset). Use 0 for default.
    #[prop(optional)]
    icon_size_px: u32,
    /// Custom title font size in pixels. Use 0 for default (14px).
    #[prop(optional)]
    title_font_size_px: u32,
    /// Allow clicking on steps.
    #[prop(optional)]
    clickable: bool,
    /// Show progress dot instead of number/icon.
    #[prop(optional)]
    progress_dot: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Step change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<usize>>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let direction = direction.unwrap_or_default();
    let steps_type = steps_type.unwrap_or_default();
    let current_status = status.unwrap_or(StepStatus::Process);
    let size = size.unwrap_or_default();
    let label_placement = label_placement.unwrap_or_default();
    let icon_type = icon_type.unwrap_or_default();
    let line_weight = line_weight.unwrap_or_default();
    let responsive = responsive.unwrap_or_default();

    let steps_prefix = format!("fx-steps-{}", design_system);
    let direction_class = direction.class(&steps_prefix);
    let type_class = steps_type.class(&steps_prefix);

    let combined_class = {
        let mut parts = vec![steps_prefix.clone(), direction_class, type_class];
        if size == StepsSize::Small {
            parts.push(format!("{}-small", steps_prefix));
        }
        if progress_dot {
            parts.push(format!("{}-dot", steps_prefix));
        }
        if label_placement == StepsLabelPlacement::Vertical {
            parts.push(format!("{}-label-vertical", steps_prefix));
        }
        // Add responsive class
        parts.push(responsive.class(&steps_prefix));
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let total = items.len();
    let is_horizontal = direction == StepsDirection::Horizontal;
    let is_label_vertical = label_placement == StepsLabelPlacement::Vertical;
    // Use custom sizes if provided (non-zero), otherwise use defaults
    let icon_size = if icon_size_px > 0 { icon_size_px } else if size == StepsSize::Small { 24 } else { 32 };
    let title_font_size = if title_font_size_px > 0 { title_font_size_px } else { 14 };
    let icon_font_size = if icon_size <= 24 { 11 } else if icon_size <= 28 { 12 } else { 14 };
    let line_thickness = line_weight.as_px();

    view! {
        <div class=combined_class role="navigation" aria-label="Steps">
            {items.into_iter().enumerate().map(|(idx, item)| {
                let is_last = idx == total - 1;
                let steps_prefix = steps_prefix.clone();
                let on_change = on_change.clone();
                let custom_icon = item.icon.clone();

                let step_status = move || {
                    if let Some(s) = item.status {
                        s
                    } else {
                        let curr = current.get();
                        if idx < curr {
                            StepStatus::Finish
                        } else if idx == curr {
                            current_status
                        } else {
                            StepStatus::Wait
                        }
                    }
                };

                if is_label_vertical {
                    view! {
                        <div
                            class=move || {
                                let mut cls = vec![format!("{}-item", steps_prefix)];
                                cls.push(step_status().class(&steps_prefix));
                                if item.disabled { cls.push(format!("{}-item-disabled", steps_prefix)); }
                                if clickable && !item.disabled { cls.push(format!("{}-item-clickable", steps_prefix)); }
                                cls.join(" ")
                            }
                            on:click=move |_| {
                                if clickable && !item.disabled {
                                    if let Some(ref cb) = on_change { cb.run(idx); }
                                }
                            }
                            style=move || {
                                if clickable && !item.disabled {
                                    "display: flex; flex-direction: column; align-items: center; flex: 1; cursor: pointer;"
                                } else {
                                    "display: flex; flex-direction: column; align-items: center; flex: 1;"
                                }
                            }
                            role="listitem"
                        >
                            <div style="display: flex; align-items: center; width: 100%;">
                                {if idx > 0 && is_horizontal {
                                    view! {
                                        <span style=move || {
                                            let color = if idx <= current.get() {
                                                "var(--fx-color-primary, #1677ff)"
                                            } else {
                                                "var(--fx-color-border, rgba(255,255,255,0.25))"
                                            };
                                            format!("flex: 1; height: {}px; background: {};", line_thickness, color)
                                        }/>
                                    }.into_any()
                                } else if is_horizontal {
                                    view! { <span style="flex: 1;"/> }.into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}

                                <div
                                    class=format!("{}-item-icon", steps_prefix)
                                    style=move || {
                                        let status = step_status();
                                        let (bg, border, text_color, border_width, box_shadow) = get_icon_styles(status, icon_type);
                                        format!(
                                            "width: {}px; height: {}px; min-width: {}px; background: {}; border: {}px solid {}; color: {}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: {}px; font-weight: 500; flex-shrink: 0; transition: all 0.3s; position: relative; box-shadow: {};",
                                            icon_size, icon_size, icon_size, bg, border_width, border, text_color,
                                            icon_font_size,
                                            box_shadow
                                        )
                                    }
                                >
                                    {move || {
                                        let status = step_status();
                                        if progress_dot {
                                            let dot_size = if status == StepStatus::Process { 10 } else { 8 };
                                            let color = match status {
                                                StepStatus::Finish | StepStatus::Process => "var(--fx-color-primary, #1677ff)",
                                                StepStatus::Error => "var(--fx-color-error, #ff4d4f)",
                                                StepStatus::Wait => "var(--fx-color-border, #d9d9d9)",
                                            };
                                            view! { <span style=format!("width: {}px; height: {}px; background: {}; border-radius: 50%; display: block;", dot_size, dot_size, color)/> }.into_any()
                                        } else if let Some(ref icon) = custom_icon {
                                            view! { <span inner_html=icon.clone() /> }.into_any()
                                        } else {
                                            match status {
                                                StepStatus::Finish => view! { <span style="display: flex; align-items: center; justify-content: center;">{icons::check_icon()}</span> }.into_any(),
                                                StepStatus::Error => view! { <span style="display: flex; align-items: center; justify-content: center;">{icons::close_icon()}</span> }.into_any(),
                                                StepStatus::Process if icon_type == StepsIconType::Dot => view! {
                                                    <span>{idx + 1}</span>
                                                    <span style="position: absolute; bottom: -2px; left: 50%; transform: translateX(-50%); width: 6px; height: 6px; background: var(--fx-color-primary, #1677ff); border-radius: 50%;"/>
                                                }.into_any(),
                                                _ => view! { <span>{idx + 1}</span> }.into_any(),
                                            }
                                        }
                                    }}
                                </div>

                                {if !is_last && is_horizontal {
                                    view! {
                                        <span style=move || {
                                            let color = if step_status() == StepStatus::Finish {
                                                "var(--fx-color-primary, #1677ff)"
                                            } else {
                                                "var(--fx-color-border, rgba(255,255,255,0.25))"
                                            };
                                            format!("flex: 1; height: {}px; background: {};", line_thickness, color)
                                        }/>
                                    }.into_any()
                                } else if is_horizontal {
                                    view! { <span style="flex: 1;"/> }.into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}
                            </div>

                            <div
                                class=format!("{}-item-title", steps_prefix)
                                style=move || {
                                    let text_color = match step_status() {
                                        StepStatus::Error => "var(--fx-color-error, #ff4d4f)",
                                        StepStatus::Wait => "var(--fx-color-text-quaternary, rgba(255,255,255,0.45))",
                                        _ => "var(--fx-color-primary, #1677ff)",
                                    };
                                    format!("margin-top: 8px; font-weight: 500; font-size: {}px; text-align: center; color: {};", title_font_size, text_color)
                                }
                            >
                                {item.title.clone()}
                            </div>

                            {item.description.clone().map(|d| view! {
                                <div class=format!("{}-item-description", steps_prefix) style="font-size: 12px; color: var(--fx-color-text-secondary, rgba(255,255,255,0.65)); margin-top: 4px; text-align: center;">
                                    {d}
                                </div>
                            })}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div
                            class=move || {
                                let mut cls = vec![format!("{}-item", steps_prefix)];
                                cls.push(step_status().class(&steps_prefix));
                                if item.disabled { cls.push(format!("{}-item-disabled", steps_prefix)); }
                                if clickable && !item.disabled { cls.push(format!("{}-item-clickable", steps_prefix)); }
                                cls.join(" ")
                            }
                            on:click=move |_| {
                                if clickable && !item.disabled {
                                    if let Some(ref cb) = on_change { cb.run(idx); }
                                }
                            }
                            style=move || {
                                if clickable && !item.disabled {
                                    "display: flex; align-items: flex-start; gap: 8px; cursor: pointer;"
                                } else {
                                    "display: flex; align-items: flex-start; gap: 8px;"
                                }
                            }
                            role="listitem"
                        >
                            <div
                                class=format!("{}-item-icon", steps_prefix)
                                style=move || {
                                    let status = step_status();
                                    let (bg, border, text_color, border_width, box_shadow) = get_icon_styles(status, icon_type);
                                    format!(
                                        "width: {}px; height: {}px; min-width: {}px; background: {}; border: {}px solid {}; color: {}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: {}px; font-weight: 500; flex-shrink: 0; transition: all 0.3s; position: relative; box-shadow: {};",
                                        icon_size, icon_size, icon_size, bg, border_width, border, text_color,
                                        icon_font_size,
                                        box_shadow
                                    )
                                }
                            >
                                {move || {
                                    let status = step_status();
                                    if progress_dot {
                                        let dot_size = if status == StepStatus::Process { 10 } else { 8 };
                                        let color = match status {
                                            StepStatus::Finish | StepStatus::Process => "var(--fx-color-primary, #1677ff)",
                                            StepStatus::Error => "var(--fx-color-error, #ff4d4f)",
                                            StepStatus::Wait => "var(--fx-color-border, #d9d9d9)",
                                        };
                                        view! { <span style=format!("width: {}px; height: {}px; background: {}; border-radius: 50%; display: block;", dot_size, dot_size, color)/> }.into_any()
                                    } else if let Some(ref icon) = custom_icon {
                                        view! { <span inner_html=icon.clone() /> }.into_any()
                                    } else {
                                        match status {
                                            StepStatus::Finish => view! { <span style="display: flex; align-items: center; justify-content: center;">{icons::check_icon()}</span> }.into_any(),
                                            StepStatus::Error => view! { <span style="display: flex; align-items: center; justify-content: center;">{icons::close_icon()}</span> }.into_any(),
                                            StepStatus::Process if icon_type == StepsIconType::Dot => view! {
                                                <span>{idx + 1}</span>
                                                <span style="position: absolute; bottom: -2px; left: 50%; transform: translateX(-50%); width: 6px; height: 6px; background: var(--fx-color-primary, #1677ff); border-radius: 50%;"/>
                                            }.into_any(),
                                            _ => view! { <span>{idx + 1}</span> }.into_any(),
                                        }
                                    }
                                }}
                            </div>

                            <div
                                class=format!("{}-item-content", steps_prefix)
                                style=move || {
                                    let text_color = match step_status() {
                                        StepStatus::Error => "var(--fx-color-error, #ff4d4f)",
                                        StepStatus::Wait => "var(--fx-color-text-quaternary, rgba(255,255,255,0.45))",
                                        _ => "var(--fx-color-text, rgba(255,255,255,0.88))",
                                    };
                                    format!("color: {}; flex: 1; min-width: 0;", text_color)
                                }
                            >
                                <div class=format!("{}-item-title", steps_prefix) style=format!("display: flex; align-items: center; white-space: nowrap;")>
                                    <span style=format!("font-weight: 500; font-size: {}px;", title_font_size)>{item.title.clone()}</span>
                                    {if !is_last && is_horizontal {
                                        view! {
                                            <span class=format!("{}-item-tail", steps_prefix) style=move || {
                                                let color = if step_status() == StepStatus::Finish {
                                                    "var(--fx-color-primary, #1677ff)"
                                                } else {
                                                    "var(--fx-color-text-quaternary, rgba(255,255,255,0.25))"
                                                };
                                                format!("position: static; flex: 1; height: {}px; background: {}; margin: 0 12px; min-width: 32px;", line_thickness, color)
                                            }/>
                                        }.into_any()
                                    } else {
                                        view! { <span/> }.into_any()
                                    }}
                                </div>
                                {item.description.clone().map(|d| view! {
                                    <div class=format!("{}-item-description", steps_prefix) style="font-size: 12px; color: var(--fx-color-text-secondary, rgba(255,255,255,0.65)); margin-top: 4px;">
                                        {d}
                                    </div>
                                })}
                            </div>
                        </div>
                    }.into_any()
                }
            }).collect_view()}
        </div>
    }
}

/// Individual Step component for composition pattern.
#[component]
pub fn Step(
    #[prop(into)] title: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional, into)] status: Option<StepStatus>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] step_number: Option<usize>,
    #[prop(optional, into)] size: Option<StepsSize>,
    #[prop(optional, into)] icon_type: Option<StepsIconType>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx.map(|ctx| ctx.class_prefix()).unwrap_or("ant");

    let status = status.unwrap_or(StepStatus::Wait);
    let size = size.unwrap_or_default();
    let icon_type = icon_type.unwrap_or_default();
    let step_number = step_number.unwrap_or(1);

    let steps_prefix = format!("fx-steps-{}", design_system);
    let icon_size = if size == StepsSize::Small { 24 } else { 32 };

    let combined_class = {
        let mut parts = vec![format!("{}-item", steps_prefix), status.class(&steps_prefix)];
        if disabled { parts.push(format!("{}-item-disabled", steps_prefix)); }
        if let Some(ref custom) = class { parts.push(custom.clone()); }
        parts.join(" ")
    };

    let (bg, border, text_color, border_width, box_shadow) = get_icon_styles(status, icon_type);

    let title_color = match status {
        StepStatus::Error => "var(--fx-color-error, #ff4d4f)",
        StepStatus::Wait => "var(--fx-color-text-quaternary, rgba(255,255,255,0.45))",
        _ => "var(--fx-color-text, rgba(255,255,255,0.88))",
    };

    view! {
        <div class=combined_class style="display: flex; align-items: flex-start; gap: 12px;">
            <div
                class=format!("{}-item-icon", steps_prefix)
                style=format!(
                    "width: {}px; height: {}px; background: {}; border: {}px solid {}; color: {}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: {}px; font-weight: 500; flex-shrink: 0; position: relative; box-shadow: {};",
                    icon_size, icon_size, bg, border_width, border, text_color,
                    if size == StepsSize::Small { 12 } else { 14 },
                    box_shadow
                )
            >
                {if let Some(ref custom_icon) = icon {
                    view! { <span inner_html=custom_icon.clone() /> }.into_any()
                } else {
                    match status {
                        StepStatus::Finish => view! { <span style="display: flex; align-items: center; justify-content: center;">{icons::check_icon()}</span> }.into_any(),
                        StepStatus::Error => view! { <span style="display: flex; align-items: center; justify-content: center;">{icons::close_icon()}</span> }.into_any(),
                        StepStatus::Process if icon_type == StepsIconType::Dot => view! {
                            <span>{step_number}</span>
                            <span style="position: absolute; bottom: -2px; left: 50%; transform: translateX(-50%); width: 6px; height: 6px; background: var(--fx-color-primary, #1677ff); border-radius: 50%;"/>
                        }.into_any(),
                        _ => view! { <span>{step_number}</span> }.into_any(),
                    }
                }}
            </div>
            <div class=format!("{}-item-content", steps_prefix) style=format!("color: {};", title_color)>
                <div class=format!("{}-item-title", steps_prefix) style="font-weight: 500; line-height: 32px; font-size: 14px;">
                    {title}
                    {subtitle.map(|s| view! {
                        <span class=format!("{}-item-subtitle", steps_prefix) style="margin-left: 8px; font-weight: 400; font-size: 12px; color: var(--fx-color-text-secondary, rgba(255,255,255,0.65));">
                            {s}
                        </span>
                    })}
                </div>
                {description.map(|d| view! {
                    <div class=format!("{}-item-description", steps_prefix) style="font-size: 12px; color: var(--fx-color-text-secondary, rgba(255,255,255,0.65)); margin-top: 4px;">
                        {d}
                    </div>
                })}
            </div>
        </div>
    }
}
