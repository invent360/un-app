//! Pagination Leptos component.

use leptos::prelude::*;
use super::types::PaginationSize;
use crate::try_use_theme;

/// Pagination component.
///
/// Page navigation for tables and lists.
///
/// # Props
///
/// - `current` - Current page number (1-indexed)
/// - `total` - Total number of items
/// - `page_size` - Items per page
/// - `size` - Pagination size (Small, Default)
/// - `show_total` - Show total item count
/// - `show_quick_jumper` - Show quick jump input
/// - `show_size_changer` - Show page size selector
/// - `disabled` - Whether pagination is disabled
/// - `on_change` - Page change callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::navigation::Pagination;
///
/// let current = RwSignal::new(1);
/// let total = 100;
///
/// view! {
///     <Pagination
///         current=current
///         total=total
///         on_change=move |page| current.set(page)
///     />
/// }
/// ```
#[component]
pub fn Pagination(
    /// Current page (1-indexed).
    #[prop(into)]
    current: RwSignal<usize>,
    /// Total number of items.
    #[prop(into)]
    total: usize,
    /// Items per page.
    #[prop(optional)]
    page_size: Option<usize>,
    /// Pagination size.
    #[prop(optional, into)]
    size: Option<PaginationSize>,
    /// Show total count.
    #[prop(optional)]
    show_total: bool,
    /// Show quick jumper.
    #[prop(optional)]
    show_quick_jumper: bool,
    /// Whether disabled.
    #[prop(optional)]
    disabled: bool,
    /// Simple mode (only prev/next).
    #[prop(optional)]
    simple: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Page change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<usize>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let page_size = page_size.unwrap_or(10);
    let size = size.unwrap_or_default();

    // Calculate total pages
    let total_pages = (total + page_size - 1) / page_size;

    // Build CSS classes
    let pagination_prefix = format!("fx-pagination-{}", design_system);
    let size_class = if size == PaginationSize::Small {
        Some(size.class(&pagination_prefix))
    } else {
        None
    };

    let combined_class = {
        let mut parts = vec![pagination_prefix.clone()];
        if let Some(ref sc) = size_class {
            parts.push(sc.clone());
        }
        if disabled {
            parts.push(format!("{}-disabled", pagination_prefix));
        }
        if simple {
            parts.push(format!("{}-simple", pagination_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Class names
    let item_class = format!("{}-item", pagination_prefix);
    let prev_class = format!("{}-prev", pagination_prefix);
    let next_class = format!("{}-next", pagination_prefix);
    let jump_prev_class = format!("{}-jump-prev", pagination_prefix);
    let _jump_next_class = format!("{}-jump-next", pagination_prefix);
    let total_class = format!("{}-total-text", pagination_prefix);

    // Clone for closures
    let item_class_for_prev = item_class.clone();
    let item_class_for_pages = item_class.clone();
    let item_class_for_next = item_class.clone();
    let pagination_prefix_for_prev = pagination_prefix.clone();
    let pagination_prefix_for_pages = pagination_prefix.clone();
    let pagination_prefix_for_next = pagination_prefix.clone();
    let pagination_prefix_for_jumper = pagination_prefix.clone();

    // Generate page numbers to display
    let generate_pages = move || {
        let curr = current.get();
        let mut pages = Vec::new();

        if total_pages <= 7 {
            // Show all pages
            for i in 1..=total_pages {
                pages.push(Some(i));
            }
        } else {
            // Always show first page
            pages.push(Some(1));

            if curr > 4 {
                pages.push(None); // Ellipsis
            }

            // Show pages around current
            let start = if curr <= 4 { 2 } else { curr - 2 };
            let end = if curr >= total_pages - 3 { total_pages - 1 } else { curr + 2 };

            for i in start..=end {
                if i > 1 && i < total_pages {
                    pages.push(Some(i));
                }
            }

            if curr < total_pages - 3 {
                pages.push(None); // Ellipsis
            }

            // Always show last page
            if total_pages > 1 {
                pages.push(Some(total_pages));
            }
        }

        pages
    };

    // Handle page change
    let handle_change = move |page: usize| {
        if !disabled && page >= 1 && page <= total_pages {
            current.set(page);
            if let Some(ref cb) = on_change {
                cb.run(page);
            }
        }
    };

    view! {
        <ul class=combined_class role="navigation" aria-label="Pagination">
            // Total count
            {if show_total {
                Some(view! {
                    <li class=total_class.clone()>
                        {format!("Total {} items", total)}
                    </li>
                })
            } else {
                None
            }}

            // Previous button
            <li
                class=move || {
                    let mut cls = vec![item_class_for_prev.clone(), prev_class.clone()];
                    if current.get() <= 1 || disabled {
                        cls.push(format!("{}-item-disabled", pagination_prefix_for_prev));
                    }
                    cls.join(" ")
                }
                on:click=move |_| {
                    if current.get() > 1 {
                        handle_change(current.get() - 1);
                    }
                }
                aria-label="Previous"
            >
                "‹"
            </li>

            // Page numbers
            {move || {
                if simple {
                    // Simple mode: just show current/total
                    view! {
                        <li class=format!("{}-simple-pager", pagination_prefix_for_pages)>
                            {move || format!("{} / {}", current.get(), total_pages)}
                        </li>
                    }.into_any()
                } else {
                    // Full mode: show page buttons
                    generate_pages().into_iter().map(|page_opt| {
                        let item_class = item_class_for_pages.clone();
                        let pagination_prefix = pagination_prefix_for_pages.clone();
                        let jump_prev_class = jump_prev_class.clone();

                        if let Some(page) = page_opt {
                            view! {
                                <li
                                    class=move || {
                                        let mut cls = vec![item_class.clone()];
                                        if current.get() == page {
                                            cls.push(format!("{}-item-active", pagination_prefix));
                                        }
                                        cls.join(" ")
                                    }
                                    on:click=move |_| handle_change(page)
                                >
                                    {page}
                                </li>
                            }.into_any()
                        } else {
                            // Ellipsis
                            view! {
                                <li class=format!("{} {}", item_class, jump_prev_class)>
                                    "•••"
                                </li>
                            }.into_any()
                        }
                    }).collect_view().into_any()
                }
            }}

            // Next button
            <li
                class=move || {
                    let mut cls = vec![item_class_for_next.clone(), next_class.clone()];
                    if current.get() >= total_pages || disabled {
                        cls.push(format!("{}-item-disabled", pagination_prefix_for_next));
                    }
                    cls.join(" ")
                }
                on:click=move |_| {
                    if current.get() < total_pages {
                        handle_change(current.get() + 1);
                    }
                }
                aria-label="Next"
            >
                "›"
            </li>

            // Quick jumper
            {if show_quick_jumper && !simple {
                let jumper_class = format!("{}-options-quick-jumper", pagination_prefix_for_jumper);
                Some(view! {
                    <li class=jumper_class>
                        <span>"Go to "</span>
                        <input
                            type="number"
                            min="1"
                            max=total_pages
                            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    if let Ok(page) = event_target_value(&ev).parse::<usize>() {
                                        handle_change(page);
                                    }
                                }
                            }
                        />
                    </li>
                })
            } else {
                None
            }}
        </ul>
    }
}
