//! Expanded chat window component

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;
use super::message_bubble::{Message, MessageBubble, MessageSender};
use crate::api::send_chat_message;
use crate::hooks::t;

/// Chat window component (expanded view)
#[component]
pub fn ChatWindow(
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    // Get initial greeting from translations
    let initial_greeting = t("chat.welcome");

    let (messages, set_messages) = signal(vec![
        Message {
            content: initial_greeting,
            sender: MessageSender::Bot,
        },
    ]);
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    let do_send = move || {
        let user_message = input_value.get();
        if user_message.trim().is_empty() {
            return;
        }

        // Add user message
        set_messages.update(|msgs| {
            msgs.push(Message {
                content: user_message.clone(),
                sender: MessageSender::User,
            });
        });
        set_input_value.set(String::new());
        set_is_loading.set(true);

        // Send to server and get response
        let user_msg = user_message.clone();
        spawn_local(async move {
            match send_chat_message(user_msg).await {
                Ok(response) => {
                    set_messages.update(|msgs| {
                        msgs.push(Message {
                            content: response,
                            sender: MessageSender::Bot,
                        });
                    });
                }
                Err(e) => {
                    let error_prefix = t("chat.error_prefix");
                    set_messages.update(|msgs| {
                        msgs.push(Message {
                            content: format!("{} {}", error_prefix, e),
                            sender: MessageSender::Bot,
                        });
                    });
                }
            }
            set_is_loading.set(false);
        });
    };

    let handle_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            do_send();
        }
    };

    let handle_click = move |_: web_sys::MouseEvent| {
        do_send();
    };

    view! {
        <div class="chat-window">
            <div class="chat-header">
                <h3>{move || t("chat.title")}</h3>
                <button class="chat-close-btn" on:click=move |_| on_close.run(())>
                    "×"
                </button>
            </div>

            <div class="chat-messages">
                <For
                    each=move || messages.get().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(_, msg)| {
                        view! { <MessageBubble message=msg.clone() /> }
                    }
                />
                {move || is_loading.get().then(|| view! {
                    <div class="message-bubble message-bot typing">
                        <span class="typing-indicator">{move || t("chat.typing_indicator")}</span>
                    </div>
                })}
            </div>

            <div class="chat-input-area">
                <input
                    type="text"
                    class="chat-input"
                    placeholder=move || t("chat.input_placeholder")
                    prop:value=move || input_value.get()
                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                    on:keypress=handle_keypress
                />
                <button
                    class="chat-send-btn"
                    on:click=handle_click
                    disabled=move || is_loading.get() || input_value.get().trim().is_empty()
                >
                    {move || t("chat.send")}
                </button>
            </div>
        </div>
    }
}
