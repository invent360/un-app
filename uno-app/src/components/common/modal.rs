//! Modal component - re-exports from ember-fx
//!
//! This module provides Modal components from ember-fx with
//! full feature support (focus trapping, escape key handling, accessibility).

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::layout::{
    Modal,
    ModalSize,
};

// Re-export for backward compatibility: when ember-fx is not available, provide stubs
#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
mod fallback {
    use leptos::prelude::*;

    /// Modal size for fallback
    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum ModalSize {
        Small,
        #[default]
        Default,
        Large,
        FullScreen,
    }

    /// Fallback Modal component (stub - full implementation requires ember-fx features)
    #[component]
    pub fn Modal(
        #[prop(into)] open: Signal<bool>,
        #[prop(into)] on_close: Callback<()>,
        #[prop(optional, into)] title: Option<String>,
        #[allow(unused_variables)]
        children: Children,
    ) -> impl IntoView {
        let title_clone = title.clone();
        view! {
            {move || {
                let title_inner = title_clone.clone();
                open.get().then(move || view! {
                    <div
                        class="modal-overlay"
                        on:click=move |_| on_close.run(())
                    >
                        <div
                            class="modal-content"
                            on:click=move |_| { /* stop propagation handled by CSS/JS in full implementation */ }
                        >
                            <div class="modal-header">
                                {title_inner.map(|t| view! { <h3 class="modal-title">{t}</h3> })}
                                <button class="modal-close" on:click=move |_| on_close.run(())>
                                    "×"
                                </button>
                            </div>
                            <div class="modal-body">
                                // Modal body - children rendered in full implementation
                            </div>
                        </div>
                    </div>
                })
            }}
        }
    }
}

#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
pub use fallback::*;
