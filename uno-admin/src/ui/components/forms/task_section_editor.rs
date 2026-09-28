//! Task Section Editor Component
//!
//! A specialized editor for Task sections that displays sections in a repeater-style
//! list with collapsible accordions for viewing details, and uses a modal for adding new sections.
//! Each task section contains: status, cover images/videos, difficulty, duration, earnings tiers, and requirements.
//!
//! Also provides TaskContentEditor - a complete task form component that mirrors the Guide form layout:
//! - Title field (required)
//! - Description field with rich text (required)
//! - Sections with Add Section button

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use crate::components::common::icon::{Icon, IconName};
use crate::components::forms::ValidationError;
use super::gcs_media_upload::GcsMediaUploadList;

/// Data structure for an earnings tier
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EarningsTierData {
    pub name: String,
    pub min_earnings: f64,
    pub max_earnings: f64,
    pub period: String,
    pub features: Vec<String>,
    pub is_popular: bool,
}

impl EarningsTierData {
    pub fn from_json(value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            name: obj.and_then(|o| o.get("name")).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            min_earnings: obj.and_then(|o| o.get("min_earnings")).and_then(|v| v.as_f64()).unwrap_or(0.0),
            max_earnings: obj.and_then(|o| o.get("max_earnings")).and_then(|v| v.as_f64()).unwrap_or(0.0),
            period: obj.and_then(|o| o.get("period")).and_then(|v| v.as_str()).unwrap_or("month").to_string(),
            features: obj.and_then(|o| o.get("features"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            is_popular: obj.and_then(|o| o.get("is_popular")).and_then(|v| v.as_bool()).unwrap_or(false),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "min_earnings": self.min_earnings,
            "max_earnings": self.max_earnings,
            "period": self.period,
            "features": self.features,
            "is_popular": self.is_popular
        })
    }
}

/// Data structure for a task section
#[derive(Clone, Debug, PartialEq)]
pub struct TaskSectionData {
    pub title: String,
    pub description: String,
    pub task_status: String,
    pub task_type: String,  // "active" or "passive"
    pub cover_images: Vec<String>,
    pub cover_videos: Vec<String>,
    pub difficulty: String,
    pub duration: String,
    pub earnings_tiers: Vec<EarningsTierData>,
    pub requirements: Vec<String>,
    pub is_visible: bool,
}

impl Default for TaskSectionData {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            task_status: String::new(),
            task_type: "active".to_string(),
            cover_images: Vec::new(),
            cover_videos: Vec::new(),
            difficulty: String::new(),
            duration: String::new(),
            earnings_tiers: Vec::new(),
            requirements: Vec::new(),
            is_visible: true,
        }
    }
}

