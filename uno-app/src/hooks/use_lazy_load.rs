//! Lazy loading utilities for performance optimization

use leptos::prelude::*;

/// A lazy-loaded image component using native loading attribute
#[component]
pub fn LazyImage(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] width: Option<u32>,
    #[prop(into, optional)] height: Option<u32>,
) -> impl IntoView {
    view! {
        <img
            src=src
            alt=alt
            class=class.unwrap_or_default()
            width=width.map(|w| w.to_string())
            height=height.map(|h| h.to_string())
            loading="lazy"
            decoding="async"
        />
    }
}

/// Resource hints for preloading critical assets
#[component]
pub fn ResourceHints() -> impl IntoView {
    view! {
        // Preconnect to external domains
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />

        // DNS prefetch for analytics (if any)
        <link rel="dns-prefetch" href="https://www.google-analytics.com" />

        // Preload critical fonts
        <link
            rel="preload"
            href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
            r#as="style"
        />
    }
}

/// Skeleton loading placeholder
#[component]
pub fn Skeleton(
    #[prop(default = "100%")] width: &'static str,
    #[prop(default = "1rem")] height: &'static str,
    #[prop(default = false)] rounded: bool,
) -> impl IntoView {
    let border_radius = if rounded { "50%" } else { "var(--radius-sm)" };

    view! {
        <div
            class="skeleton-line"
            style=format!("width: {}; height: {}; border-radius: {};", width, height, border_radius)
        />
    }
}

/// Multiple skeleton lines for content placeholders
#[component]
pub fn SkeletonText(
    #[prop(default = 3)] lines: usize,
) -> impl IntoView {
    view! {
        <div class="skeleton-text">
            {(0..lines).map(|i| {
                let width = match i % 3 {
                    0 => "100%",
                    1 => "85%",
                    _ => "70%",
                };
                view! {
                    <Skeleton width=width />
                }
            }).collect_view()}
        </div>
    }
}

/// Skeleton card for loading states
#[component]
pub fn SkeletonCard() -> impl IntoView {
    view! {
        <div class="skeleton-card">
            <Skeleton height="2rem" width="60%" />
            <SkeletonText lines=3 />
        </div>
    }
}
