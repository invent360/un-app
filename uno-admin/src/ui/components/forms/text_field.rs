//! Text field component for single-line and multi-line text input

use leptos::prelude::*;
use crate::api::schema_types::TextFieldConfig;

// Styles
const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const INPUT_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; transition: border-color 0.15s;";
const TEXTAREA_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; transition: border-color 0.15s; min-height: 100px; resize: vertical;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;";
const ERROR_STYLE: &str = "color: #ef4444; font-size: 0.75rem; margin-top: 0.25rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";

#[component]
pub fn TextField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current value
    value: RwSignal<String>,
    /// Field configuration
    #[prop(optional)]
    config: Option<TextFieldConfig>,
    /// Description/help text
    
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// Validation error message
    #[prop(optional)]
    error: Option<Signal<Option<String>>>,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let is_multiline = config.multiline;
    let placeholder = config.placeholder.clone().unwrap_or_default();
    let max_length = config.max_length;

    let on_input = move |ev: web_sys::Event| {
        let new_value = event_target_value(&ev);
        value.set(new_value.clone());
        if let Some(cb) = on_change {
            cb.run(new_value);
        }
    };

    let char_count = move || {
        if max_length.is_some() {
            Some(value.get().len())
        } else {
            None
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

            {if is_multiline {
                view! {
                    <textarea
                        style=TEXTAREA_STYLE
                        prop:value=move || value.get()
                        on:input=on_input.clone()
                        placeholder=placeholder.clone()
                        maxlength=max_length.map(|m| m.to_string())
                    />
                }.into_any()
            } else {
                view! {
                    <input
                        type="text"
                        style=INPUT_STYLE
                        prop:value=move || value.get()
                        on:input=on_input.clone()
                        placeholder=placeholder.clone()
                        maxlength=max_length.map(|m| m.to_string())
                    />
                }.into_any()
            }}

            <div style="display: flex; justify-content: space-between; align-items: center;">
                {move || {
                    if let Some(err_signal) = error {
                        if let Some(err) = err_signal.get() {
                            return view! { <span style=ERROR_STYLE>{err}</span> }.into_any();
                        }
                    }
                    if let Some(ref desc) = description {
                        return view! { <span style=HELP_TEXT_STYLE>{desc.clone()}</span> }.into_any();
                    }
                    view! { <span></span> }.into_any()
                }}

                {move || {
                    if let (Some(count), Some(max)) = (char_count(), max_length) {
                        view! {
                            <span style=HELP_TEXT_STYLE>
                                {count} "/" {max}
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
