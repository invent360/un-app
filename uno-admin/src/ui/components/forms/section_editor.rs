//! Section Editor Component
//!
//! A specialized editor for Guide sections that displays sections in a repeater-style
//! list with collapsible accordions for viewing details, and uses a modal for adding new sections.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::common::icon::{Icon, IconName};
use super::gcs_media_upload::GcsMediaUploadList;

/// Data structure for a stage within a section
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StageData {
    pub order: Option<f64>,
    pub title: String,
    pub description: String,
    pub images: Vec<String>,
    pub videos: Vec<String>,
}

/// Data structure for a section
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SectionData {
    pub title: String,
    pub description: String,
    pub cover_images: Vec<String>,
    pub stages: Vec<StageData>,
    pub difficulty: String,
    pub duration_minutes: Option<i32>,
}

impl SectionData {
    pub fn from_json(value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            title: obj.and_then(|o| o.get("title")).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            // Support both "description" (new) and "summary" (legacy) for backward compatibility
            description: obj.and_then(|o| o.get("description").or_else(|| o.get("summary"))).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            cover_images: obj.and_then(|o| o.get("cover_images"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_else(|| {
                    obj.and_then(|o| o.get("cover_image"))
                        .and_then(|v| v.as_str())
                        .map(|s| vec![s.to_string()])
                        .unwrap_or_default()
                }),
            stages: obj.and_then(|o| o.get("stages").or_else(|| o.get("steps")))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(StageData::from_json).collect())
                .unwrap_or_default(),
            difficulty: obj.and_then(|o| o.get("difficulty")).and_then(|v| v.as_str()).unwrap_or("easy").to_string(),
            duration_minutes: obj.and_then(|o| o.get("duration_minutes")).and_then(|v| v.as_i64()).map(|v| v as i32),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "title": self.title,
            "description": self.description,
            "cover_images": self.cover_images,
            "stages": self.stages.iter().map(|s| s.to_json()).collect::<Vec<_>>(),
            "difficulty": self.difficulty,
            "duration_minutes": self.duration_minutes
        })
    }
}

