//! Tree Leptos component.

use leptos::prelude::*;
use super::types::TreeNode;
use crate::try_use_theme;

/// Tree component.
///
/// Hierarchical tree view with expandable nodes.
///
/// # Props
///
/// - `data` - Tree node data
/// - `checkable` - Show checkboxes
/// - `default_expanded_keys` - Initially expanded node keys
/// - `default_selected_keys` - Initially selected node keys
/// - `show_line` - Show connecting lines
/// - `show_icon` - Show icons
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::visualization::{Tree, TreeNode};
///
/// let data = vec![
///     TreeNode::new("parent-1", "Parent 1").children(vec![
///         TreeNode::new("child-1-1", "Child 1-1"),
///         TreeNode::new("child-1-2", "Child 1-2"),
///     ]),
///     TreeNode::new("parent-2", "Parent 2"),
/// ];
///
/// view! {
///     <Tree data=data />
/// }
/// ```
#[component]
pub fn Tree(
    /// Tree node data.
    #[prop(into)]
    data: Vec<TreeNode>,
    /// Show checkboxes.
    #[prop(optional)]
    checkable: bool,
    /// Initially expanded keys.
    #[prop(optional, into)]
    default_expanded_keys: Option<Vec<String>>,
    /// Initially selected keys.
    #[prop(optional, into)]
    default_selected_keys: Option<Vec<String>>,
    /// Show connecting lines.
    #[prop(optional)]
    show_line: bool,
    /// Show icons.
    #[prop(optional)]
    show_icon: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Select callback.
    #[prop(optional, into)]
    on_select: Option<Callback<String>>,
    /// Check callback.
    #[prop(optional, into)]
    on_check: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // State
    let expanded_keys = RwSignal::new(default_expanded_keys.unwrap_or_default());
    let selected_keys = RwSignal::new(default_selected_keys.unwrap_or_default());
    let checked_keys = RwSignal::new(Vec::<String>::new());

    let show_icon = show_icon.unwrap_or(true);

    // Build CSS classes
    let tree_prefix = format!("fx-tree-{}", design_system);

    let combined_class = {
        let mut parts = vec![tree_prefix.clone()];
        if show_line {
            parts.push(format!("{}-show-line", tree_prefix));
        }
        if checkable {
            parts.push(format!("{}-checkable", tree_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            <TreeNodeList
                nodes=data
                level=0usize
                expanded_keys=expanded_keys
                selected_keys=selected_keys
                checked_keys=checked_keys
                checkable=checkable
                show_icon=show_icon
                on_select=on_select
                on_check=on_check
            />
        </div>
    }
}

/// Internal component for rendering tree node list.
#[component]
fn TreeNodeList(
    nodes: Vec<TreeNode>,
    level: usize,
    expanded_keys: RwSignal<Vec<String>>,
    selected_keys: RwSignal<Vec<String>>,
    checked_keys: RwSignal<Vec<String>>,
    checkable: bool,
    show_icon: bool,
    on_select: Option<Callback<String>>,
    on_check: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let tree_prefix = format!("fx-tree-{}", design_system);

    view! {
        <ul class=format!("{}-list", tree_prefix) style=format!("padding-left: {}px;", level * 18)>
            {nodes.into_iter().map(|node| {
                let key = node.key.clone();
                let key_for_expand = key.clone();
                let key_for_select = key.clone();
                let key_for_check = key.clone();
                let key_for_class = key.clone();
                let key_for_switcher = key.clone();
                let key_for_children = key.clone();
                let has_children = !node.children.is_empty();
                let is_leaf = node.is_leaf || node.children.is_empty();
                let tree_prefix = tree_prefix.clone();
                let on_select = on_select.clone();
                let on_check = on_check.clone();

                view! {
                    <li class=move || {
                        let mut cls = vec![format!("{}-node", tree_prefix)];
                        if expanded_keys.get().contains(&key_for_class) {
                            cls.push(format!("{}-node-expanded", tree_prefix));
                        }
                        if selected_keys.get().contains(&key_for_class) {
                            cls.push(format!("{}-node-selected", tree_prefix));
                        }
                        if node.disabled {
                            cls.push(format!("{}-node-disabled", tree_prefix));
                        }
                        if is_leaf {
                            cls.push(format!("{}-node-leaf", tree_prefix));
                        }
                        cls.join(" ")
                    }>
                        <div class=format!("{}-node-content", tree_prefix)>
                            // Switcher (expand/collapse)
                            <span
                                class=format!("{}-switcher", tree_prefix)
                                on:click=move |_| {
                                    if has_children && !node.disabled {
                                        let mut keys = expanded_keys.get();
                                        if keys.contains(&key_for_expand) {
                                            keys.retain(|k| k != &key_for_expand);
                                        } else {
                                            keys.push(key_for_expand.clone());
                                        }
                                        expanded_keys.set(keys);
                                    }
                                }
                            >
                                {move || {
                                    if is_leaf {
                                        " ".to_string()
                                    } else if expanded_keys.get().contains(&key_for_switcher) {
                                        "▼".to_string()
                                    } else {
                                        "▶".to_string()
                                    }
                                }}
                            </span>

                            // Checkbox
                            {if checkable {
                                let key = key_for_check.clone();
                                Some(view! {
                                    <span class=format!("{}-checkbox", tree_prefix)>
                                        <input
                                            type="checkbox"
                                            checked=move || checked_keys.get().contains(&key)
                                            disabled=node.disabled
                                            on:change=move |_| {
                                                let mut keys = checked_keys.get();
                                                if keys.contains(&key_for_check) {
                                                    keys.retain(|k| k != &key_for_check);
                                                } else {
                                                    keys.push(key_for_check.clone());
                                                }
                                                checked_keys.set(keys.clone());
                                                if let Some(ref cb) = on_check {
                                                    cb.run(keys);
                                                }
                                            }
                                        />
                                    </span>
                                })
                            } else {
                                None
                            }}

                            // Icon
                            {if show_icon {
                                let icon = node.icon.clone().unwrap_or_else(|| {
                                    if is_leaf { "📄" } else { "📁" }.to_string()
                                });
                                Some(view! {
                                    <span class=format!("{}-icon", tree_prefix)>{icon}</span>
                                })
                            } else {
                                None
                            }}

                            // Title
                            <span
                                class=format!("{}-title", tree_prefix)
                                on:click=move |_| {
                                    if !node.disabled {
                                        selected_keys.set(vec![key_for_select.clone()]);
                                        if let Some(ref cb) = on_select {
                                            cb.run(key_for_select.clone());
                                        }
                                    }
                                }
                            >
                                {node.title.clone()}
                            </span>
                        </div>

                        // Children
                        {move || {
                            if has_children && expanded_keys.get().contains(&key_for_children) {
                                view! {
                                    <TreeNodeList
                                        nodes=node.children.clone()
                                        level=level + 1
                                        expanded_keys=expanded_keys
                                        selected_keys=selected_keys
                                        checked_keys=checked_keys
                                        checkable=checkable
                                        show_icon=show_icon
                                        on_select=on_select.clone()
                                        on_check=on_check.clone()
                                    />
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}
