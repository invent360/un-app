//! Task categories information page

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::common::PreviewBanner;

#[cfg(feature = "hydrate")]
use crate::components::tasks::{TaskList, TaskDataSource};
#[cfg(feature = "hydrate")]
use crate::hooks::{t, use_locale};
#[cfg(feature = "hydrate")]
use crate::api::{get_tasks_with_preview, TasksPageResponse, TaskSection};
#[cfg(feature = "hydrate")]
use ember_fx_components::{TaskCardSize, TaskCardLayout, TaskData, TaskStatus, TaskDifficulty, EarningsTier, EarningsPeriod};

/// Convert a TaskSection from the API to TaskData for the UI component
#[cfg(feature = "hydrate")]
fn convert_task_section_to_task_data(section: &TaskSection) -> TaskData {
    let mut task = TaskData::new(
        &section.slug,
        &section.slug,
        &section.title,
        &section.description,
    );

    // Add cover images
    for (i, img) in section.cover_images.iter().enumerate() {
        if i == 0 {
            task = task.image(img);
        } else {
            task = task.add_image(img);
        }
    }

    // Set status
    task = match section.task_status.as_str() {
        "active" => task.status(TaskStatus::Active),
        "coming_soon" => task.status(TaskStatus::ComingSoon),
        "on_demand" => task.status(TaskStatus::Active),
        _ => task.status(TaskStatus::Active),
    };

    // Set task type (active/passive)
    if !section.task_type.is_empty() {
        task = task.task_type(section.task_type.as_str());
    }

    // Set difficulty
    if !section.difficulty.is_empty() {
        task = match section.difficulty.as_str() {
            "easy" => task.difficulty(TaskDifficulty::Easy),
            "medium" => task.difficulty(TaskDifficulty::Medium),
            "hard" => task.difficulty(TaskDifficulty::Hard),
            _ => task.difficulty(TaskDifficulty::Easy),
        };
    }

    // Set duration
    if !section.duration.is_empty() {
        task = task.duration(&section.duration);
    }

    // Add earnings tiers
    for tier_data in &section.earnings_tiers {
        let period_enum = match tier_data.period.as_str() {
            "day" => EarningsPeriod::Day,
            "week" => EarningsPeriod::Week,
            _ => EarningsPeriod::Month,
        };

        let mut tier = EarningsTier::new(&tier_data.name, tier_data.min_earnings, tier_data.max_earnings)
            .period(period_enum);

        if tier_data.is_popular {
            tier = tier.popular();
        }

        for feature in &tier_data.features {
            tier = tier.feature(feature);
        }

        task = task.tier(tier);
    }

    // Add requirements
    for req in &section.requirements {
        task = task.requirement(req);
    }

    task
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

/// Client-only tasks content component - only rendered after hydration
#[cfg(feature = "hydrate")]
#[component]
fn TasksContent(
    /// Preview token from query string
    preview_token: Option<String>,
    /// Callback to set preview mode state
    set_is_preview: WriteSignal<bool>,
) -> impl IntoView {
    let ctx = use_locale();
    let token = preview_token.clone();

    // Fetch tasks from database with optional preview support
    let tasks_resource = Resource::new(
        move || (ctx.locale.get().code().to_string(), token.clone()),
        |(locale, token)| async move {
            match get_tasks_with_preview(locale, token).await {
                Ok(response) => {
                    leptos::logging::log!("Tasks API success: name='{}', tasks={}", response.name, response.tasks.len());
                    response
                }
                Err(e) => {
                    leptos::logging::error!("Tasks API error: {:?}", e);
                    default_tasks_response()
                }
            }
        }
    );

    // Update preview state when resource loads
    Effect::new(move || {
        if let Some(response) = tasks_resource.get() {
            set_is_preview.set(response.is_preview);
        }
    });

    view! {
        <Suspense fallback=move || view! {
            // Loading state - no header, just spinner
            <section class="tasks-section">
                <div class="loading-state" style="text-align: center; padding: 3rem;">
                    <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                </div>
                <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
            </section>
        }>
            {move || {
                // Check if resource is still loading
                let Some(response) = tasks_resource.get() else {
                    // Still loading - show spinner only
                    return view! {
                        <section class="tasks-section">
                            <div class="loading-state" style="text-align: center; padding: 3rem;">
                                <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                            </div>
                            <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                        </section>
                    }.into_any();
                };

                let page_name = response.name.clone();
                let page_description = response.description.clone();

                // Convert TaskSections to TaskData for the UI - no fallback, use CMS data only
                let task_data: Vec<TaskData> = response.tasks.iter()
                    .map(convert_task_section_to_task_data)
                    .collect();

                // Show empty state if no tasks from CMS - only "Coming Soon", no header
                if task_data.is_empty() {
                    return view! {
                        <section class="tasks-section">
                            <div class="empty-state" style="text-align: center; padding: 3rem;">
                                <p class="placeholder-text">{move || t("tasks.coming_soon")}</p>
                            </div>
                        </section>
                    }.into_any();
                }

                view! {
                    // Page header from API response
                    <header class="page-header">
                        <h1>{page_name}</h1>
                        <p>{page_description}</p>
                    </header>

                    <section class="tasks-section">
                        <TaskList
                            source=TaskDataSource::Static(task_data)
                            size=TaskCardSize::Medium
                            layout=TaskCardLayout::Stacked
                            columns=3
                            items_per_page=6
                            show_pagination=true
                            summary_mode=true
                            show_earnings=true
                            show_requirements=false
                        />
                    </section>
                }.into_any()
            }}
        </Suspense>
    }
}

/// Tasks content view - uses client-only pattern to avoid hydration mismatch
fn tasks_content_view(
    preview_token: Option<String>,
    set_is_preview: WriteSignal<bool>,
) -> impl IntoView {
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
        // Container div - consistent between SSR and client
        <div class="tasks-dynamic-container">
            {move || {
                if is_mounted.get() {
                    // Client-only: render the actual tasks content
                    #[cfg(feature = "hydrate")]
                    {
                        view! { <TasksContent preview_token=preview_token.clone() set_is_preview=set_is_preview /> }.into_any()
                    }
                    #[cfg(not(feature = "hydrate"))]
                    {
                        view! {}.into_any()
                    }
                } else {
                    // SSR and initial client render: show loading state with spinner
                    view! {
                        <div class="loading-state" style="text-align: center; padding: 3rem;">
                            <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                        </div>
                        <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Tasks page component
#[component]
pub fn TasksPage() -> impl IntoView {
    let query = use_query_map();

    // Extract preview_token from query string - use get_untracked for initial value
    let preview_token = query.get_untracked().get("preview_token").map(|s| s.to_string());

    // Track if we're in preview mode
    let (is_preview, set_is_preview) = signal(preview_token.is_some());

    view! {
        <div class="page tasks-page">
            // Show preview banner if in preview mode
            {move || is_preview.get().then(|| view! { <PreviewBanner /> })}

            // Page header and content come from API via TasksContent
            {tasks_content_view(preview_token.clone(), set_is_preview)}
        </div>
    }
}
