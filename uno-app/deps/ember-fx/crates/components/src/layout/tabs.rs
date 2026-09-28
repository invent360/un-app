//! Tabs Leptos component.

use leptos::prelude::*;
use super::types::{TabPosition, TabType, TabItem};
use crate::try_use_theme;

/// Tabs component.
///
/// Tabbed content navigation.
///
/// # Props
///
/// - `active_key` - Currently active tab key
/// - `items` - Tab items configuration
/// - `tab_type` - Tab style (Line, Card, EditableCard)
/// - `position` - Tab position (Top, Right, Bottom, Left)
/// - `centered` - Whether to center the tabs
/// - `on_change` - Tab change callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::layout::{Tabs, TabItem};
///
/// let active = RwSignal::new("tab1".to_string());
/// let items = vec![
///     TabItem::new("tab1", "Tab 1"),
///     TabItem::new("tab2", "Tab 2"),
///     TabItem::new("tab3", "Tab 3"),
/// ];
///
/// view! {
///     <Tabs active_key=active items=items>
///         // Tab content rendered based on active_key
///     </Tabs>
/// }
/// ```
#[component]
pub fn Tabs(
    /// Currently active tab key.
    #[prop(into)]
    active_key: RwSignal<String>,
    /// Tab items.
    #[prop(into)]
    items: Vec<TabItem>,
    /// Tab style type.
    #[prop(optional, into)]
    tab_type: Option<TabType>,
    /// Tab position.
    #[prop(optional, into)]
    position: Option<TabPosition>,
    /// Whether to center tabs.
    #[prop(optional)]
    centered: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Tab change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Tab edit callback (for editable-card type).
    #[prop(optional, into)]
    _on_edit: Option<Callback<(String, String)>>,
    /// Tab pane content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let tab_type = tab_type.unwrap_or_default();
    let position = position.unwrap_or_default();

    // Build CSS classes
    let tabs_prefix = format!("fx-tabs-{}", design_system);
    let type_class = tab_type.class(&tabs_prefix);
    let position_class = position.class(&tabs_prefix);

    // Class names
    let nav_class = format!("{}-nav", tabs_prefix);
    let nav_list_class = format!("{}-nav-list", tabs_prefix);
    let tab_class = format!("{}-tab", tabs_prefix);
    let tab_btn_class = format!("{}-tab-btn", tabs_prefix);
    let content_class = format!("{}-content", tabs_prefix);
    let ink_bar_class = format!("{}-ink-bar", tabs_prefix);

    // Clone for closure
    let tabs_prefix_for_class = tabs_prefix.clone();

    let combined_class = {
        let mut parts = vec![tabs_prefix_for_class.clone(), type_class.clone(), position_class.clone()];
        if centered {
            parts.push(format!("{}-centered", tabs_prefix_for_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Render children
    let children_view = children();

    view! {
        <div class=combined_class>
            // Tab navigation
            <div class=nav_class.clone()>
                <div class=nav_list_class.clone()>
                    {items.clone().into_iter().map(|item| {
                        let key = item.key.clone();
                        let key_for_click = key.clone();
                        let key_for_active = key.clone();
                        let on_change = on_change.clone();
                        let tab_class = tab_class.clone();
                        let tab_btn_class = tab_btn_class.clone();
                        let tabs_prefix = tabs_prefix.clone();

                        view! {
                            <div
                                class=move || {
                                    let mut cls = vec![tab_class.clone()];
                                    if active_key.get() == key_for_active {
                                        cls.push(format!("{}-tab-active", tabs_prefix));
                                    }
                                    if item.disabled {
                                        cls.push(format!("{}-tab-disabled", tabs_prefix));
                                    }
                                    cls.join(" ")
                                }
                            >
                                <button
                                    class=tab_btn_class.clone()
                                    disabled=item.disabled
                                    on:click=move |_| {
                                        if !item.disabled {
                                            active_key.set(key_for_click.clone());
                                            if let Some(ref cb) = on_change {
                                                cb.run(key_for_click.clone());
                                            }
                                        }
                                    }
                                >
                                    {item.icon.clone().map(|i| view! {
                                        <span class=format!("{}-tab-icon", tabs_prefix)>{i}</span>
                                    })}
                                    {item.label.clone()}
                                </button>
                            </div>
                        }
                    }).collect_view()}
                    <div class=ink_bar_class.clone()></div>
                </div>
            </div>

            // Tab content
            <div class=content_class.clone()>
                {children_view}
            </div>
        </div>
    }
}

/// Tab pane component for use with Tabs.
///
/// Represents the content of a single tab.
#[component]
pub fn TabPane(
    /// Tab key (must match one of the tab items).
    #[prop(into)]
    tab_key: String,
    /// Currently active key (from parent Tabs).
    #[prop(into)]
    active_key: Signal<String>,
    /// Tab pane content.
    children: Children,
) -> impl IntoView {
    // Render children
    let children_view = children();

    view! {
        <div
            class="fx-tabpane"
            style=move || {
                if active_key.get() == tab_key {
                    None
                } else {
                    Some("display: none;")
                }
            }
        >
            {children_view}
        </div>
    }
}