impl TaskSectionData {
    pub fn from_json(value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            title: obj.and_then(|o| o.get("title")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            description: obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            task_status: obj.and_then(|o| o.get("task_status").or_else(|| o.get("status")))
                .and_then(|v| v.as_str()).unwrap_or("active").to_string(),
            task_type: obj.and_then(|o| o.get("task_type"))
                .and_then(|v| v.as_str()).unwrap_or("active").to_string(),
            cover_images: obj.and_then(|o| o.get("cover_images"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_else(|| {
                    // Fallback to single image field
                    obj.and_then(|o| o.get("image").or_else(|| o.get("cover_image")))
                        .and_then(|v| v.as_str())
                        .map(|s| vec![s.to_string()])
                        .unwrap_or_default()
                }),
            cover_videos: obj.and_then(|o| o.get("cover_videos"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            difficulty: obj.and_then(|o| o.get("difficulty")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            duration: obj.and_then(|o| o.get("duration")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            earnings_tiers: obj.and_then(|o| o.get("earnings_tiers").or_else(|| o.get("earnings")))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(EarningsTierData::from_json).collect())
                .unwrap_or_default(),
            requirements: obj.and_then(|o| o.get("requirements"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            is_visible: obj.and_then(|o| o.get("is_visible"))
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "title": self.title,
            "description": self.description,
            "task_status": self.task_status,
            "task_type": self.task_type,
            "cover_images": self.cover_images,
            "cover_videos": self.cover_videos,
            "difficulty": self.difficulty,
            "duration": self.duration,
            "earnings_tiers": self.earnings_tiers.iter().map(|t| t.to_json()).collect::<Vec<_>>(),
            "requirements": self.requirements,
            "is_visible": self.is_visible
        })
    }

    /// Get display title for the section
    pub fn display_title(&self) -> String {
        if !self.title.is_empty() {
            self.title.clone()
        } else {
            let status = match self.task_status.as_str() {
                "active" => "Active",
                "on_demand" => "On Demand",
                "coming_soon" => "Coming Soon",
                "deprecated" => "Deprecated",
                _ => &self.task_status,
            };
            format!("Task - {}", status)
        }
    }
}

/// Task Section Editor Component - mimics the repeater field style
#[component]
pub fn TaskSectionEditor(
    /// Current sections value
    value: RwSignal<Vec<serde_json::Value>>,
    /// Label for the field
    #[prop(default = "Task Sections".to_string())]
    label: String,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<serde_json::Value>>>,
    /// Whether the editor is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    let sections = Signal::derive(move || {
        value.get().iter().map(TaskSectionData::from_json).collect::<Vec<_>>()
    });

    let expanded_sections = RwSignal::new(std::collections::HashSet::<usize>::new());
    let show_modal = RwSignal::new(false);
    let modal_section = RwSignal::new(TaskSectionData::default());
    let editing_index = RwSignal::new(None::<usize>);

    let toggle_section = move |index: usize| {
        expanded_sections.update(|set| {
            if set.contains(&index) {
                set.remove(&index);
            } else {
                set.insert(index);
            }
        });
    };

    let open_new_section_modal = move |_| {
        editing_index.set(None);
        modal_section.set(TaskSectionData {
            task_status: "active".to_string(),
            ..Default::default()
        });
        show_modal.set(true);
    };

    let open_edit_section_modal = move |index: usize| {
        let secs = value.get();
        if let Some(section_json) = secs.get(index) {
            editing_index.set(Some(index));
            modal_section.set(TaskSectionData::from_json(section_json));
            show_modal.set(true);
        }
    };

    let save_section = move |_| {
        let section = modal_section.get();
        let mut current = value.get();

        if let Some(idx) = editing_index.get() {
            if idx < current.len() {
                current[idx] = section.to_json();
            }
        } else {
            current.push(section.to_json());
        }

        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
        editing_index.set(None);
        show_modal.set(false);
    };

    let delete_section = move |index: usize| {
        let mut current = value.get();
        if index < current.len() {
            current.remove(index);
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    let move_up = move |index: usize| {
        if index > 0 {
            let mut current = value.get();
            current.swap(index, index - 1);
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    let move_down = move |index: usize| {
        let mut current = value.get();
        if index < current.len() - 1 {
            current.swap(index, index + 1);
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    view! {
        <div class="task-section-editor" style="margin-bottom: 1.5rem;">
            <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                {label}
                {move || if required {
                    view! { <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </label>

            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                {move || {
                    let secs = sections.get();
                    let total = secs.len();
                    secs.iter().enumerate().map(|(index, section)| {
                        let section = section.clone();
                        let section_title = section.display_title();
                        let is_expanded = move || expanded_sections.get().contains(&index);

                        view! {
                            <div style="border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                                <div
                                    style="display: flex; align-items: center; padding: 0.75rem 1rem; background: #1e293b;"
                                >
                                    // Toggle expand/collapse button (left side)
                                    <button
                                        type="button"
                                        style="background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; margin-right: 0.75rem;"
                                        title="Toggle details"
                                        on:click=move |_| toggle_section(index)
                                    >
                                        {move || if is_expanded() {
                                            view! { <Icon name=IconName::Minus size=14 /> }.into_any()
                                        } else {
                                            view! { <Icon name=IconName::Plus size=14 /> }.into_any()
                                        }}
                                    </button>

                                    // Title
                                    <div
                                        style="flex: 1; color: #e2e8f0; font-size: 0.875rem; font-weight: 500; cursor: pointer;"
                                        on:click=move |_| toggle_section(index)
                                    >
                                        {section_title.clone()}
                                    </div>

                                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                                        {if !read_only {
                                            view! {
                                                // Edit button (blue)
                                                <button
                                                    type="button"
                                                    style="background: #3b82f6; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    title="Edit section"
                                                    on:click=move |_| open_edit_section_modal(index)
                                                >
                                                    <Icon name=IconName::Edit size=14 />
                                                </button>

                                                // Move up button
                                                <button
                                                    type="button"
                                                    style={if index > 0 {
                                                        "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    } else {
                                                        "background: #374151; border: none; color: #64748b; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    }}
                                                    title="Move up"
                                                    disabled={index == 0}
                                                    on:click=move |_| { if index > 0 { move_up(index) } }
                                                >
                                                    <Icon name=IconName::ChevronUp size=14 />
                                                </button>

                                                // Move down button
                                                <button
                                                    type="button"
                                                    style={if index < total - 1 {
                                                        "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    } else {
                                                        "background: #374151; border: none; color: #64748b; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    }}
                                                    title="Move down"
                                                    disabled={index >= total - 1}
                                                    on:click=move |_| { if index < total - 1 { move_down(index) } }
                                                >
                                                    <Icon name=IconName::ChevronDown size=14 />
                                                </button>

                                                // Delete button (red)
                                                <button
                                                    type="button"
                                                    style="background: #ef4444; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    title="Delete section"
                                                    on:click=move |_| delete_section(index)
                                                >
                                                    <Icon name=IconName::Trash size=14 />
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                </div>

                                {move || if is_expanded() {
                                    let section = section.clone();
                                    view! {
                                        <div style="padding: 1rem; background: #0f172a; border-top: 1px solid #334155;">
                                            // Status and Type badges
                                            <div style="margin-bottom: 1rem; display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                                <span style={format!("display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; {}",
                                                    match section.task_status.as_str() {
                                                        "active" => "background: rgba(34, 197, 94, 0.2); color: #22c55e;",
                                                        "coming_soon" => "background: rgba(59, 130, 246, 0.2); color: #3b82f6;",
                                                        "deprecated" => "background: rgba(107, 114, 128, 0.2); color: #6b7280;",
                                                        _ => "background: rgba(148, 163, 184, 0.2); color: #94a3b8;",
                                                    }
                                                )}>
                                                    {match section.task_status.as_str() {
                                                        "active" => "Active",
                                                        "coming_soon" => "Coming Soon",
                                                        "deprecated" => "Deprecated",
                                                        _ => &section.task_status,
                                                    }}
                                                </span>
                                                <span style={format!("display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; {}",
                                                    match section.task_type.as_str() {
                                                        "passive" => "background: rgba(168, 85, 247, 0.2); color: #a855f7;",
                                                        _ => "background: rgba(251, 191, 36, 0.2); color: #fbbf24;",
                                                    }
                                                )}>
                                                    {match section.task_type.as_str() {
                                                        "passive" => "Passive",
                                                        _ => "Active",
                                                    }}
                                                </span>
                                            </div>

                                            // Cover media
                                            {if !section.cover_images.is_empty() || !section.cover_videos.is_empty() {
                                                let total_media = section.cover_images.len() + section.cover_videos.len();
                                                view! {
                                                    <div style="margin-bottom: 1rem;">
                                                        <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">
                                                            {format!("Cover Media ({})", total_media)}
                                                        </div>
                                                        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                                            {section.cover_images.iter().map(|img| {
                                                                let img = img.clone();
                                                                view! {
                                                                    <div style="width: 48px; height: 48px; border-radius: 0.25rem; overflow: hidden; border: 1px solid #334155;">
                                                                        <img src=img style="width: 100%; height: 100%; object-fit: cover;" />
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                            {section.cover_videos.iter().map(|_| {
                                                                view! {
                                                                    <div style="width: 48px; height: 48px; border-radius: 0.25rem; overflow: hidden; border: 1px solid #334155; background: #334155; display: flex; align-items: center; justify-content: center;">
                                                                        <Icon name=IconName::Document size=20 />
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}

                                            // Difficulty and Duration row
                                            <div style="display: flex; gap: 2rem; margin-bottom: 1rem;">
                                                {if !section.difficulty.is_empty() {
                                                    view! {
                                                        <div>
                                                            <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.25rem;">"Difficulty"</div>
                                                            <div style="color: #e2e8f0; font-size: 0.875rem; text-transform: capitalize;">{section.difficulty.clone()}</div>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}

                                                {if !section.duration.is_empty() {
                                                    view! {
                                                        <div>
                                                            <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.25rem;">"Duration"</div>
                                                            <div style="color: #e2e8f0; font-size: 0.875rem;">{section.duration.clone()}</div>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>

                                            // Earnings Tiers
                                            {if !section.earnings_tiers.is_empty() {
                                                view! {
                                                    <div style="margin-bottom: 1rem;">
                                                        <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">
                                                            {format!("Earnings Tiers ({})", section.earnings_tiers.len())}
                                                        </div>
                                                        <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                                                            {section.earnings_tiers.iter().map(|tier| {
                                                                let tier_name = tier.name.clone();
                                                                let tier_range = format!("${:.2} - ${:.2}/{}", tier.min_earnings, tier.max_earnings, tier.period);
                                                                view! {
                                                                    <div style="padding: 0.5rem 0.75rem; background: #1e293b; border-radius: 0.25rem; border: 1px solid #334155;">
                                                                        <div style="color: #e2e8f0; font-size: 0.875rem; font-weight: 500;">{tier_name}</div>
                                                                        <div style="color: #64748b; font-size: 0.75rem;">{tier_range}</div>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}

                                            // Requirements
                                            {if !section.requirements.is_empty() {
                                                view! {
                                                    <div>
                                                        <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">
                                                            {format!("Requirements ({})", section.requirements.len())}
                                                        </div>
                                                        <div style="display: flex; flex-wrap: wrap; gap: 0.25rem;">
                                                            {section.requirements.iter().map(|req| {
                                                                let req = req.clone();
                                                                view! {
                                                                    <span style="display: inline-block; padding: 0.25rem 0.5rem; background: #334155; border-radius: 0.25rem; font-size: 0.75rem; color: #94a3b8;">
                                                                        {req}
                                                                    </span>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }
                    }).collect_view()
                }}

                // Add Section button
                {if !read_only {
                    view! {
                        <button
                            type="button"
                            style="width: 100%; padding: 0.75rem 1rem; background: #334155; color: #94a3b8; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.5rem;"
                            on:click=open_new_section_modal
                        >
                            <Icon name=IconName::Plus size=14 />
                            "Add Section"
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            <div style="margin-top: 0.5rem; font-size: 0.75rem; color: #64748b;">
                {move || {
                    let count = sections.get().len();
                    format!("{} item{}", count, if count == 1 { "" } else { "s" })
                }}
            </div>

            {move || if show_modal.get() {
                view! {
                    <TaskSectionModal
                        section=modal_section
                        is_editing=Signal::derive(move || editing_index.get().is_some())
                        on_save=save_section
                        on_cancel=move |_| {
                            editing_index.set(None);
                            show_modal.set(false);
                        }
                    />
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

/// Modal for adding or editing a task section
#[component]
fn TaskSectionModal(
    section: RwSignal<TaskSectionData>,
    is_editing: Signal<bool>,
    on_save: impl Fn(()) + 'static + Copy,
    on_cancel: impl Fn(()) + 'static + Copy,
) -> impl IntoView {
    // Track if tier form is expanded
    let show_tier_form = RwSignal::new(false);
    let new_tier = RwSignal::new(EarningsTierData::default());
    let new_requirement = RwSignal::new(String::new());
    let new_feature = RwSignal::new(String::new());

    let toggle_tier_form = move |_| {
        if !show_tier_form.get() {
            new_tier.set(EarningsTierData {
                period: "month".to_string(),
                ..Default::default()
            });
        }
        show_tier_form.update(|v| *v = !*v);
    };

    let add_tier = move |_| {
        let tier = new_tier.get();
        section.update(|sec| {
            sec.earnings_tiers.push(tier.clone());
        });
        show_tier_form.set(false);
        new_tier.set(EarningsTierData::default());
    };

    let delete_tier = move |index: usize| {
        section.update(|sec| {
            if index < sec.earnings_tiers.len() {
                sec.earnings_tiers.remove(index);
            }
        });
    };

    let add_requirement = move |_| {
        let req = new_requirement.get();
        if !req.trim().is_empty() {
            section.update(|sec| {
                sec.requirements.push(req.trim().to_string());
            });
            new_requirement.set(String::new());
        }
    };

    let delete_requirement = move |index: usize| {
        section.update(|sec| {
            if index < sec.requirements.len() {
                sec.requirements.remove(index);
            }
        });
    };

    view! {
        <div
            class="modal-overlay"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            on:click=move |_| on_cancel(())
        >
            <div
                class="modal-content"
                style="background: #1e293b; border-radius: 0.5rem; width: 90%; max-width: 700px; max-height: 90vh; overflow-y: auto; border: 1px solid #334155;"
                on:click=move |e| e.stop_propagation()
            >
                // Modal Header
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 1rem 1.5rem; border-bottom: 1px solid #334155;">
                    <h2 style="color: #e2e8f0; font-size: 1.125rem; font-weight: 600; margin: 0;">
                        {move || if is_editing.get() { "Edit Section" } else { "Add" }}
                    </h2>
                    <button
                        type="button"
                        style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 0.25rem;"
                        on:click=move |_| on_cancel(())
                    >
                        <Icon name=IconName::Close size=24 />
                    </button>
                </div>

                // Modal Body
                <div style="padding: 1.5rem;">
                    // Title (Required)
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Title"
                            <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                        </label>
                        <input
                            type="text"
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                            placeholder="e.g., Getting Started"
                            prop:value=move || section.get().title
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                section.update(|s| s.title = val);
                            }
                        />
                    </div>

                    // Description (rich text)
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Description"
                        </label>
                        <div style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                            <div style="display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #1e293b;">
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-weight: bold;" title="Bold">"B"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-style: italic;" title="Italic">"I"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="Link">"Link"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="List">"List"</button>
                            </div>
                            <textarea
                                style="width: 100%; background: transparent; border: none; padding: 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 100px; resize: vertical;"
                                placeholder="Brief description of this section..."
                                prop:value=move || section.get().description
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    section.update(|s| s.description = val);
                                }
                            />
                        </div>
                    </div>

                    // Difficulty and Duration row
                    <div style="display: flex; gap: 1rem; margin-bottom: 1.5rem;">
                        // Difficulty dropdown
                        <div style="flex: 1;">
                            <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                                "Difficulty"
                            </label>
                            <select
                                style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    section.update(|s| s.difficulty = val);
                                }
                            >
                                <option value="easy" selected=move || section.get().difficulty.is_empty() || section.get().difficulty == "easy">"Easy"</option>
                                <option value="medium" selected=move || section.get().difficulty == "medium">"Medium"</option>
                                <option value="hard" selected=move || section.get().difficulty == "hard">"Hard"</option>
                            </select>
                        </div>

                        // Duration dropdown
                        <div style="flex: 1;">
                            <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                                "Duration (minutes)"
                            </label>
                            <input
                                type="text"
                                style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="e.g., 5"
                                prop:value=move || section.get().duration
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    section.update(|s| s.duration = val);
                                }
                            />
                        </div>
                    </div>

                    // Cover Images with GCS upload
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Cover Images"
                        </label>
                        <GcsMediaUploadList
                            value=Signal::derive(move || section.get().cover_images)
                            on_change=Callback::new(move |images: Vec<String>| {
                                section.update(|s| s.cover_images = images);
                            })
                            content_type="task".to_string()
                            accept="image/*".to_string()
                            label="images".to_string()
                        />
                    </div>

                    // Task Status
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Task Status"
                            <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                        </label>
                        <select
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                section.update(|s| s.task_status = val);
                            }
                        >
                            <option value="active" selected=move || section.get().task_status.is_empty() || section.get().task_status == "active">"Active"</option>
                            <option value="on_demand" selected=move || section.get().task_status == "on_demand">"On Demand"</option>
                            <option value="coming_soon" selected=move || section.get().task_status == "coming_soon">"Coming Soon"</option>
                            <option value="deprecated" selected=move || section.get().task_status == "deprecated">"Deprecated"</option>
                        </select>
                    </div>

                    // Task Type
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Task Type"
                        </label>
                        <select
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                section.update(|s| s.task_type = val);
                            }
                        >
                            <option value="active" selected=move || section.get().task_type.is_empty() || section.get().task_type == "active">"Active"</option>
                            <option value="passive" selected=move || section.get().task_type == "passive">"Passive"</option>
                        </select>
                        <p style="color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;">
                            "Active tasks require user participation. Passive tasks run in the background."
                        </p>
                    </div>

                    // Earnings Tiers Section
                    <div style="margin-bottom: 1.5rem;">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
                            <label style="color: #94a3b8; font-size: 0.875rem; font-weight: 500;">
                                "Earnings Tiers"
                            </label>
                        </div>

                        // Add Tier Button (always visible)
                        <button
                            type="button"
                            style="width: 100%; padding: 0.75rem 1rem; background: #334155; color: #94a3b8; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.5rem; margin-bottom: 0.75rem;"
                            on:click=toggle_tier_form
                        >
                            <Icon name=IconName::Plus size=14 />
                            "Add Tier"
                        </button>

                        // Inline Tier Form
                        {move || if show_tier_form.get() {
                            view! {
                                <div style="border: 1px solid #3b82f6; border-radius: 0.375rem; padding: 1rem; margin-bottom: 0.75rem; background: #0f172a;">
                                    <div style="font-size: 0.875rem; color: #3b82f6; margin-bottom: 0.75rem; font-weight: 500;">
                                        "New Tier"
                                    </div>

                                    // Tier Name
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Tier Name" <span style="color: #ef4444;">"*"</span>
                                        </label>
                                        <input
                                            type="text"
                                            style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                            placeholder="e.g., Basic, Pro, Enterprise"
                                            prop:value=move || new_tier.get().name
                                            on:input=move |ev| {
                                                let val = event_target_value(&ev);
                                                new_tier.update(|t| t.name = val);
                                            }
                                        />
                                    </div>

                                    // Min and Max Earnings row
                                    <div style="display: flex; gap: 1rem; margin-bottom: 0.75rem;">
                                        <div style="flex: 1;">
                                            <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                                "Min Earnings ($)" <span style="color: #ef4444;">"*"</span>
                                            </label>
                                            <input
                                                type="number"
                                                min="0"
                                                step="0.01"
                                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                                prop:value=move || new_tier.get().min_earnings.to_string()
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                                    new_tier.update(|t| t.min_earnings = val);
                                                }
                                            />
                                        </div>
                                        <div style="flex: 1;">
                                            <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                                "Max Earnings ($)" <span style="color: #ef4444;">"*"</span>
                                            </label>
                                            <input
                                                type="number"
                                                min="0"
                                                step="0.01"
                                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                                prop:value=move || new_tier.get().max_earnings.to_string()
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                                    new_tier.update(|t| t.max_earnings = val);
                                                }
                                            />
                                        </div>
                                    </div>

                                    // Period dropdown
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Period"
                                        </label>
                                        <select
                                            style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                                            on:change=move |ev| {
                                                let val = event_target_value(&ev);
                                                new_tier.update(|t| t.period = val);
                                            }
                                        >
                                            <option value="hour" selected=move || new_tier.get().period == "hour">"Hour"</option>
                                            <option value="day" selected=move || new_tier.get().period == "day">"Day"</option>
                                            <option value="week" selected=move || new_tier.get().period == "week">"Week"</option>
                                            <option value="month" selected=move || new_tier.get().period == "month">"Month"</option>
                                        </select>
                                    </div>

                                    // Features list
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Features"
                                        </label>
                                        // Input row for adding features
                                        <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                                            <input
                                                type="text"
                                                style="flex: 1; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                                placeholder="Add a feature..."
                                                prop:value=move || new_feature.get()
                                                on:input=move |ev| {
                                                    new_feature.set(event_target_value(&ev));
                                                }
                                                on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                    if ev.key() == "Enter" {
                                                        ev.prevent_default();
                                                        let feat = new_feature.get();
                                                        if !feat.trim().is_empty() {
                                                            new_tier.update(|t| t.features.push(feat.trim().to_string()));
                                                            new_feature.set(String::new());
                                                        }
                                                    }
                                                }
                                            />
                                            <button
                                                type="button"
                                                style="background: #334155; color: #94a3b8; border: none; border-radius: 0.25rem; padding: 0.5rem 0.75rem; cursor: pointer; font-size: 0.75rem;"
                                                on:click=move |_| {
                                                    let feat = new_feature.get();
                                                    if !feat.trim().is_empty() {
                                                        new_tier.update(|t| t.features.push(feat.trim().to_string()));
                                                        new_feature.set(String::new());
                                                    }
                                                }
                                            >
                                                "Add"
                                            </button>
                                        </div>
                                        // Display existing features as tags
                                        {move || {
                                            let features = new_tier.get().features;
                                            if !features.is_empty() {
                                                view! {
                                                    <div style="display: flex; flex-wrap: wrap; gap: 0.375rem;">
                                                        {features.iter().enumerate().map(|(index, feature)| {
                                                            let feature = feature.clone();
                                                            view! {
                                                                <div style="display: flex; align-items: center; gap: 0.25rem; padding: 0.25rem 0.5rem; background: #334155; border-radius: 0.25rem; font-size: 0.75rem; color: #e2e8f0;">
                                                                    <span>{feature}</span>
                                                                    <button
                                                                        type="button"
                                                                        style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0; font-size: 0.875rem; line-height: 1;"
                                                                        on:click=move |_| {
                                                                            new_tier.update(|t| {
                                                                                if index < t.features.len() {
                                                                                    t.features.remove(index);
                                                                                }
                                                                            });
                                                                        }
                                                                    >
                                                                        "×"
                                                                    </button>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }}
                                    </div>

                                    // Popular toggle
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                            <input
                                                type="checkbox"
                                                style="width: 16px; height: 16px; cursor: pointer;"
                                                prop:checked=move || new_tier.get().is_popular
                                                on:change=move |ev| {
                                                    let checked = event_target::<HtmlInputElement>(&ev).checked();
                                                    new_tier.update(|t| t.is_popular = checked);
                                                }
                                            />
                                            <span style="color: #94a3b8; font-size: 0.875rem;">"Mark as Popular"</span>
                                        </label>
                                    </div>

                                    // Form Actions
                                    <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;">
                                        <button
                                            type="button"
                                            style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem;"
                                            on:click=move |_| show_tier_form.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            type="button"
                                            style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem;"
                                            on:click=add_tier
                                        >
                                            "Add Tier"
                                        </button>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Tiers List
                        <div style="border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                            {move || {
                                let sec = section.get();
                                if sec.earnings_tiers.is_empty() {
                                    view! {
                                        <div style="padding: 1rem; text-align: center; color: #64748b; font-size: 0.875rem;">
                                            "No tiers yet"
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            {sec.earnings_tiers.iter().enumerate().map(|(index, tier)| {
                                                let tier = tier.clone();
                                                view! {
                                                    <div style="display: flex; align-items: center; padding: 0.75rem; border-bottom: 1px solid #334155; background: #0f172a;">
                                                        <div style="flex: 1;">
                                                            <div style="color: #e2e8f0; font-size: 0.875rem; font-weight: 500;">
                                                                {tier.name.clone()}
                                                                {if tier.is_popular {
                                                                    view! {
                                                                        <span style="margin-left: 0.5rem; padding: 0.125rem 0.375rem; background: #f59e0b; color: #1e293b; font-size: 0.625rem; border-radius: 9999px; font-weight: 600;">"POPULAR"</span>
                                                                    }.into_any()
                                                                } else {
                                                                    view! { <span></span> }.into_any()
                                                                }}
                                                            </div>
                                                            <div style="color: #64748b; font-size: 0.75rem;">
                                                                {format!("${:.2} - ${:.2} / {}", tier.min_earnings, tier.max_earnings, tier.period)}
                                                            </div>
                                                        </div>
                                                        <button
                                                            type="button"
                                                            style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                                                            on:click=move |_| delete_tier(index)
                                                        >
                                                            <Icon name=IconName::Trash size=14 />
                                                        </button>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                        <div style="margin-top: 0.25rem; font-size: 0.75rem; color: #64748b;">
                            {move || format!("{} items (min: 1)", section.get().earnings_tiers.len())}
                        </div>
                    </div>

                    // Requirements Section
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Requirements"
                        </label>

                        // Add requirement input with button
                        <div style="display: flex; gap: 0.5rem; margin-bottom: 0.75rem;">
                            <input
                                type="text"
                                style="flex: 1; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="Add new item..."
                                prop:value=move || new_requirement.get()
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    new_requirement.set(val);
                                }
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        ev.prevent_default();
                                        add_requirement(());
                                    }
                                }
                            />
                            <button
                                type="button"
                                style="background: #3b82f6; color: white; border: none; border-radius: 0.375rem; padding: 0.625rem 1rem; cursor: pointer; font-size: 0.875rem; font-weight: 500;"
                                on:click=move |_| add_requirement(())
                            >
                                "Add"
                            </button>
                        </div>

                        // Requirements list
                        {move || {
                            let reqs = section.get().requirements;
                            if !reqs.is_empty() {
                                view! {
                                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                                        {reqs.iter().enumerate().map(|(index, req)| {
                                            let req = req.clone();
                                            view! {
                                                <div style="display: flex; align-items: center; gap: 0.25rem; padding: 0.375rem 0.5rem; background: #334155; border-radius: 0.25rem;">
                                                    <span style="color: #e2e8f0; font-size: 0.875rem;">{req}</span>
                                                    <button
                                                        type="button"
                                                        style="background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 0; display: flex; align-items: center;"
                                                        on:click=move |_| delete_requirement(index)
                                                    >
                                                        <Icon name=IconName::Close size=14 />
                                                    </button>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}

                        <div style="margin-top: 0.25rem; font-size: 0.75rem; color: #64748b;">
                            {move || format!("{} items", section.get().requirements.len())}
                        </div>
                    </div>

                    // Section is visible on page checkbox
                    <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.5rem; padding-top: 1rem; border-top: 1px solid #334155;">
                        <input
                            type="checkbox"
                            id="section-visible"
                            style="width: 16px; height: 16px; accent-color: #22c55e; cursor: pointer;"
                            prop:checked=move || section.get().is_visible
                            on:change=move |ev| {
                                let checked = event_target::<HtmlInputElement>(&ev).checked();
                                section.update(|s| s.is_visible = checked);
                            }
                        />
                        <label for="section-visible" style="color: #94a3b8; font-size: 0.875rem; cursor: pointer;">
                            "Section is visible on page"
                        </label>
                    </div>
                </div>

                // Modal Footer
                <div style="display: flex; justify-content: flex-end; gap: 0.75rem; padding: 1rem 1.5rem; border-top: 1px solid #334155;">
                    <button
                        type="button"
                        style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer;"
                        on:click=move |_| on_cancel(())
                    >
                        "Cancel"
                    </button>
                    <button
                        type="button"
                        style="background: #3b82f6; color: white; border: none; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer;"
                        on:click=move |_| on_save(())
                    >
                        {move || if is_editing.get() { "Save" } else { "Add" }}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Styling constants for the task content editor
const FORM_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem;";
const EDITOR_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 200px;";
const TOOLBAR_STYLE: &str = "display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #0f172a;";
const TOOLBAR_BTN_STYLE: &str = "padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;";

/// Task Content Editor Component
///
/// A complete form component for task content that mirrors the Guide form layout:
/// - Title field (required)
/// - Description field with rich text editor (required)
/// - Sections with Add Section button
///
/// This component wraps TaskSectionEditor and provides the proper form structure
/// for task content, making it consistent with the guide creation experience.
#[component]
pub fn TaskContentEditor(
    /// The form data signal to read from and write to
    data: RwSignal<serde_json::Value>,
    /// Validation errors
    #[prop(optional)]
    errors: Option<Signal<Vec<ValidationError>>>,
    /// Whether the form is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    // Initialize signals directly from form data (untracked to avoid SSR issues)
    let initial_data = data.get_untracked();

    let initial_title = initial_data
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let initial_description = initial_data
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let initial_sections = initial_data
        .get("sections")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // Create RwSignals with initial values
    let title_signal = RwSignal::new(initial_title);
    let description_signal = RwSignal::new(initial_description);
    let sections_signal = RwSignal::new(initial_sections);

    // Handle title change - update both signal and data
    let on_title_input = move |ev: web_sys::Event| {
        let new_title = event_target_value(&ev);
        title_signal.set(new_title.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("title".to_string(), serde_json::Value::String(new_title));
            }
        });
    };

    // Handle description change - update both signal and data
    let on_description_input = move |ev: web_sys::Event| {
        let new_desc = event_target_value(&ev);
        description_signal.set(new_desc.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("description".to_string(), serde_json::Value::String(new_desc));
            }
        });
    };

    // Handle sections change
    let on_sections_change = Callback::new(move |new_sections: Vec<serde_json::Value>| {
        sections_signal.set(new_sections.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("sections".to_string(), serde_json::Value::Array(new_sections));
            }
        });
    });

    // Helper to check if a field has errors
    let has_error = move |field_name: &str| -> bool {
        if let Some(errs) = errors {
            errs.get().iter().any(|e| e.field == field_name)
        } else {
            false
        }
    };

    // Helper to get error message for a field
    let get_error_message = move |field_name: &str| -> Option<String> {
        if let Some(errs) = errors {
            errs.get().iter().find(|e| e.field == field_name).map(|e| e.message.clone())
        } else {
            None
        }
    };

    view! {
        <div class="task-content-editor" style=FORM_STYLE>
            // Validation errors summary
            {move || {
                if let Some(errs) = errors {
                    let error_list = errs.get();
                    if !error_list.is_empty() {
                        view! {
                            <div style="background: rgba(239, 68, 68, 0.1); border: 1px solid #ef4444; border-radius: 0.375rem; padding: 0.75rem; margin-bottom: 1rem;">
                                <ul style="color: #ef4444; font-size: 0.875rem; margin: 0; padding-left: 1rem;">
                                    {error_list.iter().map(|err| {
                                        view! {
                                            <li>{format!("{}: {}", err.field, err.message)}</li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Title field (required) - matches guide's "Page Name"
            <div style="margin-bottom: 1.5rem;">
                <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                    "Title"
                    <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                </label>
                <input
                    type="text"
                    style=move || {
                        let base = "width: 100%; background: #1e293b; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
                        if has_error("title") {
                            format!("{} border: 2px solid #ef4444;", base)
                        } else {
                            format!("{} border: 1px solid #334155;", base)
                        }
                    }
                    placeholder="Enter task title..."
                    prop:value=move || title_signal.get()
                    on:input=on_title_input
                    disabled=read_only
                />
                <p
                    style=move || {
                        let has_err = get_error_message("title").is_some();
                        format!("color: #ef4444; font-size: 0.75rem; margin-top: 0.25rem; display: {}", if has_err { "block" } else { "none" })
                    }
                >
                    {move || get_error_message("title").unwrap_or_default()}
                </p>
            </div>

            // Description field (required) - matches guide's "Page Description" with rich text toolbar
            <div style="margin-bottom: 1.5rem;">
                <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                    "Description"
                    <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                </label>
                <div style=move || {
                    let base = "width: 100%; background: #1e293b; border-radius: 0.375rem; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 200px;";
                    if has_error("description") {
                        format!("{} border: 2px solid #ef4444;", base)
                    } else {
                        format!("{} border: 1px solid #334155;", base)
                    }
                }>
                    // Toolbar with formatting buttons (B, I, Link, List) - matches guide form
                    <div style=TOOLBAR_STYLE>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="Bold">"B"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="Italic">"I"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="Link">"Link"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="List">"List"</button>
                    </div>
                    // Content area
                    <textarea
                        style="width: 100%; background: transparent; border: none; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 150px; padding: 0.75rem; resize: vertical;"
                        prop:value=move || description_signal.get()
                        on:input=on_description_input
                        placeholder="Enter task description..."
                        disabled=read_only
                    />
                </div>
                <p
                    style=move || {
                        let has_err = get_error_message("description").is_some();
                        format!("color: #ef4444; font-size: 0.75rem; margin-top: 0.25rem; display: {}", if has_err { "block" } else { "none" })
                    }
                >
                    {move || get_error_message("description").unwrap_or_default()}
                </p>
            </div>

            // Sections with Add Section button - matches guide's "Sections"
            <TaskSectionEditor
                value=sections_signal
                label="Sections".to_string()
                required=true
                on_change=on_sections_change
                read_only=read_only
            />
        </div>
    }
}
