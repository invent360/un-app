//! Individual chat message bubble component

use leptos::prelude::*;

/// Message sender type
#[derive(Clone, Copy, PartialEq)]
pub enum MessageSender {
    User,
    Bot,
}

/// Props for message bubble
#[derive(Clone)]
pub struct Message {
    pub content: String,
    pub sender: MessageSender,
}

/// Individual message bubble component
#[component]
pub fn MessageBubble(message: Message) -> impl IntoView {
    let sender_class = match message.sender {
        MessageSender::User => "message-user",
        MessageSender::Bot => "message-bot",
    };

    view! {
        <div class=format!("message-bubble {}", sender_class)>
            <p class="message-content">{message.content}</p>
        </div>
    }
}
