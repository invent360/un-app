//! Review dashboard page for content approval workflow

use leptos::prelude::*;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, LoadingOverlay, SpinnerSize};
use crate::api::content_client::{PendingReviewsResponse, ReviewWithContent};
use crate::pages::cms::CmsMenuBar;

#[cfg(feature = "ssr")]
use crate::api::content_client::ContentClient;

/// Server function to get pending reviews
#[server(GetPendingReviewsList, "/api")]
pub async fn get_pending_reviews_list() -> Result<PendingReviewsResponse, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.get_pending_reviews(Some(50))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Server function to approve a review
#[server(ApproveContentReview, "/api")]
pub async fn approve_content_review(
    review_id: i32,
    reviewed_by: String,
    notes: Option<String>,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.approve_review(review_id, &reviewed_by, notes.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Server function to request changes on a review
#[server(RequestReviewChanges, "/api")]
pub async fn request_review_changes(
    review_id: i32,
    reviewed_by: String,
    notes: String,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.request_changes(review_id, &reviewed_by, &notes)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Server function to reject a review
#[server(RejectContentReview, "/api")]
pub async fn reject_content_review(
    review_id: i32,
    reviewed_by: String,
    notes: Option<String>,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.reject_review(review_id, &reviewed_by, notes.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Server function to create preview for review
#[server(CreateReviewPreview, "/api")]
pub async fn create_review_preview(version_id: i32) -> Result<String, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let token = client.create_preview_token(version_id, "reviewer", Some(24))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let uno_app_url = std::env::var("UNO_APP_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    Ok(format!("{}/preview?token={}", uno_app_url, token.token))
}

/// Reviews dashboard page
#[component]
pub fn ReviewsDashboardPage() -> impl IntoView {
    // State
    let (action_error, set_action_error) = signal(Option::<String>::None);
    let (action_success, set_action_success) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);
    let loading_message = RwSignal::new(Option::<String>::None);

    // Modal state for request changes
    let (show_changes_modal, set_show_changes_modal) = signal(false);
    let (modal_review_id, set_modal_review_id) = signal(Option::<i32>::None);
    let (change_notes, set_change_notes) = signal(String::new());

    // Fetch pending reviews
    let reviews_resource = Resource::new(
        || (),
        |_| async move { get_pending_reviews_list().await }
    );

    // Action handlers
    let approve_action = Action::new(move |review_id: &i32| {
        let rid = *review_id;
        async move {
            set_is_loading.set(true);
            loading_message.set(Some("Approving review...".to_string()));
            set_action_error.set(None);
            set_action_success.set(None);

            match approve_content_review(rid, "admin".to_string(), None).await {
                Ok(_) => {
                    set_action_success.set(Some("Review approved".to_string()));
                    // Refetch reviews
                    reviews_resource.refetch();
                }
                Err(e) => set_action_error.set(Some(e.to_string())),
            }
            set_is_loading.set(false);
            loading_message.set(None);
        }
    });

    let reject_action = Action::new(move |review_id: &i32| {
        let rid = *review_id;
        async move {
            set_is_loading.set(true);
            loading_message.set(Some("Rejecting review...".to_string()));
            set_action_error.set(None);
            set_action_success.set(None);

            match reject_content_review(rid, "admin".to_string(), None).await {
                Ok(_) => {
                    set_action_success.set(Some("Review rejected".to_string()));
                    reviews_resource.refetch();
                }
                Err(e) => set_action_error.set(Some(e.to_string())),
            }
            set_is_loading.set(false);
            loading_message.set(None);
        }
    });

    let submit_changes_action = Action::new(move |_: &()| {
        let rid = modal_review_id.get();
        let notes = change_notes.get();
        async move {
            if let Some(review_id) = rid {
                if notes.trim().is_empty() {
                    set_action_error.set(Some("Notes are required when requesting changes".to_string()));
                    return;
                }

                set_is_loading.set(true);
                loading_message.set(Some("Submitting feedback...".to_string()));
                set_action_error.set(None);
                set_action_success.set(None);

                match request_review_changes(review_id, "admin".to_string(), notes).await {
                    Ok(_) => {
                        set_action_success.set(Some("Changes requested".to_string()));
                        set_show_changes_modal.set(false);
                        set_change_notes.set(String::new());
                        reviews_resource.refetch();
                    }
                    Err(e) => set_action_error.set(Some(e.to_string())),
                }
                set_is_loading.set(false);
                loading_message.set(None);
            }
        }
    });

    view! {
        <div>
            // Full-page loading overlay for approve/reject actions
            <LoadingOverlay
                visible=Signal::derive(move || is_loading.get())
                message=Signal::derive(move || loading_message.get())
            />

            <Header title="CMS".to_string() show_search=false />
            <CmsMenuBar />

            <div class="px-4 py-4 space-y-4">
                // Messages
                {move || action_error.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm flex items-center justify-between">
                        {msg}
                        <button
                            class="text-red-500 hover:text-red-700"
                            on:click=move |_| set_action_error.set(None)
                        >
                            <Icon name=IconName::Close size=16 />
                        </button>
                    </div>
                })}

                {move || action_success.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 text-sm flex items-center justify-between">
                        {msg}
                        <button
                            class="text-green-500 hover:text-green-700"
                            on:click=move |_| set_action_success.set(None)
                        >
                            <Icon name=IconName::Close size=16 />
                        </button>
                    </div>
                })}

                // Reviews list
                <Suspense fallback=move || view! {
                    <div style="display: flex; align-items: center; justify-content: center; padding: 48px; background: #0f172a; border: 1px solid #1e293b; border-radius: 12px;">
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }>
                    {move || {
                        match reviews_resource.get() {
                            None => view! {
                                <div style="display: flex; align-items: center; justify-content: center; padding: 48px;">
                                    <ProgressSpinner size=SpinnerSize::Default />
                                </div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(response)) => {
                                if response.reviews.is_empty() {
                                    view! {
                                        <div class="text-center py-12">
                                            <div class="text-slate-400 dark:text-slate-500 mb-4">
                                                <Icon name=IconName::CheckCircle size=48 class="mx-auto opacity-50".to_string() />
                                            </div>
                                            <p class="text-slate-500 dark:text-slate-400">
                                                "No pending reviews"
                                            </p>
                                            <p class="text-sm text-slate-400 dark:text-slate-500 mt-2">
                                                "All content has been reviewed!"
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    let reviews = response.reviews.clone();
                                    let approve = approve_action.clone();
                                    let reject = reject_action.clone();

                                    view! {
                                        <div class="space-y-4">
                                            <div class="flex items-center justify-between">
                                                <h2 class="text-lg font-semibold text-slate-900 dark:text-white">
                                                    {format!("Pending Reviews ({})", response.total)}
                                                </h2>
                                            </div>

                                            {reviews.into_iter().map(|review| {
                                                let rid = review.review.id;
                                                let approve = approve.clone();
                                                let reject = reject.clone();

                                                view! {
                                                    <ReviewCard
                                                        review=review
                                                        on_approve=move || { let _ = approve.dispatch(rid); }
                                                        on_request_changes=move || {
                                                            set_modal_review_id.set(Some(rid));
                                                            set_show_changes_modal.set(true);
                                                        }
                                                        on_reject=move || { let _ = reject.dispatch(rid); }
                                                    />
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    }}
                </Suspense>

                // Request Changes Modal
                {move || show_changes_modal.get().then(|| view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center">
                        // Backdrop
                        <div
                            class="absolute inset-0 bg-black/50"
                            on:click=move |_| set_show_changes_modal.set(false)
                        />

                        // Modal
                        <div class="relative bg-white dark:bg-slate-800 rounded-xl p-6 w-full max-w-lg mx-4 shadow-xl">
                            <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">
                                "Request Changes"
                            </h3>

                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                                        "Feedback Notes"
                                    </label>
                                    <textarea
                                        class="w-full h-32 px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-white"
                                        placeholder="Describe what changes are needed..."
                                        prop:value=move || change_notes.get()
                                        on:input=move |ev| set_change_notes.set(event_target_value(&ev))
                                    />
                                </div>

                                <div class="flex items-center justify-end gap-3">
                                    <button
                                        class="px-4 py-2 text-sm font-medium text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition"
                                        on:click=move |_| {
                                            set_show_changes_modal.set(false);
                                            set_change_notes.set(String::new());
                                        }
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        class="px-4 py-2 text-sm font-medium bg-amber-500 text-white rounded-lg hover:bg-amber-600 transition"
                                        on:click=move |_| { let _ = submit_changes_action.dispatch(()); }
                                    >
                                        "Submit Feedback"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}

/// Review card component
#[component]
fn ReviewCard(
    review: ReviewWithContent,
    on_approve: impl Fn() + 'static,
    on_request_changes: impl Fn() + 'static,
    on_reject: impl Fn() + 'static,
) -> impl IntoView {
    let type_badge = match review.content_type.as_str() {
        "task" => "bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400",
        "guide" => "bg-cyan-100 text-cyan-700 dark:bg-cyan-900/30 dark:text-cyan-400",
        "faq" => "bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400",
        "error" => "bg-rose-100 text-rose-700 dark:bg-rose-900/30 dark:text-rose-400",
        _ => "bg-slate-100 text-slate-600 dark:bg-slate-700 dark:text-slate-400",
    };

    // Preview action
    let (preview_url, set_preview_url) = signal(Option::<String>::None);
    let version_id = review.review.version_id;

    let preview_action = Action::new(move |_: &()| {
        async move {
            match create_review_preview(version_id).await {
                Ok(url) => set_preview_url.set(Some(url)),
                Err(_) => {}
            }
        }
    });

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-xl border border-slate-200 dark:border-slate-700 overflow-hidden">
            // Header
            <div class="px-6 py-4 border-b border-slate-200 dark:border-slate-700">
                <div class="flex items-start justify-between">
                    <div>
                        <div class="flex items-center gap-3">
                            <Icon name=IconName::Document size=20 class="text-slate-400".to_string() />
                            <h3 class="font-semibold text-slate-900 dark:text-white">
                                {review.title.clone()}
                            </h3>
                            <span class={format!("px-2 py-0.5 rounded text-xs font-medium {}", type_badge)}>
                                {review.content_type.clone()}
                            </span>
                        </div>
                        <p class="text-sm text-slate-500 dark:text-slate-400 mt-1">
                            {"Submitted by "}<strong>{review.review.submitted_by.clone()}</strong>
                            {" on "}{format_date(&review.review.submitted_at)}
                        </p>
                    </div>
                    <span class="text-sm text-slate-500 dark:text-slate-400">
                        {"Version "}{review.version}
                    </span>
                </div>
            </div>

            // Content
            <div class="px-6 py-4">
                {review.change_summary.as_ref().map(|summary| view! {
                    <div class="mb-4">
                        <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Changes: "</span>
                        <span class="text-sm text-slate-600 dark:text-slate-400">{summary.clone()}</span>
                    </div>
                })}

                {review.review.submitted_notes.as_ref().map(|notes| view! {
                    <div class="mb-4 p-3 rounded-lg bg-slate-50 dark:bg-slate-700/50">
                        <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Notes: "</span>
                        <span class="text-sm text-slate-600 dark:text-slate-400">{notes.clone()}</span>
                    </div>
                })}

                // Preview URL if generated
                {move || preview_url.get().map(|url| {
                    let url_display = url.clone();
                    view! {
                        <div class="mb-4 p-3 rounded-lg bg-blue-50 dark:bg-blue-900/20">
                            <span class="text-sm font-medium text-blue-700 dark:text-blue-400">"Preview: "</span>
                            <a
                                href={url}
                                target="_blank"
                                class="text-sm text-blue-600 dark:text-blue-400 underline hover:no-underline"
                            >
                                {url_display}
                            </a>
                        </div>
                    }
                })}
            </div>

            // Actions
            <div class="px-6 py-4 bg-slate-50 dark:bg-slate-700/30 flex items-center gap-3">
                <button
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 transition"
                    on:click=move |_| { let _ = preview_action.dispatch(()); }
                >
                    <Icon name=IconName::Eye size=16 />
                    "Preview"
                </button>

                <div class="flex-1" />

                <button
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition"
                    on:click=move |_| on_reject()
                >
                    <Icon name=IconName::Close size=16 />
                    "Reject"
                </button>

                <button
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-amber-600 dark:text-amber-400 hover:bg-amber-50 dark:hover:bg-amber-900/20 rounded-lg transition"
                    on:click=move |_| on_request_changes()
                >
                    <Icon name=IconName::Edit size=16 />
                    "Request Changes"
                </button>

                <button
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium bg-green-500 text-white rounded-lg hover:bg-green-600 transition"
                    on:click=move |_| on_approve()
                >
                    <Icon name=IconName::CheckCircle size=16 />
                    "Approve"
                </button>
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
