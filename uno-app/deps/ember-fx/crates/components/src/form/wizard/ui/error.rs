//! Wizard error and success display components.
//!
//! Components for displaying validation errors and success messages.

use leptos::prelude::*;
use crate::try_use_theme;
use super::super::types::ErrorDisplayStyle;

/// SVG icons for error/success displays.
mod icons {
    use leptos::prelude::*;

    pub fn error_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 20 20" fill="currentColor" class="ant-wizard-error-icon">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
            </svg>
        }
    }

    pub fn success_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 20 20" fill="currentColor" class="ant-wizard-success-icon">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
            </svg>
        }
    }

    pub fn close_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 20 20" fill="currentColor" class="ant-wizard-close-icon">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
            </svg>
        }
    }

    pub fn warning_icon() -> impl IntoView {
        view! {
            <svg viewBox="0 0 20 20" fill="currentColor" class="ant-wizard-warning-icon">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
            </svg>
        }
    }
}

/// Wizard error display component.
///
/// Displays validation errors with various styles.
#[component]
pub fn WizardErrorDisplay(
    /// Error messages to display.
    #[prop(into)]
    messages: Signal<Vec<String>>,
    /// Display style.
    #[prop(optional, into)]
    style: Option<ErrorDisplayStyle>,
    /// Error title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Callback when dismissed.
    #[prop(optional, into)]
    on_dismiss: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-error", design_system);
    let style = style.unwrap_or_default();
    let title = title.unwrap_or_else(|| "Please correct the following errors:".to_string());

    let has_errors = Signal::derive(move || !messages.get().is_empty());
    let has_dismiss = on_dismiss.is_some();

    view! {
        <Show when=move || has_errors.get()>
            {
                let prefix = prefix.clone();
                let title = title.clone();
                let class = class.clone();
                let on_dismiss = on_dismiss.clone();

                let container_class = {
                    let prefix = prefix.clone();
                    let class = class.clone();
                    format!("{} {}-{} {}", prefix, prefix, style.class_suffix(), class.unwrap_or_default())
                };

                let handle_dismiss = {
                    let on_dismiss = on_dismiss.clone();
                    move |_| {
                        if let Some(ref cb) = on_dismiss {
                            cb.run(());
                        }
                    }
                };

                match style {
                    ErrorDisplayStyle::Toast => {
                        let prefix_toast = prefix.clone();
                        let prefix_icon = prefix.clone();
                        let prefix_body = prefix.clone();
                        let prefix_title = prefix.clone();
                        let prefix_list = prefix.clone();
                        let prefix_dismiss = prefix.clone();

                        view! {
                            <div class=container_class.clone() role="alert">
                                <div class=format!("{}-toast-content", prefix_toast)>
                                    <div class=format!("{}-icon", prefix_icon)>
                                        {icons::error_icon()}
                                    </div>
                                    <div class=format!("{}-body", prefix_body)>
                                        <h4 class=format!("{}-title", prefix_title)>{title.clone()}</h4>
                                        <ul class=format!("{}-list", prefix_list)>
                                            <For
                                                each=move || messages.get()
                                                key=|msg| msg.clone()
                                                children=move |msg| view! {
                                                    <li>{msg}</li>
                                                }
                                            />
                                        </ul>
                                    </div>
                                    <Show when=move || has_dismiss>
                                        <button
                                            type="button"
                                            class=format!("{}-dismiss", prefix_dismiss)
                                            on:click=handle_dismiss.clone()
                                            aria-label="Dismiss"
                                        >
                                            {icons::close_icon()}
                                        </button>
                                    </Show>
                                </div>
                            </div>
                        }.into_any()
                    }
                    ErrorDisplayStyle::Alert => {
                        let prefix_alert = prefix.clone();
                        let prefix_icon = prefix.clone();
                        let prefix_body = prefix.clone();
                        let prefix_title = prefix.clone();
                        let prefix_list = prefix.clone();
                        let prefix_dismiss = prefix.clone();

                        view! {
                            <div class=container_class.clone() role="alert">
                                <div class=format!("{}-alert-content", prefix_alert)>
                                    <div class=format!("{}-icon", prefix_icon)>
                                        {icons::error_icon()}
                                    </div>
                                    <div class=format!("{}-body", prefix_body)>
                                        <h4 class=format!("{}-title", prefix_title)>{title.clone()}</h4>
                                        <ul class=format!("{}-list", prefix_list)>
                                            <For
                                                each=move || messages.get()
                                                key=|msg| msg.clone()
                                                children=move |msg| view! {
                                                    <li>{msg}</li>
                                                }
                                            />
                                        </ul>
                                    </div>
                                    <Show when=move || has_dismiss>
                                        <button
                                            type="button"
                                            class=format!("{}-dismiss", prefix_dismiss)
                                            on:click=handle_dismiss.clone()
                                            aria-label="Dismiss"
                                        >
                                            {icons::close_icon()}
                                        </button>
                                    </Show>
                                </div>
                            </div>
                        }.into_any()
                    }
                    ErrorDisplayStyle::Inline => {
                        let prefix_inline = prefix.clone();

                        view! {
                            <div class=container_class.clone() role="alert">
                                <For
                                    each=move || messages.get()
                                    key=|msg| msg.clone()
                                    children={
                                        let prefix_inline = prefix_inline.clone();
                                        move |msg| {
                                            view! {
                                                <div class=format!("{}-inline-item", prefix_inline)>
                                                    {icons::warning_icon()}
                                                    <span>{msg}</span>
                                                </div>
                                            }
                                        }
                                    }
                                />
                            </div>
                        }.into_any()
                    }
                }
            }
        </Show>
    }
}

