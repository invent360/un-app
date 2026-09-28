//! Mobile tab bar component.

use leptos::prelude::*;

/// A tab item for the mobile tab bar.
#[derive(Clone)]
pub struct TabItem {
    /// Unique key for this tab.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Icon SVG content.
    pub icon: String,
    /// Optional active icon SVG (if different from inactive).
    pub active_icon: Option<String>,
    /// Optional badge count.
    pub badge: Option<u32>,
}

impl TabItem {
    /// Create a new tab item.
    pub fn new(key: impl Into<String>, label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            icon: icon.into(),
            active_icon: None,
            badge: None,
        }
    }

    /// Set an active icon.
    pub fn active_icon(mut self, icon: impl Into<String>) -> Self {
        self.active_icon = Some(icon.into());
        self
    }

    /// Set a badge count.
    pub fn badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }
}

/// Mobile tab bar component.
///
/// A bottom navigation bar for mobile apps with support for icons, labels, and badges.
///
/// # Example
///
/// ```ignore
/// use ember_fx_mobile::{MobileTabBar, TabItem};
///
/// let active_tab = RwSignal::new("home".to_string());
/// let tabs = vec![
///     TabItem::new("home", "Home", HOME_ICON),
///     TabItem::new("search", "Search", SEARCH_ICON),
///     TabItem::new("profile", "Profile", PROFILE_ICON).badge(3),
/// ];
///
/// view! {
///     <MobileTabBar
///         items=tabs
///         active=active_tab
///         on_change=Callback::new(move |key: String| active_tab.set(key))
///     />
/// }
/// ```
#[component]
pub fn MobileTabBar(
    /// Tab items to display.
    items: Vec<TabItem>,
    /// Currently active tab key.
    #[prop(into)]
    active: Signal<String>,
    /// Callback when a tab is selected.
    on_change: Callback<String>,
    /// Whether to hide labels.
    #[prop(optional)]
    hide_labels: Option<bool>,
    /// Custom height in pixels.
    #[prop(optional)]
    height: Option<u32>,
) -> impl IntoView {
    let hide_labels = hide_labels.unwrap_or(false);
    let height = height.unwrap_or(56);

    view! {
        <nav
            class="fx-mobile-tab-bar"
            style=format!(
                "position: fixed; bottom: 0; left: 0; right: 0; height: {}px; display: flex; align-items: center; justify-content: space-around; background: var(--fx-bg-container, #fff); border-top: 1px solid var(--fx-border-color, #e8e8e8); padding-bottom: env(safe-area-inset-bottom, 0);",
                height
            )
        >
            {items.into_iter().map(|item| {
                let key_for_style = item.key.clone();
                let key_for_inner = item.key.clone();
                let key_for_click = item.key.clone();
                let label = item.label.clone();
                let icon = item.icon.clone();
                let active_icon = item.active_icon.clone();
                let badge = item.badge;

                view! {
                    <button
                        type="button"
                        class="fx-mobile-tab-item"
                        style=move || {
                            let is_active = active.get() == key_for_style;
                            format!(
                                "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 2px; border: none; background: transparent; cursor: pointer; padding: 8px 4px; color: {}; transition: color 0.2s;",
                                if is_active { "var(--fx-color-primary, #1890ff)" } else { "var(--fx-text-secondary, #8c8c8c)" }
                            )
                        }
                        on:click=move |_| {
                            on_change.run(key_for_click.clone());
                        }
                    >
                        <div style="position: relative; width: 24px; height: 24px;">
                            <span
                                style="display: flex; align-items: center; justify-content: center; width: 100%; height: 100%;"
                                inner_html=move || {
                                    let is_active = active.get() == key_for_inner;
                                    if is_active {
                                        active_icon.clone().unwrap_or_else(|| icon.clone())
                                    } else {
                                        icon.clone()
                                    }
                                }
                            />
                            {badge.map(|count| {
                                if count > 0 {
                                    view! {
                                        <span
                                            class="fx-mobile-tab-badge"
                                            style="position: absolute; top: -4px; right: -8px; min-width: 16px; height: 16px; padding: 0 4px; background: var(--fx-color-error, #ff4d4f); color: #fff; font-size: 10px; font-weight: 500; border-radius: 8px; display: flex; align-items: center; justify-content: center;"
                                        >
                                            {if count > 99 { "99+".to_string() } else { count.to_string() }}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <span /> }.into_any()
                                }
                            })}
                        </div>
                        {if !hide_labels {
                            view! {
                                <span style="font-size: 10px; line-height: 1.2;">
                                    {label}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }}
                    </button>
                }
            }).collect_view()}
        </nav>
    }
}
