//! Guide Section Editor Component
//!
//! A specialized editor for Guide sections that displays sections in a repeater-style
//! list with collapsible accordions for viewing details, and uses a modal for adding new sections.
//! Each guide section contains: section_type, title, description, images, videos, highlights,
//! links, display_order, and is_visible.
//!
//! Also provides GuideContentEditor - a complete guide form component that mirrors the Task form layout:
//! - Title field (required)
//! - Description field with rich text (required)
//! - Sections with Add Section button (modal-based)

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::common::icon::{Icon, IconName};
use crate::components::forms::ValidationError;
use crate::api::section_types::{SectionLink, LinkPlatform};
use super::gcs_media_upload::GcsMediaUploadList;

// Styling constants
const FORM_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem;";
const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const INPUT_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
const TOOLBAR_STYLE: &str = "display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #0f172a;";
const TOOLBAR_BTN_STYLE: &str = "padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;";

/// Data structure for a guide step
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuideStepData {
    pub order: i32,
    pub title: String,
    pub description: String,
    pub images: Vec<String>,
}

impl GuideStepData {
    pub fn from_json(value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            order: obj.and_then(|o| o.get("order")).and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            title: obj.and_then(|o| o.get("title")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            description: obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            images: obj.and_then(|o| o.get("images"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "order": self.order,
            "title": self.title,
            "description": self.description,
            "images": self.images
        })
    }

    pub fn display_title(&self) -> String {
        if !self.title.is_empty() {
            format!("Step {}: {}", self.order, self.title)
        } else {
            format!("Step {}", self.order)
        }
    }
}

/// Data structure for a guide section
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuideSectionData {
    pub title: String,
    pub description: String,
    pub display_order: i32,
    pub is_visible: bool,
    pub highlights: Vec<String>,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub links: Vec<SectionLink>,
    pub steps: Vec<GuideStepData>,
    // New fields for section dependencies and metadata
    pub pre_steps: Vec<usize>,      // Indices of prerequisite sections
    pub complexity: String,          // "easy" or "medium"
    pub estimated_time: String,      // e.g., "5 mins"
}

impl GuideSectionData {
    pub fn from_json(value: &serde_json::Value) -> Self {
        let obj = value.as_object();
        Self {
            title: obj.and_then(|o| o.get("title")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
            description: obj.and_then(|o| o.get("description").or_else(|| o.get("body")))
                .and_then(|v| v.as_str()).unwrap_or("").to_string(),
            display_order: obj.and_then(|o| o.get("display_order")).and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            is_visible: obj.and_then(|o| o.get("is_visible")).and_then(|v| v.as_bool()).unwrap_or(true),
            highlights: obj.and_then(|o| o.get("highlights"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            images: obj.and_then(|o| o.get("cover_images").or_else(|| o.get("images")))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            videos: obj.and_then(|o| o.get("cover_videos").or_else(|| o.get("videos")))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            links: obj.and_then(|o| o.get("links"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(|v| {
                    let link_obj = v.as_object();
                    SectionLink {
                        platform: link_obj.and_then(|o| o.get("platform"))
                            .and_then(|v| v.as_str())
                            .map(|s| match s {
                                "app_store" => LinkPlatform::AppStore,
                                "google_play" => LinkPlatform::GooglePlay,
                                "apk" => LinkPlatform::Apk,
                                _ => LinkPlatform::Custom,
                            })
                            .unwrap_or(LinkPlatform::Custom),
                        label: link_obj.and_then(|o| o.get("label")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        sublabel: link_obj.and_then(|o| o.get("sublabel")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        href: link_obj.and_then(|o| o.get("href")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        target: link_obj.and_then(|o| o.get("target")).and_then(|v| v.as_str()).unwrap_or("_blank").to_string(),
                        rel: link_obj.and_then(|o| o.get("rel")).and_then(|v| v.as_str()).unwrap_or("noopener noreferrer").to_string(),
                        icon: link_obj.and_then(|o| o.get("icon")).and_then(|v| v.as_str()).map(String::from),
                        is_visible: link_obj.and_then(|o| o.get("is_visible")).and_then(|v| v.as_bool()).unwrap_or(true),
                    }
                }).collect())
                .unwrap_or_default(),
            steps: obj.and_then(|o| o.get("steps").or_else(|| o.get("stages")))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(GuideStepData::from_json).collect())
                .unwrap_or_default(),
            // New fields
            pre_steps: obj.and_then(|o| o.get("pre_steps"))
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
                .unwrap_or_default(),
            complexity: obj.and_then(|o| o.get("complexity"))
                .and_then(|v| v.as_str())
                .unwrap_or("easy")
                .to_string(),
            estimated_time: obj.and_then(|o| o.get("estimated_time"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "title": self.title,
            "description": self.description,
            "display_order": self.display_order,
            "is_visible": self.is_visible,
            "highlights": self.highlights,
            "cover_images": self.images,
            "cover_videos": self.videos,
            "links": self.links.iter().map(|l| {
                serde_json::json!({
                    "platform": match l.platform {
                        LinkPlatform::AppStore => "app_store",
                        LinkPlatform::GooglePlay => "google_play",
                        LinkPlatform::Apk => "apk",
                        LinkPlatform::Custom => "custom",
                    },
                    "label": l.label,
                    "sublabel": l.sublabel,
                    "href": l.href,
                    "target": l.target,
                    "rel": l.rel,
                    "icon": l.icon,
                    "is_visible": l.is_visible
                })
            }).collect::<Vec<_>>(),
            "steps": self.steps.iter().map(|s| s.to_json()).collect::<Vec<_>>(),
            // New fields
            "pre_steps": self.pre_steps,
            "complexity": self.complexity,
            "estimated_time": self.estimated_time
        })
    }

    /// Get display title for the section
    pub fn display_title(&self) -> String {
        if !self.title.is_empty() {
            self.title.clone()
        } else {
            "Untitled Section".to_string()
        }
    }

    /// Get default section for new items
    pub fn default() -> Self {
        Self {
            is_visible: true,
            ..Default::default()
        }
    }
}

/// Guide Section Editor Component - mimics the repeater field style
#[component]
pub fn GuideSectionEditor(
    /// Current sections value
    value: Signal<Vec<serde_json::Value>>,
    /// Callback when sections change
    on_change: Callback<Vec<serde_json::Value>>,
    /// Whether the editor is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    // State for expanded accordions (track by index)
    let expanded_sections = RwSignal::new(std::collections::HashSet::<usize>::new());

    // State for the modal
    let show_modal = RwSignal::new(false);
    let modal_section = RwSignal::new(GuideSectionData::default());
    let editing_index = RwSignal::new(None::<usize>);

    // Toggle accordion expanded state
    let toggle_expanded = move |index: usize| {
        expanded_sections.update(|set| {
            if set.contains(&index) {
                set.remove(&index);
            } else {
                set.insert(index);
            }
        });
    };

    // Open modal for new section
    let open_new_section_modal = move |_| {
        editing_index.set(None);
        let mut default_section = GuideSectionData::default();
        default_section.display_order = value.get().len() as i32;
        modal_section.set(default_section);
        show_modal.set(true);
    };

    // Open modal for editing existing section
    let open_edit_section_modal = move |index: usize| {
        let sections = value.get();
        if let Some(section_json) = sections.get(index) {
            editing_index.set(Some(index));
            modal_section.set(GuideSectionData::from_json(section_json));
            show_modal.set(true);
        }
    };

    // Save section from modal
    let save_section = move |_| {
        let section_data = modal_section.get();
        let new_json = section_data.to_json();

        let mut sections = value.get();

        if let Some(index) = editing_index.get() {
            // Update existing section
            if index < sections.len() {
                sections[index] = new_json;
            }
        } else {
            // Add new section
            sections.push(new_json);
        }

        on_change.run(sections);
        show_modal.set(false);
    };

    // Cancel modal
    let cancel_modal = move |_| {
        show_modal.set(false);
    };

    // Delete section
    let delete_section = move |index: usize| {
        let mut sections = value.get();
        if index < sections.len() {
            sections.remove(index);
            on_change.run(sections);
        }
    };

    // Move section up
    let move_up = move |index: usize| {
        if index > 0 {
            let mut sections = value.get();
            sections.swap(index, index - 1);
            on_change.run(sections);
        }
    };

    // Move section down
    let move_down = move |index: usize| {
        let sections = value.get();
        if index < sections.len() - 1 {
            let mut sections = sections;
            sections.swap(index, index + 1);
            on_change.run(sections);
        }
    };

    view! {
        <div class="guide-section-editor">
            // Section list
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                {move || {
                    let sections = value.get();
                    let sections_len = sections.len();
                    sections.into_iter().enumerate().map(|(index, section_json)| {
                        let section_json_clone = section_json.clone();
                        let section_data = GuideSectionData::from_json(&section_json);
                        let is_expanded = expanded_sections.get().contains(&index);
                        let is_first = index == 0;
                        let is_last = index == sections_len - 1;

                        view! {
                            <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.5rem; overflow: hidden;">
                                // Accordion header
                                <div
                                    style="display: flex; align-items: center; padding: 0.75rem 1rem; gap: 0.75rem;"
                                >
                                    // Toggle expand/collapse button (left side)
                                    <button
                                        type="button"
                                        style="background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                        title="Toggle details"
                                        on:click=move |_| toggle_expanded(index)
                                    >
                                        {if is_expanded {
                                            view! { <Icon name=IconName::Minus size=14 /> }.into_any()
                                        } else {
                                            view! { <Icon name=IconName::Plus size=14 /> }.into_any()
                                        }}
                                    </button>

                                    // Section title
                                    <div
                                        style="flex: 1; display: flex; align-items: center; gap: 0.75rem; cursor: pointer;"
                                        on:click=move |_| toggle_expanded(index)
                                    >
                                        <span style="color: #e2e8f0; font-size: 0.875rem; font-weight: 500;">
                                            {section_data.display_title()}
                                        </span>
                                        {if !section_data.is_visible {
                                            view! {
                                                <span style="background: #64748b; color: white; font-size: 0.65rem; padding: 0.125rem 0.5rem; border-radius: 9999px;">
                                                    "Hidden"
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>

                                    // Action buttons (only when not read-only)
                                    {move || if !read_only {
                                        view! {
                                            <div style="display: flex; gap: 0.5rem;" on:click=move |e| e.stop_propagation()>
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
                                                    style={if is_first {
                                                        "background: #374151; border: none; color: #64748b; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    } else {
                                                        "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    }}
                                                    title="Move up"
                                                    disabled=is_first
                                                    on:click=move |_| move_up(index)
                                                >
                                                    <Icon name=IconName::ChevronUp size=14 />
                                                </button>
                                                // Move down button
                                                <button
                                                    type="button"
                                                    style={if is_last {
                                                        "background: #374151; border: none; color: #64748b; cursor: not-allowed; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    } else {
                                                        "background: #374151; border: none; color: white; cursor: pointer; padding: 0.5rem; border-radius: 0.375rem; display: flex; align-items: center; justify-content: center; width: 32px; height: 32px;"
                                                    }}
                                                    title="Move down"
                                                    disabled=is_last
                                                    on:click=move |_| move_down(index)
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
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                </div>

                                // Accordion content (expanded details)
                                {move || if is_expanded {
                                    let data = GuideSectionData::from_json(&section_json_clone);
                                    view! {
                                        <div style="padding: 1rem; border-top: 1px solid #334155; background: #0f172a;">
                                            <div style="display: grid; gap: 0.75rem;">
                                                <div>
                                                    <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Title"</span>
                                                    <span style="color: #e2e8f0; font-size: 0.875rem;">{data.title.clone()}</span>
                                                </div>
                                                {if !data.description.is_empty() {
                                                    view! {
                                                        <div>
                                                            <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Description"</span>
                                                            <div
                                                                style="color: #e2e8f0; font-size: 0.875rem; max-height: 100px; overflow-y: auto;"
                                                                inner_html=data.description.clone()
                                                            />
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                                // Complexity and Estimated Time badges
                                                <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                                    <span style={format!("display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; {}",
                                                        match data.complexity.as_str() {
                                                            "medium" => "background: rgba(251, 191, 36, 0.2); color: #fbbf24;",
                                                            _ => "background: rgba(34, 197, 94, 0.2); color: #22c55e;",
                                                        }
                                                    )}>
                                                        {if data.complexity == "medium" { "Medium" } else { "Easy" }}
                                                    </span>
                                                    {if !data.estimated_time.is_empty() && data.estimated_time != "0" {
                                                        view! {
                                                            <span style="display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; background: rgba(59, 130, 246, 0.2); color: #3b82f6;">
                                                                {format!("⏱ {} mins", data.estimated_time.clone())}
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                    {if !data.pre_steps.is_empty() {
                                                        view! {
                                                            <span style="display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; background: rgba(168, 85, 247, 0.2); color: #a855f7;">
                                                                {format!("{} prerequisite{}", data.pre_steps.len(), if data.pre_steps.len() == 1 { "" } else { "s" })}
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                </div>
                                                <div style="display: flex; gap: 2rem;">
                                                    <div>
                                                        <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Images"</span>
                                                        <span style="color: #e2e8f0; font-size: 0.875rem;">{data.images.len()}</span>
                                                    </div>
                                                    <div>
                                                        <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Videos"</span>
                                                        <span style="color: #e2e8f0; font-size: 0.875rem;">{data.videos.len()}</span>
                                                    </div>
                                                    <div>
                                                        <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Highlights"</span>
                                                        <span style="color: #e2e8f0; font-size: 0.875rem;">{data.highlights.len()}</span>
                                                    </div>
                                                    <div>
                                                        <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Links"</span>
                                                        <span style="color: #e2e8f0; font-size: 0.875rem;">{data.links.len()}</span>
                                                    </div>
                                                    <div>
                                                        <span style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Steps"</span>
                                                        <span style="color: #e2e8f0; font-size: 0.875rem;">{data.steps.len()}</span>
                                                    </div>
                                                </div>
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
            </div>

            // Add Section button (only when not read-only)
            {move || if !read_only {
                view! {
                    <button
                        type="button"
                        style="width: 100%; padding: 0.75rem 1rem; background: #334155; color: #94a3b8; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.5rem; margin-top: 0.5rem;"
                        on:click=open_new_section_modal
                    >
                        <Icon name=IconName::Plus size=14 />
                        "Add Section"
                    </button>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Items count
            <p style="font-size: 0.75rem; color: #64748b; margin-top: 0.5rem;">
                {move || {
                    let count = value.get().len();
                    format!("{} items (min: 1)", count)
                }}
            </p>

            // Modal for adding/editing sections
            {move || if show_modal.get() {
                view! {
                    <GuideSectionModal
                        section=modal_section
                        editing_index=editing_index
                        all_sections=value
                        on_save=save_section
                        on_cancel=cancel_modal
                    />
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

/// Modal component for adding/editing guide sections
#[component]
fn GuideSectionModal(
    section: RwSignal<GuideSectionData>,
    editing_index: RwSignal<Option<usize>>,
    /// All sections in the guide (for pre-steps selection)
    all_sections: Signal<Vec<serde_json::Value>>,
    on_save: impl Fn(()) + 'static + Copy,
    on_cancel: impl Fn(()) + 'static + Copy,
) -> impl IntoView {
    let is_editing = move || editing_index.get().is_some();

    // State for collapsible sections
    let highlights_expanded = RwSignal::new(false);
    let links_expanded = RwSignal::new(false);
    let pre_steps_expanded = RwSignal::new(false);

    // State for adding new highlights
    let new_highlight = RwSignal::new(String::new());

    // Add highlight
    let add_highlight = move |_| {
        let text = new_highlight.get();
        if !text.trim().is_empty() {
            section.update(|s| s.highlights.push(text));
            new_highlight.set(String::new());
        }
    };

    // Remove highlight
    let remove_highlight = move |index: usize| {
        section.update(|s| {
            if index < s.highlights.len() {
                s.highlights.remove(index);
            }
        });
    };

    view! {
        // Modal backdrop
        <div style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); z-index: 1000; display: flex; align-items: center; justify-content: center; padding: 1rem;">
            // Modal content
            <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.75rem; width: 100%; max-width: 700px; max-height: 90vh; overflow-y: auto;">
                // Modal header
                <div style="display: flex; align-items: center; justify-content: space-between; padding: 1rem 1.5rem; border-bottom: 1px solid #334155; position: sticky; top: 0; background: #1e293b; z-index: 10;">
                    <h3 style="color: #e2e8f0; font-size: 1.125rem; font-weight: 600; margin: 0;">
                        {move || if is_editing() { "Edit Section" } else { "Add Section" }}
                    </h3>
                    <button
                        type="button"
                        style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 0.25rem;"
                        on:click=move |_| on_cancel(())
                    >
                        <Icon name=IconName::Close size=20 />
                    </button>
                </div>

                // Modal body
                <div style="padding: 1.5rem; display: flex; flex-direction: column; gap: 1.5rem;">
                    // Title field
                    <div>
                        <label style=LABEL_STYLE>
                            "Title"
                            <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                        </label>
                        <input
                            type="text"
                            style=INPUT_STYLE
                            placeholder="Enter section title..."
                            prop:value=move || section.get().title
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                section.update(|s| s.title = value);
                            }
                        />
                    </div>

                    // Description field with rich text toolbar
                    <div>
                        <label style=LABEL_STYLE>"Description"</label>
                        <div style="border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                            // Toolbar
                            <div style=TOOLBAR_STYLE>
                                <button type="button" style=TOOLBAR_BTN_STYLE>"B"</button>
                                <button type="button" style=TOOLBAR_BTN_STYLE>"I"</button>
                                <button type="button" style=TOOLBAR_BTN_STYLE>"Link"</button>
                                <button type="button" style=TOOLBAR_BTN_STYLE>"List"</button>
                            </div>
                            // Text area
                            <textarea
                                style="width: 100%; background: #1e293b; border: none; padding: 0.75rem; color: #e2e8f0; font-size: 0.875rem; min-height: 150px; resize: vertical; outline: none;"
                                placeholder="Enter description..."
                                prop:value=move || section.get().description
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    section.update(|s| s.description = value);
                                }
                            />
                        </div>
                    </div>

                    // Pre-Steps (collapsible) - prerequisite sections
                    <div>
                        <div
                            style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; cursor: pointer;"
                            on:click=move |_| pre_steps_expanded.update(|v| *v = !*v)
                        >
                            <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; font-weight: 500; cursor: pointer;">
                                <Icon name=IconName::Link size=16 />
                                "Pre-requisite Steps"
                                <span style="color: #64748b; font-size: 0.75rem;">
                                    {move || {
                                        let count = section.get().pre_steps.len();
                                        if count > 0 { format!("({})", count) } else { "(optional)".to_string() }
                                    }}
                                </span>
                            </label>
                            {move || if pre_steps_expanded.get() {
                                view! { <Icon name=IconName::ChevronDown size=14 /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::ChevronRight size=16 /> }.into_any()
                            }}
                        </div>
                        {move || if pre_steps_expanded.get() {
                            let current_index = editing_index.get();
                            let sections = all_sections.get();
                            let selected_pre_steps = section.get().pre_steps.clone();

                            view! {
                                <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
                                    <p style="color: #64748b; font-size: 0.75rem; margin-bottom: 0.75rem;">
                                        "Select sections that must be completed before this one:"
                                    </p>
                                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                        {sections.iter().enumerate().filter_map(|(idx, sec_json)| {
                                            // Skip the current section being edited
                                            if current_index == Some(idx) {
                                                return None;
                                            }
                                            let sec_data = GuideSectionData::from_json(sec_json);
                                            let title = if sec_data.title.is_empty() {
                                                format!("Section {}", idx + 1)
                                            } else {
                                                sec_data.title.clone()
                                            };
                                            let is_selected = selected_pre_steps.contains(&idx);
                                            Some(view! {
                                                <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer; padding: 0.5rem; background: #1e293b; border-radius: 0.25rem;">
                                                    <input
                                                        type="checkbox"
                                                        style="width: 16px; height: 16px; accent-color: #3b82f6; cursor: pointer;"
                                                        checked=is_selected
                                                        on:change=move |ev| {
                                                            let checked = event_target_checked(&ev);
                                                            section.update(|s| {
                                                                if checked {
                                                                    if !s.pre_steps.contains(&idx) {
                                                                        s.pre_steps.push(idx);
                                                                    }
                                                                } else {
                                                                    s.pre_steps.retain(|&i| i != idx);
                                                                }
                                                            });
                                                        }
                                                    />
                                                    <span style="color: #e2e8f0; font-size: 0.875rem;">{title}</span>
                                                </label>
                                            })
                                        }).collect_view()}
                                        {if sections.len() <= 1 || (sections.len() == 2 && current_index.is_some()) {
                                            view! {
                                                <div style="color: #64748b; font-size: 0.75rem; font-style: italic;">
                                                    "No other sections available. Add more sections to set prerequisites."
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    // Complexity and Estimated Time row
                    <div style="display: flex; gap: 1rem;">
                        // Complexity dropdown
                        <div style="flex: 1;">
                            <label style=LABEL_STYLE>"Complexity"</label>
                            <select
                                style=INPUT_STYLE
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    section.update(|s| s.complexity = val);
                                }
                            >
                                <option value="easy" selected=move || section.get().complexity.is_empty() || section.get().complexity == "easy">"Easy"</option>
                                <option value="medium" selected=move || section.get().complexity == "medium">"Medium"</option>
                            </select>
                        </div>
                        // Estimated Time stepper
                        <div style="flex: 1;">
                            <label style=LABEL_STYLE>"Estimated Time (mins)"</label>
                            <div style="display: flex; align-items: center; gap: 0.5rem;">
                                <button
                                    type="button"
                                    style="width: 32px; height: 32px; background: #334155; border: 1px solid #475569; border-radius: 0.25rem; color: #e2e8f0; cursor: pointer; font-size: 1rem; display: flex; align-items: center; justify-content: center;"
                                    on:click=move |_| {
                                        section.update(|s| {
                                            let current: i32 = s.estimated_time.parse().unwrap_or(0);
                                            if current > 1 {
                                                s.estimated_time = (current - 1).to_string();
                                            }
                                        });
                                    }
                                >
                                    "-"
                                </button>
                                <input
                                    type="number"
                                    min="1"
                                    step="1"
                                    style="width: 80px; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; text-align: center; outline: none;"
                                    prop:value=move || section.get().estimated_time
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev);
                                        section.update(|s| s.estimated_time = value);
                                    }
                                />
                                <button
                                    type="button"
                                    style="width: 32px; height: 32px; background: #334155; border: 1px solid #475569; border-radius: 0.25rem; color: #e2e8f0; cursor: pointer; font-size: 1rem; display: flex; align-items: center; justify-content: center;"
                                    on:click=move |_| {
                                        section.update(|s| {
                                            let current: i32 = s.estimated_time.parse().unwrap_or(0);
                                            s.estimated_time = (current + 1).to_string();
                                        });
                                    }
                                >
                                    "+"
                                </button>
                                <span style="color: #64748b; font-size: 0.875rem;">"mins"</span>
                            </div>
                        </div>
                    </div>

                    // Highlights (collapsible)
                    <div>
                        <div
                            style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; cursor: pointer;"
                            on:click=move |_| highlights_expanded.update(|v| *v = !*v)
                        >
                            <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; font-weight: 500; cursor: pointer;">
                                <Icon name=IconName::Star size=16 />
                                "Highlights"
                                <span style="color: #64748b; font-size: 0.75rem;">
                                    {move || {
                                        let count = section.get().highlights.len();
                                        if count > 0 { format!("({})", count) } else { "(optional)".to_string() }
                                    }}
                                </span>
                            </label>
                            {move || if highlights_expanded.get() {
                                view! { <Icon name=IconName::ChevronDown size=14 /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::ChevronRight size=16 /> }.into_any()
                            }}
                        </div>
                        {move || if highlights_expanded.get() {
                            view! {
                                <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
                                    // Add new highlight input
                                    <div style="display: flex; gap: 0.5rem; margin-bottom: 0.75rem;">
                                        <input
                                            type="text"
                                            style="flex: 1; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                            placeholder="Enter highlight text..."
                                            prop:value=move || new_highlight.get()
                                            on:input=move |ev| new_highlight.set(event_target_value(&ev))
                                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                if ev.key() == "Enter" {
                                                    add_highlight(());
                                                }
                                            }
                                        />
                                        <button
                                            type="button"
                                            style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                            on:click=move |_| add_highlight(())
                                        >
                                            "Add"
                                        </button>
                                    </div>
                                    // Highlights list as chips
                                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                                        {move || {
                                            let highlights = section.get().highlights.clone();
                                            if highlights.is_empty() {
                                                view! {
                                                    <div style="color: #64748b; font-size: 0.75rem; font-style: italic;">
                                                        "No highlights added yet."
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; width: 100%;">
                                                        {highlights.into_iter().enumerate().map(|(i, text)| {
                                                            view! {
                                                                <span style="display: flex; align-items: center; gap: 0.5rem; background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); color: white; border-radius: 9999px; padding: 0.375rem 0.75rem 0.375rem 1rem; font-size: 0.8rem; font-weight: 500;">
                                                                    {text}
                                                                    <button
                                                                        type="button"
                                                                        style="background: rgba(255,255,255,0.2); border: none; color: white; cursor: pointer; padding: 2px; border-radius: 50%; line-height: 1; display: flex; align-items: center; justify-content: center;"
                                                                        on:click=move |_| remove_highlight(i)
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
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    // Images upload (GCS)
                    <div>
                        <label style=LABEL_STYLE>"Images"</label>
                        <GcsMediaUploadList
                            value=Signal::derive(move || section.get().images)
                            on_change=Callback::new(move |images: Vec<String>| {
                                section.update(|s| s.images = images);
                            })
                            content_type="guide".to_string()
                            accept="image/*".to_string()
                            label="images".to_string()
                        />
                    </div>

                    // Videos upload (GCS)
                    <div>
                        <label style=LABEL_STYLE>"Videos"</label>
                        <GcsMediaUploadList
                            value=Signal::derive(move || section.get().videos)
                            on_change=Callback::new(move |videos: Vec<String>| {
                                section.update(|s| s.videos = videos);
                            })
                            content_type="guide".to_string()
                            accept="video/*".to_string()
                            label="videos".to_string()
                        />
                    </div>

                    // Download Links / Buttons (collapsible)
                    <div>
                        <div
                            style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; cursor: pointer;"
                            on:click=move |_| links_expanded.update(|v| *v = !*v)
                        >
                            <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; font-weight: 500; cursor: pointer;">
                                <Icon name=IconName::Link size=16 />
                                "Download Links / Buttons"
                                <span style="color: #64748b; font-size: 0.75rem;">
                                    {move || {
                                        let count = section.get().links.len();
                                        if count > 0 { format!("({})", count) } else { "(optional)".to_string() }
                                    }}
                                </span>
                            </label>
                            {move || if links_expanded.get() {
                                view! { <Icon name=IconName::ChevronDown size=14 /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::ChevronRight size=16 /> }.into_any()
                            }}
                        </div>
                        {move || if links_expanded.get() {
                            view! {
                                <LinksField
                                    items=Signal::derive(move || section.get().links)
                                    on_change=Callback::new(move |links: Vec<SectionLink>| {
                                        section.update(|s| s.links = links);
                                    })
                                />
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    // Steps section (similar to Earnings Tiers in Task)
                    <div>
                        <label style=LABEL_STYLE>"Steps"</label>
                        <StepsField
                            items=Signal::derive(move || section.get().steps)
                            on_change=Callback::new(move |steps: Vec<GuideStepData>| {
                                section.update(|s| s.steps = steps);
                            })
                        />
                    </div>

                    // Feature Highlights (simple list with + Add button)
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <label style=LABEL_STYLE>"Feature Highlights"</label>
                        <button
                            type="button"
                            style="background: #334155; color: white; border: none; border-radius: 0.25rem; padding: 0.375rem 0.75rem; cursor: pointer; font-size: 0.75rem; display: flex; align-items: center; gap: 0.25rem;"
                            on:click=move |_| highlights_expanded.set(true)
                        >
                            <Icon name=IconName::Plus size=12 />
                            "Add"
                        </button>
                    </div>

                    // Section is visible on page checkbox
                    <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.5rem; padding-top: 1rem; border-top: 1px solid #334155;">
                        <input
                            type="checkbox"
                            id="section-visible"
                            style="width: 16px; height: 16px; accent-color: #22c55e; cursor: pointer;"
                            prop:checked=move || section.get().is_visible
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                section.update(|s| s.is_visible = checked);
                            }
                        />
                        <label for="section-visible" style="color: #94a3b8; font-size: 0.875rem; cursor: pointer;">
                            "Section is visible on page"
                        </label>
                    </div>
                </div>

                // Modal footer
                <div style="display: flex; justify-content: flex-end; gap: 0.75rem; padding: 1rem 1.5rem; border-top: 1px solid #334155; position: sticky; bottom: 0; background: #1e293b;">
                    <button
                        type="button"
                        style="background: #475569; border: none; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem;"
                        on:click=move |_| on_cancel(())
                    >
                        "Cancel"
                    </button>
                    <button
                        type="button"
                        style="background: #3b82f6; border: none; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem;"
                        on:click=move |_| on_save(())
                    >
                        {move || if is_editing() { "Save Changes" } else { "Add" }}
                    </button>
                </div>
            </div>
        </div>
    }
}


/// Links field component for download buttons
#[component]
fn LinksField(
    items: Signal<Vec<SectionLink>>,
    on_change: Callback<Vec<SectionLink>>,
) -> impl IntoView {
    let show_add_form = RwSignal::new(false);
    let new_platform = RwSignal::new("app_store".to_string());
    let new_label = RwSignal::new(String::new());
    let new_href = RwSignal::new(String::new());

    let add_link = move |_| {
        let href = new_href.get();
        if !href.trim().is_empty() {
            let platform_str = new_platform.get();
            let platform = match platform_str.as_str() {
                "app_store" => LinkPlatform::AppStore,
                "google_play" => LinkPlatform::GooglePlay,
                "apk" => LinkPlatform::Apk,
                _ => LinkPlatform::Custom,
            };

            let label = if new_label.get().is_empty() {
                match platform {
                    LinkPlatform::AppStore => "App Store",
                    LinkPlatform::GooglePlay => "Google Play",
                    LinkPlatform::Apk => "APK Download",
                    LinkPlatform::Custom => "Download",
                }.to_string()
            } else {
                new_label.get()
            };

            let link = SectionLink {
                platform,
                label,
                sublabel: String::new(),
                href,
                target: "_blank".to_string(),
                rel: "noopener noreferrer".to_string(),
                icon: None,
                is_visible: true,
            };

            let mut current = items.get();
            current.push(link);
            on_change.run(current);

            // Reset form
            new_platform.set("app_store".to_string());
            new_label.set(String::new());
            new_href.set(String::new());
            show_add_form.set(false);
        }
    };

    let remove_link = move |index: usize| {
        let mut current = items.get();
        if index < current.len() {
            current.remove(index);
            on_change.run(current);
        }
    };

    view! {
        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
            // Existing links list
            {move || {
                let links = items.get();
                if !links.is_empty() {
                    view! {
                        <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
                            {links.into_iter().enumerate().map(|(i, link)| {
                                let platform_label = match link.platform {
                                    LinkPlatform::AppStore => "App Store",
                                    LinkPlatform::GooglePlay => "Google Play",
                                    LinkPlatform::Apk => "APK",
                                    LinkPlatform::Custom => "Custom",
                                };
                                view! {
                                    <div style="display: flex; align-items: center; gap: 0.5rem; background: #1e293b; padding: 0.5rem 0.75rem; border-radius: 0.25rem;">
                                        <span style="background: #3b82f6; color: white; font-size: 0.65rem; padding: 0.125rem 0.5rem; border-radius: 9999px;">
                                            {platform_label}
                                        </span>
                                        <span style="flex: 1; color: #e2e8f0; font-size: 0.875rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                                            {link.label.clone()}
                                        </span>
                                        <button
                                            type="button"
                                            style="background: #ef4444; border: none; color: white; padding: 0.25rem; border-radius: 0.25rem; cursor: pointer;"
                                            on:click=move |_| remove_link(i)
                                        >
                                            <Icon name=IconName::Trash size=12 />
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

            // Add link form
            {move || if show_add_form.get() {
                view! {
                    <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                        // Platform selector
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Platform"</label>
                            <select
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                prop:value=move || new_platform.get()
                                on:change=move |ev| new_platform.set(event_target_value(&ev))
                            >
                                <option value="app_store">"App Store"</option>
                                <option value="google_play">"Google Play"</option>
                                <option value="apk">"APK Download"</option>
                                <option value="custom">"Custom"</option>
                            </select>
                        </div>
                        // Label
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Label (optional)"</label>
                            <input
                                type="text"
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="Default based on platform..."
                                prop:value=move || new_label.get()
                                on:input=move |ev| new_label.set(event_target_value(&ev))
                            />
                        </div>
                        // URL
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"URL *"</label>
                            <input
                                type="text"
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="https://..."
                                prop:value=move || new_href.get()
                                on:input=move |ev| new_href.set(event_target_value(&ev))
                            />
                        </div>
                        // Buttons
                        <div style="display: flex; gap: 0.5rem; justify-content: flex-end;">
                            <button
                                type="button"
                                style="background: #475569; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                on:click=move |_| show_add_form.set(false)
                            >
                                "Cancel"
                            </button>
                            <button
                                type="button"
                                style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                on:click=add_link
                            >
                                "Add Link"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <button
                        type="button"
                        style="width: 100%; background: #334155; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.25rem;"
                        on:click=move |_| show_add_form.set(true)
                    >
                        <Icon name=IconName::Plus size=14 />
                        "Add Link"
                    </button>
                }.into_any()
            }}
        </div>
    }
}

/// Steps field component for guide steps (similar to Earnings Tiers in Task)
#[component]
fn StepsField(
    items: Signal<Vec<GuideStepData>>,
    on_change: Callback<Vec<GuideStepData>>,
) -> impl IntoView {
    let show_add_form = RwSignal::new(false);
    let editing_index = RwSignal::new(None::<usize>);

    // Form fields for new/editing step
    let step_order = RwSignal::new(1_i32);
    let step_title = RwSignal::new(String::new());
    let step_description = RwSignal::new(String::new());
    let step_images = RwSignal::new(Vec::<String>::new());

    // Reset form
    let reset_form = move || {
        step_order.set(items.get().len() as i32 + 1);
        step_title.set(String::new());
        step_description.set(String::new());
        step_images.set(Vec::new());
        editing_index.set(None);
    };

    // Open form for new step
    let open_new_form = move |_| {
        reset_form();
        show_add_form.set(true);
    };

    // Open form for editing existing step
    let open_edit_form = move |index: usize| {
        let steps = items.get();
        if let Some(step) = steps.get(index) {
            step_order.set(step.order);
            step_title.set(step.title.clone());
            step_description.set(step.description.clone());
            step_images.set(step.images.clone());
            editing_index.set(Some(index));
            show_add_form.set(true);
        }
    };

    // Save step (add new or update existing)
    let save_step = move |_| {
        let title = step_title.get();
        if !title.trim().is_empty() {
            let new_step = GuideStepData {
                order: step_order.get(),
                title,
                description: step_description.get(),
                images: step_images.get(),
            };

            let mut current = items.get();

            if let Some(index) = editing_index.get() {
                // Update existing step
                if index < current.len() {
                    current[index] = new_step;
                }
            } else {
                // Add new step
                current.push(new_step);
            }

            on_change.run(current);
            show_add_form.set(false);
            reset_form();
        }
    };

    // Cancel form
    let cancel_form = move |_| {
        show_add_form.set(false);
        reset_form();
    };

    // Remove step
    let remove_step = move |index: usize| {
        let mut current = items.get();
        if index < current.len() {
            current.remove(index);
            on_change.run(current);
        }
    };

    view! {
        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem;">
            // Existing steps list
            {move || {
                let steps = items.get();
                if !steps.is_empty() {
                    view! {
                        <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
                            {steps.into_iter().enumerate().map(|(i, step)| {
                                view! {
                                    <div style="display: flex; align-items: center; gap: 0.75rem; background: #1e293b; padding: 0.75rem; border-radius: 0.375rem;">
                                        // Order badge
                                        <span style="background: #3b82f6; color: white; font-size: 0.75rem; min-width: 24px; height: 24px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 600;">
                                            {step.order}
                                        </span>
                                        // Step info
                                        <div style="flex: 1; min-width: 0;">
                                            <div style="color: #e2e8f0; font-size: 0.875rem; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                                {step.title.clone()}
                                            </div>
                                            {if !step.description.is_empty() {
                                                view! {
                                                    <div style="color: #64748b; font-size: 0.75rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                                        {step.description.chars().take(50).collect::<String>()}{if step.description.len() > 50 { "..." } else { "" }}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                        // Images count
                                        {if !step.images.is_empty() {
                                            view! {
                                                <span style="background: #475569; color: #94a3b8; font-size: 0.65rem; padding: 0.125rem 0.5rem; border-radius: 9999px;">
                                                    {format!("{} img", step.images.len())}
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        // Action buttons
                                        <div style="display: flex; gap: 0.25rem;">
                                            <button
                                                type="button"
                                                style="background: #3b82f6; border: none; color: white; padding: 0.25rem 0.5rem; border-radius: 0.25rem; cursor: pointer; font-size: 0.75rem;"
                                                on:click=move |_| open_edit_form(i)
                                            >
                                                "Edit"
                                            </button>
                                            <button
                                                type="button"
                                                style="background: #ef4444; border: none; color: white; padding: 0.25rem; border-radius: 0.25rem; cursor: pointer;"
                                                on:click=move |_| remove_step(i)
                                            >
                                                <Icon name=IconName::Trash size=12 />
                                            </button>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="color: #64748b; font-size: 0.875rem; font-style: italic; text-align: center; padding: 1rem;">
                            "No steps yet. Add steps to guide users through the process."
                        </div>
                    }.into_any()
                }
            }}

            // Add/Edit step form
            {move || if show_add_form.get() {
                view! {
                    <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem;">
                        <div style="color: #e2e8f0; font-size: 0.875rem; font-weight: 600; margin-bottom: 0.25rem;">
                            {move || if editing_index.get().is_some() { "Edit Step" } else { "Add Step" }}
                        </div>

                        // Order field
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Order *"</label>
                            <input
                                type="number"
                                style="width: 80px; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                min="1"
                                prop:value=move || step_order.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        step_order.set(val);
                                    }
                                }
                            />
                        </div>

                        // Title field
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Title *"</label>
                            <input
                                type="text"
                                style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                placeholder="Enter step title..."
                                prop:value=move || step_title.get()
                                on:input=move |ev| step_title.set(event_target_value(&ev))
                            />
                        </div>

                        // Description field (rich text)
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Description"</label>
                            <div style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.25rem; overflow: hidden;">
                                <div style="display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #1e293b;">
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-weight: bold;" title="Bold">"B"</button>
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-style: italic;" title="Italic">"I"</button>
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="Link">"Link"</button>
                                    <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="List">"List"</button>
                                </div>
                                <textarea
                                    style="width: 100%; background: transparent; border: none; padding: 0.5rem; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 80px; resize: vertical;"
                                    placeholder="Enter step description..."
                                    prop:value=move || step_description.get()
                                    on:input=move |ev| step_description.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        // Images upload (GCS)
                        <div>
                            <label style="color: #64748b; font-size: 0.75rem; display: block; margin-bottom: 0.25rem;">"Images"</label>
                            <GcsMediaUploadList
                                value=Signal::derive(move || step_images.get())
                                on_change=Callback::new(move |images: Vec<String>| {
                                    step_images.set(images);
                                })
                                content_type="guide".to_string()
                                accept="image/*".to_string()
                                label="images".to_string()
                            />
                        </div>

                        // Form buttons
                        <div style="display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 0.5rem;">
                            <button
                                type="button"
                                style="background: #475569; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                on:click=cancel_form
                            >
                                "Cancel"
                            </button>
                            <button
                                type="button"
                                style="background: #3b82f6; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.875rem;"
                                on:click=save_step
                            >
                                {move || if editing_index.get().is_some() { "Save Changes" } else { "Add Step" }}
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <button
                        type="button"
                        style="width: 100%; background: #334155; color: white; border: none; border-radius: 0.25rem; padding: 0.5rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.25rem;"
                        on:click=open_new_form
                    >
                        <Icon name=IconName::Plus size=14 />
                        "Add Step"
                    </button>
                }.into_any()
            }}

            // Item count
            <p style="font-size: 0.75rem; color: #64748b; margin-top: 0.5rem; margin-bottom: 0;">
                {move || format!("{} steps", items.get().len())}
            </p>
        </div>
    }
}

// ============================================
// Guide Content Editor (for Content Editor page)
// ============================================

/// Guide Content Editor - similar to TaskContentEditor but for Guide content
/// Uses GuideSectionEditor with its modal for adding sections
#[component]
pub fn GuideContentEditor(
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

    let initial_name = initial_data
        .get("name")
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
    let name_signal = RwSignal::new(initial_name);
    let description_signal = RwSignal::new(initial_description);
    let sections_signal = RwSignal::new(initial_sections);

    // Handle name change - update both signal and data
    let on_name_input = move |ev: web_sys::Event| {
        let new_name = event_target_value(&ev);
        name_signal.set(new_name.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("name".to_string(), serde_json::Value::String(new_name));
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
        <div class="guide-content-editor" style=FORM_STYLE>
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

            // Title field (required)
            <div style="margin-bottom: 1.5rem;">
                <label style=LABEL_STYLE>
                    "Title"
                    <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                </label>
                <input
                    type="text"
                    style=move || {
                        let base = "width: 100%; background: #1e293b; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
                        if has_error("name") {
                            format!("{} border: 2px solid #ef4444;", base)
                        } else {
                            format!("{} border: 1px solid #334155;", base)
                        }
                    }
                    placeholder="Enter page name..."
                    prop:value=move || name_signal.get()
                    on:input=on_name_input
                    disabled=read_only
                />
                <div style="display: flex; justify-content: space-between; margin-top: 0.25rem;">
                    <p
                        style=move || {
                            let has_err = get_error_message("name").is_some();
                            format!("color: #ef4444; font-size: 0.75rem; margin: 0; display: {}", if has_err { "block" } else { "none" })
                        }
                    >
                        {move || get_error_message("name").unwrap_or_default()}
                    </p>
                    <span style="color: #64748b; font-size: 0.75rem;">
                        {move || format!("{}/100", name_signal.get().len())}
                    </span>
                </div>
            </div>

            // Description field (required) - with rich text toolbar
            <div style="margin-bottom: 1.5rem;">
                <label style=LABEL_STYLE>
                    "Description"
                    <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                </label>
                <div style=move || {
                    let base = "border-radius: 0.375rem; overflow: hidden;";
                    if has_error("description") {
                        format!("{} border: 2px solid #ef4444;", base)
                    } else {
                        format!("{} border: 1px solid #334155;", base)
                    }
                }>
                    // Toolbar with formatting buttons
                    <div style=TOOLBAR_STYLE>
                        <button type="button" style=TOOLBAR_BTN_STYLE>"B"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE>"I"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE>"Link"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE>"List"</button>
                    </div>
                    <textarea
                        style="width: 100%; background: #1e293b; border: none; padding: 0.75rem; color: #e2e8f0; font-size: 0.875rem; min-height: 150px; resize: vertical; outline: none;"
                        placeholder="Enter content..."
                        prop:value=move || description_signal.get()
                        on:input=on_description_input
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

            // Sections with Add Section button (using GuideSectionEditor with modal)
            <div>
                <label style=LABEL_STYLE>
                    "Sections"
                    <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                </label>
                <GuideSectionEditor
                    value=Signal::derive(move || sections_signal.get())
                    on_change=on_sections_change
                    read_only=read_only
                />
            </div>
        </div>
    }
}
