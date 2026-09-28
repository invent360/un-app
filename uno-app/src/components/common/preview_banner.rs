//! Preview mode banner component
//!
//! Displays a floating banner on the right side indicating preview mode.
//! When clicked, opens a panel with preview settings.

use leptos::prelude::*;

/// Preview banner shown when viewing unpublished content via preview token
#[component]
pub fn PreviewBanner(
    /// Optional callback when exit is clicked
    #[prop(optional)]
    on_exit: Option<Callback<()>>,
) -> impl IntoView {
    let (is_panel_open, set_is_panel_open) = signal(false);

    let handle_exit = move |_| {
        if let Some(cb) = on_exit {
            cb.run(());
        } else {
            // Default: navigate to same page without preview_token
            #[cfg(feature = "hydrate")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(location) = window.location().pathname() {
                        let _ = window.location().set_href(&location);
                    }
                }
            }
        }
    };

    view! {
        // Floating banner on right side
        <div
            class="preview-banner"
            style="
                position: fixed;
                top: 80px;
                right: 0;
                z-index: 9998;
            "
        >
            <button
                on:click=move |_| set_is_panel_open.update(|v| *v = !*v)
                style="
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
                    color: white;
                    padding: 12px 20px;
                    border: none;
                    border-radius: 2px;
                    cursor: pointer;
                    font-size: 14px;
                    font-weight: 700;
                    letter-spacing: 0.5px;
                    box-shadow: 0 4px 16px rgba(245, 158, 11, 0.4);
                    transition: transform 0.2s, box-shadow 0.2s;
                    font-family: system-ui, -apple-system, sans-serif;
                "
            >
                // Caution/Warning triangle icon
                <svg
                    width="18"
                    height="18"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
                    <line x1="12" y1="9" x2="12" y2="13"></line>
                    <line x1="12" y1="17" x2="12.01" y2="17"></line>
                </svg>
                "PREVIEW MODE"
            </button>

            // Settings panel (shown when clicked)
            <Show when=move || is_panel_open.get()>
                <div
                    style="
                        position: absolute;
                        top: 52px;
                        right: 0;
                        width: 280px;
                        background: #1e293b;
                        border: 1px solid #334155;
                        border-radius: 12px;
                        padding: 16px;
                        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
                        font-family: system-ui, -apple-system, sans-serif;
                    "
                >
                    <div style="margin-bottom: 16px;">
                        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 8px;">
                            <svg
                                width="18"
                                height="18"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="#f59e0b"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
                                <line x1="12" y1="9" x2="12" y2="13"></line>
                                <line x1="12" y1="17" x2="12.01" y2="17"></line>
                            </svg>
                            <span style="color: #f59e0b; font-weight: 600; font-size: 14px;">
                                "Preview Mode Active"
                            </span>
                        </div>
                        <p style="color: #94a3b8; font-size: 13px; line-height: 1.5; margin: 0;">
                            "You are viewing unpublished draft content. Changes are not visible to the public."
                        </p>
                    </div>

                    <div style="border-top: 1px solid #334155; padding-top: 12px;">
                        <div style="color: #64748b; font-size: 12px; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px;">
                            "Preview Settings"
                        </div>

                        // Settings placeholder
                        <div style="display: flex; align-items: center; justify-content: space-between; padding: 8px 0;">
                            <span style="color: #e2e8f0; font-size: 13px;">"Show guides overview"</span>
                            <span style="color: #64748b; font-size: 12px;">"Enabled"</span>
                        </div>
                    </div>

                    <button
                        on:click=handle_exit
                        style="
                            width: 100%;
                            margin-top: 12px;
                            background: #dc2626;
                            border: none;
                            color: white;
                            padding: 10px 16px;
                            border-radius: 8px;
                            cursor: pointer;
                            font-size: 13px;
                            font-weight: 500;
                            transition: background 0.2s;
                        "
                    >
                        "Exit Preview Mode"
                    </button>
                </div>
            </Show>
        </div>
    }
}
