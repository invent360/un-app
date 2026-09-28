//! Boolean field component (checkbox/toggle)

use leptos::prelude::*;
use crate::api::schema_types::BooleanFieldConfig;

const CONTAINER_STYLE: &str = "display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;";
const TOGGLE_STYLE: &str = "position: relative; width: 44px; height: 24px; background: #334155; border-radius: 12px; cursor: pointer; transition: background 0.2s;";
const TOGGLE_ACTIVE_STYLE: &str = "position: relative; width: 44px; height: 24px; background: #3b82f6; border-radius: 12px; cursor: pointer; transition: background 0.2s;";
const TOGGLE_KNOB_STYLE: &str = "position: absolute; top: 2px; left: 2px; width: 20px; height: 20px; background: white; border-radius: 50%; transition: transform 0.2s;";
const TOGGLE_KNOB_ACTIVE_STYLE: &str = "position: absolute; top: 2px; left: 2px; width: 20px; height: 20px; background: white; border-radius: 50%; transition: transform 0.2s; transform: translateX(20px);";
const LABEL_STYLE: &str = "color: #e2e8f0; font-size: 0.875rem;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;";

#[component]
pub fn BooleanField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current value
    value: RwSignal<bool>,
    /// Field configuration
    #[prop(optional)]
    config: Option<BooleanFieldConfig>,
    /// Description/help text
    
    description: Option<String>,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
) -> impl IntoView {
    let toggle = move |_| {
        let new_value = !value.get();
        value.set(new_value);
        if let Some(cb) = on_change {
            cb.run(new_value);
        }
    };

    view! {
        <div class="form-field">
            <div style=CONTAINER_STYLE>
                <button
                    type="button"
                    style=move || if value.get() { TOGGLE_ACTIVE_STYLE } else { TOGGLE_STYLE }
                    on:click=toggle
                >
                    <span style=move || if value.get() { TOGGLE_KNOB_ACTIVE_STYLE } else { TOGGLE_KNOB_STYLE }></span>
                </button>
                <span style=LABEL_STYLE>{label}</span>
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
