//! Preview page - redirects to actual content pages with preview token
//!
//! The preview system works by adding a preview_token to the actual page URL,
//! not by creating duplicate preview pages. This ensures preview renders
//! identically to the live page.
//!
//! Example: /guides?preview_token=xxx will show draft guide content
//! using the exact same GuidesPage component.

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// Preview page component - shows token-based preview for legacy compatibility
///
/// Note: This is being phased out. New preview links should go directly to
/// the actual page (e.g., /guides?preview_token=xxx) rather than /preview?token=xxx
#[component]
pub fn PreviewPage() -> impl IntoView {
    let query = use_query_map();

    // Extract token and content_type from query string
    let token = move || query.get().get("token");
    let content_type = move || query.get().get("type").unwrap_or_else(|| "guide".to_string());

    view! {
        <div class="page">
            <header class="page-header">
                <h1>"Preview"</h1>
                <p>"Redirecting to content preview..."</p>
            </header>

            <section class="guides-content">
                <div class="guides-content-wrapper">
                    {move || {
                        match (token(), content_type()) {
                            (Some(t), ct) => {
                                // Redirect to actual page with preview token
                                let redirect_url = match ct.as_str() {
                                    "guide" => format!("/guides?preview_token={}", t),
                                    "task" => format!("/tasks?preview_token={}", t),
                                    "faq" => format!("/faq?preview_token={}", t),
                                    _ => format!("/guides?preview_token={}", t),
                                };

                                view! {
                                    <div class="text-center py-8">
                                        <p class="mb-4">"Redirecting to preview..."</p>
                                        <a href={redirect_url.clone()} class="btn btn-primary">
                                            "Click here if not redirected"
                                        </a>
                                        // Client-side redirect
                                        <script>
                                            {format!("window.location.href = '{}';", redirect_url)}
                                        </script>
                                    </div>
                                }.into_any()
                            }
                            (None, _) => {
                                view! {
                                    <div class="empty-state">
                                        <p class="placeholder-text">"No preview token provided."</p>
                                        <p class="text-sm text-gray-500 mt-2">
                                            "Preview links should include a token parameter."
                                        </p>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </div>
            </section>
        </div>
    }
}
