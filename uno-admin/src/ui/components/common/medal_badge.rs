use leptos::prelude::*;

/// Medal rank for top performers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MedalRank {
    Gold,    // 1st place
    Silver,  // 2nd place
    Bronze,  // 3rd place
}

impl MedalRank {
    pub fn from_rank(rank: u8) -> Option<Self> {
        match rank {
            1 => Some(MedalRank::Gold),
            2 => Some(MedalRank::Silver),
            3 => Some(MedalRank::Bronze),
            _ => None,
        }
    }

    fn color(&self) -> &'static str {
        match self {
            MedalRank::Gold => "#FFD700",
            MedalRank::Silver => "#C0C0C0",
            MedalRank::Bronze => "#CD7F32",
        }
    }

    fn text_color(&self) -> &'static str {
        match self {
            MedalRank::Gold => "#92400E",    // amber-800
            MedalRank::Silver => "#1E293B",  // slate-800
            MedalRank::Bronze => "#FFFFFF",  // white
        }
    }

    fn label(&self) -> &'static str {
        match self {
            MedalRank::Gold => "1",
            MedalRank::Silver => "2",
            MedalRank::Bronze => "3",
        }
    }
}

/// Medal badge component showing rank with circular medal icon
#[component]
pub fn MedalBadge(
    rank: MedalRank,
    #[prop(default = 24)] size: u32,
) -> impl IntoView {
    let color = rank.color();
    let text_color = rank.text_color();
    let label = rank.label();
    let font_size = (size as f64 * 0.5) as u32;

    view! {
        <div
            class="inline-flex items-center justify-center rounded-full font-bold shadow-sm"
            style=format!(
                "width: {}px; height: {}px; background-color: {}; color: {}; font-size: {}px;",
                size, size, color, text_color, font_size
            )
        >
            {label}
        </div>
    }
}

/// Medal badge with SVG medal icon (more elaborate design)
#[component]
pub fn MedalIcon(
    rank: MedalRank,
    #[prop(default = 24)] size: u32,
) -> impl IntoView {
    let (medal_color, ribbon_color) = match rank {
        MedalRank::Gold => ("#FFD700", "#FEF3C7"),
        MedalRank::Silver => ("#C0C0C0", "#E2E8F0"),
        MedalRank::Bronze => ("#CD7F32", "#FED7AA"),
    };

    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            class="medal-icon"
        >
            // Ribbon tails
            <path
                d="M7 1L9 7L7 14L5 8L7 1Z"
                fill=ribbon_color
            />
            <path
                d="M17 1L15 7L17 14L19 8L17 1Z"
                fill=ribbon_color
            />
            // Medal circle
            <circle
                cx="12"
                cy="15"
                r="7"
                fill=medal_color
                stroke=medal_color
                stroke-width="1"
            />
            // Medal shine
            <circle
                cx="12"
                cy="15"
                r="5"
                fill="none"
                stroke="white"
                stroke-width="0.5"
                opacity="0.5"
            />
            // Rank number
            <text
                x="12"
                y="17"
                text-anchor="middle"
                font-size="6"
                font-weight="bold"
                fill="#1E293B"
            >
                {rank.label()}
            </text>
        </svg>
    }
}
