//! Number field component for numeric input

use leptos::prelude::*;
use crate::api::schema_types::NumberFieldConfig;

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const INPUT_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
const INPUT_WITH_UNIT_STYLE: &str = "flex: 1; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem 0 0 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; border-right: none;";
const UNIT_STYLE: &str = "background: #0f172a; border: 1px solid #334155; border-radius: 0 0.375rem 0.375rem 0; padding: 0.625rem 0.75rem; color: #64748b; font-size: 0.875rem;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";

#[component]
pub fn NumberField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current value
    value: RwSignal<Option<f64>>,
    /// Field configuration
    #[prop(optional)]
    config: Option<NumberFieldConfig>,
    /// Description/help text
    
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Option<f64>>>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let min = config.min;
    let max = config.max;
    let step = config.step.unwrap_or(1.0);
    let unit = config.unit.clone();

    let string_value = RwSignal::new(
        value.get().map(|v| v.to_string()).unwrap_or_default()
    );

    let on_input = move |ev: web_sys::Event| {
        let str_val = event_target_value(&ev);
        string_value.set(str_val.clone());

        let parsed = if str_val.is_empty() {
            None
        } else {
            str_val.parse::<f64>().ok()
        };

        value.set(parsed);
        if let Some(cb) = on_change {
            cb.run(parsed);
        }
    };

    let has_unit = unit.is_some();

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

            {if has_unit {
                let unit_text = unit.clone().unwrap_or_default();
                view! {
                    <div style="display: flex;">
                        <input
                            type="number"
                            style=INPUT_WITH_UNIT_STYLE
                            prop:value=move || string_value.get()
                            on:input=on_input.clone()
                            min=min.map(|m| m.to_string())
                            max=max.map(|m| m.to_string())
                            step=step.to_string()
                        />
                        <span style=UNIT_STYLE>{unit_text}</span>
                    </div>
                }.into_any()
            } else {
                view! {
                    <input
                        type="number"
                        style=INPUT_STYLE
                        prop:value=move || string_value.get()
                        on:input=on_input.clone()
                        min=min.map(|m| m.to_string())
                        max=max.map(|m| m.to_string())
                        step=step.to_string()
                    />
                }.into_any()
            }}

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
