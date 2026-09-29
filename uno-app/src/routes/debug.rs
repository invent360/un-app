//! Debug Dashboard - Temporary page to verify data sync from uno-admin
//!
//! WARNING: This module exposes sensitive data and should NEVER be enabled in production.
//! It is gated behind the `debug-routes` feature flag.
//!
//! Displays:
//! - All licenses from PostgreSQL
//! - All referrals from PostgreSQL
//! - Visitor stats by country

#![cfg(feature = "debug-routes")]

use leptos::prelude::*;

/// Convert a UUID string to 0x-prefixed hex format
/// e.g., "164ca20c-8fda-4bd0-84a5-2f325322ae21" -> "0x164ca20c8fda4bd084a52f325322ae21"
fn uuid_to_hex(uuid: &str) -> String {
    format!("0x{}", uuid.replace('-', ""))
}

/// Debug license DTO for display
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugLicense {
    pub id: String,
    pub lease_code: String,
    pub split_type: String,
    pub claimed: bool,
    pub is_valid: bool,
    pub is_expired: bool,
    pub created_at: String,
}

/// Debug referral DTO for display
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugReferral {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub country_code: String,
    pub referral_code: String,
    pub status: String,
    pub created_at: String,
}

/// Debug visitor stats DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugVisitorStats {
    pub country_code: String,
    pub visitor_count: i64,
    pub flag: String,
}

/// Server function to get all licenses
#[server(GetAllLicensesDebug, "/api")]
pub async fn get_all_licenses_debug() -> Result<Vec<DebugLicense>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use uno_api::models::PaginationParams;
    use uno_api::traits::LicenseFilters;

    let factory: Data<ServiceFactory> = extract().await?;

    // Use search with empty filters to get all licenses
    let filters = LicenseFilters::new();
    let pagination = PaginationParams {
        page: 1,
        per_page: 1000,
    };

    let result = factory.license_admin_service
        .search_licenses(filters, pagination)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let licenses: Vec<DebugLicense> = result.items
        .into_iter()
        .map(|l| DebugLicense {
            id: l.id,
            lease_code: l.lease_code,
            split_type: format!("{:?}", l.split_type),
            claimed: l.claimed,
            is_valid: l.is_valid,
            is_expired: l.is_expired,
            created_at: l.created_at.format("%Y-%m-%d %H:%M").to_string(),
        })
        .collect();

    Ok(licenses)
}

/// Server function to get all referrals
#[server(GetAllReferralsDebug, "/api")]
pub async fn get_all_referrals_debug() -> Result<Vec<DebugReferral>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    let referrals = factory.referral_repository
        .list_all(1000)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let result: Vec<DebugReferral> = referrals
        .into_iter()
        .map(|r| DebugReferral {
            id: r.id,
            username: r.username,
            email: r.email,
            country_code: r.country_code,
            referral_code: r.referral_code,
            status: r.status.to_string(),
            created_at: r.created_at.format("%Y-%m-%d %H:%M").to_string(),
        })
        .collect();

    Ok(result)
}

/// Server function to get visitor stats
#[server(GetVisitorStatsDebug, "/api")]
pub async fn get_visitor_stats_debug(period: String) -> Result<(Vec<DebugVisitorStats>, i64, i64), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    let period_days = match period.as_str() {
        "daily" => 1,
        "weekly" => 7,
        "monthly" => 30,
        "all" => 3650,
        _ => 30,
    };

    let total_visitors = factory.stats_repository
        .get_total_visits(period_days)
        .await
        .unwrap_or(0);

    let unique_visitors = factory.stats_repository
        .get_unique_visitor_count(period_days)
        .await
        .unwrap_or(0);

    let country_stats = factory.stats_repository
        .get_visitors_by_country(period_days, 20)
        .await
        .unwrap_or_default();

    let stats: Vec<DebugVisitorStats> = country_stats
        .into_iter()
        .map(|s| DebugVisitorStats {
            country_code: s.country_code,
            visitor_count: s.visitor_count,
            flag: s.flag,
        })
        .collect();

    Ok((stats, total_visitors, unique_visitors))
}

