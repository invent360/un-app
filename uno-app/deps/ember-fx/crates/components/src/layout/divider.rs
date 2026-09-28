//! Divider Leptos component.

use leptos::prelude::*;
use super::types::{DividerType, DividerOrientation};
use crate::try_use_theme;

/// Divider component.
///
/// A visual separator for content.
///
/// # Props
///
/// - `divider_type` - Horizontal or Vertical
/// - `dashed` - Use dashed line style
/// - `orientation` - Text orientation (Left, Center, Right)
/// - `text` - Text to display in the divider
/// - `plain` - Plain style (no bold text)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::layout::Divider;
///
/// view! {
///     <p>"Content above"</p>
///     <Divider />
///     <p>"Content below"</p>
///
///     <Divider text="Section Title" />
///
///     <div style="display: flex; height: 100px;">
///         <span>"Left"</span>
///         <Divider divider_type=DividerType::Vertical />
///         <span>"Right"</span>
///     </div>
/// }
/// ```
#[component]
pub fn Divider(
    /// Divider type (Horizontal or Vertical).
    #[prop(optional, into)]
    divider_type: Option<DividerType>,
    /// Use dashed line style.
    #[prop(optional)]
    dashed: bool,
    /// Text orientation when text is present.
    #[prop(optional, into)]
    orientation: Option<DividerOrientation>,
    /// Text to display in the divider.
    #[prop(optional, into)]
    text: Option<String>,
    /// Plain style (no bold text).
    #[prop(optional)]
    plain: bool,
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
    let divider_type = divider_type.unwrap_or_default();
    let orientation = orientation.unwrap_or_default();

    // Build CSS classes
    let divider_prefix = format!("fx-divider-{}", design_system);
    let type_class = divider_type.class(&divider_prefix);

    let combined_class = {
        let mut parts = vec![divider_prefix.clone(), type_class];
        if dashed {
            parts.push(format!("{}-dashed", divider_prefix));
        }
        if text.is_some() {
            parts.push(format!("{}-with-text", divider_prefix));
            parts.push(orientation.class(&divider_prefix));
        }
        if plain {
            parts.push(format!("{}-plain", divider_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Inner text class
    let inner_class = format!("{}-inner-text", divider_prefix);

    view! {
        <div class=combined_class role="separator">
            {text.clone().map(|t| view! {
                <span class=inner_class.clone()>{t}</span>
            })}
        </div>
    }
}
