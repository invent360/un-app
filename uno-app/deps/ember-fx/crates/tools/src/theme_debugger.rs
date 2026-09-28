//! Theme debugger component for inspecting and switching themes.

use leptos::prelude::*;
use ember_fx_core::{try_use_theme, ThemeRegistry};

/// Theme debugger panel for development.
///
/// Displays current theme information and allows switching themes at runtime.
///
/// # Example
///
/// ```ignore
/// use ember_fx_tools::ThemeDebugger;
///
/// view! {
///     <ThemeDebugger />
/// }
/// ```
#[component]
pub fn ThemeDebugger(
    /// Whether the debugger panel is initially open.
    #[prop(optional)]
    initial_open: Option<bool>,
    /// Position of the debugger panel.
    #[prop(optional, into)]
    position: Option<String>,
) -> impl IntoView {
    let is_open = RwSignal::new(initial_open.unwrap_or(false));
    let position = position.unwrap_or_else(|| "bottom-right".to_string());

    let theme_ctx = try_use_theme();

    let position_style = match position.as_str() {
        "top-left" => "top: 16px; left: 16px;",
        "top-right" => "top: 16px; right: 16px;",
        "bottom-left" => "bottom: 16px; left: 16px;",
        _ => "bottom: 16px; right: 16px;",
    };

    view! {
        <div
            class="fx-theme-debugger"
            style=format!(
                "position: fixed; {}; z-index: 9999; font-family: monospace; font-size: 12px;",
                position_style
            )
        >
            <button
                type="button"
                class="fx-theme-debugger-toggle"
                style="padding: 8px 12px; background: #1a1a2e; color: #eee; border: 1px solid #333; border-radius: 4px; cursor: pointer;"
                on:click=move |_| is_open.update(|v| *v = !*v)
            >
                {move || if is_open.get() { "Close" } else { "Theme" }}
            </button>

            <Show when=move || is_open.get()>
                <div
                    class="fx-theme-debugger-panel"
                    style="margin-top: 8px; padding: 16px; background: #1a1a2e; border: 1px solid #333; border-radius: 8px; min-width: 250px; color: #eee;"
                >
                    <h3 style="margin: 0 0 12px 0; font-size: 14px; font-weight: 600;">
                        "Theme Debugger"
                    </h3>

                    {move || {
                        if let Some(ctx) = theme_ctx {
                            let current_theme = ctx.theme();
                            let current_system = ctx.design_system();
                            let system_str = current_system.as_str();

                            // Get available themes for this design system
                            let themes: Vec<String> = ThemeRegistry::themes_for_system(system_str)
                                .into_iter()
                                .map(|t| t.name.to_string())
                                .collect();

                            view! {
                                <div class="fx-debugger-info" style="margin-bottom: 12px;">
                                    <div style="margin-bottom: 8px;">
                                        <span style="color: #888;">"Design System: "</span>
                                        <span style="color: #4fc3f7;">{format!("{:?}", current_system)}</span>
                                    </div>
                                    <div style="margin-bottom: 8px;">
                                        <span style="color: #888;">"Current Theme: "</span>
                                        <span style="color: #81c784;">{current_theme.clone()}</span>
                                    </div>
                                </div>

                                <div class="fx-debugger-themes">
                                    <label style="display: block; margin-bottom: 8px; color: #888;">
                                        "Switch Theme:"
                                    </label>
                                    <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                                        {themes.into_iter().map(|theme| {
                                            let theme_name = theme.clone();
                                            let theme_for_click = theme.clone();
                                            let is_active = current_theme == theme;
                                            view! {
                                                <button
                                                    type="button"
                                                    style=format!(
                                                        "padding: 4px 8px; border-radius: 4px; cursor: pointer; border: 1px solid {}; background: {}; color: {};",
                                                        if is_active { "#4fc3f7" } else { "#444" },
                                                        if is_active { "#1e3a5f" } else { "#2a2a3e" },
                                                        if is_active { "#4fc3f7" } else { "#aaa" }
                                                    )
                                                    on:click=move |_| {
                                                        ctx.set_theme(&theme_for_click);
                                                    }
                                                >
                                                    {theme_name}
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div style="color: #f44336;">
                                    "No ThemeProvider found. Wrap your app in ThemeProvider."
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </Show>
        </div>
    }
}
