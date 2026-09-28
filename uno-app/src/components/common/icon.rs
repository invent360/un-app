//! Icon component using simple SVG icons

use leptos::prelude::*;

/// Icon names supported by the app
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconName {
    Close,
    Check,
    Copy,
    ChevronDown,
    ChevronUp,
    ChevronRight,
    Menu,
    Search,
    Settings,
    User,
    Help,
    Globe,
    Send,
}

/// Icon component
#[component]
pub fn Icon(
    name: IconName,
    #[prop(optional)] size: Option<u32>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(20);
    let class_name = class.unwrap_or("");

    let path = match name {
        IconName::Close => "M18 6L6 18M6 6l12 12",
        IconName::Check => "M5 13l4 4L19 7",
        IconName::Copy => "M8 4H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2v-2M16 4h2a2 2 0 012 2v2M8 16V8a2 2 0 012-2h8",
        IconName::ChevronDown => "M19 9l-7 7-7-7",
        IconName::ChevronUp => "M5 15l7-7 7 7",
        IconName::ChevronRight => "M9 5l7 7-7 7",
        IconName::Menu => "M4 6h16M4 12h16M4 18h16",
        IconName::Search => "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
        IconName::Settings => "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z",
        IconName::User => "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
        IconName::Help => "M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
        IconName::Globe => "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9",
        IconName::Send => "M12 19l9 2-9-18-9 18 9-2zm0 0v-8",
    };

    view! {
        <svg
            class=format!("icon {}", class_name)
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d=path />
        </svg>
    }
}
