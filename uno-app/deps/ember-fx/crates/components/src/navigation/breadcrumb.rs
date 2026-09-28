//! Breadcrumb Leptos component.

use leptos::prelude::*;
use super::types::BreadcrumbItem;
use crate::try_use_theme;

/// Breadcrumb component.
///
/// Navigation path display.
///
/// # Props
///
/// - `items` - Breadcrumb items
/// - `separator` - Custom separator character
/// - `max_items` - Maximum items to show before collapsing
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::navigation::{Breadcrumb, BreadcrumbItem};
///
/// let items = vec![
///     BreadcrumbItem::new("Home").href("/"),
///     BreadcrumbItem::new("Products").href("/products"),
///     BreadcrumbItem::new("Details"),
/// ];
///
/// view! {
///     <Breadcrumb items=items />
/// }
/// ```
#[component]
pub fn Breadcrumb(
    /// Breadcrumb items.
    #[prop(into)]
    items: Vec<BreadcrumbItem>,
    /// Custom separator.
    #[prop(optional, into)]
    separator: Option<String>,
    /// Maximum items before collapse.
    #[prop(optional)]
    max_items: Option<usize>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Item click callback.
    #[prop(optional, into)]
    on_click: Option<Callback<usize>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let separator = separator.unwrap_or_else(|| "/".to_string());

    // Build CSS classes
    let breadcrumb_prefix = format!("fx-breadcrumb-{}", design_system);

    let combined_class = {
        let mut parts = vec![breadcrumb_prefix.clone()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Class names
    let item_class = format!("{}-item", breadcrumb_prefix);
    let link_class = format!("{}-link", breadcrumb_prefix);
    let separator_class = format!("{}-separator", breadcrumb_prefix);
    let icon_class = format!("{}-icon", breadcrumb_prefix);

    // Handle collapse if max_items is set
    let display_items = if let Some(max) = max_items {
        if items.len() > max && max >= 3 {
            // Show first, ellipsis, and last (max-2) items
            let first = items.first().cloned();
            let last_count = max - 2;
            let last_items: Vec<_> = items.iter().skip(items.len() - last_count).cloned().collect();

            let mut result = Vec::new();
            if let Some(f) = first {
                result.push(f);
            }
            result.push(BreadcrumbItem::new("..."));
            result.extend(last_items);
            result
        } else {
            items.clone()
        }
    } else {
        items.clone()
    };

    let total = display_items.len();

    view! {
        <nav class=combined_class aria-label="Breadcrumb">
            <ol>
                {display_items.into_iter().enumerate().map(|(idx, item)| {
                    let is_last = idx == total - 1;
                    let item_class = item_class.clone();
                    let link_class = link_class.clone();
                    let separator_class = separator_class.clone();
                    let icon_class = icon_class.clone();
                    let separator = separator.clone();
                    let on_click = on_click.clone();

                    view! {
                        <li class=item_class.clone()>
                            {if let Some(ref href) = item.href {
                                view! {
                                    <a
                                        class=link_class.clone()
                                        href=href.clone()
                                        on:click=move |e| {
                                            if let Some(ref cb) = on_click {
                                                e.prevent_default();
                                                cb.run(idx);
                                            }
                                        }
                                    >
                                        {item.icon.clone().map(|i| view! {
                                            <span class=icon_class.clone()>{i}</span>
                                        })}
                                        {item.label.clone()}
                                    </a>
                                }.into_any()
                            } else {
                                view! {
                                    <span>
                                        {item.icon.clone().map(|i| view! {
                                            <span class=icon_class.clone()>{i}</span>
                                        })}
                                        {item.label.clone()}
                                    </span>
                                }.into_any()
                            }}
                            {if !is_last {
                                Some(view! {
                                    <span class=separator_class.clone()>{separator.clone()}</span>
                                })
                            } else {
                                None
                            }}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}
