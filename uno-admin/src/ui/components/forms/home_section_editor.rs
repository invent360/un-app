//! Home Section Editor Component
//!
//! A generic, schema-driven editor for Home page sections.
//! Uses a single modal component that renders fields based on section schema definitions.

use leptos::prelude::*;
use crate::components::common::icon::{Icon, IconName};
use crate::api::section_types::*;
use super::gcs_media_upload::GcsMediaUploadList;

// ============================================
// Media Type for Section Media Fields
// ============================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    Image,
    Video,
}

/// Home Section Editor Component
#[component]
pub fn HomeSectionEditor(
    /// Current sections value
    value: RwSignal<Vec<serde_json::Value>>,
    /// Label for the field
    #[prop(default = "Sections".to_string())]
    label: String,
    /// Whether the field is required
    #[prop(default = true)]
    _required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<serde_json::Value>>>,
    /// Whether the editor is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    let expanded_sections = RwSignal::new(std::collections::HashSet::<usize>::new());
    let show_modal = RwSignal::new(false);
    let editing_section = RwSignal::new(None::<Section>);
    let editing_index = RwSignal::new(None::<usize>);

    let toggle_expand = move |index: usize| {
        expanded_sections.update(|set| {
            if set.contains(&index) {
                set.remove(&index);
            } else {
                set.insert(index);
            }
        });
    };

    // Open modal directly with default section type (can be changed in modal)
    let open_add_modal = move |_| {
        let order = value.get().len() as i32 + 1;
        let section = Section::new("hero", order); // Default to hero, user can change in dropdown
        editing_section.set(Some(section));
        editing_index.set(None);
        show_modal.set(true);
    };

    let open_edit_modal = move |index: usize| {
        let sections = value.get();
        if let Some(json) = sections.get(index) {
            if let Ok(section) = serde_json::from_value::<Section>(json.clone()) {
                editing_section.set(Some(section));
                editing_index.set(Some(index));
                show_modal.set(true);
            }
        }
    };

    let save_section = move |section: Section| {
        let json = serde_json::to_value(&section).unwrap_or_default();
        let mut current = value.get();

        if let Some(idx) = editing_index.get() {
            if idx < current.len() {
                current[idx] = json;
            }
        } else {
            current.push(json);
        }

        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
        editing_section.set(None);
        editing_index.set(None);
        show_modal.set(false);
    };

    let delete_section = move |index: usize| {
        let mut current = value.get();
        if index < current.len() {
            current.remove(index);
            // Update display_order
            for (i, section) in current.iter_mut().enumerate() {
                if let Some(obj) = section.as_object_mut() {
                    obj.insert("display_order".to_string(), serde_json::json!(i + 1));
                }
            }
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
            for (i, section) in current.iter_mut().enumerate() {
                if let Some(obj) = section.as_object_mut() {
                    obj.insert("display_order".to_string(), serde_json::json!(i + 1));
                }
            }
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
            for (i, section) in current.iter_mut().enumerate() {
                if let Some(obj) = section.as_object_mut() {
                    obj.insert("display_order".to_string(), serde_json::json!(i + 1));
                }
            }
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    let toggle_visibility = move |index: usize| {
        let mut current = value.get();
        if let Some(section) = current.get_mut(index) {
            if let Some(obj) = section.as_object_mut() {
                let visible = obj.get("is_visible").and_then(|v| v.as_bool()).unwrap_or(true);
                obj.insert("is_visible".to_string(), serde_json::json!(!visible));
            }
        }
        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
    };

    view! {
        <div class="home-section-editor">
            // Sections Label
            <label style="display: block; color: #e2e8f0; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem;">
                {label}
                <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
            </label>

            // Sections List
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                {move || {
                    let sections = value.get();
                    let sections_len = sections.len();
                    if sections.is_empty() {
                        view! {
                            <div style="padding: 2rem; text-align: center; background: #1e293b; border: 2px dashed #334155; border-radius: 0.5rem; color: #64748b;">
                                "No sections yet. Click \"Add Section\" to create one."
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div>
                                {sections.into_iter().enumerate().map(|(i, json)| {
                                    let section: Section = serde_json::from_value(json).unwrap_or_default();
                                    let is_expanded = expanded_sections.get().contains(&i);
                                    let schema = get_section_schema(&section.section_type);
                                    let color = schema.as_ref().map(|s| s.color).unwrap_or("#64748b");
                                    let section_clone = section.clone();
                                    let is_last = i == sections_len - 1;

                                    view! {
                                        <div style="background: #1e293b; border-radius: 0.5rem; border: 1px solid #334155; overflow: hidden; margin-bottom: 0.5rem;">
                                            // Section Header
                                            <div
                                                style=format!("display: flex; align-items: center; padding: 0.75rem 1rem; border-left: 4px solid {};", color)
                                            >
                                                // Toggle expand/collapse button (left side)
                                                <button
                                                    type="button"
                                                    style="background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; margin-right: 0.75rem;"
                                                    title="Toggle details"
                                                    on:click=move |_| toggle_expand(i)
                                                >
                                                    <Icon name={if is_expanded { IconName::Minus } else { IconName::Plus }} size=14 />
                                                </button>

                                                // Section title
                                                <div
                                                    style="flex: 1; display: flex; align-items: center; gap: 0.75rem; cursor: pointer;"
                                                    on:click=move |_| toggle_expand(i)
                                                >
                                                    <span style="color: #e2e8f0; font-weight: 500;">
                                                        {section.display_title()}
                                                    </span>
                                                    {if !section.is_visible {
                                                        view! {
                                                            <span style="background: #475569; color: #94a3b8; padding: 0.125rem 0.5rem; border-radius: 9999px; font-size: 0.625rem; text-transform: uppercase;">
                                                                "Hidden"
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                </div>

                                                {if !read_only {
                                                    view! {
                                                        <div style="display: flex; gap: 0.5rem;" on:click=move |e| e.stop_propagation()>
                                                            // Edit button (blue)
                                                            <button
                                                                type="button"
                                                                title="Edit"
                                                                style="background: #3b82f6; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                                on:click=move |_| open_edit_modal(i)
                                                            >
                                                                <Icon name=IconName::Edit size=14 />
                                                            </button>
                                                            // Move up button
                                                            <button
                                                                type="button"
                                                                title="Move up"
                                                                style={if i == 0 {
                                                                    "background: #374151; border: none; color: #64748b; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                                } else {
                                                                    "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                                }}
                                                                disabled=i == 0
                                                                on:click=move |_| move_up(i)
                                                            >
                                                                <Icon name=IconName::ChevronUp size=14 />
                                                            </button>
                                                            // Move down button
                                                            <button
                                                                type="button"
                                                                title="Move down"
                                                                style={if is_last {
                                                                    "background: #374151; border: none; color: #64748b; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                                } else {
                                                                    "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                                }}
                                                                disabled=is_last
                                                                on:click=move |_| move_down(i)
                                                            >
                                                                <Icon name=IconName::ChevronDown size=14 />
                                                            </button>
                                                            // Delete button (red)
                                                            <button
                                                                type="button"
                                                                title="Delete"
                                                                style="background: #ef4444; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                                on:click=move |_| delete_section(i)
                                                            >
                                                                <Icon name=IconName::Trash size=14 />
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>

                                            // Section Content (expanded)
                                            {if is_expanded {
                                                view! {
                                                    <div style="padding: 1rem; background: #0f172a; border-top: 1px solid #334155;">
                                                        <SectionPreview section=section_clone />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Add Section Button at bottom
            {move || if !read_only {
                view! {
                    <button
                        type="button"
                        style="width: 100%; padding: 0.75rem 1rem; background: #334155; color: #94a3b8; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.5rem; margin-top: 0.5rem;"
                        on:click=open_add_modal
                    >
                        <Icon name=IconName::Plus size=14 />
                        "Add Section"
                    </button>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Section Editor Modal
            {move || if show_modal.get() {
                if let Some(section) = editing_section.get() {
                    view! {
                        <SectionEditorModal
                            section=RwSignal::new(section)
                            is_editing=Signal::derive(move || editing_index.get().is_some())
                            on_save=Callback::new(move |s: Section| save_section(s))
                            on_cancel=Callback::new(move |_| {
                                editing_section.set(None);
                                editing_index.set(None);
                                show_modal.set(false);
                            })
                        />
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

// ============================================
// Section Preview Component
// ============================================

#[component]
fn SectionPreview(section: Section) -> impl IntoView {
    let has_highlights = section.has_highlights();
    let highlights_count = section.highlights.len();
    let has_media = section.has_media();
    let images_count = section.images.len();
    let videos_count = section.videos.len();
    let has_links = section.has_links();
    let links_count = section.links.len();

    view! {
        <div style="font-size: 0.875rem;">
            // Title
            {if !section.title.is_empty() {
                view! {
                    <div style="margin-bottom: 0.5rem;">
                        <span style="color: #64748b;">Title: </span>
                        <span style="color: #e2e8f0;">{section.title.clone()}</span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Description
            {if !section.description.is_empty() {
                view! {
                    <div style="margin-bottom: 0.5rem;">
                        <span style="color: #64748b;">Description: </span>
                        <span style="color: #94a3b8;">{section.description.clone()}</span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Highlights, Media, and Links counts (common fields)
            {if has_highlights || has_media || has_links {
                view! {
                    <div style="display: flex; flex-wrap: wrap; gap: 1rem; margin-bottom: 0.5rem;">
                        {if highlights_count > 0 {
                            view! {
                                <span style="display: flex; align-items: center; gap: 0.25rem; color: #64748b;">
                                    <Icon name=IconName::Star size=14 />
                                    {format!("{} highlight{}", highlights_count, if highlights_count == 1 { "" } else { "s" })}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        {if images_count > 0 {
                            view! {
                                <span style="display: flex; align-items: center; gap: 0.25rem; color: #64748b;">
                                    <Icon name=IconName::Image size=14 />
                                    {format!("{} image{}", images_count, if images_count == 1 { "" } else { "s" })}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        {if videos_count > 0 {
                            view! {
                                <span style="display: flex; align-items: center; gap: 0.25rem; color: #64748b;">
                                    <Icon name=IconName::Play size=14 />
                                    {format!("{} video{}", videos_count, if videos_count == 1 { "" } else { "s" })}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        {if links_count > 0 {
                            view! {
                                <span style="display: flex; align-items: center; gap: 0.25rem; color: #64748b;">
                                    <Icon name=IconName::Link size=14 />
                                    {format!("{} link{}", links_count, if links_count == 1 { "" } else { "s" })}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Type-specific data preview
            {render_data_preview(&section)}
        </div>
    }
}

fn render_data_preview(section: &Section) -> impl IntoView {
    let data = &section.data;

    match section.section_type.as_str() {
        "hero" => {
            let highlights = section.get_string_array("highlights");
            let images = section.get_string_array("images");
            view! {
                <div>
                    {if !highlights.is_empty() {
                        view! {
                            <div style="margin-bottom: 0.5rem;">
                                <span style="color: #64748b;">{format!("Highlights ({}): ", highlights.len())}</span>
                                <span style="color: #94a3b8;">{highlights.join(", ")}</span>
                            </div>
                        }.into_any()
                    } else { view! { <span></span> }.into_any() }}
                    {if !images.is_empty() {
                        view! {
                            <div>
                                <span style="color: #64748b;">{format!("Images: {}", images.len())}</span>
                            </div>
                        }.into_any()
                    } else { view! { <span></span> }.into_any() }}
                </div>
            }.into_any()
        }
        "how_it_works" => {
            let steps = data.get("steps").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
            view! {
                <div style="color: #64748b;">{format!("Steps: {}", steps)}</div>
            }.into_any()
        }
        "earnings" => {
            let tiers = data.get("tiers").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
            view! {
                <div style="color: #64748b;">{format!("Tiers: {}", tiers)}</div>
            }.into_any()
        }
        "testimonials" => {
            let testimonials = data.get("testimonials").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
            let has_api = data.get("api_endpoint").and_then(|v| v.as_str()).is_some();
            let display_text = if has_api {
                "Using API".to_string()
            } else {
                format!("Static: {} testimonials", testimonials)
            };
            view! {
                <div style="color: #64748b;">
                    {display_text}
                </div>
            }.into_any()
        }
        _ => view! { <span></span> }.into_any()
    }
}

// ============================================
// Section Type Picker Modal
// ============================================

#[component]
fn SectionTypePicker(
    on_select: Callback<String>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let schemas = get_all_section_schemas();

    view! {
        <div
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            on:click=move |_| on_cancel.run(())
        >
            <div
                style="background: #1e293b; border-radius: 0.5rem; width: 90%; max-width: 500px; border: 1px solid #334155;"
                on:click=move |e| e.stop_propagation()
            >
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 1rem 1.5rem; border-bottom: 1px solid #334155;">
                    <h2 style="color: #e2e8f0; font-size: 1.125rem; font-weight: 600; margin: 0;">
                        "Choose Section Type"
                    </h2>
                    <button
                        type="button"
                        style="background: transparent; border: none; color: #94a3b8; cursor: pointer;"
                        on:click=move |_| on_cancel.run(())
                    >
                        <Icon name=IconName::Close size=24 />
                    </button>
                </div>

                <div style="padding: 1rem; display: grid; gap: 0.75rem;">
                    {schemas.into_iter().map(|schema| {
                        let section_type = schema.section_type.to_string();
                        let on_select = on_select.clone();
                        view! {
                            <button
                                type="button"
                                style=format!("display: flex; align-items: center; gap: 1rem; padding: 1rem; background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; cursor: pointer; text-align: left; border-left: 4px solid {};", schema.color)
                                on:click=move |_| on_select.run(section_type.clone())
                            >
                                <div style="flex: 1;">
                                    <div style="color: #e2e8f0; font-weight: 500; margin-bottom: 0.25rem;">
                                        {schema.label}
                                    </div>
                                    <div style="color: #64748b; font-size: 0.875rem;">
                                        {schema.description}
                                    </div>
                                </div>
                                <Icon name=IconName::ChevronRight size=20 />
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

// ============================================
// Generic Section Editor Modal
// ============================================

#[component]
fn SectionEditorModal(
    section: RwSignal<Section>,
    is_editing: Signal<bool>,
    on_save: Callback<Section>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let section_type = Signal::derive(move || section.get().section_type.clone());
    let schema = Signal::derive(move || get_section_schema(&section_type.get()));

    view! {
        <div
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            on:click=move |_| on_cancel.run(())
        >
            <div
                style="background: #1e293b; border-radius: 0.5rem; width: 90%; max-width: 700px; max-height: 90vh; overflow-y: auto; border: 1px solid #334155;"
                on:click=move |e| e.stop_propagation()
            >
                // Header
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 1rem 1.5rem; border-bottom: 1px solid #334155;">
                    <div style="display: flex; align-items: center; gap: 0.75rem;">
                        // Small colored badge with section type
                        <span style=move || format!(
                            "display: inline-flex; align-items: center; padding: 0.25rem 0.75rem; border-radius: 0.25rem; font-size: 0.75rem; font-weight: 600; color: white; background: {};",
                            schema.get().map(|s| s.color).unwrap_or("#64748b")
                        )>
                            {move || schema.get().map(|s| s.label).unwrap_or("Section")}
                        </span>
                        <h2 style="color: #e2e8f0; font-size: 1.125rem; font-weight: 600; margin: 0;">
                            {move || if is_editing.get() { "Edit Section" } else { "Add Section" }}
                        </h2>
                    </div>
                    <button
                        type="button"
                        style="background: transparent; border: none; color: #94a3b8; cursor: pointer;"
                        on:click=move |_| on_cancel.run(())
                    >
                        <Icon name=IconName::Close size=24 />
                    </button>
                </div>

                // Body - Common Fields
                <div style="padding: 1.5rem;">
                    // Section Type Dropdown (only for new sections)
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Section Type" <span style="color: #ef4444;">*</span>
                        </label>
                        <select
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                            prop:value=move || section.get().section_type
                            on:change=move |ev| {
                                let new_type = event_target_value(&ev);
                                section.update(|s| s.section_type = new_type);
                            }
                        >
                            <option value="hero" selected=move || section.get().section_type == "hero">"Hero"</option>
                            <option value="how_it_works" selected=move || section.get().section_type == "how_it_works">"How It Works"</option>
                            <option value="earnings" selected=move || section.get().section_type == "earnings">"Real Earnings"</option>
                            <option value="testimonials" selected=move || section.get().section_type == "testimonials">"Testimonials"</option>
                        </select>
                    </div>

                    // Title (common)
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Title"
                            <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                        </label>
                        <input
                            type="text"
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                            placeholder="e.g., Welcome to Uno"
                            prop:value=move || section.get().title
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                section.update(|s| s.title = val);
                            }
                        />
                    </div>

                    // Description (rich text style)
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Description"
                        </label>
                        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                            // Rich text toolbar
                            <div style="display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #1e293b;">
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-weight: bold;" title="Bold">"B"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-style: italic;" title="Italic">"I"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="Link">"Link"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="Bullet List">"List"</button>
                            </div>
                            // Content area
                            <textarea
                                style="width: 100%; min-height: 120px; background: transparent; border: none; padding: 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; resize: vertical;"
                                prop:value=move || section.get().description
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    section.update(|s| s.description = val);
                                }
                                placeholder="Enter description..."
                            ></textarea>
                        </div>
                    </div>

                    // Highlights (common, optional)
                    <SectionHighlightsField
                        items=Signal::derive(move || section.get().highlights.clone())
                        on_change=Callback::new(move |items: Vec<String>| {
                            section.update(|s| s.highlights = items);
                        })
                    />

                    // Images (common, optional)
                    <SectionMediaField
                        label="Images".to_string()
                        media_type=MediaType::Image
                        items=Signal::derive(move || section.get().images.clone())
                        on_change=Callback::new(move |items: Vec<String>| {
                            section.update(|s| s.images = items);
                        })
                    />

                    // Videos (common, optional)
                    <SectionMediaField
                        label="Videos".to_string()
                        media_type=MediaType::Video
                        items=Signal::derive(move || section.get().videos.clone())
                        on_change=Callback::new(move |items: Vec<String>| {
                            section.update(|s| s.videos = items);
                        })
                    />

                    // Links (common, optional - for download buttons, CTAs)
                    <SectionLinksField
                        items=Signal::derive(move || section.get().links.clone())
                        on_change=Callback::new(move |items: Vec<SectionLink>| {
                            section.update(|s| s.links = items);
                        })
                    />

                    // Type-specific fields (rendered from schema)
                    {move || {
                        if let Some(schema) = schema.get() {
                            view! {
                                <SchemaFields
                                    fields=schema.fields
                                    data=Signal::derive(move || section.get().data.clone())
                                    on_change=Callback::new(move |data: serde_json::Value| {
                                        section.update(|s| s.data = data);
                                    })
                                />
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                    // Visibility Toggle
                    <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 1rem; padding-top: 1rem; border-top: 1px solid #334155;">
                        <input
                            type="checkbox"
                            id="section-visible"
                            style="width: 16px; height: 16px; accent-color: #22c55e;"
                            prop:checked=move || section.get().is_visible
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                section.update(|s| s.is_visible = checked);
                            }
                        />
                        <label for="section-visible" style="color: #94a3b8; font-size: 0.875rem;">
                            "Section is visible on page"
                        </label>
                    </div>
                </div>

                // Footer
                <div style="display: flex; justify-content: flex-end; gap: 0.75rem; padding: 1rem 1.5rem; border-top: 1px solid #334155;">
                    <button
                        type="button"
                        style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer;"
                        on:click=move |_| on_cancel.run(())
                    >
                        "Cancel"
                    </button>
                    <button
                        type="button"
                        style="background: #3b82f6; color: white; border: none; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer;"
                        on:click=move |_| on_save.run(section.get())
                    >
                        {move || if is_editing.get() { "Save Changes" } else { "Add" }}
                    </button>
                </div>
            </div>
        </div>
    }
}

// ============================================
// Schema-driven Field Renderer
// ============================================

#[component]
fn SchemaFields(
    fields: Vec<FieldDef>,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    view! {
        <div>
            {fields.into_iter().map(|field| {
                view! {
                    <SchemaField
                        field=field
                        data=data
                        on_change=on_change.clone()
                    />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn SchemaField(
    field: FieldDef,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    let key = field.key.to_string();
    let label = field.label.to_string();
    let placeholder = field.placeholder.to_string();

    match field.field_type {
        FieldType::Text => {
            let key_clone = key.clone();
            view! {
                <div style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                    </label>
                    <input
                        type="text"
                        style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                        placeholder=placeholder
                        prop:value=move || data.get().get(&key).and_then(|v| v.as_str()).unwrap_or("").to_string()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            let mut d = data.get();
                            if let Some(obj) = d.as_object_mut() {
                                obj.insert(key_clone.clone(), serde_json::json!(val));
                            }
                            on_change.run(d);
                        }
                    />
                </div>
            }.into_any()
        }
        FieldType::TextArea => {
            let key_clone = key.clone();
            view! {
                <div style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                    </label>
                    <textarea
                        style="width: 100%; min-height: 80px; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; resize: vertical;"
                        placeholder=placeholder
                        prop:value=move || data.get().get(&key).and_then(|v| v.as_str()).unwrap_or("").to_string()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            let mut d = data.get();
                            if let Some(obj) = d.as_object_mut() {
                                obj.insert(key_clone.clone(), serde_json::json!(val));
                            }
                            on_change.run(d);
                        }
                    ></textarea>
                </div>
            }.into_any()
        }
        FieldType::StringList => {
            view! {
                <StringListField
                    key=key
                    label=label
                    data=data
                    on_change=on_change
                />
            }.into_any()
        }
        FieldType::MediaList => {
            view! {
                <MediaListField
                    key=key
                    label=label
                    data=data
                    on_change=on_change
                />
            }.into_any()
        }
        FieldType::Number => {
            let key_clone = key.clone();
            view! {
                <div style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                    </label>
                    <input
                        type="number"
                        style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                        prop:value=move || data.get().get(&key).and_then(|v| v.as_f64()).unwrap_or(0.0).to_string()
                        on:input=move |ev| {
                            let val: f64 = event_target_value(&ev).parse().unwrap_or(0.0);
                            let mut d = data.get();
                            if let Some(obj) = d.as_object_mut() {
                                obj.insert(key_clone.clone(), serde_json::json!(val));
                            }
                            on_change.run(d);
                        }
                    />
                </div>
            }.into_any()
        }
        FieldType::Boolean => {
            let key_clone = key.clone();
            view! {
                <div style="margin-bottom: 1rem; display: flex; align-items: center; gap: 0.5rem;">
                    <input
                        type="checkbox"
                        style="width: 16px; height: 16px; accent-color: #22c55e;"
                        prop:checked=move || data.get().get(&key).and_then(|v| v.as_bool()).unwrap_or(false)
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            let mut d = data.get();
                            if let Some(obj) = d.as_object_mut() {
                                obj.insert(key_clone.clone(), serde_json::json!(checked));
                            }
                            on_change.run(d);
                        }
                    />
                    <label style="color: #94a3b8; font-size: 0.875rem;">
                        {label}
                    </label>
                </div>
            }.into_any()
        }
        FieldType::OptionalText => {
            let key_clone = key.clone();
            view! {
                <div style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                    </label>
                    <input
                        type="text"
                        style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                        placeholder=placeholder
                        prop:value=move || data.get().get(&key).and_then(|v| v.as_str()).unwrap_or("").to_string()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            let mut d = data.get();
                            if let Some(obj) = d.as_object_mut() {
                                if val.is_empty() {
                                    obj.insert(key_clone.clone(), serde_json::Value::Null);
                                } else {
                                    obj.insert(key_clone.clone(), serde_json::json!(val));
                                }
                            }
                            on_change.run(d);
                        }
                    />
                    <div style="font-size: 0.75rem; color: #64748b; margin-top: 0.25rem;">
                        "Leave empty to disable"
                    </div>
                </div>
            }.into_any()
        }
        FieldType::Repeater(sub_fields) => {
            // Use specialized StepsField for "steps" key (How It Works section)
            if key == "steps" {
                view! {
                    <StepsField
                        key=key
                        label=label
                        data=data
                        on_change=on_change
                    />
                }.into_any()
            } else if key == "tiers" {
                // Use specialized TiersField for "tiers" key (Earnings section)
                view! {
                    <TiersField
                        key=key
                        label=label
                        data=data
                        on_change=on_change
                    />
                }.into_any()
            } else {
                view! {
                    <RepeaterField
                        key=key
                        label=label
                        sub_fields=sub_fields
                        data=data
                        on_change=on_change
                    />
                }.into_any()
            }
        }
        FieldType::Select(options) => {
            let key_clone = key.clone();
            let key_for_options = key.clone();
            let options_clone = options.clone();
            view! {
                <div style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                    </label>
                    <select
                        style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            let mut d = data.get();
                            if let Some(obj) = d.as_object_mut() {
                                obj.insert(key_clone.clone(), serde_json::json!(val));
                            }
                            on_change.run(d);
                        }
                    >
                        {options_clone.into_iter().map(|(value, opt_label)| {
                            let value_str = value.to_string();
                            let key_inner = key_for_options.clone();
                            let is_selected = move || {
                                data.get().get(&key_inner).and_then(|v| v.as_str()).unwrap_or("") == value
                            };
                            view! {
                                <option value=value_str selected=is_selected>
                                    {opt_label}
                                </option>
                            }
                        }).collect_view()}
                    </select>
                </div>
            }.into_any()
        }
    }
}

// ============================================
// String List Field
// ============================================

#[component]
fn StringListField(
    key: String,
    label: String,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    let new_item = RwSignal::new(String::new());
    let show_input = RwSignal::new(false);

    let key_for_derive = key.clone();
    let items = Signal::derive(move || {
        data.get().get(&key_for_derive)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
            .unwrap_or_default()
    });

    let key_for_add = key.clone();
    let key_for_remove = key;

    let add_item = Callback::new(move |_: ()| {
        let text = new_item.get();
        if !text.trim().is_empty() {
            let mut d = data.get();
            if let Some(obj) = d.as_object_mut() {
                let mut arr = obj.get(&key_for_add)
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                arr.push(serde_json::json!(text));
                obj.insert(key_for_add.clone(), serde_json::json!(arr));
            }
            on_change.run(d);
            new_item.set(String::new());
            show_input.set(false);
        }
    });

    let remove_item = Callback::new(move |index: usize| {
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            if let Some(arr) = obj.get_mut(&key_for_remove).and_then(|v| v.as_array_mut()) {
                if index < arr.len() {
                    arr.remove(index);
                }
            }
        }
        on_change.run(d);
    });

    view! {
        <div style="margin-bottom: 1rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                <label style="color: #94a3b8; font-size: 0.875rem; font-weight: 500;">
                    {label}
                </label>
                <button
                    type="button"
                    style="background: #334155; color: #e2e8f0; border: none; border-radius: 0.25rem; padding: 0.25rem 0.5rem; cursor: pointer; font-size: 0.75rem; display: flex; align-items: center; gap: 0.25rem;"
                    on:click=move |_| show_input.update(|v| *v = !*v)
                >
                    <Icon name=IconName::Plus size=14 />
                    "Add"
                </button>
            </div>

            <Show
                when=move || show_input.get()
                fallback=|| view! { <span></span> }
            >
                <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                    <input
                        type="text"
                        style="flex: 1; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                        placeholder="Enter item..."
                        prop:value=move || new_item.get()
                        on:input=move |ev| new_item.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                add_item.run(());
                            }
                        }
                    />
                    <button
                        type="button"
                        style="background: #22c55e; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem; cursor: pointer;"
                        on:click=move |_| add_item.run(())
                    >
                        <Icon name=IconName::CheckCircle size=16 />
                    </button>
                </div>
            </Show>

            <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                {move || {
                    let current_items = items.get();
                    current_items.into_iter().enumerate().map(|(i, text)| {
                        view! {
                            <span style="display: flex; align-items: center; gap: 0.25rem; background: #0f172a; border: 1px solid #334155; border-radius: 9999px; padding: 0.25rem 0.5rem 0.25rem 0.75rem; font-size: 0.75rem; color: #94a3b8;">
                                {text}
                                <button
                                    type="button"
                                    style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 0; line-height: 1;"
                                    on:click=move |_| remove_item.run(i)
                                >
                                    <Icon name=IconName::Close size=14 />
                                </button>
                            </span>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================
// Media List Field
// ============================================

#[component]
fn MediaListField(
    key: String,
    label: String,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    let new_url = RwSignal::new(String::new());

    let key_for_derive = key.clone();
    let items = Signal::derive(move || {
        data.get().get(&key_for_derive)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
            .unwrap_or_default()
    });

    let key_for_add = key.clone();
    let key_for_remove = key;

    let add_item = Callback::new(move |_: ()| {
        let url = new_url.get();
        if !url.trim().is_empty() {
            let mut d = data.get();
            if let Some(obj) = d.as_object_mut() {
                let mut arr = obj.get(&key_for_add)
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                arr.push(serde_json::json!(url));
                obj.insert(key_for_add.clone(), serde_json::json!(arr));
            }
            on_change.run(d);
            new_url.set(String::new());
        }
    });

    let remove_item = Callback::new(move |index: usize| {
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            if let Some(arr) = obj.get_mut(&key_for_remove).and_then(|v| v.as_array_mut()) {
                if index < arr.len() {
                    arr.remove(index);
                }
            }
        }
        on_change.run(d);
    });

    view! {
        <div style="margin-bottom: 1rem;">
            <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                {label}
            </label>

            <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                <input
                    type="text"
                    style="flex: 1; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                    placeholder="Enter image URL or upload..."
                    prop:value=move || new_url.get()
                    on:input=move |ev| new_url.set(event_target_value(&ev))
                />
                <button
                    type="button"
                    style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer;"
                    on:click=move |_| add_item.run(())
                >
                    "Add"
                </button>
            </div>

            <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                {move || {
                    let current_items = items.get();
                    current_items.into_iter().enumerate().map(|(i, url)| {
                        view! {
                            <div style="position: relative; width: 80px; height: 80px; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; overflow: hidden;">
                                <img src=url style="width: 100%; height: 100%; object-fit: cover;" />
                                <button
                                    type="button"
                                    style="position: absolute; top: 2px; right: 2px; background: rgba(0,0,0,0.7); border: none; color: white; cursor: pointer; padding: 2px; border-radius: 2px;"
                                    on:click=move |_| remove_item.run(i)
                                >
                                    <Icon name=IconName::Close size=12 />
                                </button>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================
// Section Highlights Field (for common highlights)
// ============================================

#[component]
fn SectionHighlightsField(
    items: Signal<Vec<String>>,
    on_change: Callback<Vec<String>>,
) -> impl IntoView {
    let new_item = RwSignal::new(String::new());
    let is_expanded = RwSignal::new(false);

    let add_item = Callback::new(move |_: ()| {
        let text = new_item.get();
        if !text.trim().is_empty() {
            let mut current = items.get();
            current.push(text);
            on_change.run(current);
            new_item.set(String::new());
        }
    });

    let remove_item = Callback::new(move |index: usize| {
        let mut current = items.get();
        if index < current.len() {
            current.remove(index);
            on_change.run(current);
        }
    });

    view! {
        <div style="margin-bottom: 1rem;">
            // Header with toggle
            <div
                style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; cursor: pointer;"
                on:click=move |_| is_expanded.update(|v| *v = !*v)
            >
                <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; font-weight: 500; cursor: pointer;">
                    <Icon name=IconName::Star size=16 />
                    "Highlights"
                    <span style="color: #64748b; font-size: 0.75rem;">
                        {move || {
                            let count = items.get().len();
                            if count > 0 { format!("({})", count) } else { "(optional)".to_string() }
                        }}
                    </span>
                </label>
                {move || if is_expanded.get() {
                    view! { <Icon name=IconName::ChevronDown size=14 /> }.into_any()
                } else {
                    view! { <Icon name=IconName::ChevronRight size=16 /> }.into_any()
                }}
            </div>

            // Expandable content
            <Show
                when=move || is_expanded.get()
                fallback=|| view! { <span></span> }
            >
                <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
                    // Add new item input
                    <div style="display: flex; gap: 0.5rem; margin-bottom: 0.75rem;">
                        <input
                            type="text"
                            style="flex: 1; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                            placeholder="Enter highlight text (e.g., '100% Free', 'No Setup Required')..."
                            prop:value=move || new_item.get()
                            on:input=move |ev| new_item.set(event_target_value(&ev))
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    add_item.run(());
                                }
                            }
                        />
                        <button
                            type="button"
                            style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                            on:click=move |_| add_item.run(())
                        >
                            "Add"
                        </button>
                    </div>

                    // Items list as chips
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                        {move || {
                            let current_items = items.get();
                            if current_items.is_empty() {
                                view! {
                                    <div style="color: #64748b; font-size: 0.75rem; font-style: italic;">
                                        "No highlights added yet. Add feature bullets like \"100% Free\" or \"Works Offline\"."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; width: 100%;">
                                        {current_items.into_iter().enumerate().map(|(i, text)| {
                                            view! {
                                                <span style="display: flex; align-items: center; gap: 0.5rem; background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); color: white; border-radius: 9999px; padding: 0.375rem 0.75rem 0.375rem 1rem; font-size: 0.8rem; font-weight: 500;">
                                                    {text}
                                                    <button
                                                        type="button"
                                                        style="background: rgba(255,255,255,0.2); border: none; color: white; cursor: pointer; padding: 2px; border-radius: 50%; line-height: 1; display: flex; align-items: center; justify-content: center;"
                                                        on:click=move |_| remove_item.run(i)
                                                    >
                                                        <Icon name=IconName::Close size=12 />
                                                    </button>
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}

// ============================================
// Section Media Field (for common images/videos) - Uses GCS Upload
// ============================================

#[component]
fn SectionMediaField(
    label: String,
    media_type: MediaType,
    items: Signal<Vec<String>>,
    on_change: Callback<Vec<String>>,
) -> impl IntoView {
    let accept_types = match media_type {
        MediaType::Image => "image/*".to_string(),
        MediaType::Video => "video/*".to_string(),
    };

    let upload_label = match media_type {
        MediaType::Image => "images".to_string(),
        MediaType::Video => "videos".to_string(),
    };

    view! {
        <div style="margin-bottom: 1.5rem;">
            <label style="display: block; color: #94a3b8; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem;">
                {label.clone()}
            </label>
            <GcsMediaUploadList
                value=items
                on_change=on_change
                content_type="home".to_string()
                accept=accept_types
                label=upload_label
            />
        </div>
    }
}

// ============================================
// Steps Field (for "How It Works" section)
// ============================================

/// Icon options for steps
const STEP_ICON_OPTIONS: &[(&str, &str)] = &[
    ("download", "Download"),
    ("lightning", "Lightning"),
    ("wallet", "Wallet"),
    ("phone", "Phone"),
    ("settings", "Settings"),
    ("star", "Star"),
    ("check", "Check"),
    ("rocket", "Rocket"),
    ("shield", "Shield"),
    ("clock", "Clock"),
    ("chart", "Chart"),
    ("users", "Users"),
];

const PERIOD_OPTIONS: &[(&str, &str)] = &[
    ("second", "Second"),
    ("minute", "Minute"),
    ("hour", "Hour"),
    ("day", "Day"),
    ("week", "Week"),
    ("month", "Month"),
    ("quarter", "Quarter"),
    ("year", "Year"),
];

#[component]
fn StepsField(
    key: String,
    label: String,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    // Control whether the add form is visible
    let show_form = RwSignal::new(false);

    // Form state for new step
    let new_icon = RwSignal::new("download".to_string());
    let new_title = RwSignal::new(String::new());
    let new_description = RwSignal::new(String::new());

    let key_for_derive = key.clone();
    let steps = Signal::derive(move || {
        data.get().get(&key_for_derive)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    });

    let key_for_add = key.clone();
    let add_step = Callback::new(move |_: ()| {
        let icon = new_icon.get();
        let title = new_title.get();
        let description = new_description.get();

        if title.trim().is_empty() {
            return; // Don't add empty steps
        }

        let new_step = serde_json::json!({
            "icon": icon,
            "title": title,
            "description": description,
            "order": steps.get().len() + 1
        });

        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            let mut arr = obj.get(&key_for_add)
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            arr.push(new_step);
            obj.insert(key_for_add.clone(), serde_json::json!(arr));
        }
        on_change.run(d);

        // Clear form and hide it
        new_icon.set("download".to_string());
        new_title.set(String::new());
        new_description.set(String::new());
        show_form.set(false);
    });

    let cancel_form = move |_| {
        // Clear form and hide it
        new_icon.set("download".to_string());
        new_title.set(String::new());
        new_description.set(String::new());
        show_form.set(false);
    };

    let key_for_remove = key;
    let remove_step = Callback::new(move |index: usize| {
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            if let Some(arr) = obj.get_mut(&key_for_remove).and_then(|v| v.as_array_mut()) {
                if index < arr.len() {
                    arr.remove(index);
                }
            }
        }
        on_change.run(d);
    });

    view! {
        <div style="margin-bottom: 1.5rem;">
            // Header with label and Add Step button
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
                <label style="color: #94a3b8; font-size: 0.875rem; font-weight: 500;">
                    {label}
                </label>
                {move || if !show_form.get() {
                    view! {
                        <button
                            type="button"
                            style="background: #1e293b; color: #e2e8f0; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem; display: flex; align-items: center; gap: 0.25rem;"
                            on:click=move |_| show_form.set(true)
                        >
                            <Icon name=IconName::Plus size=12 />
                            "Add Step"
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Add new step form (only shown when show_form is true)
            {move || if show_form.get() {
                view! {
                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; padding: 1rem; margin-bottom: 1rem;">
                        // Icon dropdown
                        <div style="margin-bottom: 0.75rem;">
                            <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                "Icon"
                            </label>
                            <select
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                                prop:value=move || new_icon.get()
                                on:change=move |ev| new_icon.set(event_target_value(&ev))
                            >
                                {STEP_ICON_OPTIONS.iter().map(|(value, label)| {
                                    let val = value.to_string();
                                    let val_for_select = value.to_string();
                                    view! {
                                        <option value=val selected=move || new_icon.get() == val_for_select>
                                            {*label}
                                        </option>
                                    }
                                }).collect_view()}
                            </select>
                        </div>

                        // Step Title
                        <div style="margin-bottom: 0.75rem;">
                            <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                "Step Title"
                            </label>
                            <input
                                type="text"
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="e.g., Download App"
                                prop:value=move || new_title.get()
                                on:input=move |ev| new_title.set(event_target_value(&ev))
                            />
                        </div>

                        // Step Description (rich text)
                        <div style="margin-bottom: 0.75rem;">
                            <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                "Step Description"
                            </label>
                            <div style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                                <div style="display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #0f172a;">
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-weight: bold;" title="Bold">"B"</button>
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-style: italic;" title="Italic">"I"</button>
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="Link">"Link"</button>
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="List">"List"</button>
                                </div>
                                <textarea
                                    style="width: 100%; min-height: 60px; background: transparent; border: none; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; resize: vertical;"
                                    placeholder="Describe what happens in this step..."
                                    prop:value=move || new_description.get()
                                    on:input=move |ev| new_description.set(event_target_value(&ev))
                                ></textarea>
                            </div>
                        </div>

                        // Cancel and Add Step buttons
                        <div style="display: flex; justify-content: flex-end; gap: 0.5rem;">
                            <button
                                type="button"
                                style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                on:click=cancel_form
                            >
                                "Cancel"
                            </button>
                            <button
                                type="button"
                                style="background: #3b82f6; color: white; border: none; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                on:click=move |_| add_step.run(())
                            >
                                "Add Step"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Steps table
            {move || {
                let current_steps = steps.get();
                if current_steps.is_empty() {
                    view! {
                        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; padding: 1.5rem; text-align: center;">
                            <p style="color: #64748b; font-size: 0.875rem; margin: 0;">
                                "No steps added yet. Click \"Add Step\" to create one."
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; overflow: hidden;">
                            // Table header
                            <div style="display: grid; grid-template-columns: 40px 80px 1fr 2fr 50px; gap: 0.5rem; padding: 0.75rem 1rem; background: #1e293b; border-bottom: 1px solid #334155; font-size: 0.75rem; font-weight: 600; color: #94a3b8;">
                                <span>"#"</span>
                                <span>"Icon"</span>
                                <span>"Title"</span>
                                <span>"Description"</span>
                                <span></span>
                            </div>
                            // Table rows
                            {current_steps.into_iter().enumerate().map(|(i, step)| {
                                let icon = step.get("icon").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let title = step.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let description = step.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                view! {
                                    <div style="display: grid; grid-template-columns: 40px 80px 1fr 2fr 50px; gap: 0.5rem; padding: 0.75rem 1rem; border-bottom: 1px solid #334155; font-size: 0.875rem; color: #e2e8f0; align-items: center;">
                                        <span style="color: #64748b;">{i + 1}</span>
                                        <span style="color: #94a3b8; font-size: 0.75rem;">{icon}</span>
                                        <span style="font-weight: 500;">{title}</span>
                                        <span style="color: #94a3b8; font-size: 0.75rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                                            {description}
                                        </span>
                                        <button
                                            type="button"
                                            style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                                            on:click=move |_| remove_step.run(i)
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
    }
}

// ============================================
// Tiers Field (for Earnings section)
// ============================================

#[component]
fn TiersField(
    key: String,
    label: String,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    // Control whether the add form is visible
    let show_form = RwSignal::new(false);

    // Form state for new tier
    let new_name = RwSignal::new(String::new());
    let new_min_earnings = RwSignal::new(String::new());
    let new_max_earnings = RwSignal::new(String::new());
    let new_period = RwSignal::new("month".to_string());
    let new_features: RwSignal<Vec<String>> = RwSignal::new(Vec::new());
    let new_feature_input = RwSignal::new(String::new());
    let new_is_popular = RwSignal::new(false);

    let key_for_derive = key.clone();
    let tiers = Signal::derive(move || {
        data.get().get(&key_for_derive)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    });

    // Add a feature to the list
    let add_feature = Callback::new(move |_: ()| {
        let feature = new_feature_input.get();
        if !feature.trim().is_empty() {
            new_features.update(|f| f.push(feature.trim().to_string()));
            new_feature_input.set(String::new());
        }
    });

    // Remove a feature from the list
    let remove_feature = move |index: usize| {
        new_features.update(|f| {
            if index < f.len() {
                f.remove(index);
            }
        });
    };

    let key_for_add = key.clone();
    let add_tier = Callback::new(move |_: ()| {
        let name = new_name.get();
        let min_earnings_str = new_min_earnings.get();
        let max_earnings_str = new_max_earnings.get();
        let period = new_period.get();
        let features = new_features.get();
        let is_popular = new_is_popular.get();

        if name.trim().is_empty() {
            return; // Don't add empty tiers
        }

        let min_earnings: f64 = min_earnings_str.parse().unwrap_or(0.0);
        let max_earnings: f64 = max_earnings_str.parse().unwrap_or(0.0);

        let new_tier = serde_json::json!({
            "name": name,
            "min_earnings": min_earnings,
            "max_earnings": max_earnings,
            "period": period,
            "features": features,
            "is_popular": is_popular
        });

        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            let mut arr = obj.get(&key_for_add)
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            arr.push(new_tier);
            obj.insert(key_for_add.clone(), serde_json::json!(arr));
        }
        on_change.run(d);

        // Clear form and hide it
        new_name.set(String::new());
        new_min_earnings.set(String::new());
        new_max_earnings.set(String::new());
        new_period.set("month".to_string());
        new_features.set(Vec::new());
        new_feature_input.set(String::new());
        new_is_popular.set(false);
        show_form.set(false);
    });

    let cancel_form = move |_| {
        // Clear form and hide it
        new_name.set(String::new());
        new_min_earnings.set(String::new());
        new_max_earnings.set(String::new());
        new_period.set("month".to_string());
        new_features.set(Vec::new());
        new_feature_input.set(String::new());
        new_is_popular.set(false);
        show_form.set(false);
    };

    let key_for_remove = key;
    let remove_tier = Callback::new(move |index: usize| {
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            if let Some(arr) = obj.get_mut(&key_for_remove).and_then(|v| v.as_array_mut()) {
                if index < arr.len() {
                    arr.remove(index);
                }
            }
        }
        on_change.run(d);
    });

    view! {
        <div style="margin-bottom: 1.5rem;">
            // Header with label and Add Tier button
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
                <label style="color: #94a3b8; font-size: 0.875rem; font-weight: 500;">
                    {label}
                </label>
                {move || if !show_form.get() {
                    view! {
                        <button
                            type="button"
                            style="background: #1e293b; color: #e2e8f0; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem; display: flex; align-items: center; gap: 0.25rem;"
                            on:click=move |_| show_form.set(true)
                        >
                            <Icon name=IconName::Plus size=12 />
                            "Add Tier"
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Add new tier form (only shown when show_form is true)
            {move || if show_form.get() {
                view! {
                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; padding: 1rem; margin-bottom: 1rem;">
                        // Tier Name
                        <div style="margin-bottom: 0.75rem;">
                            <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                "Tier Name"
                            </label>
                            <input
                                type="text"
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="e.g., 1 Device, 2-3 Devices"
                                prop:value=move || new_name.get()
                                on:input=move |ev| new_name.set(event_target_value(&ev))
                            />
                        </div>

                        // Min/Max Earnings row
                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 0.75rem; margin-bottom: 0.75rem;">
                            <div>
                                <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                    "Min Earnings"
                                </label>
                                <input
                                    type="number"
                                    style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                    placeholder="5.0"
                                    prop:value=move || new_min_earnings.get()
                                    on:input=move |ev| new_min_earnings.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                    "Max Earnings"
                                </label>
                                <input
                                    type="number"
                                    style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                    placeholder="10.0"
                                    prop:value=move || new_max_earnings.get()
                                    on:input=move |ev| new_max_earnings.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label style="display: block; color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                    "Period"
                                </label>
                                <select
                                    style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                                    prop:value=move || new_period.get()
                                    on:change=move |ev| new_period.set(event_target_value(&ev))
                                >
                                    {PERIOD_OPTIONS.iter().map(|(value, label)| {
                                        let val = value.to_string();
                                        let val_for_select = value.to_string();
                                        view! {
                                            <option value=val selected=move || new_period.get() == val_for_select>
                                                {*label}
                                            </option>
                                        }
                                    }).collect_view()}
                                </select>
                            </div>
                        </div>

                        // Features section with Add Feature button
                        <div style="margin-bottom: 0.75rem;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.25rem;">
                                <label style="color: #64748b; font-size: 0.75rem;">
                                    "Features"
                                </label>
                            </div>

                            // Feature input row
                            <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                                <input
                                    type="text"
                                    style="flex: 1; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                    placeholder="Enter a feature..."
                                    prop:value=move || new_feature_input.get()
                                    on:input=move |ev| new_feature_input.set(event_target_value(&ev))
                                    on:keypress=move |ev: web_sys::KeyboardEvent| {
                                        if ev.key() == "Enter" {
                                            ev.prevent_default();
                                            add_feature.run(());
                                        }
                                    }
                                />
                                <button
                                    type="button"
                                    style="background: #22c55e; color: white; border: none; border-radius: 0.375rem; padding: 0.5rem 0.75rem; cursor: pointer; font-size: 0.75rem; white-space: nowrap;"
                                    on:click=move |_| add_feature.run(())
                                >
                                    "+ Add Feature"
                                </button>
                            </div>

                            // Display added features as chips
                            <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                                {move || {
                                    new_features.get().into_iter().enumerate().map(|(i, feature)| {
                                        view! {
                                            <span style="display: inline-flex; align-items: center; gap: 0.25rem; background: #334155; color: #e2e8f0; padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.75rem;">
                                                {feature}
                                                <button
                                                    type="button"
                                                    style="background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 0; margin-left: 0.25rem; font-size: 1rem; line-height: 1;"
                                                    on:click=move |_| remove_feature(i)
                                                >
                                                    "×"
                                                </button>
                                            </span>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        </div>

                        // Most Popular checkbox and buttons row
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                <input
                                    type="checkbox"
                                    style="width: 1rem; height: 1rem; accent-color: #3b82f6;"
                                    prop:checked=move || new_is_popular.get()
                                    on:change=move |ev| {
                                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                                        new_is_popular.set(target.checked());
                                    }
                                />
                                <span style="color: #94a3b8; font-size: 0.875rem;">"Most Popular"</span>
                            </label>
                            <div style="display: flex; gap: 0.5rem;">
                                <button
                                    type="button"
                                    style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                    on:click=cancel_form
                                >
                                    "Cancel"
                                </button>
                                <button
                                    type="button"
                                    style="background: #3b82f6; color: white; border: none; border-radius: 0.375rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                    on:click=move |_| add_tier.run(())
                                >
                                    "Add Tier"
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Tiers table
            {move || {
                let current_tiers = tiers.get();
                if current_tiers.is_empty() {
                    view! {
                        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; padding: 1.5rem; text-align: center;">
                            <p style="color: #64748b; font-size: 0.875rem; margin: 0;">
                                "No tiers added yet. Click \"Add Tier\" to create one."
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.5rem; overflow: hidden;">
                            // Table header
                            <div style="display: grid; grid-template-columns: 40px 1fr 120px 80px 70px 50px; gap: 0.5rem; padding: 0.75rem 1rem; background: #1e293b; border-bottom: 1px solid #334155; font-size: 0.75rem; font-weight: 600; color: #94a3b8;">
                                <span>"#"</span>
                                <span>"Name"</span>
                                <span>"Earnings"</span>
                                <span>"Features"</span>
                                <span>"Popular"</span>
                                <span></span>
                            </div>
                            // Table rows
                            {current_tiers.into_iter().enumerate().map(|(i, tier)| {
                                let name = tier.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let min_e = tier.get("min_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let max_e = tier.get("max_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let period = tier.get("period").and_then(|v| v.as_str()).unwrap_or("month").to_string();
                                let features_count = tier.get("features").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                                let is_popular = tier.get("is_popular").and_then(|v| v.as_bool()).unwrap_or(false);
                                let earnings_str = format!("${}-${}/{}", min_e, max_e, period);
                                view! {
                                    <div style="display: grid; grid-template-columns: 40px 1fr 120px 80px 70px 50px; gap: 0.5rem; padding: 0.75rem 1rem; border-bottom: 1px solid #334155; font-size: 0.875rem; color: #e2e8f0; align-items: center;">
                                        <span style="color: #64748b;">{i + 1}</span>
                                        <span style="font-weight: 500;">{name}</span>
                                        <span style="color: #10b981; font-size: 0.75rem;">{earnings_str}</span>
                                        <span style="color: #94a3b8; font-size: 0.75rem;">{format!("{} items", features_count)}</span>
                                        <span>
                                            {if is_popular {
                                                view! { <span style="background: #22c55e; color: white; padding: 0.125rem 0.5rem; border-radius: 9999px; font-size: 0.625rem;">"Yes"</span> }.into_any()
                                            } else {
                                                view! { <span style="color: #64748b; font-size: 0.75rem;">"-"</span> }.into_any()
                                            }}
                                        </span>
                                        <button
                                            type="button"
                                            style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                                            on:click=move |_| remove_tier.run(i)
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
    }
}

// ============================================
// Repeater Field
// ============================================

#[component]
fn RepeaterField(
    key: String,
    label: String,
    sub_fields: Vec<FieldDef>,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
) -> impl IntoView {
    // Store sub_fields for use in closures
    let sub_fields = StoredValue::new(sub_fields);
    let key_stored = StoredValue::new(key.clone());

    let key_for_derive = key.clone();
    let item_count = Signal::derive(move || {
        data.get().get(&key_for_derive)
            .and_then(|v| v.as_array())
            .map(|a| a.len())
            .unwrap_or(0)
    });

    let key_for_add = key.clone();
    let add_item = Callback::new(move |_: ()| {
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            let mut arr = obj.get(&key_for_add)
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            arr.push(serde_json::json!({}));
            obj.insert(key_for_add.clone(), serde_json::json!(arr));
        }
        on_change.run(d);
    });

    let key_for_remove = key;
    let remove_item = Callback::new(move |index: usize| {
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            if let Some(arr) = obj.get_mut(&key_for_remove).and_then(|v| v.as_array_mut()) {
                if index < arr.len() {
                    arr.remove(index);
                }
            }
        }
        on_change.run(d);
    });

    view! {
        <div style="margin-bottom: 1rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                <label style="color: #94a3b8; font-size: 0.875rem; font-weight: 500;">
                    {label}
                </label>
                <button
                    type="button"
                    style="background: #334155; color: #e2e8f0; border: none; border-radius: 0.25rem; padding: 0.25rem 0.5rem; cursor: pointer; font-size: 0.75rem; display: flex; align-items: center; gap: 0.25rem;"
                    on:click=move |_| add_item.run(())
                >
                    <Icon name=IconName::Plus size=14 />
                    "Add Item"
                </button>
            </div>

            <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                {move || {
                    let count = item_count.get();
                    (0..count).map(|i| {
                        view! {
                            <RepeaterItem
                                index=i
                                field_key=key_stored
                                sub_fields=sub_fields
                                data=data
                                on_change=on_change
                                on_remove=remove_item
                            />
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

/// Individual repeater item component to avoid closure issues
#[component]
fn RepeaterItem(
    index: usize,
    field_key: StoredValue<String>,
    sub_fields: StoredValue<Vec<FieldDef>>,
    data: Signal<serde_json::Value>,
    on_change: Callback<serde_json::Value>,
    on_remove: Callback<usize>,
) -> impl IntoView {
    let item_data = Signal::derive(move || {
        let key = field_key.get_value();
        data.get().get(&key)
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.get(index))
            .cloned()
            .unwrap_or(serde_json::json!({}))
    });

    let update_item = Callback::new(move |new_item_data: serde_json::Value| {
        let key = field_key.get_value();
        let mut d = data.get();
        if let Some(obj) = d.as_object_mut() {
            if let Some(arr) = obj.get_mut(&key).and_then(|v| v.as_array_mut()) {
                if index < arr.len() {
                    arr[index] = new_item_data;
                }
            }
        }
        on_change.run(d);
    });

    let fields = sub_fields.get_value();

    view! {
        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
                <span style="color: #64748b; font-size: 0.75rem; font-weight: 500;">
                    {format!("Item {}", index + 1)}
                </span>
                <button
                    type="button"
                    style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                    on:click=move |_| on_remove.run(index)
                >
                    <Icon name=IconName::Trash size=14 />
                </button>
            </div>

            <SchemaFields
                fields=fields
                data=item_data
                on_change=update_item
            />
        </div>
    }
}

// ============================================
// Section Links Field (for download buttons, CTAs)
// ============================================

#[component]
fn SectionLinksField(
    items: Signal<Vec<SectionLink>>,
    on_change: Callback<Vec<SectionLink>>,
) -> impl IntoView {
    let is_expanded = RwSignal::new(false);
    let show_add_form = RwSignal::new(false);

    // New link form state
    let new_platform = RwSignal::new(LinkPlatform::AppStore);
    let new_label = RwSignal::new(String::new());
    let new_sublabel = RwSignal::new(String::new());
    let new_href = RwSignal::new(String::new());

    let add_link = Callback::new(move |_: ()| {
        let href = new_href.get();
        if !href.trim().is_empty() {
            let platform = new_platform.get();
            let mut current = items.get();

            let link = SectionLink {
                platform: platform.clone(),
                label: if new_label.get().is_empty() {
                    platform.default_label().to_string()
                } else {
                    new_label.get()
                },
                sublabel: if new_sublabel.get().is_empty() {
                    platform.default_sublabel().to_string()
                } else {
                    new_sublabel.get()
                },
                href,
                target: "_blank".to_string(),
                rel: "noopener noreferrer".to_string(),
                icon: None,
                is_visible: true,
            };

            current.push(link);
            on_change.run(current);

            // Reset form
            new_platform.set(LinkPlatform::AppStore);
            new_label.set(String::new());
            new_sublabel.set(String::new());
            new_href.set(String::new());
            show_add_form.set(false);
        }
    });

    let remove_link = Callback::new(move |index: usize| {
        let mut current = items.get();
        if index < current.len() {
            current.remove(index);
            on_change.run(current);
        }
    });

    let toggle_visibility = Callback::new(move |index: usize| {
        let mut current = items.get();
        if index < current.len() {
            current[index].is_visible = !current[index].is_visible;
            on_change.run(current);
        }
    });

    view! {
        <div style="margin-bottom: 1rem;">
            // Header with toggle
            <div
                style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; cursor: pointer;"
                on:click=move |_| is_expanded.update(|v| *v = !*v)
            >
                <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; font-weight: 500; cursor: pointer;">
                    <Icon name=IconName::Link size=16 />
                    "Download Links / Buttons"
                    <span style="color: #64748b; font-size: 0.75rem;">
                        {move || {
                            let count = items.get().len();
                            if count > 0 { format!("({})", count) } else { "(optional)".to_string() }
                        }}
                    </span>
                </label>
                {move || if is_expanded.get() {
                    view! { <Icon name=IconName::ChevronDown size=14 /> }.into_any()
                } else {
                    view! { <Icon name=IconName::ChevronRight size=16 /> }.into_any()
                }}
            </div>

            // Expandable content
            <Show
                when=move || is_expanded.get()
                fallback=|| view! { <span></span> }
            >
                <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
                    // Add button
                    <div style="margin-bottom: 0.75rem;">
                        <button
                            type="button"
                            style="background: #334155; color: #e2e8f0; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; gap: 0.5rem;"
                            on:click=move |_| show_add_form.update(|v| *v = !*v)
                        >
                            <Icon name=IconName::Plus size=14 />
                            "Add Download Button"
                        </button>
                    </div>

                    // Add form
                    <Show
                        when=move || show_add_form.get()
                        fallback=|| view! { <span></span> }
                    >
                        <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem; margin-bottom: 1rem;">
                            <div style="display: grid; gap: 0.75rem;">
                                // Platform selector
                                <div>
                                    <label style="display: block; color: #94a3b8; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                        "Platform"
                                    </label>
                                    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                        <PlatformButton
                                            platform=LinkPlatform::AppStore
                                            selected=new_platform
                                            label="App Store"
                                            icon="apple"
                                        />
                                        <PlatformButton
                                            platform=LinkPlatform::GooglePlay
                                            selected=new_platform
                                            label="Google Play"
                                            icon="playstore"
                                        />
                                        <PlatformButton
                                            platform=LinkPlatform::Apk
                                            selected=new_platform
                                            label="APK"
                                            icon="android"
                                        />
                                        <PlatformButton
                                            platform=LinkPlatform::Custom
                                            selected=new_platform
                                            label="Custom"
                                            icon="link"
                                        />
                                    </div>
                                </div>

                                // URL input
                                <div>
                                    <label style="display: block; color: #94a3b8; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                        "URL" <span style="color: #ef4444;">*</span>
                                    </label>
                                    <input
                                        type="text"
                                        style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                        placeholder="https://..."
                                        prop:value=move || new_href.get()
                                        on:input=move |ev| new_href.set(event_target_value(&ev))
                                    />
                                </div>

                                // Optional label override
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem;">
                                    <div>
                                        <label style="display: block; color: #94a3b8; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Label (optional)"
                                        </label>
                                        <input
                                            type="text"
                                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                            placeholder=move || new_platform.get().default_label()
                                            prop:value=move || new_label.get()
                                            on:input=move |ev| new_label.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <div>
                                        <label style="display: block; color: #94a3b8; font-size: 0.75rem; margin-bottom: 0.25rem;">
                                            "Sublabel (optional)"
                                        </label>
                                        <input
                                            type="text"
                                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                            placeholder=move || new_platform.get().default_sublabel()
                                            prop:value=move || new_sublabel.get()
                                            on:input=move |ev| new_sublabel.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>

                                // Add button
                                <div style="display: flex; justify-content: flex-end; gap: 0.5rem;">
                                    <button
                                        type="button"
                                        style="background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                        on:click=move |_| show_add_form.set(false)
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        type="button"
                                        style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                        on:click=move |_| add_link.run(())
                                    >
                                        "Add Link"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </Show>

                    // Links list
                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        {move || {
                            let current_items = items.get();
                            if current_items.is_empty() && !show_add_form.get() {
                                view! {
                                    <div style="color: #64748b; font-size: 0.75rem; font-style: italic; text-align: center; padding: 1rem;">
                                        "No download buttons added yet. Add buttons for App Store, Google Play, or APK downloads."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                        {current_items.into_iter().enumerate().map(|(i, link)| {
                                            let platform_icon = link.platform.icon();
                                            let platform_color = match link.platform {
                                                LinkPlatform::AppStore => "#000000",
                                                LinkPlatform::GooglePlay => "#34a853",
                                                LinkPlatform::Apk => "#3ddc84",
                                                LinkPlatform::Custom => "#64748b",
                                            };

                                            view! {
                                                <div style=format!(
                                                    "display: flex; align-items: center; gap: 0.75rem; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.75rem; border-left: 3px solid {}; opacity: {};",
                                                    platform_color,
                                                    if link.is_visible { "1" } else { "0.5" }
                                                )>
                                                    // Platform icon
                                                    <div style=format!("width: 32px; height: 32px; background: {}; border-radius: 0.25rem; display: flex; align-items: center; justify-content: center;", platform_color)>
                                                        <PlatformIcon icon=platform_icon />
                                                    </div>

                                                    // Link info
                                                    <div style="flex: 1; min-width: 0;">
                                                        <div style="display: flex; align-items: center; gap: 0.5rem;">
                                                            <span style="color: #64748b; font-size: 0.65rem;">
                                                                {link.sublabel.clone()}
                                                            </span>
                                                        </div>
                                                        <div style="color: #e2e8f0; font-weight: 500; font-size: 0.875rem;">
                                                            {link.label.clone()}
                                                        </div>
                                                        <div style="color: #64748b; font-size: 0.7rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                                                            {link.href.clone()}
                                                        </div>
                                                    </div>

                                                    // Actions
                                                    <div style="display: flex; gap: 0.25rem;">
                                                        <button
                                                            type="button"
                                                            title=move || if link.is_visible { "Hide" } else { "Show" }
                                                            style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 0.25rem;"
                                                            on:click=move |_| toggle_visibility.run(i)
                                                        >
                                                            <Icon name=IconName::Eye size=16 />
                                                        </button>
                                                        <button
                                                            type="button"
                                                            title="Remove"
                                                            style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                                                            on:click=move |_| remove_link.run(i)
                                                        >
                                                            <Icon name=IconName::Trash size=14 />
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// Platform selection button
#[component]
fn PlatformButton(
    platform: LinkPlatform,
    selected: RwSignal<LinkPlatform>,
    label: &'static str,
    icon: &'static str,
) -> impl IntoView {
    let is_selected = move || selected.get() == platform;
    let platform_clone = platform.clone();

    view! {
        <button
            type="button"
            style=move || format!(
                "display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; border-radius: 0.25rem; cursor: pointer; font-size: 0.75rem; transition: all 0.15s; {}",
                if is_selected() {
                    "background: #3b82f6; color: white; border: 1px solid #3b82f6;"
                } else {
                    "background: #1e293b; color: #94a3b8; border: 1px solid #334155;"
                }
            )
            on:click=move |_| selected.set(platform_clone.clone())
        >
            <PlatformIcon icon=icon />
            {label}
        </button>
    }
}

/// Platform icon component
#[component]
fn PlatformIcon(icon: &'static str) -> impl IntoView {
    match icon {
        "apple" => view! {
            <svg style="width: 16px; height: 16px;" viewBox="0 0 24 24" fill="currentColor">
                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
            </svg>
        }.into_any(),
        "playstore" => view! {
            <svg style="width: 16px; height: 16px;" viewBox="0 0 24 24" fill="currentColor">
                <path d="M3 20.5v-17c0-.59.34-1.11.84-1.35L13.69 12l-9.85 9.85c-.5-.25-.84-.76-.84-1.35zm13.81-5.38L6.05 21.34l8.49-8.49 2.27 2.27zm3.35-4.31c.34.27.59.69.59 1.19s-.22.9-.57 1.18l-2.29 1.32-2.5-2.5 2.5-2.5 2.27 1.31zM6.05 2.66l10.76 6.22-2.27 2.27L6.05 2.66z"/>
            </svg>
        }.into_any(),
        "android" => view! {
            <svg style="width: 16px; height: 16px;" viewBox="0 0 24 24" fill="currentColor">
                <path d="M5 16c0 3.87 3.13 7 7 7s7-3.13 7-7v-4H5v4zM16.12 4.37l2.1-2.1-.82-.83-2.3 2.31C14.16 3.28 13.12 3 12 3s-2.16.28-3.09.75L6.6 1.44l-.82.83 2.1 2.1C6.14 5.64 5 7.68 5 10v1h14v-1c0-2.32-1.14-4.36-2.88-5.63zM9 9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z"/>
            </svg>
        }.into_any(),
        _ => view! {
            <Icon name=IconName::Link size=16 />
        }.into_any(),
    }
}

// ============================================
// Home Content Editor (for Content Editor page)
// ============================================

const FORM_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem;";

use super::ValidationError;

/// Home Content Editor - similar to TaskContentEditor but for Home page content
/// Uses HomeSectionEditor with its modal for adding sections
#[component]
pub fn HomeContentEditor(
    /// The form data signal to read from and write to
    data: RwSignal<serde_json::Value>,
    /// Validation errors
    #[prop(optional)]
    errors: Option<Signal<Vec<ValidationError>>>,
    /// Whether the form is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    // Initialize signals directly from form data
    let initial_data = data.get_untracked();

    let has_title = initial_data.get("title").and_then(|v| v.as_str()).is_some();
    let initial_title = initial_data
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Home Page")
        .to_string();

    let initial_sections = initial_data
        .get("sections")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // Create RwSignals with initial values
    let title_signal = RwSignal::new(initial_title.clone());
    let sections_signal = RwSignal::new(initial_sections);

    // Sync default title to data if it wasn't present (ensures it gets saved)
    // Use Effect to defer this until after hydration to avoid hydration mismatch
    if !has_title {
        let default_title = initial_title.clone();
        Effect::new(move |_| {
            data.update(|d| {
                if let Some(obj) = d.as_object_mut() {
                    if !obj.contains_key("title") {
                        obj.insert("title".to_string(), serde_json::Value::String(default_title.clone()));
                    }
                }
            });
        });
    }

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
        <div class="home-content-editor" style=FORM_STYLE>
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

            // Title field
            <div style="margin-bottom: 1.5rem;">
                <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                    "Title"
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
                    placeholder="Home Page"
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

            // Sections with Add Section button (uses modal)
            <HomeSectionEditor
                value=sections_signal
                label="Sections".to_string()
                on_change=on_sections_change
                read_only=read_only
            />
        </div>
    }
}
