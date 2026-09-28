//! Loading/Spinner/Skeleton components - re-exports from ember-fx
//!
//! This module provides loading indicator components from ember-fx with
//! full feature support (tips, nested content, animations).

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::notification::{
    Spinner,
    Skeleton,
    SpinnerSize,
    SkeletonButton,
    SkeletonInput,
    SkeletonImage,
};

// Convenience aliases
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::notification::Spinner as LoadingSpinner;

// Backward compatibility components using ember-fx
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
mod compat {
    use super::*;
    use leptos::prelude::*;

    /// Full page loading state using ember-fx Spinner.
    #[component]
    pub fn LoadingPage(
        #[prop(optional, into)] message: Option<String>,
    ) -> impl IntoView {
        view! {
            <div class="loading-page">
                <Spinner
                    size=SpinnerSize::Large
                    tip=message.unwrap_or_default()
                />
            </div>
        }
    }

    /// Skeleton line using ember-fx Skeleton.
    #[component]
    pub fn SkeletonLine(
        #[prop(optional)] width: Option<&'static str>,
        #[prop(optional)] active: bool,
    ) -> impl IntoView {
        let width_str = width.unwrap_or("100%");

        view! {
            <Skeleton
                active=active
                title=true
                title_width=width_str.to_string()
                paragraph=false
            />
        }
    }

    /// Skeleton card using ember-fx Skeleton.
    #[component]
    pub fn SkeletonCard(
        #[prop(optional)] active: bool,
        #[prop(optional)] avatar: bool,
    ) -> impl IntoView {
        view! {
            <Skeleton
                active=active
                avatar=avatar
                title=true
                paragraph=true
            />
        }
    }
}

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use compat::{LoadingPage, SkeletonLine, SkeletonCard};

// Fallback for non-ember-fx builds
#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
mod fallback {
    use leptos::prelude::*;

    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum SpinnerSize {
        Small,
        #[default]
        Default,
        Large,
    }

    #[component]
    pub fn LoadingSpinner(
        #[prop(optional)] size: Option<&'static str>,
    ) -> impl IntoView {
        let size_class = size.unwrap_or("md");

        view! {
            <div class=format!("loading-spinner spinner-{}", size_class)>
                <div class="spinner-ring"></div>
            </div>
        }
    }

    #[component]
    pub fn LoadingPage(
        #[prop(optional)] message: Option<String>,
    ) -> impl IntoView {
        view! {
            <div class="loading-page">
                <LoadingSpinner size="lg" />
                {message.map(|msg| view! { <p class="loading-message">{msg}</p> })}
            </div>
        }
    }

    #[component]
    pub fn SkeletonLine(
        #[prop(optional)] width: Option<&'static str>,
    ) -> impl IntoView {
        let style = format!("width: {}", width.unwrap_or("100%"));
        view! {
            <div class="skeleton-line" style=style></div>
        }
    }

    #[component]
    pub fn SkeletonCard() -> impl IntoView {
        view! {
            <div class="skeleton-card">
                <SkeletonLine width="60%" />
                <SkeletonLine width="100%" />
                <SkeletonLine width="80%" />
            </div>
        }
    }
}

#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
pub use fallback::*;
