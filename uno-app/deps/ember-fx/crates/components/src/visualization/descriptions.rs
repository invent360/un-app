//! Descriptions Leptos component.

use leptos::prelude::*;
use super::types::{DescriptionsLayout, DescriptionsSize, DescriptionItem};
use crate::try_use_theme;

/// Descriptions component.
///
/// Display multiple read-only fields in groups.
///
/// # Props
///
/// - `title` - Descriptions title
/// - `items` - Description items
/// - `layout` - Layout direction
/// - `bordered` - Show border
/// - `column` - Number of columns
/// - `size` - Size
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::visualization::{Descriptions, DescriptionItem};
///
/// let items = vec![
///     DescriptionItem::new("Product", "Cloud Database"),
///     DescriptionItem::new("Billing Mode", "Prepaid"),
///     DescriptionItem::new("Automatic Renewal", "YES"),
/// ];
///
/// view! {
///     <Descriptions title="User Info" items=items />
/// }
/// ```
#[component]
pub fn Descriptions(
    /// Descriptions title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Description items.
    #[prop(into)]
    items: Vec<DescriptionItem>,
    /// Layout direction.
    #[prop(optional, into)]
    layout: Option<DescriptionsLayout>,
    /// Show border.
    #[prop(optional)]
    bordered: bool,
    /// Number of columns.
    #[prop(optional)]
    column: Option<usize>,
    /// Size.
    #[prop(optional, into)]
    size: Option<DescriptionsSize>,
    /// Extra content in header.
    #[prop(optional, into)]
    extra: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let layout = layout.unwrap_or_default();
    let size = size.unwrap_or_default();
    let column = column.unwrap_or(3);

    // Build CSS classes
    let desc_prefix = format!("fx-descriptions-{}", design_system);
    let layout_class = layout.class(&desc_prefix);
    let size_class = size.class(&desc_prefix);

    let combined_class = {
        let mut parts = vec![desc_prefix.clone(), layout_class, size_class];
        if bordered {
            parts.push(format!("{}-bordered", desc_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Group items into rows based on column count and spans
    let rows = {
        let mut rows: Vec<Vec<DescriptionItem>> = Vec::new();
        let mut current_row: Vec<DescriptionItem> = Vec::new();
        let mut current_span = 0;

        for item in items {
            let item_span = item.span.min(column);
            if current_span + item_span > column && !current_row.is_empty() {
                rows.push(current_row);
                current_row = Vec::new();
                current_span = 0;
            }
            current_span += item_span;
            current_row.push(item);
        }

        if !current_row.is_empty() {
            rows.push(current_row);
        }

        rows
    };

    view! {
        <div class=combined_class>
            {title.clone().map(|t| view! {
                <div class=format!("{}-header", desc_prefix)>
                    <div class=format!("{}-title", desc_prefix)>{t}</div>
                    {extra.clone().map(|e| view! {
                        <div class=format!("{}-extra", desc_prefix)>{e}</div>
                    })}
                </div>
            })}
            <div class=format!("{}-view", desc_prefix)>
                {if bordered {
                    view! {
                        <table class=format!("{}-table", desc_prefix)>
                            <tbody>
                                {rows.clone().into_iter().map(|row| {
                                    let desc_prefix = desc_prefix.clone();
                                    view! {
                                        <tr class=format!("{}-row", desc_prefix)>
                                            {row.into_iter().map(|item| {
                                                let desc_prefix = desc_prefix.clone();
                                                view! {
                                                    <th class=format!("{}-item-label", desc_prefix)>
                                                        {item.label.clone()}
                                                    </th>
                                                    <td
                                                        class=format!("{}-item-content", desc_prefix)
                                                        colspan=item.span
                                                    >
                                                        {item.content.clone()}
                                                    </td>
                                                }
                                            }).collect_view()}
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    }.into_any()
                } else {
                    view! {
                        <div
                            class=format!("{}-grid", desc_prefix)
                            style=format!("grid-template-columns: repeat({}, 1fr);", column)
                        >
                            {rows.clone().into_iter().flat_map(|row| {
                                row.into_iter().map(|item| {
                                    let desc_prefix = desc_prefix.clone();
                                    view! {
                                        <div
                                            class=format!("{}-item", desc_prefix)
                                            style=format!("grid-column: span {};", item.span)
                                        >
                                            <span class=format!("{}-item-label", desc_prefix)>
                                                {item.label.clone()}
                                            </span>
                                            <span class=format!("{}-item-content", desc_prefix)>
                                                {item.content.clone()}
                                            </span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

/// Descriptions Item Component for composition pattern.
#[component]
pub fn DescriptionsItem(
    /// Item label.
    #[prop(into)]
    label: String,
    /// Span columns.
    #[prop(optional)]
    span: Option<usize>,
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

    let desc_prefix = format!("fx-descriptions-{}", design_system);
    let span = span.unwrap_or(1);

    let combined_class = {
        let mut parts = vec![format!("{}-item", desc_prefix)];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class style=format!("grid-column: span {};", span)>
            <span class=format!("{}-item-label", desc_prefix)>{label}</span>
            <span class=format!("{}-item-content", desc_prefix)>{children()}</span>
        </div>
    }
}
