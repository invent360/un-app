//! CMS-driven How It Works section component

use leptos::prelude::*;
use crate::api::HomeSection;

/// Step data from CMS
#[derive(Debug, Clone)]
struct StepData {
    order: i32,
    icon: String,
    title: String,
    description: String,
}

/// Download button data from CMS
#[derive(Debug, Clone)]
struct DownloadButtonData {
    platform: String,
    link: String,
    is_visible: bool,
}

/// CMS-driven How It Works section
#[component]
pub fn CmsHowItWorksSection(
    /// Section data from CMS
    section: HomeSection,
) -> impl IntoView {
    let title = section.title.clone();
    let subtitle = section.description.clone();

    // Helper to get nested data (handles both flat and nested "data.data" structure)
    let nested_data = section.data.get("data");

    // Extract steps from section.data (check both nested and flat)
    let steps: Vec<StepData> = nested_data
        .and_then(|d| d.get("steps"))
        .or_else(|| section.data.get("steps"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            let mut steps: Vec<StepData> = arr.iter()
                .filter_map(|s| {
                    Some(StepData {
                        order: s.get("order").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                        icon: s.get("icon").and_then(|v| v.as_str()).unwrap_or("download").to_string(),
                        title: s.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        description: s.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    })
                })
                .collect();
            steps.sort_by_key(|s| s.order);
            steps
        })
        .unwrap_or_default();

    // Extract download buttons (check both nested and flat)
    let download_buttons: Vec<DownloadButtonData> = nested_data
        .and_then(|d| d.get("download_buttons"))
        .or_else(|| section.data.get("download_buttons"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|b| {
                    let is_visible = b.get("is_visible").and_then(|v| v.as_bool()).unwrap_or(true);
                    if !is_visible {
                        return None;
                    }
                    Some(DownloadButtonData {
                        platform: b.get("platform").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        link: b.get("link").and_then(|v| v.as_str()).unwrap_or("#").to_string(),
                        is_visible,
                    })
                })
                .collect()
        })
        .unwrap_or_else(|| {
            // Default download buttons if not provided
            vec![
                DownloadButtonData {
                    platform: "app_store".to_string(),
                    link: "https://apps.apple.com/gb/app/unity-network-app/id6755482738".to_string(),
                    is_visible: true,
                },
                DownloadButtonData {
                    platform: "google_play".to_string(),
                    link: "https://play.google.com/store/apps/details?id=io.unetwork.app".to_string(),
                    is_visible: true,
                },
                DownloadButtonData {
                    platform: "apk".to_string(),
                    link: "https://releases.unetwork.io/android/".to_string(),
                    is_visible: true,
                },
            ]
        });

    view! {
        <section id="how-it-works" class="section how-it-works">
            <div class="container">
                <h2 class="section-title">{title}</h2>
                <p class="section-subtitle">{subtitle}</p>

                <div class="steps">
                    {steps.into_iter().enumerate().map(|(idx, step)| {
                        let show_arrow = idx < 2; // Show arrow between steps
                        view! {
                            <>
                                <div class="step">
                                    <div class="step-number">{(step.order).to_string()}</div>
                                    <div class="step-icon">
                                        <StepIcon icon=step.icon />
                                    </div>
                                    <h3 class="step-title">{step.title}</h3>
                                    <p class="step-description">{step.description}</p>
                                </div>
                                {show_arrow.then(|| view! {
                                    <div class="step-arrow">
                                        <ArrowRightIcon/>
                                    </div>
                                })}
                            </>
                        }
                    }).collect_view()}
                </div>

                <div class="app-buttons how-it-works-buttons">
                    {download_buttons.into_iter().map(|btn| {
                        view! {
                            <DownloadButton platform=btn.platform link=btn.link />
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

/// Dynamic step icon based on icon name
#[component]
fn StepIcon(icon: String) -> impl IntoView {
    match icon.as_str() {
        "download" | "phone" => view! { <PhoneIcon/> }.into_any(),
        "play" | "lightning" | "bolt" => view! { <BoltIcon/> }.into_any(),
        "wallet" | "money" => view! { <WalletIcon/> }.into_any(),
        _ => view! { <PhoneIcon/> }.into_any(),
    }
}

/// Download button component
#[component]
fn DownloadButton(platform: String, link: String) -> impl IntoView {
    let (icon_view, sublabel, label) = match platform.as_str() {
        "app_store" => (
            view! { <AppleIcon/> }.into_any(),
            "Download on the",
            "App Store",
        ),
        "google_play" => (
            view! { <PlayStoreIcon/> }.into_any(),
            "Get it on",
            "Google Play",
        ),
        "apk" => (
            view! { <ApkIcon/> }.into_any(),
            "Direct download",
            "APK File",
        ),
        _ => (
            view! { <ApkIcon/> }.into_any(),
            "Download",
            "App",
        ),
    };

    view! {
        <a href={link} target="_blank" rel="noopener noreferrer" class="app-button">
            {icon_view}
            <div class="app-button-text">
                <span class="small">{sublabel}</span>
                <span class="large">{label}</span>
            </div>
        </a>
    }
}

// Icon components

#[component]
fn PhoneIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-phone" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="5" y="2" width="14" height="20" rx="2" ry="2"></rect>
            <line x1="12" y1="18" x2="12.01" y2="18"></line>
        </svg>
    }
}

#[component]
fn BoltIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-bolt" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
        </svg>
    }
}

#[component]
fn WalletIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-wallet" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12V7H5a2 2 0 0 1 0-4h14v4"></path>
            <path d="M3 5v14a2 2 0 0 0 2 2h16v-5"></path>
            <path d="M18 12a2 2 0 0 0 0 4h4v-4Z"></path>
        </svg>
    }
}

#[component]
fn ArrowRightIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="5" y1="12" x2="19" y2="12"></line>
            <polyline points="12 5 19 12 12 19"></polyline>
        </svg>
    }
}

#[component]
fn AppleIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-apple" viewBox="0 0 24 24" fill="currentColor">
            <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"></path>
        </svg>
    }
}

#[component]
fn PlayStoreIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-playstore" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 20.5v-17c0-.59.34-1.11.84-1.35L13.69 12l-9.85 9.85c-.5-.25-.84-.76-.84-1.35zm13.81-5.38L6.05 21.34l8.49-8.49 2.27 2.27zm3.35-4.31c.34.27.59.69.59 1.19s-.22.9-.57 1.18l-2.29 1.32-2.5-2.5 2.5-2.5 2.27 1.31zM6.05 2.66l10.76 6.22-2.27 2.27L6.05 2.66z"></path>
        </svg>
    }
}

#[component]
fn ApkIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-apk" viewBox="0 0 24 24" fill="currentColor">
            <path d="M5 16c0 3.87 3.13 7 7 7s7-3.13 7-7v-4H5v4zM16.12 4.37l2.1-2.1-.82-.83-2.3 2.31C14.16 3.28 13.12 3 12 3s-2.16.28-3.09.75L6.6 1.44l-.82.83 2.1 2.1C6.14 5.64 5 7.68 5 10v1h14v-1c0-2.32-1.14-4.36-2.88-5.63zM9 9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z"/>
        </svg>
    }
}
