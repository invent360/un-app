//! Card Leptos component.

use leptos::prelude::*;
use super::types::CardSize;
use crate::try_use_theme;

/// Card component.
///
/// A content container with optional header, body, and actions.
///
/// # Props
///
/// - `title` - Card title text
/// - `extra` - Extra content in the header (typically actions)
/// - `size` - Card size (Small, Default)
/// - `bordered` - Whether to show border
/// - `hoverable` - Whether to show hover effect
/// - `loading` - Whether to show loading state
/// - `cover` - Cover image/content at top
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::layout::Card;
///
/// view! {
///     <Card title="Card Title">
///         <p>"Card content goes here"</p>
///     </Card>
/// }
/// ```
#[component]
pub fn Card(
    /// Card title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Extra content in header (actions).
    #[prop(optional, into)]
    extra: Option<String>,
    /// Card size.
    #[prop(optional, into)]
    size: Option<CardSize>,
    /// Whether to show border.
    #[prop(optional)]
    bordered: Option<bool>,
    /// Whether to show hover effect.
    #[prop(optional)]
    hoverable: bool,
    /// Whether the card is loading.
    #[prop(optional, into)]
    loading: Option<Signal<bool>>,
    /// Cover image URL or content.
    #[prop(optional, into)]
    cover: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Card body content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let bordered = bordered.unwrap_or(true);
    let loading = loading.unwrap_or_else(|| Signal::derive(|| false));

    // Build CSS classes
    let card_prefix = format!("fx-card-{}", design_system);
    let size_class = if size == CardSize::Small {
        Some(size.class(&card_prefix))
    } else {
        None
    };

    // Class names
    let head_class = format!("{}-head", card_prefix);
    let head_title_class = format!("{}-head-title", card_prefix);
    let extra_class = format!("{}-extra", card_prefix);
    let cover_class = format!("{}-cover", card_prefix);
    let body_class = format!("{}-body", card_prefix);
    let loading_class = format!("{}-loading", card_prefix);

    // Clone for closure
    let card_prefix_for_class = card_prefix.clone();

    let combined_class = move || {
        let mut parts = vec![card_prefix_for_class.clone()];
        if let Some(ref sc) = size_class {
            parts.push(sc.clone());
        }
        if bordered {
            parts.push(format!("{}-bordered", card_prefix_for_class));
        }
        if hoverable {
            parts.push(format!("{}-hoverable", card_prefix_for_class));
        }
        if loading.get() {
            parts.push(loading_class.clone());
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
            // Cover image
            {cover.clone().map(|src| view! {
                <div class=cover_class.clone()>
                    <img src=src alt="" />
                </div>
            })}

            // Header (only if title or extra provided)
            {if title.is_some() || extra.is_some() {
                Some(view! {
                    <div class=head_class.clone()>
                        <div class=head_title_class.clone()>
                            {title.clone()}
                        </div>
                        {extra.clone().map(|e| view! {
                            <div class=extra_class.clone()>{e}</div>
                        })}
                    </div>
                })
            } else {
                None
            }}

            // Body
            <div class=body_class.clone()>
                {children_view}
            </div>
        </div>
    }
}
