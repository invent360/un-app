//! Table Leptos component.

use leptos::prelude::*;
use super::types::{TableSize, TableColumn, SortOrder};
use crate::try_use_theme;
use std::collections::HashMap;

/// Table component.
///
/// Data table with sorting, filtering, and pagination support.
///
/// # Props
///
/// - `columns` - Column definitions
/// - `data` - Table data (Vec of HashMaps)
/// - `size` - Table size
/// - `bordered` - Show borders
/// - `striped` - Striped rows
/// - `hoverable` - Hoverable rows
/// - `loading` - Loading state
/// - `empty_text` - Text when no data
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::{Table, TableColumn};
/// use std::collections::HashMap;
///
/// let columns = vec![
///     TableColumn::new("name", "Name"),
///     TableColumn::new("age", "Age").sortable(true),
/// ];
///
/// let data = vec![
///     {
///         let mut row = HashMap::new();
///         row.insert("name".to_string(), "John".to_string());
///         row.insert("age".to_string(), "25".to_string());
///         row
///     }
/// ];
///
/// view! {
///     <Table columns=columns data=data />
/// }
/// ```
#[component]
pub fn Table(
    /// Column definitions.
    #[prop(into)]
    columns: Vec<TableColumn>,
    /// Table data.
    #[prop(into)]
    data: Signal<Vec<HashMap<String, String>>>,
    /// Table size.
    #[prop(optional, into)]
    size: Option<TableSize>,
    /// Show borders.
    #[prop(optional)]
    bordered: bool,
    /// Striped rows.
    #[prop(optional)]
    striped: bool,
    /// Hoverable rows.
    #[prop(optional)]
    hoverable: Option<bool>,
    /// Loading state.
    #[prop(optional, into)]
    loading: Option<Signal<bool>>,
    /// Text when no data.
    #[prop(optional, into)]
    empty_text: Option<String>,
    /// Row selection enabled.
    #[prop(optional)]
    row_selection: bool,
    /// Selected row keys.
    #[prop(optional, into)]
    _selected_row_keys: Option<RwSignal<Vec<String>>>,
    /// Row key field.
    #[prop(optional, into)]
    row_key: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Sort change callback.
    #[prop(optional, into)]
    on_sort_change: Option<Callback<(String, SortOrder)>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let hoverable = hoverable.unwrap_or(true);
    let empty_text = empty_text.unwrap_or_else(|| "No Data".to_string());
    let row_key = row_key.unwrap_or_else(|| "key".to_string());

    // Sort state
    let sort_column = RwSignal::new(None::<String>);
    let sort_order = RwSignal::new(SortOrder::None);

    // Build CSS classes
    let table_prefix = format!("fx-table-{}", design_system);
    let size_class = size.class(&table_prefix);

    let combined_class = {
        let mut parts = vec![table_prefix.clone(), size_class];
        if bordered {
            parts.push(format!("{}-bordered", table_prefix));
        }
        if striped {
            parts.push(format!("{}-striped", table_prefix));
        }
        if hoverable {
            parts.push(format!("{}-hoverable", table_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Class names
    let thead_class = format!("{}-thead", table_prefix);
    let tbody_class = format!("{}-tbody", table_prefix);
    let tr_class = format!("{}-row", table_prefix);
    let th_class = format!("{}-cell {}-cell-header", table_prefix, table_prefix);
    let td_class = format!("{}-cell", table_prefix);
    let empty_class = format!("{}-empty", table_prefix);
    let loading_class = format!("{}-loading", table_prefix);

    // Clone for closures
    let visible_columns: Vec<TableColumn> = columns.into_iter().filter(|c| !c.hidden).collect();
    let columns_for_header = visible_columns.clone();
    let columns_for_body = visible_columns.clone();

    // Clone table_prefix for different usages
    let table_prefix_for_wrapper = table_prefix.clone();
    let table_prefix_for_loading = table_prefix.clone();
    let table_prefix_for_selection = table_prefix.clone();
    let table_prefix_for_body = table_prefix.clone();

    view! {
        <div class=format!("{}-wrapper", table_prefix_for_wrapper)>
            {move || {
                let is_loading = loading.map(|l| l.get()).unwrap_or(false);
                if is_loading {
                    Some(view! {
                        <div class=loading_class.clone()>
                            <span class=format!("{}-loading-spinner", table_prefix_for_loading)>"Loading..."</span>
                        </div>
                    })
                } else {
                    None
                }
            }}
            <table class=combined_class.clone()>
                <thead class=thead_class.clone()>
                    <tr class=tr_class.clone()>
                        {if row_selection {
                            let th_class = th_class.clone();
                            Some(view! {
                                <th class=format!("{} {}-selection-column", th_class, table_prefix_for_selection)>
                                    <input type="checkbox" />
                                </th>
                            })
                        } else {
                            None
                        }}
                        {columns_for_header.iter().map(|col| {
                            let col_key = col.key.clone();
                            let col_key_for_click = col_key.clone();
                            let col_title = col.title.clone();
                            let col_sortable = col.sortable;
                            let col_align = col.align;
                            let col_width = col.width.clone();
                            let th_class = th_class.clone();
                            let table_prefix = table_prefix.clone();
                            let on_sort_change = on_sort_change.clone();

                            let style = col_width.map(|w| format!("width: {}; text-align: {};", w, col_align.as_css()))
                                .unwrap_or_else(|| format!("text-align: {};", col_align.as_css()));

                            view! {
                                <th
                                    class=move || {
                                        let mut cls = vec![th_class.clone()];
                                        if col_sortable {
                                            cls.push(format!("{}-sortable", table_prefix));
                                            if sort_column.get().as_ref() == Some(&col_key) {
                                                cls.push(format!("{}-sorted-{}", table_prefix, sort_order.get().as_suffix()));
                                            }
                                        }
                                        cls.join(" ")
                                    }
                                    style=style
                                    on:click=move |_| {
                                        if col_sortable {
                                            let new_order = if sort_column.get().as_ref() == Some(&col_key_for_click) {
                                                sort_order.get().next()
                                            } else {
                                                SortOrder::Ascend
                                            };
                                            sort_column.set(Some(col_key_for_click.clone()));
                                            sort_order.set(new_order);
                                            if let Some(ref cb) = on_sort_change {
                                                cb.run((col_key_for_click.clone(), new_order));
                                            }
                                        }
                                    }
                                >
                                    <span class=format!("{}-column-title", table_prefix)>
                                        {col_title}
                                    </span>
                                    {if col_sortable {
                                        Some(view! {
                                            <span class=format!("{}-column-sorter", table_prefix)>
                                                <span class=format!("{}-column-sorter-up", table_prefix)>"▲"</span>
                                                <span class=format!("{}-column-sorter-down", table_prefix)>"▼"</span>
                                            </span>
                                        })
                                    } else {
                                        None
                                    }}
                                </th>
                            }
                        }).collect_view()}
                    </tr>
                </thead>
                <tbody class=tbody_class.clone()>
                    {move || {
                        let rows = data.get();
                        if rows.is_empty() {
                            view! {
                                <tr class=tr_class.clone()>
                                    <td
                                        class=empty_class.clone()
                                        colspan=columns_for_body.len() + if row_selection { 1 } else { 0 }
                                    >
                                        {empty_text.clone()}
                                    </td>
                                </tr>
                            }.into_any()
                        } else {
                            rows.into_iter().enumerate().map(|(idx, row)| {
                                let row_key_value = row.get(&row_key).cloned().unwrap_or_else(|| idx.to_string());
                                let tr_class = tr_class.clone();
                                let td_class = td_class.clone();
                                let table_prefix = table_prefix_for_body.clone();

                                view! {
                                    <tr class=tr_class data-row-key=row_key_value.clone()>
                                        {if row_selection {
                                            let td_class = td_class.clone();
                                            let table_prefix = table_prefix.clone();
                                            Some(view! {
                                                <td class=format!("{} {}-selection-column", td_class, table_prefix)>
                                                    <input type="checkbox" />
                                                </td>
                                            })
                                        } else {
                                            None
                                        }}
                                        {columns_for_body.iter().map(|col| {
                                            let cell_value = row.get(&col.data_index).cloned().unwrap_or_default();
                                            let td_class = td_class.clone();
                                            let style = format!("text-align: {};", col.align.as_css());

                                            view! {
                                                <td class=td_class style=style>
                                                    {cell_value}
                                                </td>
                                            }
                                        }).collect_view()}
                                    </tr>
                                }
                            }).collect_view().into_any()
                        }
                    }}
                </tbody>
            </table>
        </div>
    }
}
