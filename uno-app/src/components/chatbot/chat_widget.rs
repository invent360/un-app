//! Floating chat widget component

use leptos::prelude::*;
use crate::hooks::t;
use super::chat_window::ChatWindow;

/// Floating chat widget that can be collapsed/expanded
#[component]
pub fn ChatWidget() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let toggle_chat = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    let close_chat = Callback::new(move |_: ()| {
        set_is_open.set(false);
    });

    view! {
        <div class="chat-widget">
            {move || if is_open.get() {
                view! {
                    <ChatWindow on_close=close_chat />
                }.into_any()
            } else {
                view! {
                    <button class="chat-toggle-btn" on:click=toggle_chat>
                        <span class="chat-icon">"?"</span>
                        <span class="chat-label">{move || t("chat.help")}</span>
                    </button>
                }.into_any()
            }}
        </div>
    }
}
