//! Toast/Notification Leptos component.

use leptos::prelude::*;
use super::types::{AlertType, ToastPlacement};
use crate::try_use_theme;

/// Toast/Notification item data.
#[derive(Clone, Debug)]
pub struct ToastItem {
    /// Unique ID.
    pub id: String,
    /// Notification type.
    pub toast_type: AlertType,
    /// Title/message.
    pub message: String,
    /// Description.
    pub description: Option<String>,
    /// Duration in ms (0 = no auto close).
    pub duration: u32,
    /// Whether closable.
    pub closable: bool,
}

impl ToastItem {
    /// Create a new toast item.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: format!("toast-{}", rand_id()),
            toast_type: AlertType::Info,
            message: message.into(),
            description: None,
            duration: 4500,
            closable: true,
        }
    }

    /// Set toast type.
    pub fn toast_type(mut self, t: AlertType) -> Self {
        self.toast_type = t;
        self
    }

    /// Set description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set duration.
    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }

    /// Set closable.
    pub fn closable(mut self, c: bool) -> Self {
        self.closable = c;
        self
    }

    /// Create info toast.
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).toast_type(AlertType::Info)
    }

    /// Create success toast.
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).toast_type(AlertType::Success)
    }

    /// Create warning toast.
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).toast_type(AlertType::Warning)
    }

    /// Create error toast.
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).toast_type(AlertType::Error)
    }
}

/// Toast context for managing notifications.
#[derive(Clone)]
pub struct ToastContext {
    /// Current toasts.
    pub toasts: RwSignal<Vec<ToastItem>>,
    /// Placement.
    pub placement: ToastPlacement,
}

impl ToastContext {
    /// Add a toast notification.
    pub fn push(&self, toast: ToastItem) {
        self.toasts.update(|list| {
            list.push(toast);
        });
    }

    /// Remove a toast by ID.
    pub fn remove(&self, id: &str) {
        self.toasts.update(|list| {
            list.retain(|t| t.id != id);
        });
    }

    /// Clear all toasts.
    pub fn clear(&self) {
        self.toasts.set(vec![]);
    }

    /// Show info toast.
    pub fn info(&self, message: impl Into<String>) {
        self.push(ToastItem::info(message));
    }

    /// Show success toast.
    pub fn success(&self, message: impl Into<String>) {
        self.push(ToastItem::success(message));
    }

    /// Show warning toast.
    pub fn warning(&self, message: impl Into<String>) {
        self.push(ToastItem::warning(message));
    }

    /// Show error toast.
    pub fn error(&self, message: impl Into<String>) {
        self.push(ToastItem::error(message));
    }
}

/// Get toast context.
pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>().expect("ToastProvider not found")
}

/// Try to get toast context.
pub fn try_use_toast() -> Option<ToastContext> {
    use_context::<ToastContext>()
}

/// Toast Provider component.
///
/// Provides toast context and renders toast container.
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::{ToastProvider, use_toast};
///
/// view! {
///     <ToastProvider>
///         <App />
///     </ToastProvider>
/// }
///
/// // In a child component:
/// let toast = use_toast();
/// toast.success("Operation completed!");
/// ```
#[component]
pub fn ToastProvider(
    /// Toast placement.
    #[prop(optional, into)]
    placement: Option<ToastPlacement>,
    /// Max toasts to show.
    #[prop(optional)]
    _max_count: Option<usize>,
    /// Children.
    children: Children,
) -> impl IntoView {
    let placement = placement.unwrap_or_default();
    let toasts = RwSignal::new(Vec::<ToastItem>::new());

    let ctx = ToastContext { toasts, placement };
    provide_context(ctx.clone());

    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let toast_prefix = format!("fx-toast-{}", design_system);
    let container_class = format!("{}-container", toast_prefix);
    let placement_class = placement.class(&toast_prefix);

    let combined_class = format!("{} {}", container_class, placement_class);

    let item_class = format!("{}-item", toast_prefix);
    let icon_class = format!("{}-icon", toast_prefix);
    let content_class = format!("{}-content", toast_prefix);
    let message_class = format!("{}-message", toast_prefix);
    let description_class = format!("{}-description", toast_prefix);
    let close_class = format!("{}-close", toast_prefix);

    view! {
        {children()}
        <div class=combined_class>
            {move || {
                let ctx = ctx.clone();
                toasts.get().into_iter().map(|toast| {
                    let ctx_for_close = ctx.clone();
                    let id_for_close = toast.id.clone();
                    let toast_type = toast.toast_type;
                    let type_class = toast_type.class(&toast_prefix);

                    view! {
                        <div class=format!("{} {}", item_class, type_class)>
                            <span class=icon_class.clone()>
                                {toast_type.default_icon()}
                            </span>
                            <div class=content_class.clone()>
                                <div class=message_class.clone()>{toast.message.clone()}</div>
                                {toast.description.clone().map(|d| view! {
                                    <div class=description_class.clone()>{d}</div>
                                })}
                            </div>
                            {if toast.closable {
                                Some(view! {
                                    <button
                                        class=close_class.clone()
                                        on:click=move |_| {
                                            ctx_for_close.remove(&id_for_close);
                                        }
                                    >
                                        "×"
                                    </button>
                                })
                            } else {
                                None
                            }}
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}

/// Single Toast component (standalone use).
#[component]
pub fn Toast(
    /// Toast type.
    #[prop(optional, into)]
    toast_type: Option<AlertType>,
    /// Toast message.
    #[prop(into)]
    message: String,
    /// Description.
    #[prop(optional, into)]
    description: Option<String>,
    /// Whether closable.
    #[prop(optional)]
    closable: bool,
    /// Custom icon.
    #[prop(optional, into)]
    icon: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Close handler.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let toast_type = toast_type.unwrap_or_default();
    let is_closed = RwSignal::new(false);

    let toast_prefix = format!("fx-toast-{}", design_system);
    let item_class = format!("{}-item", toast_prefix);
    let type_class = toast_type.class(&toast_prefix);

    let combined_class = {
        let mut parts = vec![item_class, type_class];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let icon_class = format!("{}-icon", toast_prefix);
    let content_class = format!("{}-content", toast_prefix);
    let message_class = format!("{}-message", toast_prefix);
    let description_class = format!("{}-description", toast_prefix);
    let close_class = format!("{}-close", toast_prefix);

    let icon_content = icon.unwrap_or_else(|| toast_type.default_icon().to_string());

    let handle_close = move |_| {
        is_closed.set(true);
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    view! {
        <Show when=move || !is_closed.get()>
            <div class=combined_class.clone()>
                <span class=icon_class.clone()>{icon_content.clone()}</span>
                <div class=content_class.clone()>
                    <div class=message_class.clone()>{message.clone()}</div>
                    {description.clone().map(|d| view! {
                        <div class=description_class.clone()>{d}</div>
                    })}
                </div>
                {if closable {
                    Some(view! {
                        <button
                            class=close_class.clone()
                            on:click=handle_close
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

/// Generate incremental ID.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(8000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
