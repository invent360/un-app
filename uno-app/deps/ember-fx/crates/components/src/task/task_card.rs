//! TaskCard component for displaying CMS-driven task content.
//!
//! Displays a task with image gallery, title, description, earnings tiers,
//! and status badges (active/coming soon).

use leptos::prelude::*;
use super::types::{TaskData, TaskCardSize, TaskCardVariant, TaskCardLayout, TaskStatus, TaskType};
use crate::try_use_theme;

/// TaskCard component for displaying task content.
///
/// # Props
///
/// - `task` - Task data to display
/// - `size` - Card size (Small, Medium, Large)
/// - `variant` - Display variant (Card, Compact, Featured)
/// - `layout` - Layout style (Stacked, SideBySide)
/// - `summary_mode` - Show condensed view with "More" button to expand
/// - `show_earnings` - Whether to show earnings tiers
/// - `show_requirements` - Whether to show requirements
/// - `show_difficulty` - Whether to show difficulty badge
/// - `on_click` - Callback when card is clicked
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::{TaskCard, TaskData, EarningsTier, TaskCardLayout};
///
/// let task = TaskData::new("1", "telemetry", "Telemetry Collection", "Share anonymous data")
///     .add_image("/images/telemetry1.png")
///     .add_image("/images/telemetry2.png")
///     .gallery_interval(3000)
///     .tier(EarningsTier::new("1 Device", 5.0, 8.0))
///     .tier(EarningsTier::new("2-3 Devices", 15.0, 25.0).popular());
///
/// // Summary mode - shows condensed view with "More" button
/// view! {
///     <TaskCard task=task summary_mode=true />
/// }
///
/// // Full view
/// view! {
///     <TaskCard task=task layout=TaskCardLayout::SideBySide show_earnings=true />
/// }
/// ```
#[component]
pub fn TaskCard(
    /// Task data to display.
    #[prop(into)]
    task: TaskData,
    /// Card size variant.
    #[prop(optional, into)]
    size: Option<TaskCardSize>,
    /// Display variant.
    #[prop(optional, into)]
    variant: Option<TaskCardVariant>,
    /// Layout style.
    #[prop(optional, into)]
    layout: Option<TaskCardLayout>,
    /// Summary mode - show condensed view with expand button.
    #[prop(optional)]
    summary_mode: bool,
    /// Whether to show earnings tiers.
    #[prop(optional)]
    #[prop(default = true)]
    show_earnings: bool,
    /// Whether to show requirements list.
    #[prop(optional)]
    show_requirements: bool,
    /// Whether to show difficulty badge.
    #[prop(optional)]
    #[prop(default = true)]
    show_difficulty: bool,
    /// Whether to show task type badge (Active/Passive).
    #[prop(optional)]
    #[prop(default = true)]
    show_task_type: bool,
    /// Whether to show duration.
    #[prop(optional)]
    #[prop(default = true)]
    show_duration: bool,
    /// Show toggle button label ("More"/"Less") - if false, shows icon only.
    #[prop(optional)]
    #[prop(default = true)]
    show_toggle_label: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click callback.
    #[prop(optional, into)]
    on_click: Option<Callback<TaskData>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    let layout = layout.unwrap_or_default();

    // Expanded state for summary mode
    let (is_expanded, set_is_expanded) = signal(false);

    // Store task for closure
    let task = StoredValue::new(task);

    // Get all images for gallery
    let images = task.with_value(|t| t.all_images());
    let has_gallery = images.len() > 1;
    let gallery_interval = task.with_value(|t| t.gallery_interval);

    // Gallery state - current active image index
    let active_index = RwSignal::new(0usize);
    let image_count = images.len();

    // Auto-advance timer for gallery (client-side only)
    #[cfg(target_arch = "wasm32")]
    if has_gallery {
        let interval_ms = gallery_interval as i64;
        Effect::new(move |_| {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let window = web_sys::window().expect("no window");
            let cb = Closure::wrap(Box::new(move || {
                active_index.update(|idx| {
                    *idx = (*idx + 1) % image_count;
                });
            }) as Box<dyn Fn()>);

            let id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    interval_ms as i32,
                )
                .expect("set_interval failed");

            // Keep closure alive
            cb.forget();

            // Cleanup on dispose
            on_cleanup(move || {
                if let Some(window) = web_sys::window() {
                    window.clear_interval_with_handle(id);
                }
            });
        });
    }

    // Suppress unused variable warnings on non-wasm targets
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (has_gallery, gallery_interval, active_index, image_count);

    // Build CSS class prefix
    let prefix = format!("fx-task-card-{}", design_system);

    // Build root class
    let root_class = {
        let prefix = prefix.clone();
        let custom = class.clone();
        move || {
            let mut classes = vec![
                prefix.clone(),
                format!("{}-{}", prefix, size.class_suffix()),
                format!("{}-{}", prefix, variant.class_suffix()),
                format!("{}-{}", prefix, layout.class_suffix()),
            ];

            task.with_value(|t| {
                classes.push(format!("{}-{}", prefix, t.status.class_suffix()));
                if !t.is_available() {
                    classes.push(format!("{}-unavailable", prefix));
                }
            });

            if has_gallery {
                classes.push(format!("{}-has-gallery", prefix));
            }

            // Summary mode classes
            if summary_mode {
                classes.push(format!("{}-summary", prefix));
                if is_expanded.get() {
                    classes.push(format!("{}-expanded", prefix));
                }
            }

            if let Some(ref c) = custom {
                classes.push(c.clone());
            }

            classes.join(" ")
        }
    };

    // Click handler
    let handle_click = {
        let on_click = on_click.clone();
        move |_: web_sys::MouseEvent| {
            if let Some(ref cb) = on_click {
                task.with_value(|t| cb.run(t.clone()));
            }
        }
    };

    // Direction attribute
    let dir = task.with_value(|t| t.direction.clone());

    // Render gallery/media section
    let render_media = {
        let prefix = prefix.clone();
        let images = images.clone();
        move || {
            if images.is_empty() {
                return None;
            }

            let prefix = prefix.clone();
            let images = images.clone();

            Some(view! {
                <div class=format!("{}-media", prefix)>
                    <div class=format!("{}-gallery", prefix)>
                        {images.iter().enumerate().map(|(idx, src)| {
                            let prefix_inner = prefix.clone();
                            let src = src.clone();
                            let is_active = move || active_index.get() == idx;

                            view! {
                                <div
                                    class=move || {
                                        let mut classes = vec![format!("{}-gallery-slide", prefix_inner)];
                                        if is_active() {
                                            classes.push(format!("{}-gallery-slide-active", prefix_inner));
                                        }
                                        classes.join(" ")
                                    }
                                >
                                    <img
                                        src=src
                                        alt=task.with_value(|t| t.title.clone())
                                        loading="lazy"
                                        draggable="false"
                                    />
                                </div>
                            }
                        }).collect_view()}

                        // Gallery indicators (dots)
                        {(images.len() > 1).then(|| {
                            let prefix_dots = prefix.clone();
                            let count = images.len();
                            view! {
                                <div class=format!("{}-gallery-dots", prefix_dots)>
                                    {(0..count).map(|idx| {
                                        let prefix_dot = prefix_dots.clone();
                                        let is_active = move || active_index.get() == idx;
                                        view! {
                                            <button
                                                type="button"
                                                class=move || {
                                                    let mut classes = vec![format!("{}-gallery-dot", prefix_dot)];
                                                    if is_active() {
                                                        classes.push(format!("{}-gallery-dot-active", prefix_dot));
                                                    }
                                                    classes.join(" ")
                                                }
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    active_index.set(idx);
                                                }
                                                aria-label=format!("Go to image {}", idx + 1)
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        })}
                    </div>

                    // Status badge overlay
                    {task.with_value(|t| {
                        if t.status != TaskStatus::Active {
                            let prefix = prefix.clone();
                            Some(view! {
                                <div class=format!("{}-status-badge {}-status-{}", prefix, prefix, t.status.class_suffix())>
                                    {t.status.label()}
                                </div>
                            })
                        } else {
                            None
                        }
                    })}
                </div>
            })
        }
    };

    // Helper to truncate description for summary mode
    let truncate_description = |desc: &str, max_len: usize| -> String {
        if desc.len() <= max_len {
            desc.to_string()
        } else {
            let truncated = &desc[..desc.char_indices()
                .take_while(|(i, _)| *i < max_len)
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(max_len)];
            format!("{}...", truncated.trim_end())
        }
    };

    // Render summary content (condensed view)
    let render_summary_content = {
        let prefix = prefix.clone();
        move || {
            let prefix = prefix.clone();
            let primary_earnings = task.with_value(|t| t.primary_earnings());

            view! {
                <div class=format!("{}-content {}-content-summary", prefix, prefix)>
                    // Header with title and difficulty
                    <div class=format!("{}-header", prefix)>
                        <h3 class=format!("{}-title", prefix)>
                            {task.with_value(|t| t.title.clone())}
                        </h3>
                        {show_difficulty.then(|| {
                            let prefix = prefix.clone();
                            task.with_value(|t| {
                                view! {
                                    <span class=format!("{}-difficulty {}-difficulty-{}", prefix, prefix, t.difficulty.class_suffix())>
                                        {t.difficulty.label()}
                                    </span>
                                }
                            })
                        })}
                    </div>

                    // Truncated description
                    <p class=format!("{}-description {}-description-truncated", prefix, prefix)>
                        {task.with_value(|t| truncate_description(&t.description, 100))}
                    </p>

                    // Meta row: Earnings | Task Type | Duration
                    <div class=format!("{}-meta-row", prefix)>
                        // Primary earnings (combined range)
                        {primary_earnings.map(|earnings| {
                            let prefix = prefix.clone();
                            view! {
                                <div class=format!("{}-earnings-summary", prefix)>
                                    <svg class=format!("{}-icon", prefix) viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                                        <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
                                    </svg>
                                    <span>{earnings}</span>
                                </div>
                            }
                        })}

                        // Task type badge (Active/Passive)
                        {show_task_type.then(|| {
                            let prefix = prefix.clone();
                            task.with_value(|t| {
                                view! {
                                    <span class=format!("{}-task-type {}-task-type-{}", prefix, prefix, t.task_type.class_suffix())>
                                        {t.task_type.label()}
                                    </span>
                                }
                            })
                        })}

                        // Duration
                        {show_duration.then(|| {
                            task.with_value(|t| t.duration.clone()).map(|dur| {
                                let prefix = prefix.clone();
                                view! {
                                    <span class=format!("{}-duration", prefix)>
                                        <svg class=format!("{}-icon", prefix) viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>
                                        </svg>
                                        {dur}
                                    </span>
                                }
                            })
                        }).flatten()}
                    </div>

                    // Toggle expand button - circular with icon, optional label
                    {
                        let prefix_wrapper = prefix.clone();
                        let prefix_btn = prefix.clone();
                        let prefix_icon = prefix.clone();
                        let prefix_text = prefix.clone();
                        view! {
                            <div class=format!("{}-toggle-wrapper", prefix_wrapper)>
                                <button
                                    type="button"
                                    class=move || {
                                        let mut classes = vec![format!("{}-toggle-btn", prefix_btn)];
                                        if !show_toggle_label {
                                            classes.push(format!("{}-toggle-btn-icon-only", prefix_btn));
                                        }
                                        classes.join(" ")
                                    }
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        set_is_expanded.set(true);
                                    }
                                    aria-label="Expand details"
                                >
                                    <span class=format!("{}-toggle-icon-circle", prefix_icon)>
                                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
                                            <line x1="12" y1="5" x2="12" y2="19"></line>
                                            <line x1="5" y1="12" x2="19" y2="12"></line>
                                        </svg>
                                    </span>
                                    {show_toggle_label.then(|| view! {
                                        <span class=format!("{}-toggle-text", prefix_text)>"More"</span>
                                    })}
                                </button>
                            </div>
                        }
                    }
                </div>
            }
        }
    };

    // Render full content section
    let render_full_content = {
        let prefix = prefix.clone();
        move || {
            let prefix = prefix.clone();
            view! {
                <div class=format!("{}-content", prefix)>
                    // Header with title and difficulty
                    <div class=format!("{}-header", prefix)>
                        <h3 class=format!("{}-title", prefix)>
                            {task.with_value(|t| t.title.clone())}
                        </h3>
                        {show_difficulty.then(|| {
                            let prefix = prefix.clone();
                            task.with_value(|t| {
                                view! {
                                    <span class=format!("{}-difficulty {}-difficulty-{}", prefix, prefix, t.difficulty.class_suffix())>
                                        {t.difficulty.label()}
                                    </span>
                                }
                            })
                        })}
                    </div>

                    // Description
                    <p class=format!("{}-description", prefix)>
                        {task.with_value(|t| t.description.clone())}
                    </p>

                    // Meta row with task type and duration
                    <div class=format!("{}-meta-row", prefix)>
                        // Task type badge (Active/Passive)
                        {show_task_type.then(|| {
                            let prefix = prefix.clone();
                            task.with_value(|t| {
                                view! {
                                    <span class=format!("{}-task-type {}-task-type-{}", prefix, prefix, t.task_type.class_suffix())>
                                        {t.task_type.label()}
                                    </span>
                                }
                            })
                        })}

                        // Duration
                        {show_duration.then(|| {
                            task.with_value(|t| t.duration.clone()).map(|dur| {
                                let prefix = prefix.clone();
                                view! {
                                    <span class=format!("{}-duration", prefix)>
                                        <svg class=format!("{}-icon", prefix) viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>
                                        </svg>
                                        {dur}
                                    </span>
                                }
                            })
                        }).flatten()}
                    </div>

                    // Earnings section
                    {(show_earnings && !task.with_value(|t| t.earnings_tiers.is_empty())).then(|| {
                        let prefix = prefix.clone();
                        view! {
                            <div class=format!("{}-earnings", prefix)>
                                <div class=format!("{}-earnings-header", prefix)>
                                    <svg class=format!("{}-icon", prefix) viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
                                        <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
                                    </svg>
                                    "Earnings"
                                </div>
                                <div class=format!("{}-tiers", prefix)>
                                    {task.with_value(|t| {
                                        t.earnings_tiers.iter().map(|tier| {
                                            let prefix_inner = prefix.clone();
                                            let name = tier.name.clone();
                                            let range = tier.format_range();
                                            let is_popular = tier.is_popular;
                                            let features = tier.features.clone();

                                            view! {
                                                <div class=move || {
                                                    let mut classes = vec![format!("{}-tier", prefix_inner)];
                                                    if is_popular {
                                                        classes.push(format!("{}-tier-popular", prefix_inner));
                                                    }
                                                    classes.join(" ")
                                                }>
                                                    <div class=format!("{}-tier-header", prefix_inner)>
                                                        <span class=format!("{}-tier-name", prefix_inner)>{name}</span>
                                                        {is_popular.then(|| view! {
                                                            <span class=format!("{}-tier-badge", prefix_inner)>"Popular"</span>
                                                        })}
                                                    </div>
                                                    <div class=format!("{}-tier-earnings", prefix_inner)>{range}</div>
                                                    {(!features.is_empty()).then(|| {
                                                        let prefix_feat = prefix_inner.clone();
                                                        view! {
                                                            <ul class=format!("{}-tier-features", prefix_feat)>
                                                                {features.iter().map(|f| {
                                                                    let f = f.clone();
                                                                    view! { <li>{f}</li> }
                                                                }).collect_view()}
                                                            </ul>
                                                        }
                                                    })}
                                                </div>
                                            }
                                        }).collect_view()
                                    })}
                                </div>
                            </div>
                        }
                    })}

                    // Requirements section
                    {(show_requirements && !task.with_value(|t| t.requirements.is_empty())).then(|| {
                        let prefix = prefix.clone();
                        view! {
                            <div class=format!("{}-requirements", prefix)>
                                <div class=format!("{}-requirements-header", prefix)>"Requirements"</div>
                                <ul class=format!("{}-requirements-list", prefix)>
                                    {task.with_value(|t| {
                                        t.requirements.iter().map(|r| {
                                            let r = r.clone();
                                            view! {
                                                <li>
                                                    <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                                                        <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                                                    </svg>
                                                    {r}
                                                </li>
                                            }
                                        }).collect_view()
                                    })}
                                </ul>
                            </div>
                        }
                    })}

                    // Toggle collapse button at bottom - circular with icon, optional label (only in summary mode when expanded)
                    {(summary_mode).then(|| {
                        let prefix_wrapper = prefix.clone();
                        let prefix_btn = prefix.clone();
                        let prefix_icon = prefix.clone();
                        let prefix_text = prefix.clone();
                        view! {
                            <div class=format!("{}-toggle-wrapper", prefix_wrapper)>
                                <button
                                    type="button"
                                    class=move || {
                                        let mut classes = vec![
                                            format!("{}-toggle-btn", prefix_btn),
                                            format!("{}-toggle-btn-collapse", prefix_btn),
                                        ];
                                        if !show_toggle_label {
                                            classes.push(format!("{}-toggle-btn-icon-only", prefix_btn));
                                        }
                                        classes.join(" ")
                                    }
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        set_is_expanded.set(false);
                                    }
                                    aria-label="Collapse details"
                                >
                                    <span class=format!("{}-toggle-icon-circle", prefix_icon)>
                                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
                                            <line x1="5" y1="12" x2="19" y2="12"></line>
                                        </svg>
                                    </span>
                                    {show_toggle_label.then(|| view! {
                                        <span class=format!("{}-toggle-text", prefix_text)>"Less"</span>
                                    })}
                                </button>
                            </div>
                        }
                    })}
                </div>
            }
        }
    };

    // Render content based on mode
    let render_content = move || {
        if summary_mode && !is_expanded.get() {
            render_summary_content().into_any()
        } else {
            render_full_content().into_any()
        }
    };

    view! {
        <article
            class=root_class
            dir=dir
            role="article"
            tabindex="0"
            on:click=handle_click
        >
            {render_media()}
            {render_content}
        </article>
    }
}

/// TaskCardGrid component for displaying multiple task cards in a grid.
#[component]
pub fn TaskCardGrid(
    /// Tasks to display.
    #[prop(into)]
    tasks: Vec<TaskData>,
    /// Card size.
    #[prop(optional, into)]
    size: Option<TaskCardSize>,
    /// Card layout.
    #[prop(optional, into)]
    layout: Option<TaskCardLayout>,
    /// Summary mode for all cards.
    #[prop(optional)]
    summary_mode: bool,
    /// Show toggle button label ("More"/"Less") - if false, shows icon only.
    #[prop(optional)]
    #[prop(default = true)]
    show_toggle_label: bool,
    /// Columns (responsive).
    #[prop(optional)]
    #[prop(default = 3)]
    columns: u8,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click callback.
    #[prop(optional, into)]
    on_task_click: Option<Callback<TaskData>>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-task-grid-{}", design_system);
    let size = size.unwrap_or_default();
    let layout = layout.unwrap_or_default();

    let grid_class = {
        let prefix = prefix.clone();
        let custom = class.clone();
        move || {
            let mut classes = vec![
                prefix.clone(),
                format!("{}-cols-{}", prefix, columns),
            ];
            if let Some(ref c) = custom {
                classes.push(c.clone());
            }
            classes.join(" ")
        }
    };

    view! {
        <div class=grid_class>
            {tasks.into_iter().map(|task| {
                let on_click = on_task_click.clone();
                if let Some(cb) = on_click {
                    view! {
                        <TaskCard
                            task=task
                            size=size
                            layout=layout
                            summary_mode=summary_mode
                            show_toggle_label=show_toggle_label
                            on_click=cb
                        />
                    }.into_any()
                } else {
                    view! {
                        <TaskCard
                            task=task
                            size=size
                            layout=layout
                            summary_mode=summary_mode
                            show_toggle_label=show_toggle_label
                        />
                    }.into_any()
                }
            }).collect_view()}
        </div>
    }
}
