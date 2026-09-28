//! CSS loader for dynamic theme injection.
//!
//! Provides utilities for injecting theme CSS into the DOM at runtime
//! and managing the `data-theme` attribute on the document root.

#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Style element ID for injected theme CSS.
pub const THEME_STYLE_ID: &str = "ember-theme-css";

/// Style element ID for injected component CSS.
pub const COMPONENTS_STYLE_ID: &str = "ember-theme-components";

/// Style element ID for base CSS variables.
pub const BASE_STYLE_ID: &str = "ember-theme-base";

/// Apply a theme by setting the `data-theme` attribute on `<html>`.
///
/// This is the primary mechanism for theme switching. CSS rules using
/// `[data-theme="..."]` selectors will match based on this attribute.
///
/// # Example
/// ```ignore
/// apply_theme_attribute("dark");
/// // <html data-theme="dark">
/// ```
pub fn apply_theme_attribute(#[allow(unused)] theme_name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-theme", theme_name);
                }
            }
        }
    }
}

/// Get the current `data-theme` attribute value from `<html>`.
#[must_use]
pub fn get_current_theme_attribute() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
            .and_then(|html| html.get_attribute("data-theme"))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Inject CSS into the document head via a `<style>` element.
///
/// If a style element with the given ID already exists, its content
/// will be replaced. Otherwise, a new style element is created.
///
/// # Arguments
/// * `style_id` - The ID for the style element (for later updates)
/// * `css_content` - The CSS content to inject
///
/// # Example
/// ```ignore
/// inject_css("my-theme", ":root { --color-primary: blue; }");
/// ```
pub fn inject_css(#[allow(unused)] style_id: &str, #[allow(unused)] css_content: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // Try to find existing style element
                let style_el = document.get_element_by_id(style_id).or_else(|| {
                    // Create new style element
                    let el = document.create_element("style").ok()?;
                    el.set_id(style_id);

                    // Append to head
                    document.head()?.append_child(&el).ok()?;
                    Some(el)
                });

                // Set content
                if let Some(el) = style_el {
                    el.set_text_content(Some(css_content));
                }
            }
        }
    }
}

/// Remove a style element by ID.
pub fn remove_css(#[allow(unused)] style_id: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(el) = document.get_element_by_id(style_id) {
                    el.remove();
                }
            }
        }
    }
}

/// Check if a style element with the given ID exists.
#[must_use]
pub fn has_css(#[allow(unused)] style_id: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(style_id))
            .is_some()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Inject the base CSS variables (fallback values).
///
/// These provide sensible defaults that will be overridden by theme CSS.
pub fn inject_base_css() {
    const BASE_CSS: &str = r#"
:root {
  /* Color system defaults */
  --fx-color-primary: oklch(58% 0.233 277);
  --fx-color-primary-content: oklch(96% 0.018 272);
  --fx-color-secondary: oklch(65% 0.195 275);
  --fx-color-secondary-content: oklch(96% 0.018 272);
  --fx-color-accent: oklch(70% 0.18 155);
  --fx-color-accent-content: oklch(15% 0.02 155);
  --fx-color-neutral: oklch(35% 0.02 260);
  --fx-color-neutral-content: oklch(98% 0.01 260);

  /* Base backgrounds */
  --fx-color-base-100: oklch(25% 0.016 252);
  --fx-color-base-200: oklch(22% 0.014 250);
  --fx-color-base-300: oklch(19% 0.012 248);
  --fx-color-base-content: oklch(98% 0.029 257);

  /* Semantic colors */
  --fx-color-info: oklch(74% 0.16 232);
  --fx-color-info-content: oklch(15% 0.02 232);
  --fx-color-success: oklch(76% 0.177 163);
  --fx-color-success-content: oklch(15% 0.02 163);
  --fx-color-warning: oklch(82% 0.189 84);
  --fx-color-warning-content: oklch(15% 0.02 84);
  --fx-color-error: oklch(71% 0.194 13);
  --fx-color-error-content: oklch(96% 0.02 13);

  /* Radius */
  --fx-radius-sm: 0.25rem;
  --fx-radius-md: 0.5rem;
  --fx-radius-lg: 1rem;
  --fx-radius-full: 9999px;

  /* Spacing */
  --fx-spacing-xs: 0.25rem;
  --fx-spacing-sm: 0.5rem;
  --fx-spacing-md: 1rem;
  --fx-spacing-lg: 1.5rem;
  --fx-spacing-xl: 2rem;

  /* Touch targets */
  --fx-touch-target: 44px;

  /* Transitions */
  --fx-transition-fast: 150ms;
  --fx-transition-normal: 250ms;
  --fx-transition-slow: 350ms;
}

/* Safe area support */
@supports (padding: env(safe-area-inset-bottom)) {
  .fx-safe-top { padding-top: env(safe-area-inset-top); }
  .fx-safe-bottom { padding-bottom: env(safe-area-inset-bottom); }
  .fx-safe-left { padding-left: env(safe-area-inset-left); }
  .fx-safe-right { padding-right: env(safe-area-inset-right); }
}

/* Remove tap highlight on mobile */
* {
  -webkit-tap-highlight-color: transparent;
}

/* Smooth transitions for theme changes (excluding text color) */
*, *::before, *::after {
  transition: background-color var(--fx-transition-normal),
              border-color var(--fx-transition-normal);
}
"#;

    inject_css(BASE_STYLE_ID, BASE_CSS);
}
