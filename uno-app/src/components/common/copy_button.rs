//! Copy to clipboard button

use leptos::prelude::*;
use crate::hooks::use_clipboard;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_icons::tabler::{TablerIcon, DocumentIcon};

#[component]
pub fn CopyButton(
    #[prop(into)] text: String,
    /// Optional callback when copy is triggered
    #[prop(optional)] on_copy: Option<Callback<()>>,
) -> impl IntoView {
    let (copied, copy) = use_clipboard();
    let text_to_copy = text.clone();

    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        view! {
            <button
                class="copy-btn"
                class:copied=move || copied.get()
                on:click=move |_| {
                    copy(text_to_copy.clone());
                    if let Some(callback) = on_copy {
                        callback.run(());
                    }
                }
                title="Copy to clipboard"
            >
                {move || if copied.get() {
                    view! { <TablerIcon icon=DocumentIcon::CopyCheck /> }.into_any()
                } else {
                    view! { <TablerIcon icon=DocumentIcon::Copy /> }.into_any()
                }}
            </button>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! {
            <button
                class="copy-btn"
                class:copied=move || copied.get()
                on:click=move |_| {
                    copy(text_to_copy.clone());
                    if let Some(callback) = on_copy {
                        callback.run(());
                    }
                }
                title="Copy to clipboard"
            >
                {move || if copied.get() { "✓" } else { "📋" }}
            </button>
        }
    }
}
