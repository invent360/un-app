//! Task Carousel component for homepage
//!
//! Displays a horizontally scrolling carousel of summarized task cards
//! above the "How It Works" section. Fetches tasks from the CMS database.

use leptos::prelude::*;
use crate::hooks::t;

#[cfg(feature = "hydrate")]
use crate::hooks::use_locale;
#[cfg(feature = "hydrate")]
use crate::api::{get_tasks_with_preview, TasksPageResponse, TaskSection};

/// Device icon component for carousel cards
#[cfg(feature = "hydrate")]
#[component]
fn DeviceIcon(device: String) -> impl IntoView {
    let (icon_class, title) = match device.as_str() {
        "android" => ("device-icon-android", "Android"),
        "android_apk" => ("device-icon-android-apk", "Android APK"),
        "ios" => ("device-icon-ios", "iPhone"),
        "pc" => ("device-icon-pc", "PC"),
        _ => ("device-icon-android", "Device"),
    };

    view! {
        <span class=format!("carousel-device-icon {}", icon_class) title=title></span>
    }
}

/// Mini task card for the carousel - displays image, title, and earnings
#[cfg(feature = "hydrate")]
#[component]
fn CarouselTaskCard(task: TaskSection) -> impl IntoView {
    // Get the first cover image or use a placeholder
    let cover_image = task.cover_images.first().cloned().unwrap_or_default();
    let has_image = !cover_image.is_empty();

    // Get earnings summary from first tier
    let earnings_display = task.earnings_tiers.first()
        .map(|tier| {
            if tier.min_earnings == tier.max_earnings {
                format!("${:.0}/mo", tier.min_earnings)
            } else {
                format!("${:.0}-${:.0}/mo", tier.min_earnings, tier.max_earnings)
            }
        })
        .unwrap_or_else(|| "Earn ULO".to_string());

    // Status badge
    let status_class = match task.task_status.as_str() {
        "coming_soon" => "status-coming-soon",
        "on_demand" => "status-on-demand",
        _ => "status-active",
    };

    let status_text = match task.task_status.as_str() {
        "coming_soon" => "Coming Soon",
        "on_demand" => "On Demand",
        _ => "Active",
    };

    let task_url = format!("/tasks#{}", task.slug);
    let supported_devices = task.supported_devices.clone();

    view! {
        <a href=task_url class="carousel-task-card">
            <div class="carousel-card-image">
                {if has_image {
                    view! {
                        <img src=cover_image alt=task.title.clone() loading="lazy" />
                    }.into_any()
                } else {
                    view! {
                        <div class="carousel-card-placeholder">
                            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18"></rect>
                                <line x1="7" y1="2" x2="7" y2="22"></line>
                                <line x1="17" y1="2" x2="17" y2="22"></line>
                                <line x1="2" y1="12" x2="22" y2="12"></line>
                                <line x1="2" y1="7" x2="7" y2="7"></line>
                                <line x1="2" y1="17" x2="7" y2="17"></line>
                                <line x1="17" y1="17" x2="22" y2="17"></line>
                                <line x1="17" y1="7" x2="22" y2="7"></line>
                            </svg>
                        </div>
                    }.into_any()
                }}
                <span class=format!("carousel-status-badge {}", status_class)>{status_text}</span>
            </div>
            <div class="carousel-card-content">
                <h4 class="carousel-card-title">{task.title}</h4>
                <div class="carousel-card-footer">
                    <span class="earnings-badge">{earnings_display}</span>
                    <div class="carousel-device-icons">
                        {supported_devices.into_iter().map(|device| {
                            view! { <DeviceIcon device=device /> }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </a>
    }
}

/// Default response for when API fails
#[cfg(feature = "hydrate")]
fn default_tasks_response() -> TasksPageResponse {
    TasksPageResponse {
        name: "Tasks".to_string(),
        description: "".to_string(),
        content_type: "task".to_string(),
        tasks: vec![],
        is_preview: false,
    }
}

/// Client-only carousel content component
#[cfg(feature = "hydrate")]
#[component]
fn TaskCarouselContent() -> impl IntoView {
    let ctx = use_locale();

    // Scroll state for buttons
    let carousel_ref = NodeRef::<leptos::html::Div>::new();
    let (can_scroll_left, set_can_scroll_left) = signal(false);
    let (can_scroll_right, set_can_scroll_right) = signal(true);

    // Fetch tasks from database
    let tasks_resource = Resource::new(
        move || ctx.locale.get().code().to_string(),
        |locale| async move {
            match get_tasks_with_preview(locale, None).await {
                Ok(response) => {
                    leptos::logging::log!("Carousel: loaded {} tasks", response.tasks.len());
                    response
                }
                Err(e) => {
                    leptos::logging::error!("Carousel API error: {:?}", e);
                    default_tasks_response()
                }
            }
        }
    );

    // Update scroll button states
    let update_scroll_state = move || {
        if let Some(el) = carousel_ref.get() {
            let scroll_left = el.scroll_left() as f64;
            let scroll_width = el.scroll_width() as f64;
            let client_width = el.client_width() as f64;

            set_can_scroll_left.set(scroll_left > 0.0);
            set_can_scroll_right.set((scroll_left + client_width) < (scroll_width - 10.0));
        }
    };

    // Scroll handlers
    let scroll_left = move |_| {
        if let Some(el) = carousel_ref.get() {
            let current = el.scroll_left() as f64;
            let card_width = 280.0; // Card width + gap
            let _ = el.scroll_to_with_x_and_y(current - card_width * 2.0, 0.0);
        }
    };

    let scroll_right = move |_| {
        if let Some(el) = carousel_ref.get() {
            let current = el.scroll_left() as f64;
            let card_width = 280.0; // Card width + gap
            let _ = el.scroll_to_with_x_and_y(current + card_width * 2.0, 0.0);
        }
    };

    view! {
        <Suspense fallback=move || view! {
            <div class="task-carousel-loading">
                <div class="carousel-skeleton">
                    <div class="skeleton-card"></div>
                    <div class="skeleton-card"></div>
                    <div class="skeleton-card"></div>
                    <div class="skeleton-card"></div>
                </div>
            </div>
        }>
            {move || {
                let Some(response) = tasks_resource.get() else {
                    return view! { <div></div> }.into_any();
                };

                // Filter to only show active and on_demand tasks (not coming_soon in carousel)
                let visible_tasks: Vec<TaskSection> = response.tasks.into_iter()
                    .filter(|t| t.task_status != "deprecated")
                    .collect();

                if visible_tasks.is_empty() {
                    return view! { <div></div> }.into_any();
                }

                view! {
                    <div class="task-carousel-wrapper">
                        // Left scroll button
                        <button
                            class="carousel-nav-btn carousel-nav-left"
                            class:disabled=move || !can_scroll_left.get()
                            on:click=scroll_left
                            aria-label="Scroll left"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="15 18 9 12 15 6"></polyline>
                            </svg>
                        </button>

                        // Carousel container
                        <div
                            class="task-carousel-track"
                            node_ref=carousel_ref
                            on:scroll=move |_| update_scroll_state()
                        >
                            {visible_tasks.into_iter().map(|task| {
                                view! { <CarouselTaskCard task=task /> }
                            }).collect_view()}
                        </div>

                        // Right scroll button
                        <button
                            class="carousel-nav-btn carousel-nav-right"
                            class:disabled=move || !can_scroll_right.get()
                            on:click=scroll_right
                            aria-label="Scroll right"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="9 18 15 12 9 6"></polyline>
                            </svg>
                        </button>
                    </div>

                    // View all tasks link
                    <div class="carousel-view-all">
                        <a href="/tasks" class="view-all-link">
                            {move || t("home.carousel.view_all")}
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="5" y1="12" x2="19" y2="12"></line>
                                <polyline points="12 5 19 12 12 19"></polyline>
                            </svg>
                        </a>
                    </div>
                }.into_any()
            }}
        </Suspense>
    }
}

/// Task carousel section for the homepage
/// Displays above "How It Works" section
#[component]
pub fn TaskCarousel() -> impl IntoView {
    // Signal to track if we're mounted on client
    let is_mounted = RwSignal::new(false);

    // Effect runs only on client after hydration
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move || {
            is_mounted.set(true);
        });
    }

    view! {
        <section class="task-carousel-section" style="background: #1a2635;">
            <div class="container">
                <h2 class="section-title carousel-title">{move || t("home.carousel.title")}</h2>
                <p class="section-subtitle carousel-subtitle">{move || t("home.carousel.subtitle")}</p>
                <div class="task-carousel-container">
                    {move || {
                        if is_mounted.get() {
                            #[cfg(feature = "hydrate")]
                            {
                                view! { <TaskCarouselContent /> }.into_any()
                            }
                            #[cfg(not(feature = "hydrate"))]
                            {
                                view! {}.into_any()
                            }
                        } else {
                            // SSR and initial render: show skeleton
                            view! {
                                <div class="task-carousel-loading" style="padding: 2rem; text-align: center;">
                                    <p style="color: #888;">"Loading tasks..."</p>
                                    <div class="carousel-skeleton" style="display: flex; gap: 1rem; justify-content: center;">
                                        <div class="skeleton-card" style="width: 200px; height: 150px; background: #2a3a4a; border-radius: 8px;"></div>
                                        <div class="skeleton-card" style="width: 200px; height: 150px; background: #2a3a4a; border-radius: 8px;"></div>
                                        <div class="skeleton-card" style="width: 200px; height: 150px; background: #2a3a4a; border-radius: 8px;"></div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </section>
    }
}
