//! Collapse/Accordion Leptos component.

use leptos::prelude::*;
use super::types::CollapseIconPosition;
use crate::try_use_theme;

/// Collapse component.
///
/// Expandable content panels (accordion).
///
/// # Props
///
/// - `active_keys` - Currently expanded panel keys
/// - `accordion` - Only allow one panel open at a time
/// - `bordered` - Whether to show border
/// - `ghost` - Transparent background style
/// - `expand_icon_position` - Position of expand icon
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::layout::{Collapse, CollapsePanel};
///
/// let active = RwSignal::new(vec!["1".to_string()]);
///
/// view! {
///     <Collapse active_keys=active>
///         <CollapsePanel key="1" header="Panel 1">
///             <p>"Content of panel 1"</p>
///         </CollapsePanel>
///         <CollapsePanel key="2" header="Panel 2">
///             <p>"Content of panel 2"</p>
///         </CollapsePanel>
///     </Collapse>
/// }
/// ```
#[component]
pub fn Collapse(
    /// Currently expanded panel keys.
    #[prop(into)]
    active_keys: RwSignal<Vec<String>>,
    /// Only allow one panel open at a time.
    #[prop(optional)]
    accordion: bool,
    /// Whether to show border.
    #[prop(optional)]
    bordered: Option<bool>,
    /// Transparent background style.
    #[prop(optional)]
    ghost: bool,
    /// Position of expand icon.
    #[prop(optional, into)]
    expand_icon_position: Option<CollapseIconPosition>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<String>>>,
    /// Panel children.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let bordered = bordered.unwrap_or(true);
    let expand_icon_position = expand_icon_position.unwrap_or_default();

    // Build CSS classes
    let collapse_prefix = format!("fx-collapse-{}", design_system);
    let icon_pos_class = expand_icon_position.class(&collapse_prefix);

    let combined_class = {
        let mut parts = vec![collapse_prefix.clone(), icon_pos_class];
        if bordered {
            parts.push(format!("{}-bordered", collapse_prefix));
        }
        if ghost {
            parts.push(format!("{}-ghost", collapse_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Provide context for panels
    provide_context(CollapseContext {
        active_keys,
        accordion,
        on_change,
        prefix: collapse_prefix.clone(),
    });

    view! {
        <div class=combined_class>
            {children()}
        </div>
    }
}

/// Context for Collapse panels.
#[derive(Clone)]
pub struct CollapseContext {
    pub active_keys: RwSignal<Vec<String>>,
    pub accordion: bool,
    pub on_change: Option<Callback<Vec<String>>>,
    pub prefix: String,
}

/// Collapse panel component.
///
/// A single expandable panel within a Collapse.
#[component]
pub fn CollapsePanel(
    /// Panel key (unique identifier).
    #[prop(into)]
    key: String,
    /// Panel header text.
    #[prop(into)]
    header: String,
    /// Whether the panel is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Extra content in header.
    #[prop(optional, into)]
    extra: Option<String>,
    /// Show arrow icon.
    #[prop(optional)]
    show_arrow: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Panel content.
    children: Children,
) -> impl IntoView {
    // Get context from parent Collapse
    let ctx = use_context::<CollapseContext>()
        .expect("CollapsePanel must be used within a Collapse");

    let show_arrow = show_arrow.unwrap_or(true);
    let prefix = ctx.prefix.clone();

    // Class names
    let item_class = format!("{}-item", prefix);
    let header_class = format!("{}-header", prefix);
    let arrow_class = format!("{}-arrow", prefix);
    let extra_class = format!("{}-extra", prefix);
    let content_class = format!("{}-content", prefix);
    let content_box_class = format!("{}-content-box", prefix);

    // Store active_keys and key for reactive access
    let active_keys = ctx.active_keys;
    let key_for_click = key.clone();
    let key_for_class = key.clone();
    let key_for_aria = key.clone();
    let key_for_arrow = key.clone();
    let key_for_content = key.clone();
    let prefix_for_class = prefix.clone();

    // Handle header click
    let handle_click = move |_| {
        if disabled {
            return;
        }

        let mut keys = active_keys.get_untracked();
        if keys.contains(&key_for_click) {
            // Remove key (collapse)
            keys.retain(|k| k != &key_for_click);
        } else {
            // Add key (expand)
            if ctx.accordion {
                keys.clear();
            }
            keys.push(key_for_click.clone());
        }
        active_keys.set(keys.clone());

        if let Some(ref cb) = ctx.on_change {
            cb.run(keys);
        }
    };

    let combined_class = move || {
        let mut parts = vec![item_class.clone()];
        if active_keys.get().contains(&key_for_class) {
            parts.push(format!("{}-item-active", prefix_for_class));
        }
        if disabled {
            parts.push(format!("{}-item-disabled", prefix_for_class));
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
            // Header
            <div
                class=header_class.clone()
                role="button"
                tabindex=if disabled { -1 } else { 0 }
                aria-expanded=move || active_keys.get().contains(&key_for_aria).to_string()
                on:click=handle_click
            >
                {if show_arrow {
                    Some(view! {
                        <span class=arrow_class.clone()>
                            {move || if active_keys.get().contains(&key_for_arrow) { "▼" } else { "▶" }}
                        </span>
                    })
                } else {
                    None
                }}
                <span>{header.clone()}</span>
                {extra.clone().map(|e| view! {
                    <span class=extra_class.clone()>{e}</span>
                })}
            </div>

            // Content
            <div
                class=content_class.clone()
                style=move || {
                    if active_keys.get().contains(&key_for_content) {
                        None
                    } else {
                        Some("display: none;")
                    }
                }
            >
                <div class=content_box_class.clone()>
                    {children_view}
                </div>
            </div>
        </div>
    }
}
