//! Participant Dashboard Page for R5-13
//!
//! Displays:
//! - Status banner with next action required
//! - Cohort progress (D1/D7/D30 milestones)
//! - Support ticket summary
//! - Exit status (if applicable)
//! - Activity feed

use leptos::prelude::*;

#[cfg(any(feature = "csr", feature = "hydrate"))]
use crate::api::dashboard::{fetch_dashboard, DashboardData, CohortSummary, ExitSummary};
use crate::hooks::{use_user, UserLoadState};

/// Dashboard page component (protected - requires authentication)
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="dashboard-page">
            <DashboardContent />
        </div>
    }
}

/// Dashboard content with authentication check
#[component]
fn DashboardContent() -> impl IntoView {
    let user_ctx = use_user();

    // Reactive view based on auth state
    view! {
        {move || {
            match user_ctx.state.get() {
                UserLoadState::Loading => {
                    view! { <DashboardLoading /> }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! { <DashboardUnauthenticated /> }.into_any()
                }
                UserLoadState::Error => {
                    let error_msg = user_ctx.error.get().unwrap_or_else(|| "Unknown error".to_string());
                    view! { <DashboardError message=error_msg /> }.into_any()
                }
                UserLoadState::Loaded => {
                    // User is authenticated
                    match user_ctx.user.get() {
                        Some(profile) => {
                            view! { <AuthenticatedDashboard profile=profile /> }.into_any()
                        }
                        None => {
                            view! { <DashboardUnauthenticated /> }.into_any()
                        }
                    }
                }
            }
        }}
    }
}

/// Loading state for dashboard
#[component]
fn DashboardLoading() -> impl IntoView {
    view! {
        <div class="dashboard-loading">
            <div class="loading-spinner"></div>
            <p>"Loading your dashboard..."</p>
        </div>
    }
}

/// Unauthenticated state - redirect to login
#[component]
fn DashboardUnauthenticated() -> impl IntoView {
    view! {
        <div class="dashboard-unauthenticated">
            <div class="auth-required-card">
                <h2>"Sign In Required"</h2>
                <p>"Please sign in to access your dashboard."</p>
                <a href="/" class="btn-primary">"Go to Home"</a>
            </div>
        </div>
    }
}

/// Error state for dashboard
#[component]
fn DashboardError(message: String) -> impl IntoView {
    view! {
        <div class="dashboard-error">
            <div class="error-card">
                <h2>"Something went wrong"</h2>
                <p>{message}</p>
                <button class="btn-secondary" on:click=move |_| {
                    // Refresh the page
                    #[cfg(any(feature = "csr", feature = "hydrate"))]
                    {
                        let _ = web_sys::window().unwrap().location().reload();
                    }
                }>"Try Again"</button>
            </div>
        </div>
    }
}

/// Authenticated dashboard with user data
#[component]
fn AuthenticatedDashboard(profile: crate::hooks::UserProfile) -> impl IntoView {
    let user_id = profile.id.clone();
    let license_id = profile.license_id.clone();
    let cohort_status = profile.cohort_status.clone();
    let has_active_exit = profile.has_active_exit;

    // For the status banner
    let current_milestone = cohort_status.as_ref()
        .map(|c| c.current_milestone.clone())
        .unwrap_or_else(|| "onboarding".to_string());
    let next_action = cohort_status.as_ref()
        .and_then(|c| c.next_action.clone());

    view! {
        <div class="dashboard-authenticated">
            // Status Banner
            <StatusBanner
                milestone=current_milestone
                next_action=next_action
                has_exit=has_active_exit
            />

            // Main dashboard grid
            <div class="dashboard-grid">
                // Cohort Progress Card
                <CohortProgressCard cohort=cohort_status />

                // Support Summary Card
                <SupportCard user_id=user_id.clone() />

                // License Card
                <LicenseCard
                    license_id=license_id.clone()
                    license_status=profile.license_status.clone()
                />
            </div>

            // Activity Feed (if user has license)
            {license_id.clone().map(|lid| {
                view! { <ActivityFeed user_id=user_id.clone() license_id=lid /> }
            })}

            // Quick Actions
            <QuickActions has_license=license_id.is_some() />
        </div>
    }
}

/// Status banner showing current milestone and next action
#[component]
fn StatusBanner(
    milestone: String,
    next_action: Option<String>,
    has_exit: bool,
) -> impl IntoView {
    let (banner_class, banner_icon, banner_title) = if has_exit {
        ("status-banner exit", "logout", "Exit in Progress")
    } else {
        match milestone.as_str() {
            "onboarding" => ("status-banner onboarding", "rocket", "Getting Started"),
            "d7_progress" => ("status-banner progress", "trending-up", "D7 Progress"),
            "d30_progress" => ("status-banner progress", "calendar", "D30 Progress"),
            "graduated" => ("status-banner success", "award", "Graduated"),
            _ => ("status-banner active", "check-circle", "Active"),
        }
    };

    view! {
        <div class=banner_class>
            <div class="banner-icon">
                <span class="icon" data-icon=banner_icon></span>
            </div>
            <div class="banner-content">
                <h3 class="banner-title">{banner_title}</h3>
                {next_action.map(|action| view! {
                    <p class="banner-action">{action}</p>
                })}
            </div>
        </div>
    }
}