/// Wizard success display component.
///
/// Shows success message when wizard is completed.
#[component]
pub fn WizardSuccessDisplay(
    /// Whether to show the success message.
    #[prop(into)]
    show: Signal<bool>,
    /// Success title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Success message body.
    #[prop(optional, into)]
    message: Option<String>,
    /// Callback when dismissed.
    #[prop(optional, into)]
    on_dismiss: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wizard-{}-success", design_system);
    let title = title.unwrap_or_else(|| "Success!".to_string());

    let has_dismiss = on_dismiss.is_some();
    let has_message = message.is_some();

    view! {
        <Show when=move || show.get()>
            {
                let prefix = prefix.clone();
                let prefix_content = prefix.clone();
                let prefix_icon = prefix.clone();
                let prefix_body = prefix.clone();
                let prefix_title = prefix.clone();
                let prefix_msg = prefix.clone();
                let prefix_dismiss = prefix.clone();
                let title = title.clone();
                let message = message.clone();
                let class = class.clone();
                let on_dismiss = on_dismiss.clone();

                let container_class = format!("{} {}", prefix, class.unwrap_or_default());

                let handle_dismiss = {
                    let on_dismiss = on_dismiss.clone();
                    move |_| {
                        if let Some(ref cb) = on_dismiss {
                            cb.run(());
                        }
                    }
                };

                view! {
                    <div class=container_class role="status">
                        <div class=format!("{}-content", prefix_content)>
                            <div class=format!("{}-icon", prefix_icon)>
                                {icons::success_icon()}
                            </div>
                            <div class=format!("{}-body", prefix_body)>
                                <h4 class=format!("{}-title", prefix_title)>{title.clone()}</h4>
                                <Show when=move || has_message>
                                    <p class=format!("{}-message", prefix_msg)>
                                        {message.clone().unwrap_or_default()}
                                    </p>
                                </Show>
                            </div>
                            <Show when=move || has_dismiss>
                                <button
                                    type="button"
                                    class=format!("{}-dismiss", prefix_dismiss)
                                    on:click=handle_dismiss.clone()
                                    aria-label="Dismiss"
                                >
                                    {icons::close_icon()}
                                </button>
                            </Show>
                        </div>
                    </div>
                }
            }
        </Show>
    }
}
