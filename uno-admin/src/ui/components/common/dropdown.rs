use leptos::prelude::*;
use crate::state::Period;
// NOTE: ember_fx_components not available - using stub implementation
// use ember_fx_components::{Select, SelectOption};

/// Stub SelectOption type (ember_fx_components not available)
#[allow(dead_code)]
pub struct SelectOption<T> {
    pub value: T,
    pub label: String,
}

impl<T> SelectOption<T> {
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self { value, label: label.into() }
    }
}

/// Period dropdown selector (stub - ember-fx Select not available)
#[component]
pub fn PeriodDropdown(
    value: RwSignal<Period>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <select
            class=format!("px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-white {}", class)
            on:change=move |ev| {
                let selected = event_target_value(&ev);
                if let Some(period) = Period::from_str(&selected) {
                    value.set(period);
                }
            }
        >
            {Period::all().iter().map(|p| {
                let p_str = p.to_string();
                let p_label = p.label();
                let is_selected = value.get() == *p;
                view! {
                    <option value=p_str selected=is_selected>{p_label}</option>
                }
            }).collect_view()}
        </select>
    }
}
