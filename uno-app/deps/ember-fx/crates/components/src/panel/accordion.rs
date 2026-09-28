//! Accordion component for collapsible content panels.
//!
//! A component for organizing content into expandable/collapsible sections,
//! commonly used for FAQs, settings panels, and hierarchical information.
//!
//! ## Features
//!
//! - Single or multiple items can be open simultaneously
//! - Size variants (small, default, large)
//! - Bordered variant for emphasis
//! - Smooth expand/collapse transitions
//! - Accessible with ARIA attributes
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx::components::{Accordion, AccordionItem};
//!
//! let items = vec![
//!     AccordionItem::new("1", "Question 1", "Answer to question 1"),
//!     AccordionItem::new("2", "Question 2", "Answer to question 2"),
//! ];
//!
//! view! {
//!     <Accordion items=items />
//! }
//! ```

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{AccordionItem, AccordionSize};

/// Accordion component for collapsible content panels.
///
/// # Props
///
/// - `items` - List of accordion items to display
/// - `allow_multiple` - Allow multiple items to be open simultaneously (default: false)
/// - `size` - Size variant (Small, Default, Large)
/// - `bordered` - Show emphasized borders
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// <Accordion
///     items=vec![
///         AccordionItem::new("1", "FAQ 1", "Answer 1"),
///         AccordionItem::new("2", "FAQ 2", "Answer 2"),
///     ]
///     allow_multiple=true
///     size=AccordionSize::Large
/// />
/// ```
#[component]
pub fn Accordion(
    /// List of accordion items.
    items: Vec<AccordionItem>,
    /// Allow multiple items to be open simultaneously.
    #[prop(optional)]
    allow_multiple: bool,
    /// Size variant.
    #[prop(optional)]
    size: AccordionSize,
    /// Show emphasized borders.
    #[prop(optional)]
    bordered: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context for design system prefix
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-accordion-{}", design_system);

    // State: track which items are open
    let (open_items, set_open_items) = signal(Vec::<String>::new());

    // Clone prefix for use in closures
    let prefix_for_container = prefix.clone();
    let custom_class = class.clone();

    // Build container class
    let container_class = move || {
        let mut classes = vec![
            prefix_for_container.clone(),
            size.class(&prefix_for_container),
        ];
        if bordered {
            classes.push(format!("{}-bordered", prefix_for_container));
        }
        if let Some(ref c) = custom_class {
            classes.push(c.clone());
        }
        classes.join(" ")
    };

    view! {
        <div class=container_class>
            {items.into_iter().map(|item| {
                let id = item.id.clone();
                let prefix_for_item = prefix.clone();
                let prefix_for_header = prefix.clone();
                let prefix_for_icon = prefix.clone();
                let prefix_for_title = prefix.clone();
                let prefix_for_content = prefix.clone();

                // Clone id for each closure that needs it
                let id_for_class = id.clone();
                let id_for_aria = id.clone();
                let id_for_icon = id.clone();
                let id_for_style = id.clone();
                let id_for_toggle = id.clone();

                // Toggle handler
                let toggle = move |_| {
                    let id = id_for_toggle.clone();
                    set_open_items.update(|items| {
                        if items.contains(&id) {
                            items.retain(|i| i != &id);
                        } else {
                            if !allow_multiple {
                                items.clear();
                            }
                            items.push(id);
                        }
                    });
                };

                view! {
                    <div class=move || {
                        let is_open = open_items.get().contains(&id_for_class);
                        let mut cls = format!("{}-item", prefix_for_item);
                        if is_open {
                            cls.push_str(&format!(" {}-item-open", prefix_for_item));
                        }
                        cls
                    }>
                        <button
                            class=format!("{}-header", prefix_for_header)
                            on:click=toggle
                            aria-expanded=move || open_items.get().contains(&id_for_aria).to_string()
                            type="button"
                        >
                            <span class=format!("{}-title", prefix_for_title)>
                                {item.title}
                            </span>
                            <span class=format!("{}-icon", prefix_for_icon)>
                                {move || if open_items.get().contains(&id_for_icon) { "−" } else { "+" }}
                            </span>
                        </button>
                        <div
                            class=format!("{}-content", prefix_for_content)
                            style=move || if open_items.get().contains(&id_for_style) { "display: block;" } else { "display: none;" }
                        >
                            {item.content}
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
