use leptos::prelude::*;

/// Avatar size variants
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AvatarSize {
    Small,   // 32px
    #[default]
    Medium,  // 40px
    Large,   // 48px
}

impl AvatarSize {
    pub fn class(&self) -> &'static str {
        match self {
            AvatarSize::Small => "w-8 h-8",
            AvatarSize::Medium => "w-10 h-10",
            AvatarSize::Large => "w-12 h-12",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            AvatarSize::Small => "text-xs",
            AvatarSize::Medium => "text-sm",
            AvatarSize::Large => "text-base",
        }
    }
}

/// Avatar component with image or initials fallback
#[component]
pub fn Avatar(
    #[prop(into)] name: String,
    #[prop(default = String::new(), into)] src: String,
    #[prop(default = AvatarSize::Medium)] size: AvatarSize,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let initials = name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    let bg_colors = [
        "bg-violet-500",
        "bg-blue-500",
        "bg-green-500",
        "bg-orange-500",
        "bg-pink-500",
        "bg-cyan-500",
    ];

    // Simple hash based on name to pick a consistent color
    let color_index = name.bytes().map(|b| b as usize).sum::<usize>() % bg_colors.len();
    let bg_color = bg_colors[color_index];

    let size_class = size.class();
    let text_class = size.text_class();
    let has_image = !src.is_empty();

    view! {
        <div class=format!(
            "rounded-full overflow-hidden flex-shrink-0 {} {}",
            size_class,
            class
        )>
            {if has_image {
                view! {
                    <img
                        src=src
                        alt=name.clone()
                        class="w-full h-full object-cover"
                    />
                }.into_any()
            } else {
                view! {
                    <div class=format!(
                        "w-full h-full flex items-center justify-center text-white font-semibold {} {}",
                        bg_color,
                        text_class
                    )>
                        {initials}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
