//! Account Page for R5-13
//!
//! Displays user profile, setup progress, license status, and account settings.
//! Protected route - requires authentication.

use leptos::prelude::*;
use crate::hooks::{use_user, UserLoadState, UserProfile, LicenseStatus, CohortStatus};

/// Account page component (protected)
#[component]
pub fn AccountPage() -> impl IntoView {
    view! {
        <div class="account-page">
            <AccountContent />
        </div>
    }
}

/// Account content with authentication check
#[component]
fn AccountContent() -> impl IntoView {
    let user_ctx = use_user();

    view! {
        {move || {
            match user_ctx.state.get() {
                UserLoadState::Loading => {
                    view! { <AccountLoading /> }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! { <AccountUnauthenticated /> }.into_any()
                }
                UserLoadState::Error => {
                    let error_msg = user_ctx.error.get().unwrap_or_else(|| "Unknown error".to_string());
                    view! { <AccountError message=error_msg /> }.into_any()
                }
                UserLoadState::Loaded => {
                    match user_ctx.user.get() {
                        Some(profile) => {
                            view! { <AuthenticatedAccount profile=profile /> }.into_any()
                        }
                        None => {
                            view! { <AccountUnauthenticated /> }.into_any()
                        }
                    }
                }
            }
        }}
    }
}

/// Loading state
#[component]
fn AccountLoading() -> impl IntoView {
    view! {
        <div class="account-loading">
            <div class="loading-spinner"></div>
            <p>"Loading your account..."</p>
        </div>
    }
}

/// Unauthenticated state
#[component]
fn AccountUnauthenticated() -> impl IntoView {
    view! {
        <div class="account-unauthenticated">
            <div class="auth-required-card">
                <h2>"Sign In Required"</h2>
                <p>"Please sign in to view your account."</p>
                <a href="/" class="btn-primary">"Go to Home"</a>
            </div>
        </div>
    }
}

/// Error state
#[component]
fn AccountError(message: String) -> impl IntoView {
    view! {
        <div class="account-error">
            <div class="error-card">
                <h2>"Something went wrong"</h2>
                <p>{message}</p>
                <button class="btn-secondary" on:click=move |_| {
                    #[cfg(any(feature = "csr", feature = "hydrate"))]
                    {
                        let _ = web_sys::window().unwrap().location().reload();
                    }
                }>"Try Again"</button>
            </div>
        </div>
    }
}

/// Authenticated account view
#[component]
fn AuthenticatedAccount(profile: UserProfile) -> impl IntoView {
    let user_id = profile.id.clone();
    let role = profile.role.clone();
    let license_id = profile.license_id.clone();
    let license_status = profile.license_status.clone();
    let cohort_status = profile.cohort_status.clone();
    let has_active_exit = profile.has_active_exit;

    view! {
        <div class="account-authenticated">
            <div class="account-header">
                <h1>"Your Account"</h1>
                <p class="account-subtitle">"Manage your profile and settings"</p>
            </div>

            <div class="account-grid">
                // Profile Card
                <ProfileCard user_id=user_id.clone() role=role.clone() />

                // Setup Progress Card
                <SetupProgressCard
                    license_id=license_id.clone()
                    cohort_status=cohort_status.clone()
                />

                // License Status Card
                <LicenseStatusCard
                    license_id=license_id.clone()
                    license_status=license_status.clone()
                    has_active_exit=has_active_exit
                />

                // Quick Actions Card
                <QuickActionsCard has_license=license_id.is_some() />
            </div>

            // Account Settings Section
            <AccountSettings />
        </div>
    }
}

