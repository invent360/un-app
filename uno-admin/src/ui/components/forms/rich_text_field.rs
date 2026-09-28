//! Rich text field component with basic formatting support

use leptos::prelude::*;
use crate::api::schema_types::RichTextFieldConfig;

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const EDITOR_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 200px;";
const TOOLBAR_STYLE: &str = "display: flex; gap: 0.25rem; padding: 0.5rem; border-bottom: 1px solid #334155; background: #0f172a;";
const TOOLBAR_BTN_STYLE: &str = "padding: 0.375rem 0.5rem; background: transparent; border: 1px solid #334155; border-radius: 0.25rem; color: #94a3b8; cursor: pointer; font-size: 0.75rem;";
const CONTENT_STYLE: &str = "padding: 0.75rem; min-height: 150px;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";

#[component]
pub fn RichTextField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current value (HTML content)
    value: RwSignal<String>,
    /// Field configuration
    #[prop(optional)]
    config: Option<RichTextFieldConfig>,
    /// Description/help text
    
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let placeholder = config.placeholder.clone().unwrap_or_else(|| "Enter content...".to_string());

    // For now, use a simple textarea. In production, integrate a proper rich text editor.
    let on_input = move |ev: web_sys::Event| {
        let new_value = event_target_value(&ev);
        value.set(new_value.clone());
        if let Some(cb) = on_change {
            cb.run(new_value);
        }
    };

    view! {
        <div class="form-field" style="margin-bottom: 1rem;">
            <label style=LABEL_STYLE>
                {label.clone()}
                {move || if required {
                    view! { <span style=REQUIRED_STYLE>"*"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </label>

            <div style=EDITOR_STYLE>
                // Toolbar (for future rich text integration)
                <div style=TOOLBAR_STYLE>
                    <button type="button" style=TOOLBAR_BTN_STYLE title="Bold">"B"</button>
                    <button type="button" style=TOOLBAR_BTN_STYLE title="Italic">"I"</button>
                    <button type="button" style=TOOLBAR_BTN_STYLE title="Link">"Link"</button>
                    <button type="button" style=TOOLBAR_BTN_STYLE title="List">"List"</button>
                </div>

                // Content area - using textarea for now
                <textarea
                    style="width: 100%; background: transparent; border: none; color: #e2e8f0; font-size: 0.875rem; outline: none; min-height: 150px; padding: 0.75rem; resize: vertical;"
                    prop:value=move || value.get()
                    on:input=on_input
                    placeholder=placeholder
                />
            </div>

            {move || {
                if let Some(ref desc) = description {
                    view! { <p style=HELP_TEXT_STYLE>{desc.clone()}</p> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
