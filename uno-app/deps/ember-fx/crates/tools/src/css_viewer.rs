//! CSS variable viewer for inspecting custom properties.

use leptos::prelude::*;

/// CSS Variable viewer for debugging theme variables.
///
/// Displays all CSS custom properties defined on the root element.
///
/// # Example
///
/// ```ignore
/// use ember_fx_tools::CSSVariableViewer;
///
/// view! {
///     <CSSVariableViewer />
/// }
/// ```
#[component]
pub fn CSSVariableViewer(
    /// Filter variables by prefix (e.g., "--fx-", "--ant-").
    #[prop(optional, into)]
    prefix_filter: Option<String>,
    /// Whether the panel is initially open.
    #[prop(optional)]
    initial_open: Option<bool>,
) -> impl IntoView {
    let is_open = RwSignal::new(initial_open.unwrap_or(false));
    let search_query = RwSignal::new(String::new());
    let variables = RwSignal::new(Vec::<(String, String)>::new());
    let prefix = prefix_filter.unwrap_or_default();

    // Load CSS variables on mount
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(root) = document.document_element() {
                        if let Ok(styles) = window.get_computed_style(&root) {
                            if let Some(style) = styles {
                                let mut vars = Vec::new();
                                let len = style.length();
                                for i in 0..len {
                                    if let Some(name) = style.item(i).as_string() {
                                        if name.starts_with("--") {
                                            if let Ok(value) = style.get_property_value(&name) {
                                                vars.push((name, value.trim().to_string()));
                                            }
                                        }
                                    }
                                }
                                vars.sort_by(|a, b| a.0.cmp(&b.0));
                                variables.set(vars);
                            }
                        }
                    }
                }
            }
        }
    });

    let prefix_clone = prefix.clone();
    let filtered_vars = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        let prefix_ref = &prefix_clone;
        variables.get()
            .into_iter()
            .filter(|(name, _)| {
                let matches_prefix = prefix_ref.is_empty() || name.starts_with(prefix_ref);
                let matches_query = query.is_empty() || name.to_lowercase().contains(&query);
                matches_prefix && matches_query
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div
            class="fx-css-viewer"
            style="position: fixed; top: 16px; left: 16px; z-index: 9999; font-family: monospace; font-size: 12px;"
        >
            <button
                type="button"
                style="padding: 8px 12px; background: #1a1a2e; color: #eee; border: 1px solid #333; border-radius: 4px; cursor: pointer;"
                on:click=move |_| is_open.update(|v| *v = !*v)
            >
                {move || if is_open.get() { "Close CSS" } else { "CSS Vars" }}
            </button>

            <Show when=move || is_open.get()>
                <div
                    style="margin-top: 8px; padding: 16px; background: #1a1a2e; border: 1px solid #333; border-radius: 8px; width: 350px; max-height: 500px; overflow: hidden; display: flex; flex-direction: column; color: #eee;"
                >
                    <h3 style="margin: 0 0 12px 0; font-size: 14px; font-weight: 600;">
                        "CSS Variables"
                    </h3>

                    <input
                        type="text"
                        placeholder="Search variables..."
                        style="padding: 8px; margin-bottom: 12px; background: #2a2a3e; border: 1px solid #444; border-radius: 4px; color: #eee;"
                        on:input=move |ev| {
                            search_query.set(event_target_value(&ev));
                        }
                    />

                    <div style="color: #888; margin-bottom: 8px; font-size: 11px;">
                        {move || format!("{} variables", filtered_vars.get().len())}
                    </div>

                    <div style="overflow-y: auto; flex: 1;">
                        <For
                            each=move || filtered_vars.get()
                            key=|(name, _)| name.clone()
                            children=move |(name, value)| {
                                let is_color = value.starts_with('#') ||
                                    value.starts_with("rgb") ||
                                    value.starts_with("hsl");
                                let value_clone = value.clone();

                                view! {
                                    <div style="padding: 4px 0; border-bottom: 1px solid #333;">
                                        <div style="color: #4fc3f7; word-break: break-all;">
                                            {name}
                                        </div>
                                        <div style="display: flex; align-items: center; gap: 8px; margin-top: 2px;">
                                            {if is_color {
                                                view! {
                                                    <span
                                                        style=format!(
                                                            "display: inline-block; width: 14px; height: 14px; border-radius: 2px; border: 1px solid #555; background: {};",
                                                            value_clone
                                                        )
                                                    />
                                                }.into_any()
                                            } else {
                                                view! { <span /> }.into_any()
                                            }}
                                            <span style="color: #81c784; word-break: break-all;">
                                                {value}
                                            </span>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </Show>
        </div>
    }
}