/// Cohort progress card showing D1/D7/D30 milestones
#[component]
fn CohortProgressCard(cohort: Option<crate::hooks::CohortStatus>) -> impl IntoView {
    match cohort {
        Some(c) => {
            let d7_progress = (c.d7_active_days as f32 / 4.0 * 100.0).min(100.0) as i32;
            let d7_status = if c.d7_completed {
                "Completed".to_string()
            } else {
                format!("{}/4 days", c.d7_active_days)
            };
            let progress_style = format!("width: {}%", d7_progress);

            view! {
                <div class="dashboard-card cohort-progress">
                    <h4 class="card-title">"Cohort Progress"</h4>
                    <div class="milestone-list">
                        <MilestoneItem
                            name="D1"
                            completed=c.d1_completed
                            description="First day completed"
                        />
                        <MilestoneItem
                            name="D3"
                            completed=c.d3_completed
                            description="Third day milestone"
                        />
                        <div class="milestone-item">
                            <div class="milestone-header">
                                <span class="milestone-name">"D7"</span>
                                <span class="milestone-status">{d7_status}</span>
                            </div>
                            {if !c.d7_completed {
                                view! {
                                    <div class="progress-bar">
                                        <div class="progress-fill" style=progress_style></div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span class="completed-badge">"Done"</span> }.into_any()
                            }}
                        </div>
                        <MilestoneItem
                            name="D30"
                            completed=c.d30_completed
                            description="30-day graduation"
                        />
                    </div>
                    <p class="cohort-date">"Started: "{c.cohort_date}</p>
                </div>
            }.into_any()
        }
        None => {
            view! {
                <div class="dashboard-card cohort-progress empty">
                    <h4 class="card-title">"Cohort Progress"</h4>
                    <p class="empty-message">"No active cohort. Claim a license to get started."</p>
                    <a href="/" class="btn-primary btn-sm">"Get Started"</a>
                </div>
            }.into_any()
        }
    }
}

/// Individual milestone item
#[component]
fn MilestoneItem(name: &'static str, completed: bool, description: &'static str) -> impl IntoView {
    let class = if completed { "milestone-item completed" } else { "milestone-item" };
    view! {
        <div class=class>
            <div class="milestone-header">
                <span class="milestone-name">{name}</span>
                <span class="milestone-status">
                    {if completed { "Completed" } else { "Pending" }}
                </span>
            </div>
            <p class="milestone-description">{description}</p>
        </div>
    }
}

/// Support ticket summary card
#[component]
fn SupportCard(user_id: String) -> impl IntoView {
    // In a full implementation, this would fetch support ticket data
    // For now, show a placeholder with link to support
    view! {
        <div class="dashboard-card support-card">
            <h4 class="card-title">"Support"</h4>
            <div class="support-summary">
                <p class="support-text">"Need help? Our support team is here for you."</p>
            </div>
            <a href="/contact" class="btn-secondary btn-sm">"Contact Support"</a>
        </div>
    }
}

/// License status card
#[component]
fn LicenseCard(
    license_id: Option<String>,
    license_status: Option<crate::hooks::LicenseStatus>,
) -> impl IntoView {
    match (license_id, license_status) {
        (Some(lid), Some(status)) => {
            let status_class = match status.state.as_str() {
                "active" => "status-active",
                "exiting" => "status-exiting",
                "suspended" => "status-suspended",
                _ => "status-default",
            };

            view! {
                <div class="dashboard-card license-card">
                    <h4 class="card-title">"License"</h4>
                    <div class="license-info">
                        <p class="license-id">"ID: "{&lid[..8.min(lid.len())]}"..."</p>
                        <span class=format!("license-status {}", status_class)>
                            {status.state}
                        </span>
                    </div>
                    {status.variant.map(|v| view! {
                        <p class="license-variant">"Variant: "{v}</p>
                    })}
                    {status.expires_at.map(|exp| view! {
                        <p class="license-expiry">"Expires: "{exp}</p>
                    })}
                </div>
            }.into_any()
        }
        _ => {
            view! {
                <div class="dashboard-card license-card empty">
                    <h4 class="card-title">"License"</h4>
                    <p class="empty-message">"No license bound to your account."</p>
                    <a href="/" class="btn-primary btn-sm">"Claim License"</a>
                </div>
            }.into_any()
        }
    }
}

/// Activity feed showing recent actions
#[component]
fn ActivityFeed(user_id: String, license_id: String) -> impl IntoView {
    // Placeholder - would fetch from /api/v1/dashboard/activities
    view! {
        <div class="dashboard-card activity-feed">
            <h4 class="card-title">"Recent Activity"</h4>
            <div class="activity-list">
                <p class="empty-message">"Activity tracking coming soon."</p>
            </div>
        </div>
    }
}

/// Quick action buttons
#[component]
fn QuickActions(has_license: bool) -> impl IntoView {
    view! {
        <div class="quick-actions">
            <h4 class="section-title">"Quick Actions"</h4>
            <div class="action-buttons">
                {if has_license {
                    view! {
                        <a href="/tasks" class="action-btn">"View Tasks"</a>
                        <a href="/guides" class="action-btn">"Setup Guides"</a>
                        <a href="/referrals" class="action-btn">"Referral Program"</a>
                    }.into_any()
                } else {
                    view! {
                        <a href="/" class="action-btn primary">"Get Started"</a>
                        <a href="/faq" class="action-btn">"FAQ"</a>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
