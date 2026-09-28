//! Card component - re-exports from ember-fx
//!
//! This module provides Card components from ember-fx with
//! full feature support (header/body/footer, hoverable, loading states).

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::layout::{
    Card,
    CardSize,
};

// Re-export for backward compatibility: when ember-fx is not available, provide stubs
#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
mod fallback {
    use leptos::prelude::*;

    /// Card size for fallback
    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum CardSize {
        Small,
        #[default]
        Default,
    }

    /// Fallback Card component
    #[component]
    pub fn Card(
        #[prop(optional, into)] class: Option<String>,
        #[prop(optional)] elevated: bool,
        children: Children,
    ) -> impl IntoView {
        let class_name = format!(
            "card {} {}",
            if elevated { "card-elevated" } else { "" },
            class.unwrap_or_default()
        );

        view! {
            <div class=class_name>
                {children()}
            </div>
        }
    }
}

#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
pub use fallback::*;
