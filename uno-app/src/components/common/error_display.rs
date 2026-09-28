//! Error display components for better UX

use leptos::prelude::*;
use crate::hooks::t;

/// Error severity levels
#[derive(Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl ErrorSeverity {
    fn class(&self) -> &'static str {
        match self {
            Self::Info => "error-info",
            Self::Warning => "error-warning",
            Self::Error => "error-error",
            Self::Critical => "error-critical",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Info => "i",
            Self::Warning => "!",
            Self::Error => "x",
            Self::Critical => "!!",
        }
    }
}

/// Inline error message display
#[component]
pub fn ErrorMessage(
    #[prop(into)] message: String,
    #[prop(default = ErrorSeverity::Error)] severity: ErrorSeverity,
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class=format!("error-message {}", severity.class())>
            <span class="error-icon">{severity.icon()}</span>
            <span class="error-text">{message}</span>
            {on_dismiss.map(|dismiss| {
                view! {
                    <button
                        class="error-dismiss"
                        on:click=move |_| dismiss.run(())
                    >
                        "x"
                    </button>
                }
            })}
        </div>
    }
}

/// Error toast notification
#[component]
pub fn ErrorToast(
    #[prop(into)] message: String,
    #[prop(default = ErrorSeverity::Error)] severity: ErrorSeverity,
    #[prop(default = true)] auto_dismiss: bool,
    #[prop(default = 5000)] _dismiss_ms: u32,
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
) -> impl IntoView {
    let (visible, set_visible) = signal(true);

    // Auto-dismiss after delay
    if auto_dismiss {
        #[cfg(feature = "hydrate")]
        {
            use gloo_timers::future::TimeoutFuture;
            use leptos::task::spawn_local;

            spawn_local(async move {
                TimeoutFuture::new(_dismiss_ms).await;
                set_visible.set(false);
                if let Some(dismiss) = on_dismiss {
                    dismiss.run(());
                }
            });
        }
    }

    view! {
        <Show when=move || visible.get()>
            <div class=format!("error-toast {}", severity.class())>
                <span class="error-icon">{severity.icon()}</span>
                <span class="error-text">{message.clone()}</span>
                <button
                    class="error-dismiss"
                    on:click=move |_| {
                        set_visible.set(false);
                        if let Some(dismiss) = on_dismiss {
                            dismiss.run(());
                        }
                    }
                >
                    "x"
                </button>
            </div>
        </Show>
    }
}

/// Empty state display when no data is available
#[component]
pub fn EmptyState(
    #[prop(into)] title: String,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, optional)] action_label: Option<String>,
    #[prop(optional)] on_action: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="empty-state">
            <div class="empty-state-icon">"O"</div>
            <h3 class="empty-state-title">{title}</h3>
            {description.map(|desc| {
                view! { <p class="empty-state-description">{desc}</p> }
            })}
            {action_label.map(|label| {
                view! {
                    <button
                        class="btn-primary"
                        on:click=move |_| {
                            if let Some(action) = on_action {
                                action.run(());
                            }
                        }
                    >
                        {label}
                    </button>
                }
            })}
        </div>
    }
}

/// Connection error display
#[component]
pub fn ConnectionError(
    #[prop(optional)] on_retry: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="connection-error">
            <div class="connection-error-icon">"!"</div>
            <h3>{move || t("errors.connection_title")}</h3>
            <p>{move || t("errors.connection_message")}</p>
            {on_retry.map(|retry| {
                view! {
                    <button
                        class="btn-primary"
                        on:click=move |_| retry.run(())
                    >
                        {move || t("common.retry")}
                    </button>
                }
            })}
        </div>
    }
}

/// Server error display
#[component]
pub fn ServerError(
    #[prop(into, optional)] error_code: Option<String>,
    #[prop(optional)] on_retry: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="server-error">
            <div class="server-error-icon">"500"</div>
            <h3>{move || t("errors.server_title")}</h3>
            <p>{move || t("errors.server_message")}</p>
            {error_code.map(|code| {
                view! { <p class="error-code">{move || t("errors.error_code_prefix")} " " {code.clone()}</p> }
            })}
            {on_retry.map(|retry| {
                view! {
                    <button
                        class="btn-primary"
                        on:click=move |_| retry.run(())
                    >
                        {move || t("errors.try_again")}
                    </button>
                }
            })}
        </div>
    }
}

/// Loading error with retry option
#[component]
pub fn LoadingError(
    #[prop(into)] message: String,
    #[prop(optional)] on_retry: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="loading-error">
            <ErrorMessage message=message.clone() severity=ErrorSeverity::Error />
            {on_retry.map(|retry| {
                view! {
                    <button
                        class="btn-secondary"
                        on:click=move |_| retry.run(())
                    >
                        {move || t("common.retry")}
                    </button>
                }
            })}
        </div>
    }
}