impl StageData {
    pub fn from_json(value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            order: obj.and_then(|o| o.get("order")).and_then(|v| v.as_f64()),
            title: obj.and_then(|o| o.get("title")).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            description: obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            images: obj.and_then(|o| o.get("images"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_else(|| {
                    obj.and_then(|o| o.get("image"))
                        .and_then(|v| v.as_str())
                        .map(|s| vec![s.to_string()])
                        .unwrap_or_default()
                }),
            videos: obj.and_then(|o| o.get("videos"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_else(|| {
                    obj.and_then(|o| o.get("video"))
                        .and_then(|v| v.as_str())
                        .map(|s| vec![s.to_string()])
                        .unwrap_or_default()
                }),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "order": self.order,
            "title": self.title,
            "description": self.description,
            "images": self.images,
            "videos": self.videos
        })
    }
}

/// Section Editor Component - mimics the repeater field style
#[component]
pub fn SectionEditor(
    /// Current sections value
    value: RwSignal<Vec<serde_json::Value>>,
    /// Label for the field
    #[prop(default = "Sections".to_string())]
    label: String,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<serde_json::Value>>>,
    /// Whether the editor is read-only (view mode) - hides add/delete/move controls but allows expand/collapse
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    let sections = Signal::derive(move || {
        value.get().iter().map(SectionData::from_json).collect::<Vec<_>>()
    });

    let expanded_sections = RwSignal::new(std::collections::HashSet::<usize>::new());
    let show_modal = RwSignal::new(false);
    let modal_section = RwSignal::new(SectionData::default());
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
        modal_section.set(SectionData::default());
        show_modal.set(true);
    };

    let open_edit_section_modal = move |index: usize| {
        let secs = value.get();
        if let Some(section_json) = secs.get(index) {
            editing_index.set(Some(index));
            modal_section.set(SectionData::from_json(section_json));
            show_modal.set(true);
        }
    };

    let save_section = move |_| {
        let section = modal_section.get();
        let mut current = value.get();

        if let Some(idx) = editing_index.get() {
            // Update existing section
            if idx < current.len() {
                current[idx] = section.to_json();
            }
        } else {
            // Add new section
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
        <div class="section-editor" style="margin-bottom: 1.5rem;">
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
                        let section_title = if section.title.is_empty() {
                            format!("Section {}", index + 1)
                        } else {
                            section.title.clone()
                        };
                        let is_expanded = move || expanded_sections.get().contains(&index);

                        view! {
                            <div style="border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                                <div
                                    style="display: flex; align-items: center; padding: 0.75rem 1rem; background: #1e293b; cursor: pointer;"
                                    on:click=move |_| toggle_section(index)
                                >
                                    <div style="flex: 1; color: #e2e8f0; font-size: 0.875rem; font-weight: 500;">
                                        {section_title.clone()}
                                    </div>

                                    <div style="display: flex; align-items: center; gap: 0.5rem;" on:click=move |e| e.stop_propagation()>
                                        // Action buttons - only show in edit mode
                                        {if !read_only {
                                            view! {
                                                // Edit button (blue)
                                                <button
                                                    type="button"
                                                    style="background: #3b82f6; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    title="Edit section"
                                                    on:click=move |_| open_edit_section_modal(index)
                                                >
                                                    <Icon name=IconName::Edit size=16 />
                                                </button>

                                                // Move up button (dark gray, hidden if first)
                                                <button
                                                    type="button"
                                                    style={if index > 0 {
                                                        "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    } else {
                                                        "background: #1f2937; border: none; color: #4b5563; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; opacity: 0.5;"
                                                    }}
                                                    title="Move up"
                                                    disabled={index == 0}
                                                    on:click=move |_| { if index > 0 { move_up(index) } }
                                                >
                                                    <Icon name=IconName::ChevronUp size=16 />
                                                </button>

                                                // Move down button (dark gray, hidden if last)
                                                <button
                                                    type="button"
                                                    style={if index < total - 1 {
                                                        "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    } else {
                                                        "background: #1f2937; border: none; color: #4b5563; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; opacity: 0.5;"
                                                    }}
                                                    title="Move down"
                                                    disabled={index >= total - 1}
                                                    on:click=move |_| { if index < total - 1 { move_down(index) } }
                                                >
                                                    <Icon name=IconName::ChevronDown size=16 />
                                                </button>

                                                // Toggle expand/collapse (green)
                                                <button
                                                    type="button"
                                                    style="background: #22c55e; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    title="Toggle details"
                                                    on:click=move |_| toggle_section(index)
                                                >
                                                    {move || if is_expanded() {
                                                        view! { <Icon name=IconName::Minus size=14 /> }.into_any()
                                                    } else {
                                                        view! { <Icon name=IconName::Plus size=14 /> }.into_any()
                                                    }}
                                                </button>

                                                // Delete button (red)
                                                <button
                                                    type="button"
                                                    style="background: #ef4444; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    title="Delete section"
                                                    on:click=move |_| delete_section(index)
                                                >
                                                    <Icon name=IconName::Trash size=16 />
                                                </button>
                                            }.into_any()
                                        } else {
                                            // Read-only mode: only show toggle
                                            view! {
                                                <button
                                                    type="button"
                                                    style="background: #22c55e; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    title="Toggle details"
                                                    on:click=move |_| toggle_section(index)
                                                >
                                                    {move || if is_expanded() {
                                                        view! { <Icon name=IconName::Minus size=14 /> }.into_any()
                                                    } else {
                                                        view! { <Icon name=IconName::Plus size=14 /> }.into_any()
                                                    }}
                                                </button>
                                            }.into_any()
                                        }}
                                    </div>
                                </div>

                                {move || if is_expanded() {
                                    let section = section.clone();
                                    view! {
                                        <div style="padding: 1rem; background: #0f172a; border-top: 1px solid #334155;">
                                            {if !section.description.is_empty() {
                                                view! {
                                                    <div style="margin-bottom: 1rem;">
                                                        <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.25rem; font-weight: 500;">"Description"</div>
                                                        <div style="color: #94a3b8; font-size: 0.875rem; padding: 0.5rem; background: #1e293b; border-radius: 0.25rem;" inner_html=section.description.clone()></div>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}

                                            {if !section.cover_images.is_empty() {
                                                view! {
                                                    <div style="margin-bottom: 1rem;">
                                                        <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">
                                                            {format!("Cover Images ({})", section.cover_images.len())}
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
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}

                                            <div>
                                                <div style="font-size: 0.75rem; color: #64748b; margin-bottom: 0.5rem; font-weight: 500;">
                                                    {format!("Stages ({})", section.stages.len())}
                                                </div>
                                                {if section.stages.is_empty() {
                                                    view! {
                                                        <div style="color: #475569; font-size: 0.875rem; font-style: italic;">
                                                            "No stages in this section"
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                                            {section.stages.iter().enumerate().map(|(i, step)| {
                                                                let step_num = step.order.map(|o| o as i32).unwrap_or((i + 1) as i32);
                                                                let media_count = step.images.len() + step.videos.len();
                                                                view! {
                                                                    <div style="display: flex; align-items: flex-start; gap: 0.75rem; padding: 0.5rem; background: #1e293b; border-radius: 0.25rem;">
                                                                        <div style="width: 24px; height: 24px; background: #334155; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 0.75rem; color: #94a3b8; flex-shrink: 0;">
                                                                            {step_num}
                                                                        </div>
                                                                        <div style="flex: 1; min-width: 0;">
                                                                            <div style="color: #e2e8f0; font-size: 0.875rem; font-weight: 500;">{step.title.clone()}</div>
                                                                            {if media_count > 0 {
                                                                                view! {
                                                                                    <div style="font-size: 0.75rem; color: #64748b; margin-top: 0.125rem;">
                                                                                        {format!("{} media file{}", media_count, if media_count == 1 { "" } else { "s" })}
                                                                                    </div>
                                                                                }.into_any()
                                                                            } else {
                                                                                view! { <span></span> }.into_any()
                                                                            }}
                                                                        </div>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_any()
                                                }}
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }
                    }).collect_view()
                }}

                // Add Section button - only show in edit mode
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
                    <SectionModal
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

/// Modal for adding or editing a section
#[component]
fn SectionModal(
    section: RwSignal<SectionData>,
    is_editing: Signal<bool>,
    on_save: impl Fn(()) + 'static + Copy,
    on_cancel: impl Fn(()) + 'static + Copy,
) -> impl IntoView {
    // Track if step form is expanded
    let show_step_form = RwSignal::new(false);
    let new_step = RwSignal::new(StageData::default());

    let toggle_step_form = move |_| {
        if !show_step_form.get() {
            // Reset new step when opening form
            new_step.set(StageData {
                order: Some((section.get().stages.len() + 1) as f64),
                ..Default::default()
            });
        }
        show_step_form.update(|v| *v = !*v);
    };

    let add_step = move |_| {
        let step = new_step.get();
        section.update(|sec| {
            sec.stages.push(step.clone());
        });
        show_step_form.set(false);
        new_step.set(StageData::default());
    };

    let delete_step = move |index: usize| {
        section.update(|sec| {
            if index < sec.stages.len() {
                sec.stages.remove(index);
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
                        {move || if is_editing.get() { "Edit Section" } else { "Add Section" }}
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
                    // Section Title
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Section Title"
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

                    // Section Description
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Section Description"
                        </label>
                        <textarea
                            style="width: 100%; min-height: 80px; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; resize: vertical;"
                            placeholder="Brief description of this section..."
                            prop:value=move || section.get().description
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                section.update(|s| s.description = val);
                            }
                        ></textarea>
                    </div>

                    // Difficulty and Duration row
                    <div style="display: flex; gap: 1rem; margin-bottom: 1rem;">
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
                                <option value="easy" selected=move || section.get().difficulty == "easy" || section.get().difficulty.is_empty()>"Easy"</option>
                                <option value="medium" selected=move || section.get().difficulty == "medium">"Medium"</option>
                                <option value="hard" selected=move || section.get().difficulty == "hard">"Hard"</option>
                            </select>
                        </div>

                        // Duration in minutes
                        <div style="flex: 1;">
                            <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                                "Duration (minutes)"
                            </label>
                            <input
                                type="number"
                                min="1"
                                style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="e.g., 5"
                                prop:value=move || section.get().duration_minutes.map(|d| d.to_string()).unwrap_or_default()
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<i32>().ok();
                                    section.update(|s| s.duration_minutes = val);
                                }
                            />
                        </div>
                    </div>

                    // Cover Images with file upload
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Cover Images"
                        </label>
                        <ImageUploadList
                            value=Signal::derive(move || section.get().cover_images)
                            on_change=Callback::new(move |images: Vec<String>| {
                                section.update(|s| s.cover_images = images);
                            })
                        />
                    </div>

                    // Stages Section with inline form
                    <div style="margin-bottom: 1rem;">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
                            <label style="color: #94a3b8; font-size: 0.875rem; font-weight: 500;">
                                "Stages"
                                <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                            </label>
                            <button
                                type="button"
                                style="background: #334155; color: #e2e8f0; border: none; border-radius: 0.375rem; padding: 0.375rem 0.75rem; cursor: pointer; display: flex; align-items: center; gap: 0.25rem; font-size: 0.75rem;"
                                on:click=toggle_step_form
                            >
                                <Icon name=IconName::Plus size=14 />
                                "Add Stage"
                            </button>
                        </div>

                        // Inline Stage Form (shown when Add Stage is clicked)
                        {move || if show_step_form.get() {
                            view! {
                                <div style="border: 1px solid #3b82f6; border-radius: 0.375rem; padding: 1rem; margin-bottom: 0.75rem; background: #0f172a;">
                                    <div style="font-size: 0.875rem; color: #3b82f6; margin-bottom: 0.75rem; font-weight: 500;">
                                        "New Stage"
                                    </div>

                                    // Order
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Order"
                                        </label>
                                        <input
                                            type="number"
                                            min="1"
                                            style="width: 80px; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                            prop:value=move || new_step.get().order.map(|o| o.to_string()).unwrap_or_default()
                                            on:input=move |ev| {
                                                let val = event_target_value(&ev).parse::<f64>().ok();
                                                new_step.update(|s| s.order = val);
                                            }
                                        />
                                    </div>

                                    // Title
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Stage Title" <span style="color: #ef4444;">"*"</span>
                                        </label>
                                        <input
                                            type="text"
                                            style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                            placeholder="e.g., Download the App"
                                            prop:value=move || new_step.get().title
                                            on:input=move |ev| {
                                                let val = event_target_value(&ev);
                                                new_step.update(|s| s.title = val);
                                            }
                                        />
                                    </div>

                                    // Description
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Description" <span style="color: #ef4444;">"*"</span>
                                        </label>
                                        <textarea
                                            style="width: 100%; min-height: 80px; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none; resize: vertical;"
                                            placeholder="Describe what the user should do..."
                                            prop:value=move || new_step.get().description
                                            on:input=move |ev| {
                                                let val = event_target_value(&ev);
                                                new_step.update(|s| s.description = val);
                                            }
                                        ></textarea>
                                    </div>

                                    // Stage Images
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Stage Images"
                                        </label>
                                        <ImageUploadList
                                            value=Signal::derive(move || new_step.get().images)
                                            on_change=Callback::new(move |images: Vec<String>| {
                                                new_step.update(|s| s.images = images);
                                            })
                                        />
                                    </div>

                                    // Stage Videos
                                    <div style="margin-bottom: 0.75rem;">
                                        <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Stage Videos"
                                        </label>
                                        <ImageUploadList
                                            value=Signal::derive(move || new_step.get().videos)
                                            on_change=Callback::new(move |videos: Vec<String>| {
                                                new_step.update(|s| s.videos = videos);
                                            })
                                            accept="video/*".to_string()
                                            label="videos".to_string()
                                        />
                                    </div>

                                    // Form Actions
                                    <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;">
                                        <button
                                            type="button"
                                            style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem;"
                                            on:click=move |_| show_step_form.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            type="button"
                                            style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem;"
                                            on:click=add_step
                                        >
                                            "Add Stage"
                                        </button>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Stages List
                        <div style="border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                            {move || {
                                let sec = section.get();
                                if sec.stages.is_empty() {
                                    view! {
                                        <div style="padding: 1.5rem; text-align: center; color: #64748b; font-size: 0.875rem;">
                                            "No stages yet. Click \"Add Stage\" to create one."
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            {sec.stages.iter().enumerate().map(|(index, step)| {
                                                let step = step.clone();
                                                let media_count = step.images.len() + step.videos.len();
                                                view! {
                                                    <div style="display: flex; align-items: center; padding: 0.75rem; border-bottom: 1px solid #334155; background: #0f172a;">
                                                        <div style="width: 32px; color: #64748b; font-size: 0.875rem;">
                                                            {step.order.map(|o| o as i32).unwrap_or((index + 1) as i32)}
                                                        </div>
                                                        <div style="flex: 1;">
                                                            <div style="color: #e2e8f0; font-size: 0.875rem;">{step.title.clone()}</div>
                                                            {if media_count > 0 {
                                                                view! {
                                                                    <div style="font-size: 0.75rem; color: #64748b; margin-top: 0.125rem;">
                                                                        {format!("{} media file{}", media_count, if media_count == 1 { "" } else { "s" })}
                                                                    </div>
                                                                }.into_any()
                                                            } else {
                                                                view! { <span></span> }.into_any()
                                                            }}
                                                        </div>
                                                        <button
                                                            type="button"
                                                            style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                                                            on:click=move |_| delete_step(index)
                                                        >
                                                            <Icon name=IconName::Trash size=16 />
                                                        </button>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
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

/// Component for uploading and displaying multiple images - Uses GCS Upload
#[component]
fn ImageUploadList(
    value: Signal<Vec<String>>,
    on_change: Callback<Vec<String>>,
    #[prop(default = "image/*".to_string())]
    accept: String,
    #[prop(default = "images".to_string())]
    label: String,
) -> impl IntoView {
    view! {
        <GcsMediaUploadList
            value=value
            on_change=on_change
            content_type="section".to_string()
            accept=accept
            label=label
        />
    }
}
