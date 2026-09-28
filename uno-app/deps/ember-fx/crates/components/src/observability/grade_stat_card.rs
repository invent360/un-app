//! GradeStatCard Leptos component.
//!
//! A stat card with large value display and A/B/C/D grade chips.

use leptos::prelude::*;
use crate::try_use_theme;

/// Grade level for the stat card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradeLevel {
    A,
    B,
    C,
    D,
    #[default]
    None,
}

impl GradeLevel {
    /// Returns the grade letter.
    pub fn letter(&self) -> &'static str {
        match self {
            GradeLevel::A => "A",
            GradeLevel::B => "B",
            GradeLevel::C => "C",
            GradeLevel::D => "D",
            GradeLevel::None => "",
        }
    }
}

/// Theme color preset for the card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradeCardTheme {
    #[default]
    Orange,
    Purple,
    Pink,
    Coral,
    Blue,
    Green,
    Custom,
}

impl GradeCardTheme {
    /// Returns the primary color for the theme.
    pub fn primary_color(&self) -> &'static str {
        match self {
            GradeCardTheme::Orange => "#e8a54b",
            GradeCardTheme::Purple => "#9b87d6",
            GradeCardTheme::Pink => "#d977c8",
            GradeCardTheme::Coral => "#c67c5e",
            GradeCardTheme::Blue => "#5b9bd5",
            GradeCardTheme::Green => "#5bc77e",
            GradeCardTheme::Custom => "#888888",
        }
    }

    /// Returns the background color for the theme.
    pub fn background_color(&self) -> &'static str {
        match self {
            GradeCardTheme::Orange => "rgba(232, 165, 75, 0.15)",
            GradeCardTheme::Purple => "rgba(155, 135, 214, 0.15)",
            GradeCardTheme::Pink => "rgba(217, 119, 200, 0.15)",
            GradeCardTheme::Coral => "rgba(198, 124, 94, 0.15)",
            GradeCardTheme::Blue => "rgba(91, 155, 213, 0.15)",
            GradeCardTheme::Green => "rgba(91, 199, 126, 0.15)",
            GradeCardTheme::Custom => "rgba(136, 136, 136, 0.15)",
        }
    }

    /// Returns the chip background color for unselected chips.
    pub fn chip_bg(&self) -> &'static str {
        match self {
            GradeCardTheme::Orange => "rgba(232, 165, 75, 0.3)",
            GradeCardTheme::Purple => "rgba(155, 135, 214, 0.3)",
            GradeCardTheme::Pink => "rgba(217, 119, 200, 0.3)",
            GradeCardTheme::Coral => "rgba(198, 124, 94, 0.3)",
            GradeCardTheme::Blue => "rgba(91, 155, 213, 0.3)",
            GradeCardTheme::Green => "rgba(91, 199, 126, 0.3)",
            GradeCardTheme::Custom => "rgba(136, 136, 136, 0.3)",
        }
    }
}

/// Configuration for the grade stat card.
#[derive(Debug, Clone)]
pub struct GradeStatCardConfig {
    /// Show grade chips.
    pub show_grades: bool,
    /// Border radius in pixels.
    pub border_radius: u32,
    /// Custom primary color (overrides theme).
    pub custom_color: Option<String>,
}

impl Default for GradeStatCardConfig {
    fn default() -> Self {
        Self {
            show_grades: true,
            border_radius: 12,
            custom_color: None,
        }
    }
}

