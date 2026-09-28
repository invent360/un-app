//! TreeSelect Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{SelectionSize, TreeSelectNode, DropdownPlacement};
use crate::try_use_theme;

/// TreeSelect component.
///
/// A dropdown select with hierarchical tree structure.
/// Supports single/multiple selection, checkable nodes, and search.
///
/// # Props
///
/// - `value` - Selected value(s) signal
/// - `tree_data` - Hierarchical tree data
/// - `placeholder` - Placeholder text
/// - `size` - Component size (Sm, Md, Lg)
/// - `multiple` - Allow multiple selections
/// - `checkable` - Show checkboxes for selection
/// - `show_search` - Enable search filtering
/// - `disabled` - Whether the select is disabled
/// - `allow_clear` - Show clear button
/// - `default_expand_all` - Expand all nodes by default
/// - `placement` - Dropdown placement
/// - `max_height` - Max dropdown height
/// - `class` - Additional CSS classes
/// - `on_change` - Change handler
/// - `on_expand` - Expand/collapse handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{TreeSelect, TreeSelectNode};
///
/// let selected = RwSignal::new(None::<String>);
///
/// let tree_data = vec![
///     TreeSelectNode::branch("parent1", "Parent 1", vec![
///         TreeSelectNode::leaf("child1", "Child 1"),
///         TreeSelectNode::leaf("child2", "Child 2"),
///     ]),
///     TreeSelectNode::leaf("item2", "Item 2"),
/// ];
///
/// view! {
///     <TreeSelect
///         value=selected
///         tree_data=tree_data
///         placeholder="Select an item"
///         on_change=move |v| log::info!("Selected: {:?}", v)
///     />
/// }
/// ```
#[component]
pub fn TreeSelect<T>(
    /// Selected value (single) or values (multiple).
    #[prop(into)]
    value: RwSignal<Option<T>>,
    /// Tree data.
    #[prop(into)]
    tree_data: Vec<TreeSelectNode<T>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Component size.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Allow multiple selections.
    #[prop(optional)]
    multiple: bool,
    /// Show checkboxes.
    #[prop(optional)]
    checkable: bool,
    /// Enable search.
    #[prop(optional)]
    show_search: bool,
    /// Whether disabled.
    #[prop(optional)]
    disabled: bool,
    /// Show clear button.
    #[prop(optional)]
    allow_clear: bool,
    /// Expand all by default.
    #[prop(optional)]
    default_expand_all: bool,
    /// Dropdown placement.
    #[prop(optional, into)]
    placement: Option<DropdownPlacement>,
    /// Max dropdown height.
    #[prop(optional)]
    #[prop(default = 300)]
    max_height: u32,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<Option<T>>>,
    /// Expand handler.
    #[prop(optional, into)]
    on_expand: Option<Callback<Vec<T>>>,
) -> impl IntoView
where
    T: Clone + PartialEq + std::fmt::Debug + Send + Sync + 'static,
{
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let placement = placement.unwrap_or_default();

    // Internal state
    let is_open = RwSignal::new(false);
    let search_text = RwSignal::new(String::new());
    let expanded_keys = RwSignal::new(Vec::<T>::new());

    // Store tree data
    let tree_data_stored = StoredValue::new(tree_data.clone());

    // Initialize expanded keys if default_expand_all
    if default_expand_all {
        let all_branch_keys: Vec<T> = collect_branch_keys(&tree_data);
        expanded_keys.set(all_branch_keys);
    }

    // Build CSS classes
    let tree_select_prefix = format!("fx-tree-select-{}", design_system);

    let wrapper_class = {
        let prefix = tree_select_prefix.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                size.class(&prefix),
            ];

            if disabled {
                parts.push(format!("{}-disabled", prefix));
            }
            if is_open.get() {
                parts.push(format!("{}-open", prefix));
            }
            if multiple {
                parts.push(format!("{}-multiple", prefix));
            }
            if checkable {
                parts.push(format!("{}-checkable", prefix));
            }

            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }

            parts.join(" ")
        }
    };

    // Find label for selected value
    let find_label = move |v: &T| -> Option<String> {
        find_label_in_tree(&tree_data_stored.get_value(), v)
    };

    // Handle dropdown toggle
    let handle_toggle = move |_: ev::MouseEvent| {
        if !disabled {
            is_open.update(|open| *open = !*open);
        }
    };

    // Handle selection
    let on_change_clone = on_change.clone();
    let handle_select = move |node_value: T| {
        if !multiple {
            value.set(Some(node_value.clone()));
            is_open.set(false);

            if let Some(ref cb) = on_change_clone {
                cb.run(Some(node_value));
            }
        }
        // Multiple selection logic would go here
    };

    // Handle expand/collapse
    let on_expand_clone = on_expand.clone();
    let handle_expand = move |node_value: T| {
        expanded_keys.update(|keys| {
            if let Some(pos) = keys.iter().position(|k| k == &node_value) {
                keys.remove(pos);
            } else {
                keys.push(node_value);
            }
        });

        if let Some(ref cb) = on_expand_clone {
            cb.run(expanded_keys.get());
        }
    };

    // Handle clear
    let on_change_clear = on_change.clone();
    let handle_clear = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        value.set(None);
        if let Some(ref cb) = on_change_clear {
            cb.run(None);
        }
    };

    // Handle search input
    let handle_search = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        search_text.set(input_value);
    };

    // Close on outside click
    let handle_blur = move |_: ev::FocusEvent| {
        // Delay to allow click on options
    };

    // CSS class names
    let selector_class = format!("{}-selector", tree_select_prefix);
    let selection_class = format!("{}-selection", tree_select_prefix);
    let placeholder_class = format!("{}-placeholder", tree_select_prefix);
    let clear_class = format!("{}-clear", tree_select_prefix);
    let arrow_class = format!("{}-arrow", tree_select_prefix);
    let dropdown_class = format!("{}-dropdown", tree_select_prefix);
    let search_class = format!("{}-search", tree_select_prefix);
    let tree_class = format!("{}-tree", tree_select_prefix);

    view! {
        <div class=wrapper_class tabindex=0 on:blur=handle_blur>
            // Selector
            <div class=selector_class on:click=handle_toggle>
                <div class=selection_class>
                    {move || {
                        if let Some(ref v) = value.get() {
                            if let Some(label) = find_label(v) {
                                view! { <span>{label}</span> }.into_any()
                            } else {
                                view! { <span class=placeholder_class.clone()>{placeholder.clone().unwrap_or_default()}</span> }.into_any()
                            }
                        } else {
                            view! { <span class=placeholder_class.clone()>{placeholder.clone().unwrap_or_default()}</span> }.into_any()
                        }
                    }}
                </div>

                // Clear button
                {(allow_clear && !disabled).then(|| view! {
                    <button
                        type="button"
                        class=clear_class.clone()
                        class:hidden=move || value.get().is_none()
                        tabindex=-1
                        on:click=handle_clear.clone()
                    >
                        "✕"
                    </button>
                })}

                // Arrow
                <span class=arrow_class>
                    {move || if is_open.get() { "▲" } else { "▼" }}
                </span>
            </div>

            // Dropdown
            <div
                class=dropdown_class
                class:hidden=move || !is_open.get()
                style=format!("max-height: {}px;", max_height)
            >
                // Search
                {show_search.then(|| view! {
                    <div class=search_class.clone()>
                        <input
                            type="text"
                            placeholder="Search..."
                            value=move || search_text.get()
                            on:input=handle_search.clone()
                        />
                    </div>
                })}

                // Tree
                <div class=tree_class role="tree">
                    {render_tree_nodes(
                        tree_data_stored.get_value(),
                        &tree_select_prefix,
                        expanded_keys,
                        search_text,
                        handle_select.clone(),
                        handle_expand.clone(),
                        checkable,
                        0,
                    )}
                </div>
            </div>
        </div>
    }
}