/// Debug Dashboard Page
#[component]
pub fn DebugPage() -> impl IntoView {
    // License state
    let (selected_license, set_selected_license) = signal::<Option<DebugLicense>>(None);
    let licenses = Resource::new(|| (), |_| get_all_licenses_debug());

    // Referral state
    let (selected_referral, set_selected_referral) = signal::<Option<DebugReferral>>(None);
    let referrals = Resource::new(|| (), |_| get_all_referrals_debug());

    // Visitor stats state
    let (period, set_period) = signal("monthly".to_string());
    let visitor_stats = Resource::new(
        move || period.get(),
        |p| get_visitor_stats_debug(p)
    );

    // Refresh all data
    let refresh_all = move |_| {
        licenses.refetch();
        referrals.refetch();
        visitor_stats.refetch();
    };

    view! {
        <div class="debug-page" style="padding: 20px; max-width: 1400px; margin: 0 auto;">
            <header style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;">
                <h1 style="font-size: 24px; font-weight: bold; color: var(--text-color, #fff);">"Debug Dashboard"</h1>
                <button
                    on:click=refresh_all
                    style="padding: 8px 16px; background: var(--primary-color, #3b82f6); color: white; border: none; border-radius: 6px; cursor: pointer;"
                >
                    "Refresh All"
                </button>
            </header>

            // Licenses Section
            <section style="margin-bottom: 32px;">
                <h2 style="font-size: 18px; font-weight: 600; margin-bottom: 12px; color: var(--text-color, #fff);">
                    "Licenses (from PostgreSQL)"
                </h2>
                <Suspense fallback=move || view! { <div style="color: #888;">"Loading licenses..."</div> }>
                    {move || {
                        licenses.get().map(|result| {
                            match result {
                                Ok(items) => {
                                    let count = items.len();
                                    view! {
                                        <div>
                                            <p style="color: #888; margin-bottom: 8px;">"Total: " {count} " licenses"</p>
                                            <div style="overflow-x: auto;">
                                                <table style="width: 100%; border-collapse: collapse; background: var(--card-bg, #1e293b); border-radius: 8px; overflow: hidden;">
                                                    <thead>
                                                        <tr style="background: var(--header-bg, #334155);">
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"License ID"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Split"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Claimed"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Valid"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Created"</th>
                                                            <th style="padding: 12px; text-align: center; color: #94a3b8;">"Action"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {items.into_iter().map(|license| {
                                                            let license_clone = license.clone();
                                                            let license_id_hex = uuid_to_hex(&license.id);
                                                            let split_type = license.split_type.clone();
                                                            let created_at = license.created_at.clone();
                                                            view! {
                                                                <tr style="border-bottom: 1px solid var(--border-color, #334155);">
                                                                    <td style="padding: 12px; color: var(--text-color, #fff); font-family: monospace; font-size: 13px;">{license_id_hex}</td>
                                                                    <td style="padding: 12px; color: var(--text-color, #fff);">{split_type}</td>
                                                                    <td style="padding: 12px;">
                                                                        {if license.claimed {
                                                                            view! { <span style="color: #22c55e;">"Yes"</span> }.into_any()
                                                                        } else {
                                                                            view! { <span style="color: #ef4444;">"No"</span> }.into_any()
                                                                        }}
                                                                    </td>
                                                                    <td style="padding: 12px;">
                                                                        {if license.is_expired {
                                                                            view! { <span style="color: #ef4444;">"Expired"</span> }.into_any()
                                                                        } else if license.is_valid {
                                                                            view! { <span style="color: #22c55e;">"Valid"</span> }.into_any()
                                                                        } else {
                                                                            view! { <span style="color: #f59e0b;">"Pending"</span> }.into_any()
                                                                        }}
                                                                    </td>
                                                                    <td style="padding: 12px; color: #888;">{created_at}</td>
                                                                    <td style="padding: 12px; text-align: center;">
                                                                        <button
                                                                            on:click=move |_| set_selected_license.set(Some(license_clone.clone()))
                                                                            style="padding: 4px 12px; background: transparent; border: 1px solid #3b82f6; color: #3b82f6; border-radius: 4px; cursor: pointer;"
                                                                        >
                                                                            "View"
                                                                        </button>
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                Err(e) => view! {
                                    <div style="color: #ef4444;">"Error: " {e.to_string()}</div>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </section>

            // Referrals Section
            <section style="margin-bottom: 32px;">
                <h2 style="font-size: 18px; font-weight: 600; margin-bottom: 12px; color: var(--text-color, #fff);">
                    "Referrals (from PostgreSQL)"
                </h2>
                <Suspense fallback=move || view! { <div style="color: #888;">"Loading referrals..."</div> }>
                    {move || {
                        referrals.get().map(|result| {
                            match result {
                                Ok(items) => {
                                    let count = items.len();
                                    view! {
                                        <div>
                                            <p style="color: #888; margin-bottom: 8px;">"Total: " {count} " referrals"</p>
                                            <div style="overflow-x: auto;">
                                                <table style="width: 100%; border-collapse: collapse; background: var(--card-bg, #1e293b); border-radius: 8px; overflow: hidden;">
                                                    <thead>
                                                        <tr style="background: var(--header-bg, #334155);">
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Username"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Email"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Country"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Code"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Status"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Created"</th>
                                                            <th style="padding: 12px; text-align: center; color: #94a3b8;">"Action"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {items.into_iter().map(|referral| {
                                                            let referral_clone = referral.clone();
                                                            let username = referral.username.clone();
                                                            let email = referral.email.clone();
                                                            let country_code = referral.country_code.clone();
                                                            let ref_code = referral.referral_code.clone();
                                                            let status = referral.status.clone();
                                                            let created_at = referral.created_at.clone();
                                                            let status_color = match status.as_str() {
                                                                "Active" => "#22c55e",
                                                                "Pending" => "#f59e0b",
                                                                "Suspended" => "#ef4444",
                                                                _ => "#888",
                                                            };
                                                            view! {
                                                                <tr style="border-bottom: 1px solid var(--border-color, #334155);">
                                                                    <td style="padding: 12px; color: var(--text-color, #fff);">{username}</td>
                                                                    <td style="padding: 12px; color: #888;">{email}</td>
                                                                    <td style="padding: 12px; color: var(--text-color, #fff);">{country_code}</td>
                                                                    <td style="padding: 12px; font-family: monospace; color: #3b82f6;">{ref_code}</td>
                                                                    <td style="padding: 12px;">
                                                                        <span style=format!("color: {};", status_color)>{status}</span>
                                                                    </td>
                                                                    <td style="padding: 12px; color: #888;">{created_at}</td>
                                                                    <td style="padding: 12px; text-align: center;">
                                                                        <button
                                                                            on:click=move |_| set_selected_referral.set(Some(referral_clone.clone()))
                                                                            style="padding: 4px 12px; background: transparent; border: 1px solid #3b82f6; color: #3b82f6; border-radius: 4px; cursor: pointer;"
                                                                        >
                                                                            "View"
                                                                        </button>
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                Err(e) => view! {
                                    <div style="color: #ef4444;">"Error: " {e.to_string()}</div>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </section>

            // Visitor Stats Section
            <section style="margin-bottom: 32px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                    <h2 style="font-size: 18px; font-weight: 600; color: var(--text-color, #fff);">
                        "Visitor Stats (from PostgreSQL)"
                    </h2>
                    <select
                        on:change=move |ev| set_period.set(event_target_value(&ev))
                        style="padding: 8px 12px; background: var(--card-bg, #1e293b); border: 1px solid var(--border-color, #334155); color: var(--text-color, #fff); border-radius: 6px;"
                    >
                        <option value="daily" selected=move || period.get() == "daily">"Daily"</option>
                        <option value="weekly" selected=move || period.get() == "weekly">"Weekly"</option>
                        <option value="monthly" selected=move || period.get() == "monthly">"Monthly"</option>
                        <option value="all" selected=move || period.get() == "all">"All Time"</option>
                    </select>
                </div>
                <Suspense fallback=move || view! { <div style="color: #888;">"Loading visitor stats..."</div> }>
                    {move || {
                        visitor_stats.get().map(|result| {
                            match result {
                                Ok((stats, total, unique)) => {
                                    view! {
                                        <div>
                                            <p style="color: #888; margin-bottom: 8px;">
                                                "Total: " {total} " visitors, " {unique} " unique"
                                            </p>
                                            <div style="overflow-x: auto;">
                                                <table style="width: 100%; border-collapse: collapse; background: var(--card-bg, #1e293b); border-radius: 8px; overflow: hidden;">
                                                    <thead>
                                                        <tr style="background: var(--header-bg, #334155);">
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Flag"</th>
                                                            <th style="padding: 12px; text-align: left; color: #94a3b8;">"Country"</th>
                                                            <th style="padding: 12px; text-align: right; color: #94a3b8;">"Visitors"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {stats.into_iter().map(|stat| {
                                                            let flag = stat.flag.clone();
                                                            let code = stat.country_code.clone();
                                                            view! {
                                                                <tr style="border-bottom: 1px solid var(--border-color, #334155);">
                                                                    <td style="padding: 12px; font-size: 20px;">{flag}</td>
                                                                    <td style="padding: 12px; color: var(--text-color, #fff);">{code}</td>
                                                                    <td style="padding: 12px; text-align: right; color: var(--text-color, #fff); font-weight: 600;">
                                                                        {stat.visitor_count}
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                Err(e) => view! {
                                    <div style="color: #ef4444;">"Error: " {e.to_string()}</div>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </section>

            // License Detail Modal
            {move || {
                selected_license.get().map(|license| {
                    let license_id_hex = uuid_to_hex(&license.id);
                    let lease_code = license.lease_code.clone();
                    let split_type = license.split_type.clone();
                    let created_at = license.created_at.clone();
                    view! {
                        <div style="position: fixed; inset: 0; z-index: 9999; display: flex; align-items: center; justify-content: center;">
                            <div
                                style="position: absolute; inset: 0; background: rgba(0, 0, 0, 0.5);"
                                on:click=move |_| set_selected_license.set(None)
                            />
                            <div style="position: relative; z-index: 1; background: var(--card-bg, #1e293b); border-radius: 12px; padding: 24px; width: 100%; max-width: 500px; margin: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                                    <h3 style="font-size: 18px; font-weight: 600; color: var(--text-color, #fff);">"License Details"</h3>
                                    <button
                                        on:click=move |_| set_selected_license.set(None)
                                        style="background: transparent; border: none; color: #888; cursor: pointer; font-size: 20px;"
                                    >
                                        "×"
                                    </button>
                                </div>
                                <div style="display: grid; gap: 12px;">
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"License ID"</span>
                                        <p style="color: var(--text-color, #fff); font-family: monospace; font-size: 13px; word-break: break-all;">{license_id_hex}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Lease Code"</span>
                                        <p style="color: var(--text-color, #fff);">{lease_code}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Split Type"</span>
                                        <p style="color: var(--text-color, #fff);">{split_type}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Claimed"</span>
                                        <p style=format!("color: {};", if license.claimed { "#22c55e" } else { "#ef4444" })>
                                            {if license.claimed { "Yes" } else { "No" }}
                                        </p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Status"</span>
                                        <p style=format!("color: {};", if license.is_expired { "#ef4444" } else if license.is_valid { "#22c55e" } else { "#f59e0b" })>
                                            {if license.is_expired { "Expired" } else if license.is_valid { "Valid" } else { "Pending" }}
                                        </p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Created At"</span>
                                        <p style="color: var(--text-color, #fff);">{created_at}</p>
                                    </div>
                                </div>
                                <button
                                    on:click=move |_| set_selected_license.set(None)
                                    style="margin-top: 20px; width: 100%; padding: 10px; background: var(--primary-color, #3b82f6); color: white; border: none; border-radius: 6px; cursor: pointer;"
                                >
                                    "Close"
                                </button>
                            </div>
                        </div>
                    }
                })
            }}

            // Referral Detail Modal
            {move || {
                selected_referral.get().map(|referral| {
                    let username = referral.username.clone();
                    let email = referral.email.clone();
                    let country_code = referral.country_code.clone();
                    let ref_code = referral.referral_code.clone();
                    let status = referral.status.clone();
                    let created_at = referral.created_at.clone();
                    let status_color = match status.as_str() {
                        "Active" => "#22c55e",
                        "Pending" => "#f59e0b",
                        "Suspended" => "#ef4444",
                        _ => "#888",
                    };
                    view! {
                        <div style="position: fixed; inset: 0; z-index: 9999; display: flex; align-items: center; justify-content: center;">
                            <div
                                style="position: absolute; inset: 0; background: rgba(0, 0, 0, 0.5);"
                                on:click=move |_| set_selected_referral.set(None)
                            />
                            <div style="position: relative; z-index: 1; background: var(--card-bg, #1e293b); border-radius: 12px; padding: 24px; width: 100%; max-width: 500px; margin: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                                    <h3 style="font-size: 18px; font-weight: 600; color: var(--text-color, #fff);">"Referral Details"</h3>
                                    <button
                                        on:click=move |_| set_selected_referral.set(None)
                                        style="background: transparent; border: none; color: #888; cursor: pointer; font-size: 20px;"
                                    >
                                        "×"
                                    </button>
                                </div>
                                <div style="display: grid; gap: 12px;">
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"ID"</span>
                                        <p style="color: var(--text-color, #fff);">{referral.id}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Username"</span>
                                        <p style="color: var(--text-color, #fff);">{username}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Email"</span>
                                        <p style="color: var(--text-color, #fff);">{email}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Country"</span>
                                        <p style="color: var(--text-color, #fff);">{country_code}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Referral Code"</span>
                                        <p style="color: #3b82f6; font-family: monospace;">{ref_code}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Status"</span>
                                        <p style=format!("color: {};", status_color)>{status}</p>
                                    </div>
                                    <div>
                                        <span style="color: #888; font-size: 12px;">"Created At"</span>
                                        <p style="color: var(--text-color, #fff);">{created_at}</p>
                                    </div>
                                </div>
                                <button
                                    on:click=move |_| set_selected_referral.set(None)
                                    style="margin-top: 20px; width: 100%; padding: 10px; background: var(--primary-color, #3b82f6); color: white; border: none; border-radius: 6px; cursor: pointer;"
                                >
                                    "Close"
                                </button>
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}
