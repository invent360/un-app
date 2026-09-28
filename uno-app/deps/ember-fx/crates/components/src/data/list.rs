//! List Leptos component.

use leptos::prelude::*;
use super::types::{ListSize, ListLayout};
use crate::try_use_theme;

/// List component.
///
/// Display a structured list of items.
///
/// # Props
///
/// - `size` - List size
/// - `bordered` - Show border
/// - `split` - Show split line between items
/// - `header` - List header content
/// - `footer` - List footer content
/// - `children` - List items
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::{List, ListItem};
///
/// view! {
///     <List header="Users">
///         <ListItem>"User 1"</ListItem>
///         <ListItem>"User 2"</ListItem>
///     </List>
/// }
/// ```
#[component]
pub fn List(
    /// List size.
    #[prop(optional, into)]
    size: Option<ListSize>,
    /// List layout.
    #[prop(optional, into)]
    layout: Option<ListLayout>,
    /// Show border.
    #[prop(optional)]
    bordered: bool,
    /// Show split line between items.
    #[prop(optional)]
    split: Option<bool>,
    /// List header.
    #[prop(optional, into)]
    header: Option<String>,
    /// List footer.
    #[prop(optional, into)]
    footer: Option<String>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// List items.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let layout = layout.unwrap_or_default();
    let split = split.unwrap_or(true);

    // Build CSS classes
    let list_prefix = format!("fx-list-{}", design_system);
    let size_class = size.class(&list_prefix);
    let layout_class = layout.class(&list_prefix);

    let combined_class = {
        let mut parts = vec![list_prefix.clone(), size_class, layout_class];
        if bordered {
            parts.push(format!("{}-bordered", list_prefix));
        }
        if split {
            parts.push(format!("{}-split", list_prefix));
        }
        if loading {
            parts.push(format!("{}-loading", list_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            {header.clone().map(|h| view! {
                <div class=format!("{}-header", list_prefix)>{h}</div>
            })}
            <ul class=format!("{}-items", list_prefix)>
                {children()}
            </ul>
            {footer.clone().map(|f| view! {
                <div class=format!("{}-footer", list_prefix)>{f}</div>
            })}
        </div>
    }
}

/// List Item component.
///
/// Individual item in a list.
///
/// # Props
///
/// - `actions` - Action buttons
/// - `extra` - Extra content on the right
/// - `children` - Item content
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::ListItem;
///
/// view! {
///     <ListItem actions=vec!["Edit", "Delete"]>
///         "Item content"
///     </ListItem>
/// }
/// ```
#[component]
pub fn ListItem(
    /// Action labels.
    #[prop(optional, into)]
    actions: Option<Vec<String>>,
    /// Extra content.
    #[prop(optional, into)]
    extra: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Item content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let list_prefix = format!("fx-list-{}", design_system);
    let item_class = format!("{}-item", list_prefix);

    let combined_class = {
        let mut parts = vec![item_class.clone()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <li class=combined_class>
            <div class=format!("{}-item-main", list_prefix)>
                <div class=format!("{}-item-content", list_prefix)>
                    {children()}
                </div>
                {extra.clone().map(|e| view! {
                    <div class=format!("{}-item-extra", list_prefix)>{e}</div>
                })}
            </div>
            {actions.clone().map(|acts| view! {
                <ul class=format!("{}-item-actions", list_prefix)>
                    {acts.into_iter().map(|action| view! {
                        <li class=format!("{}-item-action", list_prefix)>
                            {action}
                        </li>
                    }).collect_view()}
                </ul>
            })}
        </li>
    }
}

/// List Item Meta component.
///
/// Structured metadata for list items with avatar, title, and description.
///
/// # Props
///
/// - `avatar` - Avatar element (icon or image)
/// - `title` - Item title
/// - `description` - Item description
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::{ListItem, ListItemMeta};
///
/// view! {
///     <ListItem>
///         <ListItemMeta
///             avatar="👤"
///             title="John Doe"
///             description="Software Engineer"
///         />
///     </ListItem>
/// }
/// ```
#[component]
pub fn ListItemMeta(
    /// Avatar content.
    #[prop(optional, into)]
    avatar: Option<String>,
    /// Item title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Item description.
    #[prop(optional, into)]
    description: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let list_prefix = format!("fx-list-{}", design_system);
    let meta_class = format!("{}-item-meta", list_prefix);

    let combined_class = {
        let mut parts = vec![meta_class.clone()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            {avatar.clone().map(|a| view! {
                <div class=format!("{}-item-meta-avatar", list_prefix)>{a}</div>
            })}
            <div class=format!("{}-item-meta-content", list_prefix)>
                {title.clone().map(|t| view! {
                    <h4 class=format!("{}-item-meta-title", list_prefix)>{t}</h4>
                })}
                {description.clone().map(|d| view! {
                    <div class=format!("{}-item-meta-description", list_prefix)>{d}</div>
                })}
            </div>
        </div>
    }
}
