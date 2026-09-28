use leptos::prelude::*;

/// Badge variant styles
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Success,
    Warning,
    Danger,
    New,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "bg-primary-500 text-white",
            BadgeVariant::Success => "bg-green-500 text-white",
            BadgeVariant::Warning => "bg-amber-500 text-white",
            BadgeVariant::Danger => "bg-red-500 text-white",
            BadgeVariant::New => "bg-violet-500 text-white theme-dark:bg-violet-400",
        }
    }
}

/// Badge component for labels and status indicators
#[component]
pub fn Badge(
    children: Children,
    #[prop(default = BadgeVariant::Primary)] variant: BadgeVariant,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <span class=format!(
            "inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium {} {}",
            variant.class(),
            class
        )>
            {children()}
        </span>
    }
}

/// Convenience component for "New Order" badge
#[component]
pub fn NewOrderBadge() -> impl IntoView {
    view! {
        <Badge variant=BadgeVariant::New>"New Order"</Badge>
    }
}
