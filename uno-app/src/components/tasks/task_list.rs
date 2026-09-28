//! TaskList component with static and dynamic data source support.
//!
//! Supports two modes:
//! - Static: Uses provided tasks directly
//! - Dynamic: Fetches tasks from a JSON endpoint
//!
//! Features:
//! - Pagination support with configurable items per page

use leptos::prelude::*;
use ember_fx_components::{
    TaskCardGrid, TaskData,
    TaskCardSize, TaskCardLayout,
};

/// Data source for tasks
#[derive(Debug, Clone, PartialEq)]
pub enum TaskDataSource {
    /// Static tasks provided directly
    Static(Vec<TaskData>),
    /// Dynamic tasks fetched from JSON endpoint
    Dynamic(String),
}

impl Default for TaskDataSource {
    fn default() -> Self {
        TaskDataSource::Static(Vec::new())
    }
}

/// TaskList component displaying tasks from static or dynamic sources.
///
/// # Example (Static with Pagination)
///
/// ```ignore
/// view! {
///     <TaskList
///         source=TaskDataSource::Static(my_tasks)
///         items_per_page=3
///         show_pagination=true
///     />
/// }
/// ```
///
/// # Example (Dynamic)
///
/// ```ignore
/// view! {
///     <TaskList
///         source=TaskDataSource::Dynamic("/api/v1/contents/task".to_string())
///         items_per_page=6
///     />
/// }
/// ```
#[component]
pub fn TaskList(
    /// Data source: Static(tasks) or Dynamic(url)
    #[prop(optional, into)]
    source: Option<TaskDataSource>,
    /// Card size variant
    #[prop(optional, into)]
    size: Option<TaskCardSize>,
    /// Card layout style
    #[prop(optional, into)]
    layout: Option<TaskCardLayout>,
    /// Number of grid columns
    #[prop(optional)]
    #[prop(default = 3)]
    columns: u8,
    /// Items per page (0 = no pagination, show all)
    #[prop(optional)]
    #[prop(default = 0)]
    items_per_page: usize,
    /// Show pagination controls
    #[prop(optional)]
    #[prop(default = true)]
    show_pagination: bool,
    /// Summary mode - show condensed cards with "More" button
    #[prop(optional)]
    summary_mode: bool,
    /// Show toggle button label ("More"/"Less") - if false, shows icon only
    #[prop(optional)]
    #[prop(default = false)]
    show_toggle_label: bool,
    /// Show earnings tiers
    #[prop(optional)]
    #[prop(default = true)]
    show_earnings: bool,
    /// Show requirements
    #[prop(optional)]
    show_requirements: bool,
) -> impl IntoView {
    let source = source.unwrap_or_default();
    let size = size.unwrap_or(TaskCardSize::Medium);
    let layout = layout.unwrap_or(TaskCardLayout::Stacked);

    match source {
        TaskDataSource::Static(tasks) => {
            view! {
                <StaticTaskList
                    tasks=tasks
                    size=size
                    layout=layout
                    columns=columns
                    items_per_page=items_per_page
                    show_pagination=show_pagination
                    summary_mode=summary_mode
                    show_toggle_label=show_toggle_label
                    show_earnings=show_earnings
                    show_requirements=show_requirements
                />
            }.into_any()
        }
        TaskDataSource::Dynamic(url) => {
            view! {
                <DynamicTaskList
                    url=url
                    size=size
                    layout=layout
                    columns=columns
                    items_per_page=items_per_page
                    show_pagination=show_pagination
                    summary_mode=summary_mode
                    show_toggle_label=show_toggle_label
                    show_earnings=show_earnings
                    show_requirements=show_requirements
                />
            }.into_any()
        }
    }
}

/// Static task list - renders tasks directly
#[component]
fn StaticTaskList(
    tasks: Vec<TaskData>,
    size: TaskCardSize,
    layout: TaskCardLayout,
    columns: u8,
    items_per_page: usize,
    show_pagination: bool,
    summary_mode: bool,
    show_toggle_label: bool,
    show_earnings: bool,
    show_requirements: bool,
) -> impl IntoView {
    view! {
        <div class="task-list task-list-static">
            <PaginatedTaskGrid
                tasks=tasks
                size=size
                layout=layout
                columns=columns
                items_per_page=items_per_page
                show_pagination=show_pagination
                summary_mode=summary_mode
                show_toggle_label=show_toggle_label
            />
        </div>
    }
}

