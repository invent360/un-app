//! Publish queue page for batch publishing approved content

use leptos::prelude::*;
use std::collections::HashSet;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::api::content_client::{ContentSummary, PublishResult};
use crate::pages::cms::CmsMenuBar;

#[cfg(feature = "ssr")]
use crate::api::content_client::{ContentClient, ContentListParams};

/// Server function to get approved content ready for publishing
#[server(GetApprovedContent, "/api")]
pub async fn get_approved_content() -> Result<Vec<ContentSummary>, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let params = ContentListParams {
        content_type: None,
        status: Some("approved".to_string()),
        search: None,
        page: Some(1),
        per_page: Some(100),
    };

    let response = client.list_contents(params)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(response.items)
}

/// Server function to batch publish content
#[server(BatchPublishContent, "/api")]
pub async fn batch_publish_content(
    content_ids: Vec<i32>,
    published_by: String,
) -> Result<PublishResult, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.publish_approved(content_ids, &published_by)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Publish queue page
#[component]
pub fn PublishQueuePage() -> impl IntoView {
    // State
    let (selected_ids, set_selected_ids) = signal(HashSet::<i32>::new());
    let (is_publishing, set_is_publishing) = signal(false);
    let (publish_result, set_publish_result) = signal(Option::<PublishResult>::None);
    let (error_message, set_error_message) = signal(Option::<String>::None);

    // Fetch approved content
    let content_resource = Resource::new(
        || (),
        |_| async move { get_approved_content().await }
    );

    // Toggle selection
    let toggle_selection = move |id: i32| {
        set_selected_ids.update(|ids| {
            if ids.contains(&id) {
                ids.remove(&id);
            } else {
                ids.insert(id);
            }
        });
    };

    // Select all
    let select_all = move |items: &[ContentSummary]| {
        set_selected_ids.update(|ids| {
            for item in items {
                ids.insert(item.id);
            }
        });
    };

    // Deselect all
    let deselect_all = move || {
        set_selected_ids.set(HashSet::new());
    };

    // Publish action
    let publish_action = Action::new(move |_: &()| {
        let ids: Vec<i32> = selected_ids.get().into_iter().collect();
        async move {
            if ids.is_empty() {
                set_error_message.set(Some("No content selected".to_string()));
                return;
            }

            set_is_publishing.set(true);
            set_error_message.set(None);
            set_publish_result.set(None);

            match batch_publish_content(ids, "admin".to_string()).await {
                Ok(result) => {
                    set_publish_result.set(Some(result));
                    set_selected_ids.set(HashSet::new());
                    // Refetch the list
                    content_resource.refetch();
                }
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }

            set_is_publishing.set(false);
        }
    });

    view! {
        <div>
            <Header title="CMS".to_string() show_search=false />
            <CmsMenuBar />

            <div class="px-4 py-4 space-y-4">
                // Publish result message
                {move || publish_result.get().map(|result| {
                    if result.success {
                        view! {
                            <div class="p-4 rounded-lg bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 text-sm flex items-center justify-between">
                                <span>
                                    {format!("Successfully published {} items", result.published_count)}
                                </span>
                                <button
                                    class="text-green-500 hover:text-green-700"
                                    on:click=move |_| set_publish_result.set(None)
                                >
                                    <Icon name=IconName::Close size=16 />
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="p-4 rounded-lg bg-amber-50 dark:bg-amber-900/20 text-amber-600 dark:text-amber-400 text-sm">
                                <div class="flex items-center justify-between mb-2">
                                    <span>
                                        {format!("Published {} items, {} failed", result.published_count, result.failed_count)}
                                    </span>
                                    <button
                                        class="text-amber-500 hover:text-amber-700"
                                        on:click=move |_| set_publish_result.set(None)
                                    >
                                        <Icon name=IconName::Close size=16 />
                                    </button>
                                </div>
                                {(!result.errors.is_empty()).then(|| view! {
                                    <ul class="list-disc list-inside mt-2">
                                        {result.errors.iter().map(|e| view! {
                                            <li>{format!("Content {}: {}", e.content_id, e.error)}</li>
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                })}
                            </div>
                        }.into_any()
                    }
                })}

                // Error message
                {move || error_message.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm flex items-center justify-between">
                        {msg}
                        <button
                            class="text-red-500 hover:text-red-700"
                            on:click=move |_| set_error_message.set(None)
                        >
                            <Icon name=IconName::Close size=16 />
                        </button>
                    </div>
                })}

                // Content list
                <Suspense fallback=move || view! {
                    <div class="bg-white dark:bg-slate-800 rounded-xl p-6 border border-slate-200 dark:border-slate-700">
                        <div class="animate-pulse space-y-4">
                            <div class="h-4 bg-slate-200 dark:bg-slate-700 rounded w-1/4"></div>
                            <div class="h-12 bg-slate-200 dark:bg-slate-700 rounded"></div>
                            <div class="h-12 bg-slate-200 dark:bg-slate-700 rounded"></div>
                            <div class="h-12 bg-slate-200 dark:bg-slate-700 rounded"></div>
                        </div>
                    </div>
                }>
                    {move || {
                        match content_resource.get() {
                            None => view! {
                                <div class="text-center py-12 text-slate-500">"Loading..."</div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(items)) => {
                                if items.is_empty() {
                                    view! {
                                        <div class="text-center py-12">
                                            <div class="text-slate-400 dark:text-slate-500 mb-4">
                                                <Icon name=IconName::Rocket size=48 class="mx-auto opacity-50".to_string() />
                                            </div>
                                            <p class="text-slate-500 dark:text-slate-400">
                                                "No approved content ready to publish"
                                            </p>
                                            <p class="text-sm text-slate-400 dark:text-slate-500 mt-2">
                                                "Content must be approved before it can be published."
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    let items_for_all_check = items.clone();
                                    let items_for_select = items.clone();
                                    let items_for_count = items.len();
                                    let items_for_list = items;
                                    let selected_count = move || selected_ids.get().len();

                                    view! {
                                        <div class="space-y-4">
                                            // Header with select all and publish button
                                            <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700 flex items-center justify-between">
                                                <div class="flex items-center gap-4">
                                                    <label class="flex items-center gap-2 cursor-pointer">
                                                        <input
                                                            type="checkbox"
                                                            class="w-4 h-4 rounded border-slate-300 dark:border-slate-600 text-primary-500 focus:ring-primary-500"
                                                            prop:checked=move || {
                                                                let ids = selected_ids.get();
                                                                !items_for_all_check.is_empty() && items_for_all_check.iter().all(|i| ids.contains(&i.id))
                                                            }
                                                            on:change=move |_| {
                                                                let ids = selected_ids.get();
                                                                let all_selected = !items_for_select.is_empty() && items_for_select.iter().all(|i| ids.contains(&i.id));
                                                                if all_selected {
                                                                    deselect_all();
                                                                } else {
                                                                    select_all(&items_for_select);
                                                                }
                                                            }
                                                        />
                                                        <span class="text-sm text-slate-600 dark:text-slate-400">
                                                            "Select All"
                                                        </span>
                                                    </label>

                                                    <span class="text-sm text-slate-500 dark:text-slate-400">
                                                        {move || format!("{} of {} selected", selected_count(), items_for_count)}
                                                    </span>
                                                </div>

                                                <button
                                                    class="flex items-center gap-2 px-6 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                                                    disabled=move || selected_count() == 0 || is_publishing.get()
                                                    on:click=move |_| { let _ = publish_action.dispatch(()); }
                                                >
                                                    <Icon name=IconName::Rocket size=18 />
                                                    {move || if is_publishing.get() {
                                                        "Publishing...".to_string()
                                                    } else {
                                                        format!("Publish Selected ({})", selected_count())
                                                    }}
                                                </button>
                                            </div>

                                            // Warning
                                            <div class="p-4 rounded-lg bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800">
                                                <div class="flex items-start gap-3">
                                                    <Icon name=IconName::Star size=20 class="text-amber-500 flex-shrink-0 mt-0.5".to_string() />
                                                    <p class="text-sm text-amber-700 dark:text-amber-400">
                                                        "Publishing will make the selected content live on the public site immediately."
                                                    </p>
                                                </div>
                                            </div>

                                            // Content list
                                            <div class="bg-white dark:bg-slate-800 rounded-xl overflow-hidden border border-slate-200 dark:border-slate-700">
                                                <div class="divide-y divide-slate-200 dark:divide-slate-700">
                                                    {items_for_list.into_iter().map(|item| {
                                                        let id = item.id;
                                                        let is_selected = Signal::derive(move || selected_ids.get().contains(&id));

                                                        view! {
                                                            <PublishQueueItem
                                                                item=item
                                                                selected=is_selected
                                                                on_toggle=move || toggle_selection(id)
                                                            />
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

/// Publish queue item component
#[component]
fn PublishQueueItem(
    item: ContentSummary,
    #[prop(into)] selected: Signal<bool>,
    on_toggle: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let type_badge = match item.content_type.as_str() {
        "task" => "bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400",
        "guide" => "bg-cyan-100 text-cyan-700 dark:bg-cyan-900/30 dark:text-cyan-400",
        "faq" => "bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400",
        "error" => "bg-rose-100 text-rose-700 dark:bg-rose-900/30 dark:text-rose-400",
        _ => "bg-slate-100 text-slate-600 dark:bg-slate-700 dark:text-slate-400",
    };

    let toggle1 = on_toggle.clone();
    let toggle2 = on_toggle.clone();

    view! {
        <div
            class=move || format!(
                "flex items-center gap-4 px-4 py-3 transition cursor-pointer {}",
                if selected.get() {
                    "bg-primary-50 dark:bg-primary-900/20"
                } else {
                    "hover:bg-slate-50 dark:hover:bg-slate-700/30"
                }
            )
            on:click=move |_| toggle1()
        >
            <input
                type="checkbox"
                class="w-4 h-4 rounded border-slate-300 dark:border-slate-600 text-primary-500 focus:ring-primary-500"
                prop:checked=move || selected.get()
                on:click=move |e| e.stop_propagation()
                on:change=move |_| toggle2()
            />

            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-3">
                    <span class="font-medium text-slate-900 dark:text-white truncate">
                        {item.title.clone()}
                    </span>
                    <span class={format!("px-2 py-0.5 rounded text-xs font-medium flex-shrink-0 {}", type_badge)}>
                        {item.content_type.clone()}
                    </span>
                </div>
                <p class="text-sm text-slate-500 dark:text-slate-400 mt-0.5 font-mono">
                    {item.slug.clone()}
                </p>
            </div>

            <div class="flex items-center gap-4 text-sm text-slate-500 dark:text-slate-400">
                <span>{"v"}{item.version}</span>
                <span>{format_date(&item.updated_at)}</span>
            </div>
        </div>
    }
}

/// Format date for display
fn format_date(date_str: &str) -> String {
    if date_str.len() >= 10 {
        date_str[..10].to_string()
    } else {
        date_str.to_string()
    }
}
