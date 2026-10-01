//! Operator Cockpit for R5-13
//!
//! Dashboard for operators to monitor gates, acquisition targets, and exceptions.
//! Protected route - requires operator role.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::hooks::{t, use_user, UserLoadState};
use crate::components::common::DevBanner;

/// Gate status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateStatus {
    pub name: String,
    pub status: GateHealthStatus,
    pub current_value: i64,
    pub threshold: i64,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GateHealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Acquisition target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionTarget {
    pub region: String,
    pub target: i32,
    pub current: i32,
    pub gate_prerequisites: Vec<String>,
    pub gates_met: bool,
}

/// Exception item for drill-down
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionItem {
    pub id: String,
    pub exception_type: String,
    pub description: String,
    pub severity: String,
    pub created_at: String,
    pub count: i32,
}

/// Operator page component (protected, role: operator)
#[component]
pub fn OperatorPage() -> impl IntoView {
    view! {
        <div class="operator-page">
            <OperatorContent />
        </div>
    }
}

/// Operator content with authentication and role check
#[component]
fn OperatorContent() -> impl IntoView {
    let user_ctx = use_user();

    view! {
        {move || {
            match user_ctx.state.get() {
                UserLoadState::Loading => {
                    view! {
                        <div class="operator-loading">
                            <div class="loading-spinner"></div>
                            <p>{t("operator.loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="operator-unauthenticated">
                            <div class="auth-required-card">
                                <h2>{t("operator.signin_required")}</h2>
                                <p>{t("operator.signin_message")}</p>
                                <a href="/" class="btn-primary">{t("operator.go_home")}</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="operator-error">
                            <p>{t("operator.error_loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::Loaded => {
                    match user_ctx.user.get() {
                        Some(profile) => {
                            // Check if user has operator role
                            if profile.role != "operator" && profile.role != "admin" {
                                view! {
                                    <div class="operator-unauthorized">
                                        <div class="unauthorized-card">
                                            <h2>{t("operator.access_denied")}</h2>
                                            <p>{t("operator.no_permission")}</p>
                                            <a href="/dashboard" class="btn-primary">{t("operator.go_dashboard")}</a>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <OperatorCockpit operator_id=profile.id.clone() /> }.into_any()
                            }
                        }
                        None => {
                            view! {
                                <div class="operator-unauthenticated">
                                    <p>{t("operator.signin_message")}</p>
                                    <a href="/" class="btn-primary">{t("operator.go_home")}</a>
                                </div>
                            }.into_any()
                        }
                    }
                }
            }
        }}
    }
}

/// Main operator cockpit view
#[component]
fn OperatorCockpit(operator_id: String) -> impl IntoView {
    // Track which section is active
    let (active_section, set_active_section) = signal("overview".to_string());

    // Mock data freshness timestamp
    let data_freshness = "Updated 5 minutes ago";

    view! {
        <div class="operator-cockpit">
            <DevBanner
                title="Demo Mode"
                description="Gates, targets, and metrics shown below are for demonstration purposes."
            />
            <div class="cockpit-header">
                <div class="header-main">
                    <h1>{t("operator.cockpit_title")}</h1>
                    <p class="cockpit-subtitle">{t("operator.cockpit_subtitle")}</p>
                </div>
                <div class="header-meta">
                    <span class="data-freshness">{data_freshness}</span>
                    <button class="btn-secondary btn-sm">{t("operator.refresh")}</button>
                </div>
            </div>

            // Navigation tabs
            <div class="cockpit-tabs">
                <button
                    class=move || if active_section.get() == "overview" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("overview".to_string())
                >
                    {t("operator.tab_gates_overview")}
                </button>
                <button
                    class=move || if active_section.get() == "targets" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("targets".to_string())
                >
                    {t("operator.tab_acquisition_targets")}
                </button>
                <button
                    class=move || if active_section.get() == "exceptions" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("exceptions".to_string())
                >
                    {t("operator.tab_exceptions")}
                </button>
                <button
                    class=move || if active_section.get() == "forecast" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("forecast".to_string())
                >
                    {t("operator.tab_forecast")}
                </button>
            </div>

            // Tab content
            <div class="cockpit-content">
                {move || {
                    match active_section.get().as_str() {
                        "overview" => view! { <GatesOverviewSection /> }.into_any(),
                        "targets" => view! { <AcquisitionTargetsSection /> }.into_any(),
                        "exceptions" => view! { <ExceptionsSection /> }.into_any(),
                        "forecast" => view! { <ForecastSection /> }.into_any(),
                        _ => view! { <div></div> }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}

/// Gates overview section
#[component]
fn GatesOverviewSection() -> impl IntoView {
    // Mock gate data
    let gates = vec![
        GateStatus {
            name: "Inventory".to_string(),
            status: GateHealthStatus::Healthy,
            current_value: 15_420,
            threshold: 10_000,
            last_updated: "2 min ago".to_string(),
        },
        GateStatus {
            name: "Funding".to_string(),
            status: GateHealthStatus::Warning,
            current_value: 45_000,
            threshold: 50_000,
            last_updated: "5 min ago".to_string(),
        },
        GateStatus {
            name: "Task Supply".to_string(),
            status: GateHealthStatus::Healthy,
            current_value: 2_350,
            threshold: 1_000,
            last_updated: "1 min ago".to_string(),
        },
        GateStatus {
            name: "Support Capacity".to_string(),
            status: GateHealthStatus::Critical,
            current_value: 12,
            threshold: 25,
            last_updated: "3 min ago".to_string(),
        },
    ];

    let healthy_count = gates.iter().filter(|g| g.status == GateHealthStatus::Healthy).count();
    let total_count = gates.len();

    view! {
        <div class="gates-overview">
            <div class="overview-summary">
                <h3>
                    {t("operator.system_health")} " "
                    <span class=if healthy_count == total_count { "healthy" } else { "warning" }>
                        {healthy_count}"/" {total_count} " " {t("operator.gates_healthy")}
                    </span>
                </h3>
            </div>

            <div class="gates-grid">
                {gates.into_iter().map(|gate| view! {
                    <GateCard gate=gate />
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Individual gate card
#[component]
fn GateCard(gate: GateStatus) -> impl IntoView {
    let status_class = match gate.status {
        GateHealthStatus::Healthy => "gate-card healthy",
        GateHealthStatus::Warning => "gate-card warning",
        GateHealthStatus::Critical => "gate-card critical",
        GateHealthStatus::Unknown => "gate-card unknown",
    };

    let status_text = match gate.status {
        GateHealthStatus::Healthy => t("operator.status_healthy"),
        GateHealthStatus::Warning => t("operator.status_warning"),
        GateHealthStatus::Critical => t("operator.status_critical"),
        GateHealthStatus::Unknown => t("operator.status_unknown"),
    };

    let percentage = (gate.current_value as f64 / gate.threshold as f64 * 100.0).min(100.0);
    let progress_style = format!("width: {:.0}%", percentage);

    view! {
        <div class=status_class>
            <div class="gate-header">
                <h4>{gate.name}</h4>
                <span class="gate-status">{status_text}</span>
            </div>
            <div class="gate-metric">
                <span class="metric-current">{gate.current_value.to_string()}</span>
                <span class="metric-separator">" / "</span>
                <span class="metric-threshold">{gate.threshold.to_string()}</span>
            </div>
            <div class="gate-progress">
                <div class="progress-bar">
                    <div class="progress-fill" style=progress_style></div>
                </div>
            </div>
            <div class="gate-footer">
                <span class="gate-updated">{t("operator.updated")} {gate.last_updated}</span>
            </div>
        </div>
    }
}

/// Acquisition targets section
#[component]
fn AcquisitionTargetsSection() -> impl IntoView {
    // Mock target data
    let targets = vec![
        AcquisitionTarget {
            region: "North America".to_string(),
            target: 5000,
            current: 3250,
            gate_prerequisites: vec!["Inventory".to_string(), "Funding".to_string()],
            gates_met: true,
        },
        AcquisitionTarget {
            region: "Europe".to_string(),
            target: 3000,
            current: 1800,
            gate_prerequisites: vec!["Inventory".to_string(), "Support Capacity".to_string()],
            gates_met: false,
        },
        AcquisitionTarget {
            region: "Asia Pacific".to_string(),
            target: 4000,
            current: 2100,
            gate_prerequisites: vec!["Task Supply".to_string()],
            gates_met: true,
        },
        AcquisitionTarget {
            region: "Latin America".to_string(),
            target: 2000,
            current: 850,
            gate_prerequisites: vec!["Inventory".to_string()],
            gates_met: true,
        },
    ];

    view! {
        <div class="acquisition-targets">
            <div class="targets-header">
                <h3>{t("operator.regional_targets")}</h3>
                <p class="targets-note">{t("operator.targets_note")}</p>
            </div>

            <div class="targets-table">
                <div class="table-header">
                    <span>{t("operator.region")}</span>
                    <span>{t("operator.progress")}</span>
                    <span>{t("operator.gate_prerequisites")}</span>
                    <span>{t("operator.status")}</span>
                </div>
                {targets.into_iter().map(|target| view! {
                    <TargetRow target=target />
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Individual target row
#[component]
fn TargetRow(target: AcquisitionTarget) -> impl IntoView {
    let percentage = (target.current as f64 / target.target as f64 * 100.0).min(100.0);
    let progress_style = format!("width: {:.0}%", percentage);
    let progress_text = format!("{} / {} ({:.0}%)", target.current, target.target, percentage);

    let row_class = if !target.gates_met {
        "target-row blocked"
    } else {
        "target-row"
    };

    view! {
        <div class=row_class>
            <span class="target-region">{target.region}</span>
            <div class="target-progress">
                <div class="progress-bar">
                    <div class="progress-fill" style=progress_style></div>
                </div>
                <span class="progress-text">{progress_text}</span>
            </div>
            <div class="target-gates">
                {target.gate_prerequisites.iter().map(|gate| view! {
                    <span class="gate-tag">{gate.clone()}</span>
                }).collect::<Vec<_>>()}
            </div>
            <span class=if target.gates_met { "target-status active" } else { "target-status blocked" }>
                {if target.gates_met { t("operator.active") } else { t("operator.blocked") }}
            </span>
        </div>
    }
}

/// Exceptions drill-down section
#[component]
fn ExceptionsSection() -> impl IntoView {
    // Mock exception data
    let exceptions = vec![
        ExceptionItem {
            id: "ex1".to_string(),
            exception_type: "Payment Failure".to_string(),
            description: "Failed payout attempts".to_string(),
            severity: "high".to_string(),
            created_at: "2026-09-30".to_string(),
            count: 15,
        },
        ExceptionItem {
            id: "ex2".to_string(),
            exception_type: "License Conflict".to_string(),
            description: "Duplicate license claims".to_string(),
            severity: "medium".to_string(),
            created_at: "2026-09-29".to_string(),
            count: 8,
        },
        ExceptionItem {
            id: "ex3".to_string(),
            exception_type: "Support Backlog".to_string(),
            description: "Tickets pending >48h".to_string(),
            severity: "high".to_string(),
            created_at: "2026-09-30".to_string(),
            count: 23,
        },
        ExceptionItem {
            id: "ex4".to_string(),
            exception_type: "Data Sync".to_string(),
            description: "Stale cohort metrics".to_string(),
            severity: "low".to_string(),
            created_at: "2026-09-28".to_string(),
            count: 3,
        },
    ];

    let high_severity = exceptions.iter().filter(|e| e.severity == "high").count();

    view! {
        <div class="exceptions-section">
            <div class="exceptions-header">
                <h3>
                    {t("operator.active_exceptions")}
                    {(high_severity > 0).then(|| view! {
                        <span class="high-severity-badge">{high_severity}" " {t("operator.high_severity")}</span>
                    })}
                </h3>
            </div>

            <div class="exceptions-list">
                {exceptions.into_iter().map(|exception| view! {
                    <ExceptionCard exception=exception />
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Individual exception card
#[component]
fn ExceptionCard(exception: ExceptionItem) -> impl IntoView {
    let severity_class = match exception.severity.as_str() {
        "high" => "exception-card high",
        "medium" => "exception-card medium",
        "low" => "exception-card low",
        _ => "exception-card",
    };

    view! {
        <div class=severity_class>
            <div class="exception-header">
                <span class="exception-type">{exception.exception_type}</span>
                <span class="exception-count">{exception.count}" " {t("operator.occurrences")}</span>
            </div>
            <p class="exception-description">{exception.description}</p>
            <div class="exception-footer">
                <span class="exception-date">{t("operator.since")} {exception.created_at}</span>
                <div class="exception-actions">
                    <button class="btn-text">{t("operator.view_details")}</button>
                    <button class="btn-primary btn-sm">{t("operator.investigate")}</button>
                </div>
            </div>
        </div>
    }
}

/// Forecast comparison section
#[component]
fn ForecastSection() -> impl IntoView {
    view! {
        <div class="forecast-section">
            <div class="forecast-header">
                <h3>{t("operator.scenario_comparison")}</h3>
                <p class="forecast-note">{t("operator.scenario_comparison_note")}</p>
            </div>

            <div class="scenario-grid">
                // Conservative scenario
                <div class="scenario-card">
                    <h4>{t("operator.scenario_conservative")}</h4>
                    <div class="scenario-metrics">
                        <div class="metric-row">
                            <span>{t("operator.new_users_30d")}</span>
                            <span class="metric-value">"2,500"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.license_demand")}</span>
                            <span class="metric-value">"3,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.funding_required")}</span>
                            <span class="metric-value">"$45,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.support_tickets")}</span>
                            <span class="metric-value">"~200"</span>
                        </div>
                    </div>
                    <div class="scenario-gates">
                        <span class="gate-status ok">{t("operator.all_gates_met")}</span>
                    </div>
                </div>

                // Moderate scenario
                <div class="scenario-card highlighted">
                    <div class="scenario-badge">{t("operator.current_trajectory")}</div>
                    <h4>{t("operator.scenario_moderate")}</h4>
                    <div class="scenario-metrics">
                        <div class="metric-row">
                            <span>{t("operator.new_users_30d")}</span>
                            <span class="metric-value">"5,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.license_demand")}</span>
                            <span class="metric-value">"6,500"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.funding_required")}</span>
                            <span class="metric-value">"$95,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.support_tickets")}</span>
                            <span class="metric-value">"~450"</span>
                        </div>
                    </div>
                    <div class="scenario-gates">
                        <span class="gate-status warning">{t("operator.support_capacity_needed")}</span>
                    </div>
                </div>

                // Aggressive scenario
                <div class="scenario-card">
                    <h4>{t("operator.scenario_aggressive")}</h4>
                    <div class="scenario-metrics">
                        <div class="metric-row">
                            <span>{t("operator.new_users_30d")}</span>
                            <span class="metric-value">"10,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.license_demand")}</span>
                            <span class="metric-value">"13,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.funding_required")}</span>
                            <span class="metric-value">"$190,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>{t("operator.support_tickets")}</span>
                            <span class="metric-value">"~900"</span>
                        </div>
                    </div>
                    <div class="scenario-gates">
                        <span class="gate-status critical">{t("operator.gates_blocked")}</span>
                    </div>
                </div>
            </div>

            <div class="forecast-disclaimer">
                <span class="info-icon">"ℹ"</span>
                <p>{t("operator.forecast_disclaimer")}</p>
            </div>
        </div>
    }
}
