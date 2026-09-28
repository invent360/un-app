//! Mobile navigation bar component.

use leptos::prelude::*;

/// Action button for the navigation bar.
#[derive(Clone)]
pub struct NavBarAction {
    /// Icon SVG content or text.
    pub icon: String,
    /// Accessible label.
    pub label: String,
    /// Click handler.
    pub on_click: Callback<()>,
}

impl NavBarAction {
    /// Create a new nav bar action.
    pub fn new(icon: impl Into<String>, label: impl Into<String>, on_click: Callback<()>) -> Self {
        Self {
            icon: icon.into(),
            label: label.into(),
            on_click,
        }
    }
}

/// Mobile navigation bar component.
///
/// A top navigation bar optimized for mobile interfaces with support for
/// back button, title, and action buttons.
///
/// # Example
///
/// ```ignore
/// use ember_fx_mobile::{MobileNavBar, NavBarAction};
///
/// view! {
///     <MobileNavBar
///         title="Settings"
///         show_back=true
///         on_back=Callback::new(|_| { /* navigate back */ })
///     />
/// }
/// ```
#[component]
pub fn MobileNavBar(
    /// Title text displayed in the center.
    #[prop(optional, into)]
    title: Option<String>,
    /// Whether to show the back button.
    #[prop(optional)]
    show_back: Option<bool>,
    /// Callback when back button is clicked.
    #[prop(optional)]
    on_back: Option<Callback<()>>,
    /// Right-side action buttons.
    #[prop(optional)]
    actions: Option<Vec<NavBarAction>>,
    /// Whether the bar has a bottom border.
    #[prop(optional)]
    bordered: Option<bool>,
    /// Whether the bar is transparent.
    #[prop(optional)]
    transparent: Option<bool>,
) -> impl IntoView {
    let show_back = show_back.unwrap_or(false);
    let bordered = bordered.unwrap_or(true);
    let transparent = transparent.unwrap_or(false);
    let actions = actions.unwrap_or_default();

    let bar_style = format!(
        "display: flex; align-items: center; justify-content: space-between; height: 56px; padding: 0 16px; {}{}",
        if transparent { "background: transparent;" } else { "background: var(--fx-bg-container, #fff);" },
        if bordered { "border-bottom: 1px solid var(--fx-border-color, #e8e8e8);" } else { "" }
    );

    view! {
        <header
            class="fx-mobile-nav-bar"
            style=bar_style
        >
            // Left section (back button)
            <div style="min-width: 60px; display: flex; align-items: center;">
                {if show_back {
                    view! {
                        <button
                            type="button"
                            class="fx-mobile-nav-back"
                            style="display: flex; align-items: center; justify-content: center; width: 40px; height: 40px; border: none; background: transparent; cursor: pointer; border-radius: 50%;"
                            aria-label="Go back"
                            on:click=move |_| {
                                if let Some(ref cb) = on_back {
                                    cb.run(());
                                }
                            }
                        >
                            <svg viewBox="0 0 24 24" fill="currentColor" style="width: 24px; height: 24px;">
                                <path d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/>
                            </svg>
                        </button>
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }}
            </div>

            // Center section (title)
            <div style="flex: 1; text-align: center; font-size: 17px; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                {title.unwrap_or_default()}
            </div>

            // Right section (actions)
            <div style="min-width: 60px; display: flex; align-items: center; justify-content: flex-end; gap: 8px;">
                {actions.into_iter().map(|action| {
                    let label = action.label.clone();
                    let icon = action.icon.clone();
                    let on_click = action.on_click;

                    view! {
                        <button
                            type="button"
                            class="fx-mobile-nav-action"
                            style="display: flex; align-items: center; justify-content: center; width: 40px; height: 40px; border: none; background: transparent; cursor: pointer; border-radius: 50%;"
                            aria-label=label
                            on:click=move |_| on_click.run(())
                            inner_html=icon.clone()
                        />
                    }
                }).collect_view()}
            </div>
        </header>
    }
}
