//! What Next stage component - shows guides, tasks, and next steps after claiming

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;
use crate::components::wizard::components::DownloadButtons;

/// What Next stage - final step with guides, tasks, and next actions
#[component]
pub fn WhatNextStage() -> impl IntoView {
    let state = use_wizard_state().expect("WhatNextStage must be rendered within wizard context");

    let state_for_back = state.clone();
    let state_for_close = state.clone();

    view! {
        <div class="wizard-stage wizard-what-next">
            <h2 class="stage-title">{move || t("wizard.what_next.title")}</h2>
            <p class="stage-subtitle">{move || t("wizard.what_next.subtitle")}</p>

            // Warning banner - bind license within 24 hours
            <div class="warning-banner">
                <div class="warning-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="8" x2="12" y2="12"/>
                        <line x1="12" y1="16" x2="12.01" y2="16"/>
                    </svg>
                </div>
                <div class="warning-content">
                    <strong>{move || t("wizard.what_next.warning_title")}</strong>
                    <p>{move || t("wizard.what_next.warning_text")}</p>
                </div>
            </div>

            // Download section
            <div class="what-next-section download-section">
                <h3 class="section-title">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7 10 12 15 17 10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    {move || t("wizard.what_next.download_title")}
                </h3>
                <DownloadButtons />
            </div>

            // How-to Guides section
            <div class="what-next-section guides-section">
                <h3 class="section-title">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
                        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
                    </svg>
                    {move || t("wizard.what_next.guides_title")}
                </h3>
                <div class="guides-grid">
                    <a href="/guides" class="guide-link primary">
                        <div class="guide-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                                <line x1="8" y1="21" x2="16" y2="21"/>
                                <line x1="12" y1="17" x2="12" y2="21"/>
                            </svg>
                        </div>
                        <div class="guide-content">
                            <span class="guide-name">{move || t("wizard.what_next.guide_setup")}</span>
                            <span class="guide-desc">{move || t("wizard.what_next.guide_setup_desc")}</span>
                        </div>
                        <div class="guide-arrow">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="9 18 15 12 9 6"/>
                            </svg>
                        </div>
                    </a>
                    <a href="/guides" class="guide-link">
                        <div class="guide-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                            </svg>
                        </div>
                        <div class="guide-content">
                            <span class="guide-name">{move || t("wizard.what_next.guide_activation")}</span>
                            <span class="guide-desc">{move || t("wizard.what_next.guide_activation_desc")}</span>
                        </div>
                        <div class="guide-arrow">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="9 18 15 12 9 6"/>
                            </svg>
                        </div>
                    </a>
                    <a href="/guides" class="guide-link">
                        <div class="guide-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="12" y1="1" x2="12" y2="23"/>
                                <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
                            </svg>
                        </div>
                        <div class="guide-content">
                            <span class="guide-name">{move || t("wizard.what_next.guide_withdraw")}</span>
                            <span class="guide-desc">{move || t("wizard.what_next.guide_withdraw_desc")}</span>
                        </div>
                        <div class="guide-arrow">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="9 18 15 12 9 6"/>
                            </svg>
                        </div>
                    </a>
                </div>
            </div>

            // Available Tasks section
            <div class="what-next-section tasks-section">
                <h3 class="section-title">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                        <polyline points="22 4 12 14.01 9 11.01"/>
                    </svg>
                    {move || t("wizard.what_next.tasks_title")}
                </h3>
                <div class="tasks-grid">
                    <a href="/tasks" class="task-link">
                        <div class="task-icon telemetry">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/>
                            </svg>
                        </div>
                        <span class="task-name">{move || t("wizard.what_next.task_telemetry")}</span>
                    </a>
                    <a href="/tasks" class="task-link">
                        <div class="task-icon caller-id">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"/>
                            </svg>
                        </div>
                        <span class="task-name">{move || t("wizard.what_next.task_caller_id")}</span>
                    </a>
                    <a href="/tasks" class="task-link">
                        <div class="task-icon sms">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
                            </svg>
                        </div>
                        <span class="task-name">{move || t("wizard.what_next.task_sms")}</span>
                    </a>
                    <a href="/tasks" class="task-link">
                        <div class="task-icon connectivity">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
                                <path d="M1.42 9a16 16 0 0 1 21.16 0"/>
                                <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
                                <line x1="12" y1="20" x2="12.01" y2="20"/>
                            </svg>
                        </div>
                        <span class="task-name">{move || t("wizard.what_next.task_connectivity")}</span>
                    </a>
                    <a href="/tasks" class="task-link">
                        <div class="task-icon entropy">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                            </svg>
                        </div>
                        <span class="task-name">{move || t("wizard.what_next.task_entropy")}</span>
                    </a>
                </div>
                <a href="/tasks" class="view-all-link">
                    {move || t("wizard.what_next.view_all_tasks")}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="5" y1="12" x2="19" y2="12"/>
                        <polyline points="12 5 19 12 12 19"/>
                    </svg>
                </a>
            </div>

            // Actions
            <div class="wizard-footer">
                <button
                    type="button"
                    class="btn-secondary"
                    on:click=move |_| state_for_back.prev_stage()
                >
                    {move || t("wizard.common.back")}
                </button>
                <button
                    type="button"
                    class="btn-primary"
                    on:click=move |_| state_for_close.close()
                >
                    {move || t("wizard.what_next.done")}
                </button>
            </div>
        </div>
    }
}
