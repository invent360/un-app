//! Agent Workspace for R5-13
//!
//! Dashboard for agents to manage their assigned users, referrals, and commissions.
//! Protected route - requires agent role.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::hooks::{use_user, UserLoadState, t};
use crate::components::common::DevBanner;

/// Assigned user for agent queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedUser {
    pub id: String,
    pub display_name: String,
    pub status: String,
    pub cohort_day: i32,
    pub last_active: String,
    pub needs_attention: bool,
    pub blocked_reason: Option<String>,
}

/// Referral stats for agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralStats {
    pub total_referrals: i32,
    pub active_referrals: i32,
    pub pending_activation: i32,
    pub conversion_rate: f64,
}

/// Commission summary for agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionSummary {
    pub earned_micros: i64,
    pub pending_micros: i64,
    pub paid_micros: i64,
}

/// Agent page component (protected, role: agent)
#[component]
pub fn AgentPage() -> impl IntoView {
    view! {
        <div class="agent-page">
            <AgentContent />
        </div>
    }
}

/// Agent content with authentication and role check
#[component]
fn AgentContent() -> impl IntoView {
    let user_ctx = use_user();

    view! {
        {move || {
            match user_ctx.state.get() {
                UserLoadState::Loading => {
                    view! {
                        <div class="agent-loading">
                            <div class="loading-spinner"></div>
                            <p>{t("agent.loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="agent-unauthenticated">
                            <div class="auth-required-card">
                                <h2>{t("agent.signin_required")}</h2>
                                <p>{t("agent.signin_message")}</p>
                                <a href="/" class="btn-primary">{t("agent.go_home")}</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="agent-error">
                            <p>{t("agent.error_loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::Loaded => {
                    match user_ctx.user.get() {
                        Some(profile) => {
                            // Check if user has agent role
                            if profile.role != "agent" && profile.role != "admin" {
                                view! {
                                    <div class="agent-unauthorized">
                                        <div class="unauthorized-card">
                                            <h2>{t("agent.access_denied")}</h2>
                                            <p>{t("agent.no_permission")}</p>
                                            <a href="/dashboard" class="btn-primary">{t("agent.go_dashboard")}</a>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <AgentWorkspace agent_id=profile.id.clone() /> }.into_any()
                            }
                        }
                        None => {
                            view! {
                                <div class="agent-unauthenticated">
                                    <p>{t("agent.signin_message")}</p>
                                    <a href="/" class="btn-primary">{t("agent.go_home")}</a>
                                </div>
                            }.into_any()
                        }
                    }
                }
            }
        }}
    }
}

/// Main agent workspace view
#[component]
fn AgentWorkspace(agent_id: String) -> impl IntoView {
    // Mock data - would be fetched from API
    let referral_code = "UNO-AGENT-ABC123".to_string();
    let referral_link = format!("https://uno.network/ref/{}", referral_code);

    let commission = CommissionSummary {
        earned_micros: 125_500_000, // $125.50
        pending_micros: 45_200_000, // $45.20
        paid_micros: 350_000_000,   // $350.00
    };

    let referral_stats = ReferralStats {
        total_referrals: 48,
        active_referrals: 32,
        pending_activation: 8,
        conversion_rate: 0.67,
    };

    view! {
        <div class="agent-workspace">
            <DevBanner
                title="Demo Mode"
                description="Commission and referral data shown below is for demonstration purposes."
            />
            <div class="workspace-header">
                <h1>{t("agent.workspace_title")}</h1>
                <p class="workspace-subtitle">{t("agent.workspace_subtitle")}</p>
            </div>

            // Stats overview
            <div class="agent-stats">
                <StatsCard
                    title=t("agent.total_referrals")
                    value=referral_stats.total_referrals.to_string()
                    icon="👥"
                />
                <StatsCard
                    title=t("agent.active_users")
                    value=referral_stats.active_referrals.to_string()
                    icon="✅"
                />
                <StatsCard
                    title=t("agent.conversion_rate")
                    value=format!("{:.0}%", referral_stats.conversion_rate * 100.0)
                    icon="📈"
                />
                <StatsCard
                    title=t("agent.pending_activation")
                    value=referral_stats.pending_activation.to_string()
                    icon="⏳"
                />
            </div>

            // Main content grid
            <div class="workspace-grid">
                // Referral card
                <ReferralCard
                    referral_code=referral_code.clone()
                    referral_link=referral_link.clone()
                />

                // Commission card
                <CommissionCard commission=commission />
            </div>

            // User queue
            <UserQueueSection agent_id=agent_id />
        </div>
    }
}

/// Stats card component
#[component]
fn StatsCard(title: String, value: String, icon: &'static str) -> impl IntoView {
    view! {
        <div class="stats-card">
            <div class="stats-icon">{icon}</div>
            <div class="stats-content">
                <span class="stats-value">{value}</span>
                <span class="stats-title">{title}</span>
            </div>
        </div>
    }
}

/// Referral link card
#[component]
fn ReferralCard(referral_code: String, referral_link: String) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let link_clone = referral_link.clone();

    let copy_link = move |_| {
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let link = link_clone.clone();
                let _ = clipboard.write_text(&link);
                set_copied.set(true);

                leptos::task::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    set_copied.set(false);
                });
            }
        }
    };

    view! {
        <div class="referral-card">
            <h3>{t("agent.your_referral_link")}</h3>

            <div class="referral-code-display">
                <span class="code-label">{t("agent.code_label")}</span>
                <span class="code-value">{referral_code}</span>
            </div>

            <div class="referral-link-box">
                <input
                    type="text"
                    readonly=true
                    value=referral_link.clone()
                    class="link-input"
                />
                <button class="copy-btn" on:click=copy_link>
                    {move || if copied.get() { t("agent.copied") } else { t("agent.copy") }}
                </button>
            </div>

            <div class="referral-tips">
                <h4>{t("agent.tips_title")}</h4>
                <ul>
                    <li>{t("agent.tip_1")}</li>
                    <li>{t("agent.tip_2")}</li>
                    <li>{t("agent.tip_3")}</li>
                </ul>
            </div>
        </div>
    }
}