/// GradeStatCard component.
///
/// A stat card displaying a large value, title, subtitle, and A/B/C/D grade chips.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{GradeStatCard, GradeLevel, GradeCardTheme};
///
/// view! {
///     <GradeStatCard
///         value=Signal::derive(|| 20)
///         title="Assessment Scores".to_string()
///         grade=GradeLevel::B
///         theme=GradeCardTheme::Orange
///     />
/// }
/// ```
#[component]
pub fn GradeStatCard(
    /// The main value to display.
    #[prop(into)]
    value: Signal<i32>,
    /// Card title.
    #[prop(into)]
    title: String,
    /// Optional subtitle.
    #[prop(optional)]
    subtitle: Option<String>,
    /// Selected grade level.
    #[prop(optional)]
    grade: GradeLevel,
    /// Theme color.
    #[prop(optional)]
    theme: GradeCardTheme,
    /// Configuration.
    #[prop(optional)]
    config: Option<GradeStatCardConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let show_grades = config.show_grades;
    let border_radius = config.border_radius;
    let custom_color = config.custom_color.clone();

    let prefix = format!("fx-grade-stat-card-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Determine colors
    let primary_color = custom_color.clone().unwrap_or_else(|| theme.primary_color().to_string());
    let bg_color = theme.background_color();
    let chip_bg = theme.chip_bg();

    let grades = vec![GradeLevel::A, GradeLevel::B, GradeLevel::C, GradeLevel::D];

    view! {
        <div
            class=combined_class
            style=format!(
                "background: {}; border-radius: {}px; padding: 16px 20px; display: flex; align-items: center; justify-content: space-between; min-width: 200px;",
                bg_color, border_radius
            )
        >
            // Left side: value and title
            <div style="display: flex; align-items: center; gap: 12px;">
                // Large value
                <div style=format!(
                    "font-size: 36px; font-weight: 600; color: {}; line-height: 1;",
                    primary_color
                )>
                    {move || value.get()}
                </div>

                // Title and subtitle
                <div style="display: flex; flex-direction: column; gap: 2px;">
                    <div style=format!(
                        "font-size: 14px; font-weight: 500; color: {};",
                        primary_color
                    )>
                        {title.clone()}
                    </div>
                    {subtitle.clone().map(|sub| {
                        view! {
                            <div style=format!(
                                "font-size: 12px; color: {}; opacity: 0.8;",
                                primary_color
                            )>
                                {sub}
                            </div>
                        }
                    })}
                </div>
            </div>

            // Right side: grade chips
            {if show_grades {
                let primary_color = primary_color.clone();
                let chip_bg = chip_bg.to_string();
                Some(view! {
                    <div style="display: flex; gap: 4px;">
                        {grades.into_iter().map(|g| {
                            let is_selected = g == grade;
                            let bg = if is_selected {
                                primary_color.clone()
                            } else {
                                chip_bg.clone()
                            };
                            let text_color = if is_selected {
                                "#fff"
                            } else {
                                &primary_color
                            };
                            view! {
                                <div style=format!(
                                    "width: 24px; height: 24px; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 600; background: {}; color: {};",
                                    bg, text_color
                                )>
                                    {g.letter()}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

/// A row of GradeStatCards displayed vertically.
#[component]
pub fn GradeStatCardStack(
    /// The cards to display.
    #[prop(into)]
    cards: Signal<Vec<GradeStatCardData>>,
    /// Gap between cards in pixels.
    #[prop(optional, default = 12)]
    gap: u32,
) -> impl IntoView {
    view! {
        <div style=format!("display: flex; flex-direction: column; gap: {}px;", gap)>
            {move || cards.get().into_iter().map(|card| {
                let value = card.value;
                let title = card.title.clone();
                let grade = card.grade;
                let theme = card.theme;

                if let Some(subtitle) = card.subtitle.clone() {
                    view! {
                        <GradeStatCard
                            value=Signal::derive(move || value)
                            title=title
                            subtitle=subtitle
                            grade=grade
                            theme=theme
                        />
                    }.into_any()
                } else {
                    view! {
                        <GradeStatCard
                            value=Signal::derive(move || value)
                            title=title
                            grade=grade
                            theme=theme
                        />
                    }.into_any()
                }
            }).collect_view()}
        </div>
    }
}

/// Data for a single grade stat card.
#[derive(Debug, Clone)]
pub struct GradeStatCardData {
    /// The value to display.
    pub value: i32,
    /// Card title.
    pub title: String,
    /// Optional subtitle.
    pub subtitle: Option<String>,
    /// Selected grade.
    pub grade: GradeLevel,
    /// Theme color.
    pub theme: GradeCardTheme,
}

impl GradeStatCardData {
    /// Create a new card data.
    pub fn new(value: i32, title: impl Into<String>) -> Self {
        Self {
            value,
            title: title.into(),
            subtitle: None,
            grade: GradeLevel::None,
            theme: GradeCardTheme::default(),
        }
    }

    /// Set subtitle.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set grade.
    pub fn grade(mut self, grade: GradeLevel) -> Self {
        self.grade = grade;
        self
    }

    /// Set theme.
    pub fn theme(mut self, theme: GradeCardTheme) -> Self {
        self.theme = theme;
        self
    }
}
