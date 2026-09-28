//! SplitButton Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{ButtonVariant, ButtonSize};
use crate::try_use_theme;

/// Menu item for SplitButton dropdown.
#[derive(Debug, Clone, PartialEq)]
pub struct SplitButtonItem {
    /// Unique key for this item.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Optional icon.
    pub icon: Option<String>,
    /// Whether this item is disabled.
    pub disabled: bool,
    /// Whether this is a divider.
    pub divider: bool,
}

impl SplitButtonItem {
    /// Create a new menu item.
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            divider: false,
        }
    }

    /// Add an icon to this item.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Mark this item as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Create a divider item.
    pub fn divider() -> Self {
        Self {
            key: "divider".to_string(),
            label: String::new(),
            icon: None,
            disabled: false,
            divider: true,
        }
    }
}

/// SplitButton component.
///
/// A button split into two parts: a primary action and a dropdown menu
/// with additional actions.
///
/// # Props
///
/// - `variant` - Visual variant (Primary, Secondary, etc.)
/// - `size` - Size variant (Xs, Sm, Md, Lg, Xl)
/// - `menu_items` - List of dropdown menu items
/// - `disabled` - Whether the button is disabled
/// - `loading` - Whether the button is in loading state
/// - `class` - Additional CSS classes
/// - `on_click` - Primary button click handler
/// - `on_menu_click` - Menu item click handler (receives item key)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{SplitButton, SplitButtonItem, ButtonVariant};
///
/// let items = vec![
///     SplitButtonItem::new("save-draft", "Save as Draft"),
///     SplitButtonItem::new("save-template", "Save as Template"),
///     SplitButtonItem::divider(),
///     SplitButtonItem::new("export", "Export").with_icon("📤"),
/// ];
///
/// view! {
///     <SplitButton
///         variant=ButtonVariant::Primary
///         menu_items=items
///         on_click=move |_| log::info!("Primary action!")
///         on_menu_click=move |key| log::info!("Menu item: {}", key)
///     >
///         "Save"
///     </SplitButton>
/// }
/// ```
#[component]
pub fn SplitButton(
    /// Button variant (visual style).
    #[prop(optional, into)]
    variant: Option<ButtonVariant>,
    /// Button size.
    #[prop(optional, into)]
    size: Option<ButtonSize>,
    /// Dropdown menu items.
    #[prop(into)]
    menu_items: Vec<SplitButtonItem>,
    /// Whether the button is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the button is in loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Primary button click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Menu item click handler.
    #[prop(optional, into)]
    on_menu_click: Option<Callback<String>>,
    /// Primary button content.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Resolve defaults
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();

    // Dropdown state
    let is_open = RwSignal::new(false);

    // Build CSS classes
    let split_prefix = format!("fx-split-btn-{}", design_system);
    let btn_prefix = format!("fx-btn-{}", design_system);

    // Pre-clone for use after closures
    let menu_class = format!("{}-menu", split_prefix);
    let menu_item_class = format!("{}-menu-item", split_prefix);
    let menu_divider_class = format!("{}-menu-divider", split_prefix);
    let arrow_class = format!("{}-arrow", split_prefix);

    let combined_class = {
        let mut parts = vec![split_prefix.clone()];

        if disabled || loading {
            parts.push(format!("{}-disabled", split_prefix));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    let primary_class = {
        let mut parts = vec![
            format!("{}-primary", split_prefix),
            btn_prefix.clone(),
            variant.class(&btn_prefix),
            size.class(&btn_prefix),
        ];

        if disabled || loading {
            parts.push(format!("{}-disabled", btn_prefix));
        }
        if loading {
            parts.push(format!("{}-loading", btn_prefix));
        }

        parts.join(" ")
    };

    let dropdown_btn_class = {
        let mut parts = vec![
            format!("{}-dropdown-btn", split_prefix),
            btn_prefix.clone(),
            variant.class(&btn_prefix),
            size.class(&btn_prefix),
        ];

        if disabled || loading {
            parts.push(format!("{}-disabled", btn_prefix));
        }

        parts.join(" ")
    };

    let dropdown_class = move || {
        let mut parts = vec![format!("{}-dropdown", split_prefix)];
        if is_open.get() {
            parts.push(format!("{}-dropdown-open", split_prefix));
        }
        parts.join(" ")
    };

    // Click handlers
    let handle_primary_click = move |ev: ev::MouseEvent| {
        if disabled || loading {
            ev.prevent_default();
            return;
        }
        if let Some(ref cb) = on_click {
            cb.run(ev);
        }
    };

    let handle_dropdown_toggle = move |ev: ev::MouseEvent| {
        if disabled || loading {
            ev.prevent_default();
            return;
        }
        ev.stop_propagation();
        is_open.update(|v| *v = !*v);
    };

    let on_menu_click_clone = on_menu_click.clone();
    let handle_menu_item_click = move |key: String| {
        is_open.set(false);
        if let Some(ref cb) = on_menu_click_clone {
            cb.run(key);
        }
    };

    // Close on outside click
    Effect::new(move |_| {
        if is_open.get() {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::prelude::*;
                use web_sys::{window, Event};

                if let Some(win) = window() {
                    if let Some(doc) = win.document() {
                        let is_open = is_open;
                        let closure = Closure::wrap(Box::new(move |_: Event| {
                            is_open.set(false);
                        }) as Box<dyn Fn(Event)>);

                        let _ = doc.add_event_listener_with_callback(
                            "click",
                            closure.as_ref().unchecked_ref(),
                        );

                        // Clean up after one use
                        closure.forget();
                    }
                }
            }
        }
    });

    // Spinner class
    let spinner_class = format!("{}-spinner", btn_prefix);

    view! {
        <div class=combined_class>
            // Primary button
            <button
                type="button"
                class=primary_class
                disabled=disabled || loading
                aria-disabled=move || if disabled || loading { Some("true") } else { None }
                aria-busy=move || if loading { Some("true") } else { None }
                on:click=handle_primary_click
            >
                {move || {
                    if loading {
                        view! { <span class=spinner_class.clone()></span> }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
                <span class=format!("{}-content", btn_prefix)>
                    {children()}
                </span>
            </button>

            // Dropdown toggle button
            <button
                type="button"
                class=dropdown_btn_class
                disabled=disabled || loading
                aria-expanded=move || is_open.get().to_string()
                aria-haspopup="menu"
                on:click=handle_dropdown_toggle
            >
                <span class=arrow_class>"▼"</span>
            </button>

            // Dropdown menu
            <div class=dropdown_class role="menu">
                <ul class=menu_class>
                    {menu_items.into_iter().map(|item| {
                        let key = item.key.clone();
                        let handler = handle_menu_item_click.clone();

                        if item.divider {
                            view! {
                                <li class=menu_divider_class.clone() role="separator"></li>
                            }.into_any()
                        } else {
                            let item_class = if item.disabled {
                                format!("{} {}-disabled", menu_item_class.clone(), menu_item_class.clone())
                            } else {
                                menu_item_class.clone()
                            };

                            view! {
                                <li
                                    class=item_class
                                    role="menuitem"
                                    aria-disabled=item.disabled.to_string()
                                    on:click=move |_| {
                                        if !item.disabled {
                                            handler(key.clone());
                                        }
                                    }
                                >
                                    {item.icon.map(|icon| view! {
                                        <span class=format!("{}-icon", menu_item_class.clone())>{icon}</span>
                                    })}
                                    <span class=format!("{}-label", menu_item_class.clone())>{item.label.clone()}</span>
                                </li>
                            }.into_any()
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_button_item() {
        let item = SplitButtonItem::new("save", "Save")
            .with_icon("💾");
        assert_eq!(item.key, "save");
        assert_eq!(item.label, "Save");
        assert_eq!(item.icon, Some("💾".to_string()));
        assert!(!item.disabled);
    }

    #[test]
    fn test_split_button_divider() {
        let divider = SplitButtonItem::divider();
        assert!(divider.divider);
    }
}