/// Profile information card
#[component]
fn ProfileCard(user_id: String, role: String) -> impl IntoView {
    let role_display = match role.as_str() {
        "admin" => "Administrator".to_string(),
        "operator" => "Operator".to_string(),
        "agent" => "Agent".to_string(),
        "participant" => "Participant".to_string(),
        _ => role.clone(),
    };

    let short_id = if user_id.len() > 8 {
        format!("{}...", &user_id[..8])
    } else {
        user_id.clone()
    };

    view! {
        <div class="account-card profile-card">
            <div class="card-header">
                <h3>"Profile"</h3>
            </div>
            <div class="card-content">
                <div class="profile-avatar">
                    <span class="avatar-icon">"U"</span>
                </div>
                <div class="profile-info">
                    <div class="info-row">
                        <span class="info-label">"User ID"</span>
                        <span class="info-value">{short_id}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">"Role"</span>
                        <span class="info-value role-badge">{role_display}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Setup progress card
#[component]
fn SetupProgressCard(
    license_id: Option<String>,
    cohort_status: Option<CohortStatus>,
) -> impl IntoView {
    // Calculate setup progress
    let (progress_percent, current_step, total_steps) = match (&license_id, &cohort_status) {
        (None, _) => (0, 0, 5),
        (Some(_), None) => (20, 1, 5),
        (Some(_), Some(c)) => {
            if c.d30_completed {
                (100, 5, 5)
            } else if c.d7_completed {
                (80, 4, 5)
            } else if c.d1_completed {
                (60, 3, 5)
            } else {
                (40, 2, 5)
            }
        }
    };

    let progress_style = format!("width: {}%", progress_percent);

    view! {
        <div class="account-card setup-progress-card">
            <div class="card-header">
                <h3>"Setup Progress"</h3>
                <span class="progress-label">{current_step}"/" {total_steps}</span>
            </div>
            <div class="card-content">
                <div class="progress-bar">
                    <div class="progress-fill" style=progress_style></div>
                </div>
                <div class="setup-steps">
                    <SetupStep
                        number=1
                        label="Claim License"
                        completed=license_id.is_some()
                        active=license_id.is_none()
                    />
                    <SetupStep
                        number=2
                        label="Install App"
                        completed=cohort_status.as_ref().map(|c| c.d1_completed).unwrap_or(false)
                        active=license_id.is_some() && cohort_status.as_ref().map(|c| !c.d1_completed).unwrap_or(true)
                    />
                    <SetupStep
                        number=3
                        label="First Active Day"
                        completed=cohort_status.as_ref().map(|c| c.d1_completed).unwrap_or(false)
                        active=false
                    />
                    <SetupStep
                        number=4
                        label="D7 Milestone"
                        completed=cohort_status.as_ref().map(|c| c.d7_completed).unwrap_or(false)
                        active=cohort_status.as_ref().map(|c| c.d1_completed && !c.d7_completed).unwrap_or(false)
                    />
                    <SetupStep
                        number=5
                        label="D30 Graduation"
                        completed=cohort_status.as_ref().map(|c| c.d30_completed).unwrap_or(false)
                        active=cohort_status.as_ref().map(|c| c.d7_completed && !c.d30_completed).unwrap_or(false)
                    />
                </div>

                {if license_id.is_none() {
                    view! {
                        <a href="/" class="btn-primary btn-sm">"Get Started"</a>
                    }.into_any()
                } else if cohort_status.as_ref().map(|c| !c.d1_completed).unwrap_or(true) {
                    view! {
                        <a href="/setup" class="btn-primary btn-sm">"Continue Setup"</a>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        </div>
    }
}

/// Individual setup step
#[component]
fn SetupStep(number: u8, label: &'static str, completed: bool, active: bool) -> impl IntoView {
    let class = if completed {
        "setup-step completed"
    } else if active {
        "setup-step active"
    } else {
        "setup-step"
    };

    view! {
        <div class=class>
            <div class="step-indicator">
                {if completed {
                    view! { <span class="check-icon">"✓"</span> }.into_any()
                } else {
                    view! { <span>{number}</span> }.into_any()
                }}
            </div>
            <span class="step-label">{label}</span>
        </div>
    }
}

/// License status card
#[component]
fn LicenseStatusCard(
    license_id: Option<String>,
    license_status: Option<LicenseStatus>,
    has_active_exit: bool,
) -> impl IntoView {
    view! {
        <div class="account-card license-status-card">
            <div class="card-header">
                <h3>"License"</h3>
            </div>
            <div class="card-content">
                {match (license_id, license_status) {
                    (Some(lid), Some(status)) => {
                        let status_class = match status.state.as_str() {
                            "active" => "status-badge active",
                            "exiting" => "status-badge exiting",
                            "suspended" => "status-badge suspended",
                            "expired" => "status-badge expired",
                            _ => "status-badge",
                        };

                        view! {
                            <div class="license-info">
                                <div class="license-id">
                                    <span class="label">"License ID"</span>
                                    <span class="value">{lid[..12.min(lid.len())].to_string()}"..."</span>
                                </div>
                                <div class="license-state">
                                    <span class="label">"Status"</span>
                                    <span class=status_class>{status.state}</span>
                                </div>
                                {status.variant.map(|v| view! {
                                    <div class="license-variant">
                                        <span class="label">"Variant"</span>
                                        <span class="value">{v}</span>
                                    </div>
                                })}
                                {status.expires_at.map(|exp| view! {
                                    <div class="license-expiry">
                                        <span class="label">"Expires"</span>
                                        <span class="value">{exp}</span>
                                    </div>
                                })}
                                {has_active_exit.then(|| view! {
                                    <div class="exit-notice">
                                        <span class="warning-icon">"!"</span>
                                        <span>"Exit request in progress"</span>
                                    </div>
                                })}
                            </div>
                        }.into_any()
                    }
                    _ => {
                        view! {
                            <div class="no-license">
                                <p>"No license associated with your account."</p>
                                <a href="/" class="btn-primary btn-sm">"Claim a License"</a>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

/// Quick actions card
#[component]
fn QuickActionsCard(has_license: bool) -> impl IntoView {
    view! {
        <div class="account-card quick-actions-card">
            <div class="card-header">
                <h3>"Quick Actions"</h3>
            </div>
            <div class="card-content">
                <div class="action-list">
                    <a href="/dashboard" class="action-item">
                        <span class="action-icon">"📊"</span>
                        <span class="action-label">"Dashboard"</span>
                    </a>
                    {has_license.then(|| view! {
                        <a href="/setup" class="action-item">
                            <span class="action-icon">"📱"</span>
                            <span class="action-label">"App Setup"</span>
                        </a>
                    })}
                    <a href="/guides" class="action-item">
                        <span class="action-icon">"📖"</span>
                        <span class="action-label">"Guides"</span>
                    </a>
                    <a href="/contact" class="action-item">
                        <span class="action-icon">"💬"</span>
                        <span class="action-label">"Support"</span>
                    </a>
                    <a href="/referrals" class="action-item">
                        <span class="action-icon">"👥"</span>
                        <span class="action-label">"Referrals"</span>
                    </a>
                </div>
            </div>
        </div>
    }
}

/// Account settings section
#[component]
fn AccountSettings() -> impl IntoView {
    view! {
        <div class="account-settings">
            <h2>"Settings"</h2>
            <div class="settings-grid">
                // Notification preferences
                <div class="settings-card">
                    <h4>"Notifications"</h4>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" checked=true />
                            <span>"Email notifications"</span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" checked=true />
                            <span>"Push notifications"</span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" />
                            <span>"Marketing updates"</span>
                        </label>
                    </div>
                </div>

                // Privacy settings
                <div class="settings-card">
                    <h4>"Privacy"</h4>
                    <div class="setting-item">
                        <a href="/privacy" class="settings-link">"Privacy Policy"</a>
                    </div>
                    <div class="setting-item">
                        <a href="/terms" class="settings-link">"Terms of Service"</a>
                    </div>
                    <div class="setting-item">
                        <button class="btn-text">"Download My Data"</button>
                    </div>
                </div>

                // Danger zone
                <div class="settings-card danger">
                    <h4>"Account Actions"</h4>
                    <div class="setting-item">
                        <button class="btn-text warning">"Pause License"</button>
                    </div>
                    <div class="setting-item">
                        <button class="btn-text danger">"Request Exit"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