/// Paginated task grid with navigation controls
#[component]
fn PaginatedTaskGrid(
    tasks: Vec<TaskData>,
    size: TaskCardSize,
    layout: TaskCardLayout,
    columns: u8,
    items_per_page: usize,
    show_pagination: bool,
    summary_mode: bool,
    show_toggle_label: bool,
) -> impl IntoView {
    let total_tasks = tasks.len();

    // If no pagination or all tasks fit on one page, render without pagination
    if items_per_page == 0 || total_tasks <= items_per_page {
        return view! {
            <TaskCardGrid
                tasks=tasks
                size=size
                layout=layout
                columns=columns
                summary_mode=summary_mode
                show_toggle_label=show_toggle_label
            />
        }.into_any();
    }

    // Calculate total pages
    let total_pages = (total_tasks + items_per_page - 1) / items_per_page;

    // Current page state (0-indexed)
    let (current_page, set_current_page) = signal(0usize);

    // Store tasks in a signal for reactive access
    let tasks = StoredValue::new(tasks);

    // Get current page tasks
    let current_tasks = move || {
        let page = current_page.get();
        let start = page * items_per_page;
        let end = (start + items_per_page).min(total_tasks);
        tasks.get_value()[start..end].to_vec()
    };

    // Navigation handlers
    let go_prev = move |_| {
        set_current_page.update(|p| {
            if *p > 0 {
                *p -= 1;
            }
        });
    };

    let go_next = move |_| {
        set_current_page.update(|p| {
            if *p < total_pages - 1 {
                *p += 1;
            }
        });
    };

    let go_to_page = move |page: usize| {
        set_current_page.set(page);
    };

    // Computed classes for buttons
    let prev_class = move || {
        if current_page.get() == 0 {
            "pagination-btn pagination-prev disabled"
        } else {
            "pagination-btn pagination-prev"
        }
    };

    let next_class = move || {
        if current_page.get() >= total_pages - 1 {
            "pagination-btn pagination-next disabled"
        } else {
            "pagination-btn pagination-next"
        }
    };

    view! {
        <div class="paginated-task-grid">
            {move || {
                view! {
                    <TaskCardGrid
                        tasks=current_tasks()
                        size=size
                        layout=layout
                        columns=columns
                        summary_mode=summary_mode
                        show_toggle_label=show_toggle_label
                    />
                }
            }}

            {move || {
                if show_pagination && total_pages > 1 {
                    Some(view! {
                        <nav class="pagination" aria-label="Task pagination">
                            <button
                                class=prev_class
                                on:click=go_prev
                                aria-label="Previous page"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2">
                                    <polyline points="15 18 9 12 15 6"></polyline>
                                </svg>
                            </button>

                            <div class="pagination-dots">
                                {(0..total_pages).map(|i| {
                                    let dot_class = move || {
                                        if current_page.get() == i {
                                            "pagination-dot active"
                                        } else {
                                            "pagination-dot"
                                        }
                                    };
                                    view! {
                                        <button
                                            class=dot_class
                                            on:click=move |_| go_to_page(i)
                                            aria-label=format!("Go to page {}", i + 1)
                                        >
                                            <span class="sr-only">{i + 1}</span>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            <button
                                class=next_class
                                on:click=go_next
                                aria-label="Next page"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2">
                                    <polyline points="9 18 15 12 9 6"></polyline>
                                </svg>
                            </button>

                            <span class="pagination-info">
                                {move || format!("{} / {}", current_page.get() + 1, total_pages)}
                            </span>
                        </nav>
                    })
                } else {
                    None
                }
            }}
        </div>
    }.into_any()
}

/// Dynamic task list - fetches tasks from JSON endpoint
#[component]
fn DynamicTaskList(
    url: String,
    size: TaskCardSize,
    layout: TaskCardLayout,
    columns: u8,
    items_per_page: usize,
    show_pagination: bool,
    summary_mode: bool,
    show_toggle_label: bool,
    show_earnings: bool,
    show_requirements: bool,
) -> impl IntoView {
    // Create a resource to fetch tasks
    let tasks_resource = Resource::new(
        move || url.clone(),
        |url| async move {
            fetch_tasks(&url).await
        }
    );

    view! {
        <div class="task-list task-list-dynamic">
            <Suspense fallback=move || view! {
                <div class="task-list-loading">
                    <div class="loading-spinner"></div>
                    <p>"Loading tasks..."</p>
                </div>
            }>
                {move || {
                    tasks_resource.get().map(|result| {
                        match result {
                            Ok(tasks) => {
                                view! {
                                    <PaginatedTaskGrid
                                        tasks=tasks
                                        size=size
                                        layout=layout
                                        columns=columns
                                        items_per_page=items_per_page
                                        show_pagination=show_pagination
                                        summary_mode=summary_mode
                                        show_toggle_label=show_toggle_label
                                    />
                                }.into_any()
                            }
                            Err(e) => {
                                view! {
                                    <div class="task-list-error">
                                        <p class="error-message">{format!("Failed to load tasks: {}", e)}</p>
                                        <button
                                            class="retry-button"
                                            on:click=move |_| {
                                                tasks_resource.refetch();
                                            }
                                        >
                                            "Retry"
                                        </button>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

/// Fetch tasks from JSON endpoint
#[cfg(feature = "csr")]
async fn fetch_tasks(url: &str) -> Result<Vec<TaskData>, String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, Response};

    let window = web_sys::window().ok_or("No window")?;

    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Request error: {:?}", e))?;

    request.headers()
        .set("Accept", "application/json")
        .map_err(|e| format!("Header error: {:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch error: {:?}", e))?;

    let resp: Response = resp_value.dyn_into()
        .map_err(|_| "Response cast error")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let json = JsFuture::from(resp.json().map_err(|e| format!("JSON parse error: {:?}", e))?)
        .await
        .map_err(|e| format!("JSON error: {:?}", e))?;

    let tasks: Vec<TaskData> = serde_wasm_bindgen::from_value(json)
        .map_err(|e| format!("Deserialize error: {:?}", e))?;

    Ok(tasks)
}

#[cfg(not(feature = "csr"))]
async fn fetch_tasks(_url: &str) -> Result<Vec<TaskData>, String> {
    // Server-side: return empty - content should come from CMS
    Ok(Vec::new())
}

