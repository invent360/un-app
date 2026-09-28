//! Skeleton Leptos component.

use leptos::prelude::*;
use crate::try_use_theme;

/// Skeleton component.
///
/// Content placeholder while loading.
///
/// # Props
///
/// - `loading` - Whether to show skeleton or children
/// - `active` - Whether to show animation
/// - `avatar` - Show avatar placeholder
/// - `title` - Show title placeholder
/// - `paragraph` - Show paragraph placeholder with rows
/// - `round` - Use rounded corners
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::Skeleton;
///
/// view! {
///     <Skeleton loading=is_loading active=true>
///         <div>Loaded content here</div>
///     </Skeleton>
/// }
/// ```
#[component]
pub fn Skeleton(
    /// Whether to show skeleton.
    #[prop(optional, into)]
    loading: Option<Signal<bool>>,
    /// Whether to show animation.
    #[prop(optional)]
    active: bool,
    /// Show avatar placeholder.
    #[prop(optional)]
    avatar: bool,
    /// Avatar shape (circle or square).
    #[prop(optional, into)]
    avatar_shape: Option<String>,
    /// Avatar size.
    #[prop(optional)]
    avatar_size: Option<u32>,
    /// Show title placeholder.
    #[prop(optional)]
    title: Option<bool>,
    /// Title width.
    #[prop(optional, into)]
    title_width: Option<String>,
    /// Show paragraph placeholder.
    #[prop(optional)]
    paragraph: Option<bool>,
    /// Paragraph rows count.
    #[prop(optional)]
    paragraph_rows: Option<usize>,
    /// Paragraph row widths.
    #[prop(optional, into)]
    paragraph_width: Option<Vec<String>>,
    /// Use rounded corners.
    #[prop(optional)]
    round: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Content to show when not loading.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let loading = loading.unwrap_or_else(|| Signal::derive(|| true));
    let title = title.unwrap_or(true);
    let paragraph = paragraph.unwrap_or(true);
    let paragraph_rows = paragraph_rows.unwrap_or(3);
    let avatar_shape = avatar_shape.unwrap_or_else(|| "circle".to_string());
    let avatar_size = avatar_size.unwrap_or(40);

    // Build CSS classes
    let skeleton_prefix = format!("fx-skeleton-{}", design_system);

    let combined_class = {
        let mut parts = vec![skeleton_prefix.clone()];
        if active {
            parts.push(format!("{}-active", skeleton_prefix));
        }
        if avatar {
            parts.push(format!("{}-with-avatar", skeleton_prefix));
        }
        if round {
            parts.push(format!("{}-round", skeleton_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Class names
    let header_class = format!("{}-header", skeleton_prefix);
    let avatar_class = format!("{}-avatar", skeleton_prefix);
    let content_class = format!("{}-content", skeleton_prefix);
    let title_class = format!("{}-title", skeleton_prefix);
    let paragraph_class = format!("{}-paragraph", skeleton_prefix);

    // Generate paragraph rows
    let row_widths = paragraph_width.unwrap_or_else(|| {
        (0..paragraph_rows).map(|i| {
            if i == paragraph_rows - 1 {
                "60%".to_string()
            } else {
                "100%".to_string()
            }
        }).collect()
    });

    // Render children outside the reactive closure
    let children_view = children.map(|c| c());

    view! {
        <div>
            // Skeleton placeholder (shown when loading)
            <div
                class=combined_class.clone()
                style=move || if loading.get() { None } else { Some("display: none;") }
            >
                {if avatar {
                    Some(view! {
                        <div class=header_class.clone()>
                            <span
                                class=format!("{} {}-{}", avatar_class, avatar_class, avatar_shape)
                                style=format!("width: {}px; height: {}px;", avatar_size, avatar_size)
                            ></span>
                        </div>
                    })
                } else {
                    None
                }}
                <div class=content_class.clone()>
                    {if title {
                        Some(view! {
                            <h3
                                class=title_class.clone()
                                style=title_width.clone().map(|w| format!("width: {};", w))
                            ></h3>
                        })
                    } else {
                        None
                    }}
                    {if paragraph {
                        Some(view! {
                            <ul class=paragraph_class.clone()>
                                {row_widths.clone().into_iter().map(|width| {
                                    view! {
                                        <li style=format!("width: {};", width)></li>
                                    }
                                }).collect_view()}
                            </ul>
                        })
                    } else {
                        None
                    }}
                </div>
            </div>
            // Actual content (shown when not loading)
            <div style=move || if loading.get() { Some("display: none;") } else { None }>
                {children_view}
            </div>
        </div>
    }
}

/// SkeletonButton - Button placeholder.
#[component]
pub fn SkeletonButton(
    /// Whether to show animation.
    #[prop(optional)]
    active: bool,
    /// Button size.
    #[prop(optional, into)]
    size: Option<String>,
    /// Button shape.
    #[prop(optional, into)]
    shape: Option<String>,
    /// Block mode (full width).
    #[prop(optional)]
    block: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let size = size.unwrap_or_else(|| "default".to_string());
    let shape = shape.unwrap_or_else(|| "default".to_string());

    let skeleton_prefix = format!("fx-skeleton-{}", design_system);
    let button_class = format!("{}-button", skeleton_prefix);

    let combined_class = {
        let mut parts = vec![skeleton_prefix.clone(), button_class.clone()];
        if active {
            parts.push(format!("{}-active", skeleton_prefix));
        }
        parts.push(format!("{}-{}", button_class, size));
        parts.push(format!("{}-{}", button_class, shape));
        if block {
            parts.push(format!("{}-block", button_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class></div>
    }
}

/// SkeletonInput - Input placeholder.
#[component]
pub fn SkeletonInput(
    /// Whether to show animation.
    #[prop(optional)]
    active: bool,
    /// Input size.
    #[prop(optional, into)]
    size: Option<String>,
    /// Block mode (full width).
    #[prop(optional)]
    block: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let size = size.unwrap_or_else(|| "default".to_string());

    let skeleton_prefix = format!("fx-skeleton-{}", design_system);
    let input_class = format!("{}-input", skeleton_prefix);

    let combined_class = {
        let mut parts = vec![skeleton_prefix.clone(), input_class.clone()];
        if active {
            parts.push(format!("{}-active", skeleton_prefix));
        }
        parts.push(format!("{}-{}", input_class, size));
        if block {
            parts.push(format!("{}-block", input_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <span class=combined_class></span>
    }
}

/// SkeletonImage - Image placeholder.
#[component]
pub fn SkeletonImage(
    /// Whether to show animation.
    #[prop(optional)]
    active: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let skeleton_prefix = format!("fx-skeleton-{}", design_system);
    let image_class = format!("{}-image", skeleton_prefix);

    let combined_class = {
        let mut parts = vec![skeleton_prefix.clone(), image_class.clone()];
        if active {
            parts.push(format!("{}-active", skeleton_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            <svg viewBox="0 0 1098 1024" xmlns="http://www.w3.org/2000/svg">
                <path d="M365.714286 329.142857q0 45.714286-32.036571 77.677714t-77.677714 32.036571-77.677714-32.036571-32.036571-77.677714 32.036571-77.677714 77.677714-32.036571 77.677714 32.036571 32.036571 77.677714zM950.857143 548.571429l0 256-804.571429 0 0-109.714286 182.857143-182.857143 91.428571 91.428571 292.571429-292.571429zM1005.714286 146.285714l-914.285714 0q-7.460571 0-12.873143 5.412571t-5.412571 12.873143l0 694.857143q0 7.460571 5.412571 12.873143t12.873143 5.412571l914.285714 0q7.460571 0 12.873143-5.412571t5.412571-12.873143l0-694.857143q0-7.460571-5.412571-12.873143t-12.873143-5.412571zM1097.142857 164.571429l0 694.857143q0 37.741714-26.843429 64.585143t-64.585143 26.843429l-914.285714 0q-37.741714 0-64.585143-26.843429t-26.843429-64.585143l0-694.857143q0-37.741714 26.843429-64.585143t64.585143-26.843429l914.285714 0q37.741714 0 64.585143 26.843429t26.843429 64.585143z"/>
            </svg>
        </div>
    }
}
