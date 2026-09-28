//! CMS Landing page

use leptos::prelude::*;
use crate::components::layout::Header;
use super::CmsMenuBar;

/// CMS Landing page - shows the horizontal menubar and overview
#[component]
pub fn CmsLandingPage() -> impl IntoView {
    view! {
        <Header title="CMS" show_search=false />
        <CmsMenuBar />

        <main class="p-6">
            <div class="max-w-4xl mx-auto">
                <div class="text-center py-12">
                    <h2 class="text-2xl font-bold text-slate-900 dark:text-white mb-4">
                        "Content Management System"
                    </h2>
                    <p class="text-slate-600 dark:text-slate-400 mb-8">
                        "Manage your content, review submissions, and publish to production."
                    </p>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
                        // Content card
                        <a href="/content" class="cms-overview-card">
                            <div class="cms-overview-icon content">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                </svg>
                            </div>
                            <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Content"</h3>
                            <p class="text-sm text-slate-500 dark:text-slate-400">"Create and edit content items"</p>
                        </a>

                        // Reviews card
                        <a href="/reviews" class="cms-overview-card">
                            <div class="cms-overview-icon reviews">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                            <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Reviews"</h3>
                            <p class="text-sm text-slate-500 dark:text-slate-400">"Review pending submissions"</p>
                        </a>

                        // Publish card
                        <a href="/publish" class="cms-overview-card">
                            <div class="cms-overview-icon publish">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                            </div>
                            <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Publish"</h3>
                            <p class="text-sm text-slate-500 dark:text-slate-400">"Publish approved content"</p>
                        </a>
                    </div>

                    // Page Editors section
                    <div class="mt-12">
                        <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">
                            "Page Editors"
                        </h3>
                        <p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
                            "Specialized editors for static pages"
                        </p>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            // Home page editor
                            <a href="/content/home" class="cms-overview-card">
                                <div class="cms-overview-icon" style="background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);">
                                    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z"/>
                                        <polyline points="9,22 9,12 15,12 15,22"/>
                                    </svg>
                                </div>
                                <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Home Page"</h3>
                                <p class="text-sm text-slate-500 dark:text-slate-400">"Edit home page sections"</p>
                            </a>

                            // FAQ page editor
                            <a href="/content/faq" class="cms-overview-card">
                                <div class="cms-overview-icon" style="background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);">
                                    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <circle cx="12" cy="12" r="10"/>
                                        <path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3"/>
                                        <line x1="12" y1="17" x2="12.01" y2="17"/>
                                    </svg>
                                </div>
                                <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"FAQ Page"</h3>
                                <p class="text-sm text-slate-500 dark:text-slate-400">"Manage frequently asked questions"</p>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
