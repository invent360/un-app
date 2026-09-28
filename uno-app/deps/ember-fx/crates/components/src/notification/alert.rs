//! Alert Leptos component.

use leptos::prelude::*;
use super::types::AlertType;
use crate::try_use_theme;

/// Alert component.
///
/// Contextual feedback messages for user actions.
///
/// # Props
///
/// - `alert_type` - Alert severity (Info, Success, Warning, Error)
/// - `message` - Alert message content
/// - `description` - Additional description text
/// - `closable` - Whether the alert can be closed
/// - `show_icon` - Whether to show the type icon
/// - `icon` - Custom icon content
/// - `banner` - Display as full-width banner
/// - `on_close` - Close handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::Alert;
///
/// view! {
///     <Alert
///         alert_type=AlertType::Success
///         message="Operation completed successfully!"
///         closable=true
///     />
/// }
/// ```
#[component]
pub fn Alert(
    /// Alert severity type.
    #[prop(optional, into)]
    alert_type: Option<AlertType>,
    /// Alert message.
    #[prop(into)]
    message: String,
    /// Additional description.
    #[prop(optional, into)]
    description: Option<String>,
    /// Whether the alert can be closed.
    #[prop(optional)]
    closable: bool,
    /// Whether to show the type icon.
    #[prop(optional)]
    show_icon: Option<bool>,
    /// Custom icon content.
    #[prop(optional, into)]
    icon: Option<String>,
    /// Display as banner (full width, no border radius).
    #[prop(optional)]
    banner: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Close handler.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let alert_type = alert_type.unwrap_or_default();
    let show_icon = show_icon.unwrap_or(true);

    // Closed state
    let is_closed = RwSignal::new(false);

    // Build CSS classes
    let alert_prefix = format!("fx-alert-{}", design_system);
    let type_class = alert_type.class(&alert_prefix);

    let combined_class = {
        let mut parts = vec![alert_prefix.clone(), type_class];
        if show_icon {
            parts.push(format!("{}-with-icon", alert_prefix));
        }
        if description.is_some() {
            parts.push(format!("{}-with-description", alert_prefix));
        }
        if banner {
            parts.push(format!("{}-banner", alert_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Icon content
    let icon_content = icon.unwrap_or_else(|| alert_type.default_icon().to_string());

    // Handle close
    let handle_close = move |_| {
        is_closed.set(true);
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    // Class names
    let icon_class = format!("{}-icon", alert_prefix);
    let content_class = format!("{}-content", alert_prefix);
    let message_class = format!("{}-message", alert_prefix);
    let description_class = format!("{}-description", alert_prefix);
    let close_class = format!("{}-close-icon", alert_prefix);

    view! {
        <Show when=move || !is_closed.get()>
            <div class=combined_class.clone() role="alert">
                {if show_icon {
                    Some(view! {
                        <span class=icon_class.clone()>{icon_content.clone()}</span>
                    })
                } else {
                    None
                }}
                <div class=content_class.clone()>
                    <div class=message_class.clone()>{message.clone()}</div>
                    {description.clone().map(|desc| view! {
                        <div class=description_class.clone()>{desc}</div>
                    })}
                </div>
                {if closable {
                    Some(view! {
                        <button
                            type="button"
                            class=close_class.clone()
                            on:click=handle_close
                            aria-label="Close"
                        >
                            "×"
                        </button>
                    })
                } else {
                    None
                }}
            </div>
        </Show>
    }
}