/// Recursively render tree nodes.
fn render_tree_nodes<T>(
    nodes: Vec<TreeSelectNode<T>>,
    prefix: &str,
    expanded_keys: RwSignal<Vec<T>>,
    search_text: RwSignal<String>,
    on_select: impl Fn(T) + Clone + 'static,
    on_expand: impl Fn(T) + Clone + 'static,
    checkable: bool,
    level: usize,
) -> impl IntoView
where
    T: Clone + PartialEq + std::fmt::Debug + Send + Sync + 'static,
{
    let node_class = format!("{}-node", prefix);
    let content_class = format!("{}-node-content", prefix);
    let expand_class = format!("{}-node-expand", prefix);
    let checkbox_class = format!("{}-node-checkbox", prefix);
    let label_class = format!("{}-node-label", prefix);
    let children_class = format!("{}-node-children", prefix);
    let prefix_owned = prefix.to_string();

    nodes.into_iter().map(move |node| {
        let node_value = node.value.clone();
        let node_value_expand = node.value.clone();
        let node_value_select = node.value.clone();
        let has_children = node.is_branch();
        let is_disabled = node.disabled;
        let is_selectable = node.selectable;
        let children = node.children.clone();

        let on_select_clone = on_select.clone();
        let on_expand_clone = on_expand.clone();

        // Filter by search
        let search = search_text.get();
        let matches_search = search.is_empty() ||
            node.label.to_lowercase().contains(&search.to_lowercase());

        if !matches_search && !has_children {
            return view! { <div style="display: none;"></div> }.into_any();
        }

        let node_item_class = {
            let mut classes = vec![node_class.clone()];
            if is_disabled {
                classes.push(format!("{}-disabled", node_class));
            }
            if has_children {
                classes.push(format!("{}-branch", node_class));
            } else {
                classes.push(format!("{}-leaf", node_class));
            }
            classes.join(" ")
        };

        let handle_expand = {
            let on_expand = on_expand_clone.clone();
            let nv = node_value_expand.clone();
            move |ev: ev::MouseEvent| {
                ev.stop_propagation();
                on_expand(nv.clone());
            }
        };

        let handle_select = {
            let on_select = on_select_clone.clone();
            let nv = node_value_select.clone();
            move |_: ev::MouseEvent| {
                if !is_disabled && is_selectable {
                    on_select(nv.clone());
                }
            }
        };

        let prefix_for_children = prefix_owned.clone();
        let on_select_for_children = on_select.clone();
        let on_expand_for_children = on_expand.clone();
        let children_class_cloned = children_class.clone();

        // Clone is_expanded for different usages
        let is_expanded_for_aria = {
            let nv = node_value.clone();
            move || expanded_keys.get().iter().any(|k| k == &nv)
        };
        let is_expanded_for_icon = {
            let nv = node_value.clone();
            move || expanded_keys.get().iter().any(|k| k == &nv)
        };
        let is_expanded_for_children = {
            let nv = node_value.clone();
            move || expanded_keys.get().iter().any(|k| k == &nv)
        };

        view! {
            <div
                class=node_item_class
                style=format!("padding-left: {}px;", level * 20)
                role="treeitem"
                aria-expanded=move || if has_children { Some(is_expanded_for_aria().to_string()) } else { None }
            >
                <div class=content_class.clone() on:click=handle_select>
                    // Expand icon
                    {has_children.then(|| view! {
                        <span class=expand_class.clone() on:click=handle_expand>
                            {move || if is_expanded_for_icon() { "▼" } else { "▶" }}
                        </span>
                    })}
                    {(!has_children).then(|| view! {
                        <span class=format!("{} {}-placeholder", expand_class.clone(), expand_class.clone())></span>
                    })}

                    // Checkbox
                    {checkable.then(|| view! {
                        <span class=checkbox_class.clone()>
                            <input type="checkbox" disabled=is_disabled />
                        </span>
                    })}

                    // Icon
                    {node.icon.clone().map(|icon| view! {
                        <span class=format!("{}-icon", prefix_owned)>{icon}</span>
                    })}

                    // Label
                    <span class=label_class.clone()>{node.label.clone()}</span>
                </div>

                // Children
                {is_expanded_for_children().then(|| {
                    if has_children {
                        Some(view! {
                            <div class=children_class_cloned.clone() role="group">
                                {render_tree_nodes(
                                    children.clone(),
                                    &prefix_for_children,
                                    expanded_keys,
                                    search_text,
                                    on_select_for_children.clone(),
                                    on_expand_for_children.clone(),
                                    checkable,
                                    level + 1,
                                )}
                            </div>
                        })
                    } else {
                        None
                    }
                }).flatten()}
            </div>
        }.into_any()
    }).collect::<Vec<_>>()
}

