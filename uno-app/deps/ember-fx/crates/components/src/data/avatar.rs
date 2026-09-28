//! Avatar Leptos component.

use leptos::prelude::*;
use super::types::{AvatarSize, AvatarShape};
use crate::try_use_theme;

/// Avatar component.
///
/// Display user avatar with image, icon, or initials.
///
/// # Props
///
/// - `src` - Image source URL
/// - `alt` - Alt text for image
/// - `icon` - Icon to display (when no image)
/// - `size` - Avatar size
/// - `shape` - Avatar shape (circle, square)
/// - `children` - Fallback content (typically initials)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::Avatar;
///
/// view! {
///     <Avatar src="/user.jpg" alt="John Doe" />
///     <Avatar size=AvatarSize::Large>"JD"</Avatar>
/// }
/// ```
#[component]
pub fn Avatar(
    /// Image source URL.
    #[prop(optional, into)]
    src: Option<String>,
    /// Alt text for image.
    #[prop(optional, into)]
    alt: Option<String>,
    /// Icon to display.
    #[prop(optional, into)]
    icon: Option<String>,
    /// Avatar size.
    #[prop(optional, into)]
    size: Option<AvatarSize>,
    /// Avatar shape.
    #[prop(optional, into)]
    shape: Option<AvatarShape>,
    /// Background color.
    #[prop(optional, into)]
    color: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Fallback content (initials).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let shape = shape.unwrap_or_default();

    // Build CSS classes
    let avatar_prefix = format!("fx-avatar-{}", design_system);
    let size_class = size.class(&avatar_prefix);
    let shape_class = shape.class(&avatar_prefix);

    let combined_class = {
        let mut parts = vec![avatar_prefix.clone(), size_class, shape_class];
        if src.is_some() {
            parts.push(format!("{}-image", avatar_prefix));
        } else if icon.is_some() {
            parts.push(format!("{}-icon", avatar_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Custom size style
    let size_style = if let AvatarSize::Custom(px) = size {
        Some(format!("width: {}px; height: {}px; line-height: {}px; font-size: {}px;",
            px, px, px, px / 2))
    } else {
        None
    };

    // Background color style
    let bg_style = color.map(|c| format!("background-color: {};", c));

    // Combine styles
    let style = match (size_style, bg_style) {
        (Some(s), Some(b)) => Some(format!("{} {}", s, b)),
        (Some(s), None) => Some(s),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    view! {
        <span class=combined_class style=style>
            {if let Some(ref image_src) = src {
                Some(view! {
                    <img src=image_src.clone() alt=alt.clone().unwrap_or_default() />
                }.into_any())
            } else if let Some(ref icon_content) = icon {
                Some(view! {
                    <span class=format!("{}-icon-content", avatar_prefix)>
                        {icon_content.clone()}
                    </span>
                }.into_any())
            } else if let Some(children_fn) = children {
                Some(view! {
                    <span class=format!("{}-string", avatar_prefix)>
                        {children_fn()}
                    </span>
                }.into_any())
            } else {
                None
            }}
        </span>
    }
}

/// Avatar Group component.
///
/// Display a group of avatars with overlap.
///
/// # Props
///
/// - `max_count` - Maximum avatars to show
/// - `size` - Size for all avatars
/// - `children` - Avatar children
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::{Avatar, AvatarGroup};
///
/// view! {
///     <AvatarGroup max_count=3>
///         <Avatar src="/user1.jpg" />
///         <Avatar src="/user2.jpg" />
///         <Avatar src="/user3.jpg" />
///         <Avatar src="/user4.jpg" />
///     </AvatarGroup>
/// }
/// ```
#[component]
pub fn AvatarGroup(
    /// Maximum avatars to show.
    #[prop(optional)]
    max_count: Option<usize>,
    /// Size for all avatars.
    #[prop(optional, into)]
    size: Option<AvatarSize>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Avatar children.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let avatar_prefix = format!("fx-avatar-{}", design_system);
    let group_class = format!("{}-group", avatar_prefix);

    let combined_class = {
        let mut parts = vec![group_class.clone()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Note: In a real implementation, we'd need to slice children and add overflow indicator
    // For now, we render all children with the group wrapper
    let _max_count = max_count;
    let _size = size;

    view! {
        <div class=combined_class>
            {children()}
        </div>
    }
}
