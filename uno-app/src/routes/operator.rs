//! Operator Cockpit for R5-13
//!
//! Dashboard for operators to monitor gates, acquisition targets, and exceptions.
//! Protected route - requires operator role.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::hooks::{use_user, UserLoadState};

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
                            <p>"Loading operator cockpit..."</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="operator-unauthenticated">
                            <div class="auth-required-card">
                                <h2>"Sign In Required"</h2>
                                <p>"Please sign in to access the operator cockpit."</p>
                                <a href="/" class="btn-primary">"Go to Home"</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="operator-error">
                            <p>"Error loading operator cockpit. Please try again."</p>
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
                                            <h2>"Access Denied"</h2>
                                            <p>"You don't have permission to access the operator cockpit."</p>
                                            <a href="/dashboard" class="btn-primary">"Go to Dashboard"</a>
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
                                    <p>"Please sign in to access the operator cockpit."</p>
                                    <a href="/" class="btn-primary">"Go Home"</a>
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
            <div class="cockpit-header">
                <div class="header-main">
                    <h1>"Operator Cockpit"</h1>
                    <p class="cockpit-subtitle">"Monitor gates, targets, and exceptions"</p>
                </div>
                <div class="header-meta">
                    <span class="data-freshness">{data_freshness}</span>
                    <button class="btn-secondary btn-sm">"Refresh"</button>
                </div>
            </div>

            // Navigation tabs
            <div class="cockpit-tabs">
                <button
                    class=move || if active_section.get() == "overview" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("overview".to_string())
                >
                    "Gates Overview"
                </button>
                <button
                    class=move || if active_section.get() == "targets" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("targets".to_string())
                >
                    "Acquisition Targets"
                </button>
                <button
                    class=move || if active_section.get() == "exceptions" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("exceptions".to_string())
                >
                    "Exceptions"
                </button>
                <button
                    class=move || if active_section.get() == "forecast" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("forecast".to_string())
                >
                    "Forecast"
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
                    "System Health: "
                    <span class=if healthy_count == total_count { "healthy" } else { "warning" }>
                        {healthy_count}"/" {total_count} " gates healthy"
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
        GateHealthStatus::Healthy => "Healthy",
        GateHealthStatus::Warning => "Warning",
        GateHealthStatus::Critical => "Critical",
        GateHealthStatus::Unknown => "Unknown",
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
                <span class="gate-updated">"Updated: "{gate.last_updated}</span>
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
                <h3>"Regional Acquisition Targets"</h3>
                <p class="targets-note">"Targets shown with gate prerequisite status"</p>
            </div>

            <div class="targets-table">
                <div class="table-header">
                    <span>"Region"</span>
                    <span>"Progress"</span>
                    <span>"Gate Prerequisites"</span>
                    <span>"Status"</span>
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
                {if target.gates_met { "Active" } else { "Blocked" }}
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
                    "Active Exceptions"
                    {(high_severity > 0).then(|| view! {
                        <span class="high-severity-badge">{high_severity}" high severity"</span>
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
                <span class="exception-count">{exception.count}" occurrences"</span>
            </div>
            <p class="exception-description">{exception.description}</p>
            <div class="exception-footer">
                <span class="exception-date">"Since: "{exception.created_at}</span>
                <div class="exception-actions">
                    <button class="btn-text">"View Details"</button>
                    <button class="btn-primary btn-sm">"Investigate"</button>
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
                <h3>"Scenario Comparison"</h3>
                <p class="forecast-note">"Compare different growth scenarios and their resource requirements"</p>
            </div>

            <div class="scenario-grid">
                // Conservative scenario
                <div class="scenario-card">
                    <h4>"Conservative"</h4>
                    <div class="scenario-metrics">
                        <div class="metric-row">
                            <span>"New users (30d)"</span>
                            <span class="metric-value">"2,500"</span>
                        </div>
                        <div class="metric-row">
                            <span>"License demand"</span>
                            <span class="metric-value">"3,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"Funding required"</span>
                            <span class="metric-value">"$45,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"Support tickets"</span>
                            <span class="metric-value">"~200"</span>
                        </div>
                    </div>
                    <div class="scenario-gates">
                        <span class="gate-status ok">"All gates met"</span>
                    </div>
                </div>

                // Moderate scenario
                <div class="scenario-card highlighted">
                    <div class="scenario-badge">"Current Trajectory"</div>
                    <h4>"Moderate"</h4>
                    <div class="scenario-metrics">
                        <div class="metric-row">
                            <span>"New users (30d)"</span>
                            <span class="metric-value">"5,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"License demand"</span>
                            <span class="metric-value">"6,500"</span>
                        </div>
                        <div class="metric-row">
                            <span>"Funding required"</span>
                            <span class="metric-value">"$95,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"Support tickets"</span>
                            <span class="metric-value">"~450"</span>
                        </div>
                    </div>
                    <div class="scenario-gates">
                        <span class="gate-status warning">"Support capacity needed"</span>
                    </div>
                </div>

                // Aggressive scenario
                <div class="scenario-card">
                    <h4>"Aggressive"</h4>
                    <div class="scenario-metrics">
                        <div class="metric-row">
                            <span>"New users (30d)"</span>
                            <span class="metric-value">"10,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"License demand"</span>
                            <span class="metric-value">"13,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"Funding required"</span>
                            <span class="metric-value">"$190,000"</span>
                        </div>
                        <div class="metric-row">
                            <span>"Support tickets"</span>
                            <span class="metric-value">"~900"</span>
                        </div>
                    </div>
                    <div class="scenario-gates">
                        <span class="gate-status critical">"3 gates blocked"</span>
                    </div>
                </div>
            </div>

            <div class="forecast-disclaimer">
                <span class="info-icon">"ℹ"</span>
                <p>"Forecasts are based on current trends and historical data. Actual results may vary. Data source: Analytics pipeline (updated hourly)."</p>
            </div>
        </div>
    }
}
