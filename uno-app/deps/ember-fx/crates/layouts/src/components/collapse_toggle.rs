//! Sidebar collapse toggle button.

use leptos::prelude::*;

const CHEVRON_LEFT: &str = r#"<path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>"#;
const CHEVRON_RIGHT: &str = r#"<path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>"#;

/// Collapse toggle button for the sidebar.
///
/// This button appears on the right edge of the sidebar and allows users
/// to collapse/expand it. Only visible on desktop.
#[component]
pub fn CollapseToggle(
    /// Whether the sidebar is collapsed.
    #[prop(into)]
    collapsed: RwSignal<bool>,
) -> impl IntoView {
    let toggle = move |_| {
        collapsed.update(|c| *c = !*c);
    };

    // Use Memos to derive values with proper reactive context
    let aria_label = Memo::new(move |_| {
        if collapsed.get() { "Expand sidebar" } else { "Collapse sidebar" }
    });

    let icon_html = Memo::new(move |_| {
        if collapsed.get() { CHEVRON_RIGHT } else { CHEVRON_LEFT }
    });

    view! {
        <button
            class="fx-sidebar-collapse-toggle"
            on:click=toggle
            aria-label=aria_label
            title=aria_label
        >
            <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="currentColor"
                inner_html=icon_html
            />
        </button>
    }
}
