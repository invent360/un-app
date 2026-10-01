//! Account Page for R5-13
//!
//! Displays user profile, setup progress, license status, and account settings.
//! Protected route - requires authentication.

use leptos::prelude::*;
use crate::hooks::{use_user, UserLoadState, UserProfile, LicenseStatus, CohortStatus, t};

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
                    let error_msg = user_ctx.error.get().unwrap_or_else(|| t("account.unknown_error"));
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
            <p>{t("account.loading")}</p>
        </div>
    }
}

/// Unauthenticated state
#[component]
fn AccountUnauthenticated() -> impl IntoView {
    view! {
        <div class="account-unauthenticated">
            <div class="auth-required-card">
                <h2>{t("account.signin_required")}</h2>
                <p>{t("account.signin_message")}</p>
                <a href="/" class="btn-primary">{t("account.go_home")}</a>
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
                <h2>{t("account.error_title")}</h2>
                <p>{message}</p>
                <button class="btn-secondary" on:click=move |_| {
                    #[cfg(any(feature = "csr", feature = "hydrate"))]
                    {
                        let _ = web_sys::window().unwrap().location().reload();
                    }
                }>{t("account.try_again")}</button>
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
                <h1>{t("account.title")}</h1>
                <p class="account-subtitle">{t("account.subtitle")}</p>
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
        "admin" => t("account.role_admin"),
        "operator" => t("account.role_operator"),
        "agent" => t("account.role_agent"),
        "participant" => t("account.role_participant"),
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
                <h3>{t("account.profile_section")}</h3>
            </div>
            <div class="card-content">
                <div class="profile-avatar">
                    <span class="avatar-icon">"U"</span>
                </div>
                <div class="profile-info">
                    <div class="info-row">
                        <span class="info-label">{t("account.user_id")}</span>
                        <span class="info-value">{short_id}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">{t("account.role")}</span>
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
                <h3>{t("account.setup_progress")}</h3>
                <span class="progress-label">{current_step}"/" {total_steps}</span>
            </div>
            <div class="card-content">
                <div class="progress-bar">
                    <div class="progress-fill" style=progress_style></div>
                </div>
                <div class="setup-steps">
                    <SetupStep
                        number=1
                        label=t("account.step_claim_license")
                        completed=license_id.is_some()
                        active=license_id.is_none()
                    />
                    <SetupStep
                        number=2
                        label=t("account.step_install_app")
                        completed=cohort_status.as_ref().map(|c| c.d1_completed).unwrap_or(false)
                        active=license_id.is_some() && cohort_status.as_ref().map(|c| !c.d1_completed).unwrap_or(true)
                    />
                    <SetupStep
                        number=3
                        label=t("account.step_first_active_day")
                        completed=cohort_status.as_ref().map(|c| c.d1_completed).unwrap_or(false)
                        active=false
                    />
                    <SetupStep
                        number=4
                        label=t("account.step_d7_milestone")
                        completed=cohort_status.as_ref().map(|c| c.d7_completed).unwrap_or(false)
                        active=cohort_status.as_ref().map(|c| c.d1_completed && !c.d7_completed).unwrap_or(false)
                    />
                    <SetupStep
                        number=5
                        label=t("account.step_d30_graduation")
                        completed=cohort_status.as_ref().map(|c| c.d30_completed).unwrap_or(false)
                        active=cohort_status.as_ref().map(|c| c.d7_completed && !c.d30_completed).unwrap_or(false)
                    />
                </div>

                {if license_id.is_none() {
                    view! {
                        <a href="/" class="btn-primary btn-sm">{t("account.get_started")}</a>
                    }.into_any()
                } else if cohort_status.as_ref().map(|c| !c.d1_completed).unwrap_or(true) {
                    view! {
                        <a href="/setup" class="btn-primary btn-sm">{t("account.continue_setup")}</a>
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
fn SetupStep(number: u8, label: String, completed: bool, active: bool) -> impl IntoView {
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
                <h3>{t("account.license")}</h3>
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
                                    <span class="label">{t("account.license_id")}</span>
                                    <span class="value">{lid[..12.min(lid.len())].to_string()}"..."</span>
                                </div>
                                <div class="license-state">
                                    <span class="label">{t("account.status")}</span>
                                    <span class=status_class>{status.state}</span>
                                </div>
                                {status.variant.map(|v| view! {
                                    <div class="license-variant">
                                        <span class="label">{t("account.variant")}</span>
                                        <span class="value">{v}</span>
                                    </div>
                                })}
                                {status.expires_at.map(|exp| view! {
                                    <div class="license-expiry">
                                        <span class="label">{t("account.expires")}</span>
                                        <span class="value">{exp}</span>
                                    </div>
                                })}
                                {has_active_exit.then(|| view! {
                                    <div class="exit-notice">
                                        <span class="warning-icon">"!"</span>
                                        <span>{t("account.exit_in_progress")}</span>
                                    </div>
                                })}
                            </div>
                        }.into_any()
                    }
                    _ => {
                        view! {
                            <div class="no-license">
                                <p>{t("account.no_license")}</p>
                                <a href="/" class="btn-primary btn-sm">{t("account.claim_license")}</a>
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
                <h3>{t("account.quick_actions")}</h3>
            </div>
            <div class="card-content">
                <div class="action-list">
                    <a href="/dashboard" class="action-item">
                        <span class="action-icon">"📊"</span>
                        <span class="action-label">{t("account.action_dashboard")}</span>
                    </a>
                    {has_license.then(|| view! {
                        <a href="/setup" class="action-item">
                            <span class="action-icon">"📱"</span>
                            <span class="action-label">{t("account.action_app_setup")}</span>
                        </a>
                    })}
                    <a href="/guides" class="action-item">
                        <span class="action-icon">"📖"</span>
                        <span class="action-label">{t("account.action_guides")}</span>
                    </a>
                    <a href="/contact" class="action-item">
                        <span class="action-icon">"💬"</span>
                        <span class="action-label">{t("account.action_support")}</span>
                    </a>
                    <a href="/referrals" class="action-item">
                        <span class="action-icon">"👥"</span>
                        <span class="action-label">{t("account.action_referrals")}</span>
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
            <h2>{t("account.settings")}</h2>
            <div class="settings-grid">
                // Notification preferences
                <div class="settings-card">
                    <h4>{t("account.notifications")}</h4>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" checked=true />
                            <span>{t("account.email_notifications")}</span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" checked=true />
                            <span>{t("account.push_notifications")}</span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" />
                            <span>{t("account.marketing_updates")}</span>
                        </label>
                    </div>
                </div>

                // Privacy settings
                <div class="settings-card">
                    <h4>{t("account.privacy")}</h4>
                    <div class="setting-item">
                        <a href="/privacy" class="settings-link">{t("account.privacy_policy")}</a>
                    </div>
                    <div class="setting-item">
                        <a href="/terms" class="settings-link">{t("account.terms_of_service")}</a>
                    </div>
                    <div class="setting-item">
                        <button class="btn-text">{t("account.download_my_data")}</button>
                    </div>
                </div>

                // Danger zone
                <div class="settings-card danger">
                    <h4>{t("account.account_actions")}</h4>
                    <div class="setting-item">
                        <button class="btn-text warning">{t("account.pause_license")}</button>
                    </div>
                    <div class="setting-item">
                        <button class="btn-text danger">{t("account.request_exit")}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
