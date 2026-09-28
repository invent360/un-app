//! Component inspector for debugging component hierarchy.

use leptos::prelude::*;

/// Component inspector for debugging.
///
/// Provides a way to inspect the component tree and highlight elements.
///
/// # Example
///
/// ```ignore
/// use ember_fx_tools::ComponentInspector;
///
/// view! {
///     <ComponentInspector />
/// }
/// ```
#[component]
pub fn ComponentInspector(
    /// Whether inspection mode is initially active.
    #[prop(optional)]
    initial_active: Option<bool>,
) -> impl IntoView {
    let is_active = RwSignal::new(initial_active.unwrap_or(false));
    let _hovered_element = RwSignal::new(Option::<String>::None);
    let selected_info = RwSignal::new(Option::<ElementInfo>::None);

    view! {
        <div
            class="fx-component-inspector"
            style="position: fixed; top: 16px; right: 16px; z-index: 9999; font-family: monospace; font-size: 12px;"
        >
            <button
                type="button"
                style=move || format!(
                    "padding: 8px 12px; background: {}; color: #eee; border: 1px solid {}; border-radius: 4px; cursor: pointer;",
                    if is_active.get() { "#2e7d32" } else { "#1a1a2e" },
                    if is_active.get() { "#4caf50" } else { "#333" }
                )
                on:click=move |_| is_active.update(|v| *v = !*v)
            >
                {move || if is_active.get() { "Stop Inspect" } else { "Inspect" }}
            </button>

            {move || {
                if let Some(info) = selected_info.get() {
                    let id = info.id.clone();
                    let classes = info.classes.clone();
                    let id_display = id.clone();

                    view! {
                        <div
                            style="margin-top: 8px; padding: 16px; background: #1a1a2e; border: 1px solid #333; border-radius: 8px; min-width: 280px; color: #eee;"
                        >
                            <h3 style="margin: 0 0 12px 0; font-size: 14px; font-weight: 600;">
                                "Element Info"
                            </h3>

                            <div style="margin-bottom: 8px;">
                                <span style="color: #888;">"Tag: "</span>
                                <span style="color: #ff9800;">{info.tag_name.clone()}</span>
                            </div>

                            {if !id.is_empty() {
                                view! {
                                    <div style="margin-bottom: 8px;">
                                        <span style="color: #888;">"ID: "</span>
                                        <span style="color: #4fc3f7;">{"#"}{id_display}</span>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span /> }.into_any()
                            }}

                            {if !classes.is_empty() {
                                view! {
                                    <div style="margin-bottom: 8px;">
                                        <span style="color: #888;">"Classes: "</span>
                                        <div style="margin-top: 4px; display: flex; flex-wrap: wrap; gap: 4px;">
                                            {classes.iter().map(|c| {
                                                let class_name = c.clone();
                                                view! {
                                                    <span style="padding: 2px 6px; background: #2a2a3e; border-radius: 3px; color: #81c784;">
                                                        {"."}{class_name}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span /> }.into_any()
                            }}

                            <div style="margin-bottom: 8px;">
                                <span style="color: #888;">"Size: "</span>
                                <span style="color: #eee;">
                                    {format!("{}x{}", info.width, info.height)}
                                </span>
                            </div>

                            <div>
                                <span style="color: #888;">"Position: "</span>
                                <span style="color: #eee;">
                                    {format!("({}, {})", info.x, info.y)}
                                </span>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }
            }}

            <Show when=move || is_active.get()>
                <div style="margin-top: 8px; padding: 8px 12px; background: #2e7d32; border-radius: 4px; color: #fff;">
                    "Click any element to inspect"
                </div>
            </Show>
        </div>
    }
}

/// Information about an inspected element.
#[derive(Clone, Debug, Default)]
struct ElementInfo {
    tag_name: String,
    id: String,
    classes: Vec<String>,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}
