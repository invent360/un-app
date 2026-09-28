//! FAQ Question Editor Component
//!
//! A specialized editor for FAQ questions with category filtering,
//! drag-to-reorder, and inline editing capabilities.

use leptos::prelude::*;
use crate::components::common::icon::{Icon, IconName};
use crate::api::faq_types::{FaqCategory, FaqQuestion};

// ============================================
// FAQ Question Editor Component
// ============================================

/// FAQ Question Editor - manages a list of FAQ questions
#[component]
pub fn FaqQuestionEditor(
    /// Current questions value as JSON array
    value: RwSignal<Vec<serde_json::Value>>,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<serde_json::Value>>>,
    /// Whether the editor is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    // State for category filter
    let active_category = RwSignal::new(FaqCategory::All);
    let expanded_questions = RwSignal::new(std::collections::HashSet::<usize>::new());
    let show_modal = RwSignal::new(false);
    let editing_question = RwSignal::new(None::<FaqQuestion>);
    let editing_index = RwSignal::new(None::<usize>);

    // Toggle question expansion
    let toggle_expand = move |index: usize| {
        expanded_questions.update(|set| {
            if set.contains(&index) {
                set.remove(&index);
            } else {
                set.insert(index);
            }
        });
    };

    // Open modal to add new question
    let open_add_modal = move |_| {
        let order = value.get().len() as i32 + 1;
        let mut question = FaqQuestion::default();
        question.display_order = order;
        question.is_visible = true;
        // Default to current filter category if not "All"
        if active_category.get() != FaqCategory::All {
            question.category = active_category.get();
        } else {
            question.category = FaqCategory::General;
        }
        editing_question.set(Some(question));
        editing_index.set(None);
        show_modal.set(true);
    };

    // Open modal to edit existing question
    let open_edit_modal = move |index: usize| {
        let questions = value.get();
        if let Some(json) = questions.get(index) {
            if let Ok(question) = serde_json::from_value::<FaqQuestion>(json.clone()) {
                editing_question.set(Some(question));
                editing_index.set(Some(index));
                show_modal.set(true);
            }
        }
    };

    // Save question (add or update)
    let save_question = move |question: FaqQuestion| {
        let json = serde_json::to_value(&question).unwrap_or_default();
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
        editing_question.set(None);
        editing_index.set(None);
        show_modal.set(false);
    };

    // Delete question
    let delete_question = move |index: usize| {
        let mut current = value.get();
        if index < current.len() {
            current.remove(index);
            // Update display_order for remaining questions
            for (i, q) in current.iter_mut().enumerate() {
                if let Some(obj) = q.as_object_mut() {
                    obj.insert("display_order".to_string(), serde_json::json!(i + 1));
                }
            }
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    // Move question up
    let move_up = move |index: usize| {
        if index > 0 {
            let mut current = value.get();
            current.swap(index, index - 1);
            // Update display_order
            for (i, q) in current.iter_mut().enumerate() {
                if let Some(obj) = q.as_object_mut() {
                    obj.insert("display_order".to_string(), serde_json::json!(i + 1));
                }
            }
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    // Move question down
    let move_down = move |index: usize| {
        let mut current = value.get();
        if index < current.len() - 1 {
            current.swap(index, index + 1);
            // Update display_order
            for (i, q) in current.iter_mut().enumerate() {
                if let Some(obj) = q.as_object_mut() {
                    obj.insert("display_order".to_string(), serde_json::json!(i + 1));
                }
            }
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    // Toggle visibility
    let toggle_visibility = move |index: usize| {
        let mut current = value.get();
        if let Some(q) = current.get_mut(index) {
            if let Some(obj) = q.as_object_mut() {
                let visible = obj.get("is_visible").and_then(|v| v.as_bool()).unwrap_or(true);
                obj.insert("is_visible".to_string(), serde_json::json!(!visible));
            }
        }
        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
    };

    // Toggle featured
    let toggle_featured = move |index: usize| {
        let mut current = value.get();
        if let Some(q) = current.get_mut(index) {
            if let Some(obj) = q.as_object_mut() {
                let featured = obj.get("is_featured").and_then(|v| v.as_bool()).unwrap_or(false);
                obj.insert("is_featured".to_string(), serde_json::json!(!featured));
            }
        }
        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
    };

    // Get category counts
    let category_counts = move || {
        let questions = value.get();
        let mut counts = std::collections::HashMap::new();
        let mut total = 0;
        for q in &questions {
            let cat = q.get("category")
                .and_then(|v| v.as_str())
                .and_then(FaqCategory::from_str)
                .unwrap_or(FaqCategory::General);
            *counts.entry(cat).or_insert(0) += 1;
            total += 1;
        }
        counts.insert(FaqCategory::All, total);
        counts
    };

    // Filter questions by active category
    let filtered_indices = move || {
        let questions = value.get();
        let filter = active_category.get();
        questions.iter().enumerate()
            .filter(|(_, q)| {
                if filter == FaqCategory::All {
                    true
                } else {
                    let cat = q.get("category")
                        .and_then(|v| v.as_str())
                        .and_then(FaqCategory::from_str)
                        .unwrap_or(FaqCategory::General);
                    cat == filter
                }
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    };

    view! {
        <div class="faq-question-editor">
            // Category Tabs
            <div style="display: flex; gap: 0.5rem; margin-bottom: 1rem; flex-wrap: wrap;">
                {FaqCategory::all_categories().into_iter().map(|cat| {
                    let count = move || category_counts().get(&cat).copied().unwrap_or(0);
                    let is_active = move || active_category.get() == cat;
                    view! {
                        <button
                            type="button"
                            style=move || format!(
                                "padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; cursor: pointer; display: flex; align-items: center; gap: 0.5rem; transition: all 0.2s; {}",
                                if is_active() {
                                    format!("background: {}; color: white; border: none;", cat.color())
                                } else {
                                    "background: #1e293b; color: #94a3b8; border: 1px solid #334155;".to_string()
                                }
                            )
                            on:click=move |_| active_category.set(cat)
                        >
                            <Icon name=IconName::from_str(cat.icon()) size=14 />
                            {cat.display_name()}
                            <span style="background: rgba(0,0,0,0.2); padding: 0.125rem 0.375rem; border-radius: 9999px; font-size: 0.75rem;">
                                {count}
                            </span>
                        </button>
                    }
                }).collect_view()}
            </div>

            // Questions Label
            <label style="display: block; color: #e2e8f0; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem;">
                "Questions"
                <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
            </label>

            // Questions List
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                {move || {
                    let indices = filtered_indices();
                    let questions = value.get();
                    let total_len = questions.len();

                    if indices.is_empty() {
                        view! {
                            <div style="padding: 2rem; text-align: center; background: #0f172a; border: 2px dashed #334155; border-radius: 0.5rem; color: #64748b;">
                                {move || if active_category.get() == FaqCategory::All {
                                    "No questions yet. Click \"Add Question\" to create one."
                                } else {
                                    "No questions in this category."
                                }}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div>
                                {indices.into_iter().map(|i| {
                                    let q: FaqQuestion = questions.get(i)
                                        .and_then(|j| serde_json::from_value(j.clone()).ok())
                                        .unwrap_or_default();
                                    let is_expanded = expanded_questions.get().contains(&i);
                                    let is_last = i == total_len - 1;
                                    let cat_color = q.category.color();

                                    view! {
                                        <div style="background: #1e293b; border-radius: 0.5rem; border: 1px solid #334155; overflow: hidden; margin-bottom: 0.5rem;">
                                            // Question Header
                                            <div
                                                style=format!("display: flex; align-items: center; padding: 0.75rem 1rem; border-left: 4px solid {};", cat_color)
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

                                                // Question title
                                                <div
                                                    style="flex: 1; display: flex; flex-direction: column; gap: 0.25rem; min-width: 0; cursor: pointer;"
                                                    on:click=move |_| toggle_expand(i)
                                                >
                                                    <span style="color: #e2e8f0; font-weight: 500; display: block; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                                        {q.question.clone()}
                                                    </span>
                                                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                                                        <span style=format!(
                                                            "background: {}; color: white; padding: 0.125rem 0.5rem; border-radius: 9999px; font-size: 0.625rem; text-transform: uppercase;",
                                                            cat_color
                                                        )>
                                                            {q.category.display_name()}
                                                        </span>
                                                        {if q.is_featured {
                                                            view! {
                                                                <span style="background: #f59e0b; color: white; padding: 0.125rem 0.5rem; border-radius: 9999px; font-size: 0.625rem; text-transform: uppercase;">
                                                                    "Featured"
                                                                </span>
                                                            }.into_any()
                                                        } else {
                                                            view! { <span></span> }.into_any()
                                                        }}
                                                        {if !q.is_visible {
                                                            view! {
                                                                <span style="background: #475569; color: #94a3b8; padding: 0.125rem 0.5rem; border-radius: 9999px; font-size: 0.625rem; text-transform: uppercase;">
                                                                    "Hidden"
                                                                </span>
                                                            }.into_any()
                                                        } else {
                                                            view! { <span></span> }.into_any()
                                                        }}
                                                    </div>
                                                </div>

                                                // Action Buttons
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
                                                                on:click=move |_| delete_question(i)
                                                            >
                                                                <Icon name=IconName::Trash size=14 />
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>

                                            // Question Content (expanded)
                                            {if is_expanded {
                                                view! {
                                                    <div style="padding: 1rem; background: #0f172a; border-top: 1px solid #334155;">
                                                        <div style="color: #94a3b8; font-size: 0.875rem; line-height: 1.6; white-space: pre-wrap;">
                                                            {q.answer.clone()}
                                                        </div>
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

            // Add Question button at bottom
            {move || if !read_only {
                view! {
                    <button
                        type="button"
                        style="width: 100%; padding: 0.75rem 1rem; background: #334155; color: #94a3b8; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; justify-content: center; gap: 0.5rem; margin-top: 0.5rem;"
                        on:click=open_add_modal
                    >
                        <Icon name=IconName::Plus size=14 />
                        "Add Question"
                    </button>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Question Editor Modal
            {move || if show_modal.get() {
                if let Some(question) = editing_question.get() {
                    view! {
                        <FaqQuestionModal
                            question=RwSignal::new(question)
                            is_editing=Signal::derive(move || editing_index.get().is_some())
                            on_save=Callback::new(move |q: FaqQuestion| save_question(q))
                            on_cancel=Callback::new(move |_| {
                                editing_question.set(None);
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
// FAQ Question Modal Component
// ============================================

#[component]
fn FaqQuestionModal(
    question: RwSignal<FaqQuestion>,
    is_editing: Signal<bool>,
    on_save: Callback<FaqQuestion>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    view! {
        <div style="position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000; padding: 1rem;">
            <div style="background: #1e293b; border-radius: 0.75rem; width: 100%; max-width: 600px; max-height: 90vh; overflow-y: auto; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.5);">
                // Modal Header
                <div style="padding: 1rem 1.5rem; border-bottom: 1px solid #334155; display: flex; justify-content: space-between; align-items: center;">
                    <h3 style="color: #e2e8f0; font-size: 1.125rem; font-weight: 600; margin: 0;">
                        {move || if is_editing.get() { "Edit Question" } else { "Add Question" }}
                    </h3>
                    <button
                        type="button"
                        style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 0.25rem;"
                        on:click=move |_| on_cancel.run(())
                    >
                        <Icon name=IconName::Close size=20 />
                    </button>
                </div>

                // Modal Body
                <div style="padding: 1.5rem;">
                    // Category Select
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Category"
                        </label>
                        <select
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                if let Some(cat) = FaqCategory::from_str(&val) {
                                    question.update(|q| q.category = cat);
                                }
                            }
                        >
                            {FaqCategory::selectable_categories().into_iter().map(|cat| {
                                let is_selected = move || question.get().category == cat;
                                view! {
                                    <option value=cat.as_str() selected=is_selected>
                                        {cat.display_name()}
                                    </option>
                                }
                            }).collect_view()}
                        </select>
                    </div>

                    // Question Input
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Question" <span style="color: #ef4444;">*</span>
                        </label>
                        <input
                            type="text"
                            style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                            placeholder="What is UNO?"
                            prop:value=move || question.get().question.clone()
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                question.update(|q| q.question = val);
                            }
                        />
                    </div>

                    // Answer with rich text toolbar
                    <div style="margin-bottom: 1rem;">
                        <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                            "Answer" <span style="color: #ef4444;">"*"</span>
                        </label>
                        <div style="width: 100%; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                            <div style="display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #1e293b;">
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-weight: bold;" title="Bold">"B"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem; font-style: italic;" title="Italic">"I"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="Link">"Link"</button>
                                <button type="button" style="padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;" title="List">"List"</button>
                            </div>
                            <textarea
                                style="width: 100%; min-height: 150px; background: transparent; border: none; padding: 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; resize: vertical; line-height: 1.6;"
                                placeholder="UNO is a platform that allows you to earn passive income..."
                                prop:value=move || question.get().answer.clone()
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    question.update(|q| q.answer = val);
                                }
                            ></textarea>
                        </div>
                    </div>

                    // Options Row
                    <div style="display: flex; gap: 1.5rem; margin-bottom: 1rem;">
                        // Featured Checkbox
                        <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; cursor: pointer;">
                            <input
                                type="checkbox"
                                style="width: 16px; height: 16px; accent-color: #f59e0b;"
                                prop:checked=move || question.get().is_featured
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    question.update(|q| q.is_featured = checked);
                                }
                            />
                            "Featured"
                        </label>

                        // Visible Checkbox
                        <label style="display: flex; align-items: center; gap: 0.5rem; color: #94a3b8; font-size: 0.875rem; cursor: pointer;">
                            <input
                                type="checkbox"
                                style="width: 16px; height: 16px; accent-color: #22c55e;"
                                prop:checked=move || question.get().is_visible
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    question.update(|q| q.is_visible = checked);
                                }
                            />
                            "Visible"
                        </label>
                    </div>
                </div>

                // Modal Footer
                <div style="padding: 1rem 1.5rem; border-top: 1px solid #334155; display: flex; justify-content: flex-end; gap: 0.75rem;">
                    <button
                        type="button"
                        style="background: #334155; color: #e2e8f0; border: none; border-radius: 0.375rem; padding: 0.625rem 1.25rem; cursor: pointer; font-size: 0.875rem;"
                        on:click=move |_| on_cancel.run(())
                    >
                        "Cancel"
                    </button>
                    <button
                        type="button"
                        style="background: #3b82f6; color: white; border: none; border-radius: 0.375rem; padding: 0.625rem 1.25rem; cursor: pointer; font-size: 0.875rem;"
                        disabled=move || {
                            let q = question.get();
                            q.question.trim().is_empty() || q.answer.trim().is_empty()
                        }
                        on:click=move |_| {
                            on_save.run(question.get());
                        }
                    >
                        {move || if is_editing.get() { "Save Changes" } else { "Add" }}
                    </button>
                </div>
            </div>
        </div>
    }
}

// ============================================
// Helper for IconName
// ============================================

impl IconName {
    fn from_str(s: &str) -> Self {
        match s {
            "folder" => IconName::Folder,
            "info" => IconName::Document,
            "revenue" => IconName::Revenue,
            "settings" => IconName::Settings,
            "shield" => IconName::Key,
            "question" => IconName::Star,
            _ => IconName::Document,
        }
    }
}

// ============================================
// FAQ Content Editor (for Content Editor page)
// ============================================


// Styles for FAQ Content Editor
const FORM_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem;";
const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const INPUT_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
const TOOLBAR_STYLE: &str = "display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #0f172a;";
const TOOLBAR_BTN_STYLE: &str = "background: #334155; border: none; color: #94a3b8; padding: 0.375rem 0.625rem; border-radius: 0.25rem; cursor: pointer; font-size: 0.75rem; font-weight: 600;";

/// FAQ Content Editor - similar to TaskContentEditor but for FAQ content
/// Uses FaqQuestionEditor with its modal for adding questions
#[component]
pub fn FaqContentEditor(
    /// The form data signal to read from and write to
    data: RwSignal<serde_json::Value>,
    /// Whether the form is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    // Initialize signals directly from form data
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

    let initial_questions = initial_data
        .get("questions")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // Create RwSignals with initial values
    let title_signal = RwSignal::new(initial_title);
    let description_signal = RwSignal::new(initial_description);
    let questions_signal = RwSignal::new(initial_questions);

    // Handle title change
    let on_title_input = move |ev: web_sys::Event| {
        let new_title = event_target_value(&ev);
        title_signal.set(new_title.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("title".to_string(), serde_json::Value::String(new_title));
            }
        });
    };

    // Handle description change
    let on_description_input = move |ev: web_sys::Event| {
        let new_desc = event_target_value(&ev);
        description_signal.set(new_desc.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("description".to_string(), serde_json::Value::String(new_desc));
            }
        });
    };

    // Handle questions change
    let on_questions_change = Callback::new(move |new_questions: Vec<serde_json::Value>| {
        questions_signal.set(new_questions.clone());
        data.update(|d| {
            if let Some(obj) = d.as_object_mut() {
                obj.insert("questions".to_string(), serde_json::Value::Array(new_questions));
            }
        });
    });

    view! {
        <div class="faq-content-editor" style=FORM_STYLE>
            // Title field
            <div style="margin-bottom: 1.5rem;">
                <label style=LABEL_STYLE>
                    "Title"
                    <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span>
                </label>
                <input
                    type="text"
                    style=INPUT_STYLE
                    placeholder="Enter FAQ page title..."
                    prop:value=move || title_signal.get()
                    on:input=on_title_input
                    disabled=read_only
                />
            </div>

            // Description field with rich text toolbar
            <div style="margin-bottom: 1.5rem;">
                <label style=LABEL_STYLE>
                    "Description"
                </label>
                <div style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; overflow: hidden;">
                    // Toolbar with formatting buttons
                    <div style=TOOLBAR_STYLE>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="Bold">"B"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="Italic">"I"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="Link">"Link"</button>
                        <button type="button" style=TOOLBAR_BTN_STYLE title="List">"List"</button>
                    </div>
                    // Content area
                    <textarea
                        style="width: 100%; background: transparent; border: none; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 120px; padding: 0.75rem; resize: vertical;"
                        prop:value=move || description_signal.get()
                        on:input=on_description_input
                        placeholder="Enter a description for the FAQ page..."
                        disabled=read_only
                    />
                </div>
            </div>

            // FAQ Questions Editor with modal
            <FaqQuestionEditor
                value=questions_signal
                on_change=on_questions_change
                read_only=read_only
            />
        </div>
    }
}