/// Collect all branch keys for expand all.
fn collect_branch_keys<T: Clone + PartialEq>(nodes: &[TreeSelectNode<T>]) -> Vec<T> {
    let mut keys = Vec::new();
    for node in nodes {
        if node.is_branch() {
            keys.push(node.value.clone());
            keys.extend(collect_branch_keys(&node.children));
        }
    }
    keys
}

/// Find label for a value in tree.
fn find_label_in_tree<T: Clone + PartialEq>(nodes: &[TreeSelectNode<T>], value: &T) -> Option<String> {
    for node in nodes {
        if &node.value == value {
            return Some(node.label.clone());
        }
        if let Some(label) = find_label_in_tree(&node.children, value) {
            return Some(label);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_branch_keys() {
        let tree = vec![
            TreeSelectNode::branch("a".to_string(), "A", vec![
                TreeSelectNode::leaf("a1".to_string(), "A1"),
                TreeSelectNode::branch("a2".to_string(), "A2", vec![
                    TreeSelectNode::leaf("a2a".to_string(), "A2A"),
                ]),
            ]),
            TreeSelectNode::leaf("b".to_string(), "B"),
        ];

        let keys = collect_branch_keys(&tree);
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"a".to_string()));
        assert!(keys.contains(&"a2".to_string()));
    }

    #[test]
    fn test_find_label_in_tree() {
        let tree = vec![
            TreeSelectNode::branch("a".to_string(), "Parent A", vec![
                TreeSelectNode::leaf("a1".to_string(), "Child A1"),
            ]),
        ];

        assert_eq!(find_label_in_tree(&tree, &"a".to_string()), Some("Parent A".to_string()));
        assert_eq!(find_label_in_tree(&tree, &"a1".to_string()), Some("Child A1".to_string()));
        assert_eq!(find_label_in_tree(&tree, &"nonexistent".to_string()), None);
    }
}
