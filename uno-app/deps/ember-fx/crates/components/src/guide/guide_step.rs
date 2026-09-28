//! Guide step content renderer.

use leptos::prelude::*;
use super::types::GuideStep;
use crate::try_use_theme;

/// Renders a single guide step's content.
#[component]
pub fn GuideStepView(
    /// The step data to render.
    #[prop(into)]
    step: GuideStep,
    /// Whether this step is currently active.
    #[prop(optional)]
    is_active: bool,
    /// Show the step image.
    #[prop(optional)]
    #[prop(default = true)]
    show_image: bool,
    /// Show the step title.
    #[prop(optional)]
    #[prop(default = true)]
    show_title: bool,
    /// Image position relative to content.
    #[prop(optional)]
    image_position: Option<ImagePosition>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = StoredValue::new(format!("fx-guide-{}", design_system));
    let image_position = image_position.unwrap_or_default();

    let combined_class = {
        let p = prefix.get_value();
        let mut parts = vec![
            format!("{}-step", p),
            image_position.class(&p),
        ];
        if is_active {
            parts.push("active".to_string());
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let title = step.title.clone();
    let title_for_alt = title.clone();
    let title_for_header = title.clone();
    let description = step.description.clone();
    let image = step.image.clone();
    let video = step.video.clone();

    view! {
        <div class=combined_class>
            // Media section (image or video)
            {move || {
                let p = prefix.get_value();
                if let Some(ref video_url) = video {
                    view! {
                        <div class=format!("{}-step-media", p)>
                            <video
                                src=video_url.clone()
                                class=format!("{}-step-video", p)
                                controls
                                playsinline
                            />
                        </div>
                    }.into_any()
                } else if show_image {
                    if let Some(ref image_url) = image {
                        view! {
                            <div class=format!("{}-step-media", p)>
                                <img
                                    src=image_url.clone()
                                    alt=title_for_alt.clone()
                                    class=format!("{}-step-image", p)
                                    loading="lazy"
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    }
                } else {
                    view! { <div /> }.into_any()
                }
            }}

            // Content section
            <div class=format!("{}-step-content", prefix.get_value())>
                {if show_title {
                    let p = prefix.get_value();
                    view! {
                        <h3 class=format!("{}-step-title", p)>
                            {title_for_header.clone()}
                        </h3>
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }}

                <div class=format!("{}-step-description", prefix.get_value())>
                    // Render description with basic markdown support
                    {render_markdown(&description)}
                </div>
            </div>
        </div>
    }
}

/// Image position relative to content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImagePosition {
    /// Image above content
    #[default]
    Top,
    /// Image below content
    Bottom,
    /// Image on the left
    Left,
    /// Image on the right
    Right,
}

impl ImagePosition {
    /// Get the CSS class suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            ImagePosition::Top => "image-top",
            ImagePosition::Bottom => "image-bottom",
            ImagePosition::Left => "image-left",
            ImagePosition::Right => "image-right",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Simple markdown renderer for descriptions.
fn render_markdown(text: &str) -> impl IntoView {
    // Split by double newlines for paragraphs
    let paragraphs: Vec<&str> = text.split("\n\n").collect();

    view! {
        <div class="markdown-content">
            {paragraphs.into_iter().map(|p| {
                // Handle bullet points
                if p.trim().starts_with("- ") || p.trim().starts_with("* ") {
                    let items: Vec<&str> = p.lines()
                        .filter(|l| l.trim().starts_with("- ") || l.trim().starts_with("* "))
                        .map(|l| l.trim().trim_start_matches("- ").trim_start_matches("* "))
                        .collect();

                    view! {
                        <ul>
                            {items.into_iter().map(|item| {
                                view! { <li>{render_inline_markdown(item)}</li> }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }
                // Handle numbered lists
                else if p.trim().chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
                    let items: Vec<&str> = p.lines()
                        .filter(|l| {
                            let trimmed = l.trim();
                            trimmed.chars().next().map(|c| c.is_numeric()).unwrap_or(false)
                        })
                        .map(|l| {
                            // Remove "1. " prefix
                            l.trim().split_once(". ").map(|(_, rest)| rest).unwrap_or(l)
                        })
                        .collect();

                    view! {
                        <ol>
                            {items.into_iter().map(|item| {
                                view! { <li>{render_inline_markdown(item)}</li> }
                            }).collect_view()}
                        </ol>
                    }.into_any()
                }
                // Regular paragraph
                else {
                    view! {
                        <p>{render_inline_markdown(p)}</p>
                    }.into_any()
                }
            }).collect_view()}
        </div>
    }
}

/// Render inline markdown (bold, italic, code).
fn render_inline_markdown(text: &str) -> impl IntoView {
    // For now, just return the text as-is
    // A full implementation would parse **bold**, *italic*, `code`, etc.
    view! { <span inner_html=text.replace("\\n", "<br/>") /> }
}
