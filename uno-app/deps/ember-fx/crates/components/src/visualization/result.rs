//! Result Leptos component.

use leptos::prelude::*;
use super::types::ResultStatus;
use crate::try_use_theme;

/// Result component.
///
/// Result feedback page for operations.
///
/// # Props
///
/// - `status` - Result status (Success, Error, Info, Warning, 404, 403, 500)
/// - `title` - Title text
/// - `sub_title` - Subtitle text
/// - `icon` - Custom icon
/// - `extra` - Extra actions area
/// - `children` - Additional content
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::visualization::{Result, ResultStatus};
///
/// view! {
///     <Result
///         status=ResultStatus::Success
///         title="Successfully Purchased!"
///         sub_title="Order number: 2017182818828182881"
///     >
///         <button>"Go Console"</button>
///         <button>"Buy Again"</button>
///     </Result>
/// }
/// ```
#[component]
pub fn Result(
    /// Result status.
    #[prop(optional, into)]
    status: Option<ResultStatus>,
    /// Title text.
    #[prop(into)]
    title: String,
    /// Subtitle text.
    #[prop(optional, into)]
    sub_title: Option<String>,
    /// Custom icon.
    #[prop(optional, into)]
    icon: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Extra actions.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let status = status.unwrap_or_default();

    // Build CSS classes
    let result_prefix = format!("fx-result-{}", design_system);
    let status_class = status.class(&result_prefix);

    let combined_class = {
        let mut parts = vec![result_prefix.clone(), status_class];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Get icon
    let display_icon = icon.unwrap_or_else(|| status.default_icon().to_string());

    view! {
        <div class=combined_class>
            <div class=format!("{}-icon", result_prefix)>
                <span class=format!("{}-icon-{}", result_prefix, status.as_suffix())>
                    {display_icon}
                </span>
            </div>
            <div class=format!("{}-title", result_prefix)>
                {title}
            </div>
            {sub_title.clone().map(|st| view! {
                <div class=format!("{}-subtitle", result_prefix)>
                    {st}
                </div>
            })}
            {children.map(|c| view! {
                <div class=format!("{}-extra", result_prefix)>
                    {c()}
                </div>
            })}
        </div>
    }
}
