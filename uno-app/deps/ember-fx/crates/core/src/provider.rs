//! Theme provider component for wrapping the application.
//!
//! `ThemeProvider` initializes the theme system and provides
//! context to all descendant components.

use crate::context::{load_saved_design_system, load_saved_theme, ThemeContext};
use crate::platform::Platform;
use ember_fx_common::DesignSystem;
use ember_fx_styles::{ThemeRegistry, get_component_css};
use ember_fx_utils::{
    apply_theme_attribute, inject_base_css, inject_css,
    THEME_STYLE_ID, COMPONENTS_STYLE_ID,
};
use leptos::prelude::*;

/// Theme provider component.
///
/// Wrap your application with this component to enable theming.
/// It provides:
/// - Theme context for all descendant components
/// - Automatic CSS injection
/// - Platform detection
/// - Theme persistence to localStorage
///
/// # Example
/// ```ignore
/// use ember_fx_core::{ThemeProvider, DesignSystem};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     view! {
///         <ThemeProvider
///             initial_theme="dark"
///             design_system=DesignSystem::Ant
///             persist=true
///         >
///             <Router>
///                 <Routes />
///             </Router>
///         </ThemeProvider>
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(
    /// Initial theme name (e.g., "dark", "light").
    /// If `persist` is true, this is overridden by the saved theme.
    #[prop(default = "dark")]
    initial_theme: &'static str,
    /// Design system to use. Defaults to `Ant`.
    /// If `Auto`, platform detection determines the design system.
    #[prop(default = DesignSystem::Ant)]
    design_system: DesignSystem,
    /// Whether to persist theme preference to localStorage.
    #[prop(default = true)]
    persist: bool,
    /// Child components.
    children: Children,
) -> impl IntoView {
    // Detect platform
    let platform = Platform::detect();

    // Load saved preferences if persistence is enabled
    let theme_name = if persist {
        load_saved_theme(initial_theme)
    } else {
        initial_theme.to_string()
    };

    let actual_design_system = if persist {
        let saved = load_saved_design_system(design_system);
        if saved == DesignSystem::Auto {
            platform.default_design_system()
        } else {
            saved
        }
    } else if design_system == DesignSystem::Auto {
        platform.default_design_system()
    } else {
        design_system
    };

    // Create theme context
    let ctx = ThemeContext::new(&theme_name, actual_design_system, platform);

    // Provide context to children
    provide_context(ctx);

    // Create reactive signal for theme name
    let theme_signal = ctx.theme_signal();

    // Apply initial theme and component CSS
    Effect::new(move |_| {
        inject_base_css();
        apply_theme_attribute(&theme_signal.get());

        // Load and inject theme CSS from registry
        if let Some(css) = ThemeRegistry::load_theme_css(
            ctx.design_system().as_str(),
            &theme_signal.get(),
        ) {
            inject_css(THEME_STYLE_ID, &css);
        }

        // Inject component CSS for the design system
        let component_css = get_component_css(ctx.design_system().as_str());
        if !component_css.is_empty() {
            inject_css(COMPONENTS_STYLE_ID, component_css);
        }
    });

    // Re-inject CSS when theme changes
    Effect::new(move |_| {
        let theme = theme_signal.get();
        apply_theme_attribute(&theme);

        if let Some(css) = ThemeRegistry::load_theme_css(
            ctx.design_system().as_str(),
            &theme,
        ) {
            inject_css(THEME_STYLE_ID, &css);
        }
    });

    view! {
        {children()}
    }
}

/// Minimal theme provider that only manages the data-theme attribute.
///
/// Use this when you have pre-compiled CSS and don't need runtime CSS injection.
/// This is lighter weight than `ThemeProvider`.
#[component]
pub fn MinimalThemeProvider(
    /// Initial theme name.
    #[prop(default = "dark")]
    initial_theme: &'static str,
    /// Design system to use.
    #[prop(default = DesignSystem::Ant)]
    design_system: DesignSystem,
    /// Child components.
    children: Children,
) -> impl IntoView {
    let platform = Platform::detect();
    let actual_design_system = if design_system == DesignSystem::Auto {
        platform.default_design_system()
    } else {
        design_system
    };

    let ctx = ThemeContext::new(initial_theme, actual_design_system, platform);
    provide_context(ctx);

    // Update data-theme attribute reactively
    let theme_signal = ctx.theme_signal();
    Effect::new(move |_| {
        apply_theme_attribute(&theme_signal.get());
    });

    view! {
        {children()}
    }
}