/// Commission summary card
#[component]
fn CommissionCard(commission: CommissionSummary) -> impl IntoView {
    let format_amount = |micros: i64| -> String {
        format!("${:.2}", micros as f64 / 1_000_000.0)
    };

    let earned = format_amount(commission.earned_micros);
    let pending = format_amount(commission.pending_micros);
    let paid = format_amount(commission.paid_micros);
    let total = format_amount(commission.earned_micros + commission.pending_micros + commission.paid_micros);

    view! {
        <div class="commission-card">
            <h3>{t("agent.commission_summary")}</h3>

            <div class="commission-breakdown">
                <div class="commission-row earned">
                    <span class="commission-label">{t("agent.earned")}</span>
                    <span class="commission-value">{earned}</span>
                </div>
                <div class="commission-row pending">
                    <span class="commission-label">{t("agent.pending")}</span>
                    <span class="commission-value">{pending}</span>
                </div>
                <div class="commission-row paid">
                    <span class="commission-label">{t("agent.paid_out")}</span>
                    <span class="commission-value">{paid}</span>
                </div>
                <div class="commission-row total">
                    <span class="commission-label">{t("agent.total_lifetime")}</span>
                    <span class="commission-value">{total}</span>
                </div>
            </div>

            <p class="commission-note">{t("agent.commission_note")}</p>
        </div>
    }
}

/// User queue section
#[component]
fn UserQueueSection(agent_id: String) -> impl IntoView {
    // Mock data - would be fetched from API
    let assigned_users: Vec<AssignedUser> = vec![
        AssignedUser {
            id: "u1".to_string(),
            display_name: "John D.".to_string(),
            status: "active".to_string(),
            cohort_day: 12,
            last_active: "2 hours ago".to_string(),
            needs_attention: false,
            blocked_reason: None,
        },
        AssignedUser {
            id: "u2".to_string(),
            display_name: "Sarah M.".to_string(),
            status: "blocked".to_string(),
            cohort_day: 5,
            last_active: "3 days ago".to_string(),
            needs_attention: true,
            blocked_reason: Some("Payment verification needed".to_string()),
        },
        AssignedUser {
            id: "u3".to_string(),
            display_name: "Mike P.".to_string(),
            status: "pending".to_string(),
            cohort_day: 0,
            last_active: "1 day ago".to_string(),
            needs_attention: true,
            blocked_reason: None,
        },
        AssignedUser {
            id: "u4".to_string(),
            display_name: "Emma L.".to_string(),
            status: "active".to_string(),
            cohort_day: 28,
            last_active: "30 min ago".to_string(),
            needs_attention: false,
            blocked_reason: None,
        },
    ];

    // Sort by needs_attention first, then by cohort_day
    let mut sorted_users = assigned_users;
    sorted_users.sort_by(|a, b| {
        b.needs_attention.cmp(&a.needs_attention)
            .then(a.cohort_day.cmp(&b.cohort_day))
    });

    let attention_count = sorted_users.iter().filter(|u| u.needs_attention).count();

    view! {
        <div class="user-queue-section">
            <div class="queue-header">
                <h3>
                    {t("agent.assigned_users")}
                    {(attention_count > 0).then(|| view! {
                        <span class="attention-badge">{attention_count}" "{t("agent.need_attention")}</span>
                    })}
                </h3>
            </div>

            <div class="user-queue">
                {sorted_users.into_iter().map(|user| view! {
                    <UserQueueCard user=user />
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Individual user card in queue
#[component]
fn UserQueueCard(user: AssignedUser) -> impl IntoView {
    let status_class = match user.status.as_str() {
        "active" => "status-badge active",
        "blocked" => "status-badge blocked",
        "pending" => "status-badge pending",
        "exiting" => "status-badge exiting",
        _ => "status-badge",
    };

    let card_class = if user.needs_attention {
        "user-queue-card needs-attention"
    } else {
        "user-queue-card"
    };

    // Extract first character for avatar before consuming display_name
    let avatar_initial = user.display_name.chars().next().unwrap_or('?').to_string();
    let display_name = user.display_name;
    let status = user.status;
    let cohort_day = user.cohort_day;
    let last_active = user.last_active;
    let blocked_reason = user.blocked_reason;
    let needs_attention = user.needs_attention;

    view! {
        <div class=card_class>
            <div class="user-info">
                <div class="user-avatar">{avatar_initial}</div>
                <div class="user-details">
                    <span class="user-name">{display_name}</span>
                    <span class="user-meta">
                        {t("agent.day")} " " {cohort_day} " • " {t("agent.last_active")} ": " {last_active}
                    </span>
                </div>
            </div>
            <div class="user-status">
                <span class=status_class>{status}</span>
                {blocked_reason.map(|reason| view! {
                    <span class="blocked-reason">{reason}</span>
                })}
            </div>
            <div class="user-actions">
                <button class="btn-text">{t("agent.view")}</button>
                {needs_attention.then(|| view! {
                    <button class="btn-primary btn-sm">{t("agent.resolve")}</button>
                })}
            </div>
        </div>
    }
}
