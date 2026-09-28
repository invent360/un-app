//! Per-locale translation editor component
//!
//! Provides a tabbed interface for editing translations per locale instead of raw JSON.

use leptos::prelude::*;

/// Supported locales for translation
const LOCALES: &[(&str, &str)] = &[
    ("es", "Spanish"),
    ("fr", "French"),
    ("ar", "Arabic"),
    ("hi", "Hindi"),
    ("pt", "Portuguese"),
    ("id", "Indonesian"),
    ("tl", "Tagalog"),
    ("sw", "Swahili"),
];

/// Panel styling constants (matching editor.rs dark navy theme)
const INPUT_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px;";
const TEXTAREA_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 80px; resize: vertical;";
const LABEL_STYLE: &str = "display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;";

/// Get translatable fields based on content type
fn get_translatable_fields(content_type: &str) -> Vec<&'static str> {
    match content_type {
        "task" => vec!["title", "description", "body"],
        "guide" => vec!["title", "description", "body"],
        "faq" => vec!["title", "question", "answer"],
        "error" => vec!["title", "description", "solution"],
        _ => vec!["title", "description", "body"],
    }
}

/// Translation editor component with tabbed locale interface
#[component]
pub fn TranslationEditor(
    /// Base content fields (English) as JSON string
    base_content: Signal<String>,
    /// Translations JSON string (all locales)
    translations: RwSignal<String>,
    /// Content type for determining which fields to show
    content_type: Signal<String>,
) -> impl IntoView {
    // Active locale tab
    let (active_locale, set_active_locale) = signal("es".to_string());

    // Get translation coverage stats
    let coverage_stats = move || {
        let trans = translations.get();
        if trans.is_empty() {
            return (0, LOCALES.len());
        }
        let parsed: serde_json::Value = serde_json::from_str(&trans).unwrap_or(serde_json::json!({}));

        let translated_locales = LOCALES.iter()
            .filter(|(code, _)| {
                if let Some(locale_data) = parsed.get(*code).and_then(|v| v.as_object()) {
                    locale_data.contains_key("title")
                } else {
                    false
                }
            })
            .count();

        (translated_locales, LOCALES.len())
    };

    view! {
        <div style="background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; overflow: hidden;">
            // Header with coverage stats
            <div style="padding: 16px 20px; border-bottom: 1px solid #1e293b; display: flex; align-items: center; justify-content: space-between;">
                <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0;">
                    "Translations"
                </h3>
                <span style="font-size: 13px; color: #94a3b8;">
                    {move || {
                        let (translated, total) = coverage_stats();
                        format!("{}/{} locales", translated, total)
                    }}
                </span>
            </div>

            // Locale tabs
            <div style="display: flex; border-bottom: 1px solid #1e293b; overflow-x: auto;">
                {LOCALES.iter().map(|(code, name)| {
                    let code_owned = code.to_string();
                    let code_for_click = code.to_string();
                    let name_owned = name.to_string();

                    view! {
                        <button
                            style=move || {
                                let is_active = active_locale.get() == code_owned;
                                let base = "padding: 12px 20px; font-size: 14px; font-weight: 500; border: none; cursor: pointer; transition: all 0.2s; white-space: nowrap;";
                                if is_active {
                                    format!("{} background: #1e293b; color: #22d3ee; border-bottom: 2px solid #22d3ee;", base)
                                } else {
                                    format!("{} background: transparent; color: #94a3b8; border-bottom: 2px solid transparent;", base)
                                }
                            }
                            on:click=move |_| set_active_locale.set(code_for_click.clone())
                        >
                            {name_owned}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Active locale editor
            <div style="padding: 20px;">
                <LocaleFieldEditor
                    locale=active_locale
                    base_content=base_content
                    translations=translations
                    content_type=content_type
                />
            </div>
        </div>
    }
}

/// Field editor for a specific locale
#[component]
fn LocaleFieldEditor(
    /// Current locale code
    locale: ReadSignal<String>,
    /// Base content (English) for reference
    base_content: Signal<String>,
    /// Translations JSON (all locales)
    translations: RwSignal<String>,
    /// Content type
    content_type: Signal<String>,
) -> impl IntoView {
    // Get base content value for a field
    let get_base_value = move |field: &str| -> String {
        let content = base_content.get();
        serde_json::from_str::<serde_json::Value>(&content)
            .ok()
            .and_then(|v| v.get(field).cloned())
            .map(|v| {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    v.to_string()
                }
            })
            .unwrap_or_default()
    };

    // Get translated value for a field
    let get_translated_value = move |field: &str, loc: &str| -> String {
        let trans = translations.get();
        if trans.is_empty() {
            return String::new();
        }
        serde_json::from_str::<serde_json::Value>(&trans)
            .ok()
            .and_then(|v| v.get(loc).cloned())
            .and_then(|v| v.get(field).cloned())
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default()
    };

    // Update a field in the translations JSON
    let update_locale_field = move |loc: String, field: String, value: String| {
        let trans = translations.get();
        let mut parsed: serde_json::Value = if trans.is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(&trans).unwrap_or(serde_json::json!({}))
        };

        // Ensure locale object exists
        if parsed.get(&loc).is_none() {
            parsed[&loc] = serde_json::json!({});
        }

        // Update the field
        if value.is_empty() {
            if let Some(obj) = parsed.get_mut(&loc).and_then(|v| v.as_object_mut()) {
                obj.remove(&field);
            }
        } else {
            parsed[&loc][&field] = serde_json::Value::String(value);
        }

        // Clean up empty locale objects
        if let Some(obj) = parsed.as_object_mut() {
            let empty_locales: Vec<String> = obj.iter()
                .filter(|(_, v)| v.as_object().map(|o| o.is_empty()).unwrap_or(false))
                .map(|(k, _)| k.clone())
                .collect();
            for loc in empty_locales {
                obj.remove(&loc);
            }
        }

        translations.set(serde_json::to_string_pretty(&parsed).unwrap_or_default());
    };

    view! {
        <div style="display: flex; flex-direction: column; gap: 20px;">
            {move || {
                let ct = content_type.get();
                let fields = get_translatable_fields(&ct);
                let loc = locale.get();

                fields.into_iter().map(|field| {
                    let field_name = field.to_string();
                    let field_for_update = field.to_string();
                    let locale_for_update = loc.clone();
                    let base_val = get_base_value(field);
                    let trans_val = get_translated_value(field, &loc);

                    let is_long_text = matches!(field, "description" | "body" | "answer" | "solution");

                    view! {
                        <div>
                            <label style=LABEL_STYLE>
                                {capitalize(&field_name)}
                            </label>

                            // Reference text (English)
                            <div style="margin-bottom: 8px; padding: 10px 14px; background: #1e293b; border-radius: 8px; border: 1px solid #334155;">
                                <div style="font-size: 11px; color: #64748b; margin-bottom: 4px; text-transform: uppercase;">
                                    "English (Reference)"
                                </div>
                                <div style="font-size: 14px; color: #94a3b8; white-space: pre-wrap;">
                                    {if base_val.is_empty() { "—".to_string() } else { base_val }}
                                </div>
                            </div>

                            // Translation input
                            {if is_long_text {
                                let update_fn = update_locale_field.clone();
                                let loc_clone = locale_for_update.clone();
                                let field_clone = field_for_update.clone();
                                view! {
                                    <textarea
                                        style=TEXTAREA_STYLE
                                        placeholder=format!("Enter {} translation...", &field_name)
                                        prop:value=trans_val.clone()
                                        on:input=move |ev| {
                                            let value = event_target_value(&ev);
                                            update_fn(loc_clone.clone(), field_clone.clone(), value);
                                        }
                                    />
                                }.into_any()
                            } else {
                                let update_fn = update_locale_field.clone();
                                let loc_clone = locale_for_update.clone();
                                let field_clone = field_for_update.clone();
                                view! {
                                    <input
                                        type="text"
                                        style=INPUT_STYLE
                                        placeholder=format!("Enter {} translation...", &field_name)
                                        prop:value=trans_val.clone()
                                        on:input=move |ev| {
                                            let value = event_target_value(&ev);
                                            update_fn(loc_clone.clone(), field_clone.clone(), value);
                                        }
                                    />
                                }.into_any()
                            }}
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}

/// Capitalize first letter of a string
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(chars).collect(),
    }
}
