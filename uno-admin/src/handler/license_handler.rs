//! License handler - Direct integration with Unetwork API
//!
//! This handler fetches license data directly from the Unetwork API
//! without local database persistence. The Unetwork API is the single
//! source of truth for all license data.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use uno_api::client::{UnetworkClient, UnetworkConfig};
#[cfg(feature = "ssr")]
use uno_api::services::unetwork::{UnetworkLicenseService, UnetworkLicenseTrait};
#[cfg(feature = "ssr")]
use ember_multichain::siwe::unetwork;
#[cfg(feature = "ssr")]
use crate::db::get_db;
#[cfg(feature = "ssr")]
use crate::repository::LicenseRepository;
#[cfg(feature = "ssr")]
use crate::repository::traits::LicenseRepositoryTrait;

/// License DTO returned to the UI
/// Compatible with the existing LicenseEntity structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseDto {
    pub id: String,
    pub license_id: String,
    pub node_id: String,
    pub alias: Option<String>,
    pub owner_wallet_address: Option<String>,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub uptime: f64,
    pub is_online: bool,
    pub is_leased: bool,
    pub is_bound: bool,
    pub lease_share_percentage: f64,
    pub lease_min_uptime_percentage: f64,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
    pub activation_start_at: Option<String>,
    pub activation_end_at: Option<String>,
    pub validation_last_success_at: Option<String>,
    // Marketplace fields
    pub is_on_marketplace: bool,
    pub is_on_uno_marketplace: bool,
    // Split percentage fields (from ScyllaDB LicenseEntity)
    pub uno_share: Option<f64>,
    pub ulo_share: Option<f64>,
    pub agent_share: Option<f64>,
}

impl LicenseDto {
    /// Get shortened license ID for display
    pub fn license_id_short(&self) -> String {
        if self.license_id.len() > 16 {
            format!(
                "{}...{}",
                &self.license_id[..10],
                &self.license_id[self.license_id.len() - 6..]
            )
        } else {
            self.license_id.clone()
        }
    }

    /// Get shortened node ID for display
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!(
                "{}...{}",
                &self.node_id[..10],
                &self.node_id[self.node_id.len() - 6..]
            )
        } else {
            self.node_id.clone()
        }
    }

    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }

    /// Format split percentage for display
    /// Returns "ULO:UNO" or "ULO:REF:UNO" if referral/agent exists
    pub fn split_display(&self) -> String {
        match (self.ulo_share, self.agent_share, self.uno_share) {
            (Some(ulo), Some(agent), Some(uno)) if agent > 0.0 => {
                format!("{:.0}:{:.0}:{:.0}", ulo, agent, uno)
            }
            (Some(ulo), _, Some(uno)) => {
                format!("{:.0}:{:.0}", ulo, uno)
            }
            _ => "-".to_string()
        }
    }
}

/// Summary statistics for licenses
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LicensesSummaryDto {
    pub total_licenses: i64,
    pub online_count: i64,
    pub offline_count: i64,
    pub leased_count: i64,
    pub bound_count: i64,
    pub avg_uptime: f64,
    pub unique_agents: i64,
    pub total_groups: i64,
}

/// UNO Licenses Summary from RPC endpoint
/// Response from: /rest/v1/rpc/licenses_get_uno_licenses_summary
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnoLicensesSummaryDto {
    #[serde(rename = "totalLicensesCount")]
    pub total_licenses_count: i64,
    #[serde(rename = "activeLicensesCount")]
    pub active_licenses_count: i64,
    #[serde(rename = "leasedLicensesCount")]
    pub leased_licenses_count: i64,
    #[serde(rename = "recentlyValidLicensesCount")]
    pub recently_valid_licenses_count: i64,
    #[serde(rename = "boundLicensesCount")]
    pub bound_licenses_count: i64,
}

/// Allocations Summary from RPC endpoint
/// Response from: /rest/v1/rpc/rewards_get_allocations_summary
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AllocationsSummaryDto {
    #[serde(rename = "totalAmountMicros")]
    pub total_amount_micros: i64,
    #[serde(rename = "last7DaysAmountMicros")]
    pub last_7_days_amount_micros: i64,
    #[serde(rename = "thisWeekAmountMicros")]
    pub this_week_amount_micros: i64,
    #[serde(rename = "todayAmountMicros")]
    pub today_amount_micros: i64,
}

impl AllocationsSummaryDto {
    /// Convert micros to standard units (divide by 1,000,000)
    pub fn total_amount(&self) -> f64 {
        self.total_amount_micros as f64 / 1_000_000.0
    }

    pub fn last_7_days_amount(&self) -> f64 {
        self.last_7_days_amount_micros as f64 / 1_000_000.0
    }

    pub fn this_week_amount(&self) -> f64 {
        self.this_week_amount_micros as f64 / 1_000_000.0
    }

    pub fn today_amount(&self) -> f64 {
        self.today_amount_micros as f64 / 1_000_000.0
    }
}

// ========================================
// Paginated Licenses API
// ========================================

/// Request payload for paginated licenses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedLicenseRequest {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leased: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grouped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bound: Option<bool>,
    pub page: i32,
    pub page_size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    pub skip: i32,
    pub take: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_max: Option<f64>,
}

impl PaginatedLicenseRequest {
    pub fn new(page: i32, page_size: i32) -> Self {
        Self {
            role: "uno".to_string(),
            is_leased: None,   // None = omit field = show all
            is_grouped: None,  // None = omit field = show all
            is_bound: None,    // None = omit field = show all
            page,
            page_size,
            search: None,      // None = omit field = no search
            is_online: None,   // None = omit field = show all
            skip: (page - 1) * page_size,
            take: page_size,
            uptime_min: None,
            uptime_max: None,
        }
    }

    pub fn with_filters(
        mut self,
        is_leased: Option<bool>,
        is_bound: Option<bool>,
        is_online: Option<bool>,
    ) -> Self {
        // API semantics:
        // - Some(true) = filter for items WHERE field is true
        // - Some(false) = filter for items WHERE field is false
        // - None = omit field = no filter (show all)
        self.is_leased = is_leased;
        self.is_bound = is_bound;
        self.is_online = is_online;
        self
    }

    pub fn with_uptime_filter(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.uptime_min = min;
        self.uptime_max = max;
        self
    }

    pub fn with_search(mut self, search: Option<String>) -> Self {
        self.search = search;
        self
    }
}

/// License item from paginated API response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedLicenseItem {
    pub id: String,
    pub node_id: Option<String>,
    pub owner_wallet_address: Option<String>,
    pub alias: Option<String>,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub activation_start_at: Option<String>,
    pub activation_end_at: Option<String>,
    pub lease_user_id: Option<String>,
    pub lease_share_percentage: Option<f64>,
    pub lease_min_uptime_percentage: Option<f64>,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
    pub validation_last_success_at: Option<String>,
    pub uptime: Option<f64>,
    pub total_count: i64,
    pub is_online: Option<bool>,
    /// Whether the license is leased (from API)
    pub is_leased: Option<bool>,
    /// Whether the license is bound (from API)
    pub is_bound: Option<bool>,
}

/// Paginated licenses response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedLicenseResponse {
    pub licenses: Vec<LicenseDto>,
    pub total_count: i64,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

/// Fetch paginated licenses from Unetwork Edge Functions API
///
/// Parameters:
/// - page: 1-indexed page number
/// - page_size: number of items per page
/// - is_leased: Some(true) = only leased, Some(false) = only not leased, None = all
/// - is_bound: Some(true) = only bound, Some(false) = only not bound, None = all
/// - is_online: Some(true) = only online, Some(false) = only offline, None = all
/// - uptime_min: optional minimum uptime filter (0.0 to 1.0, where 1.0 = 100%)
/// - uptime_max: optional maximum uptime filter (0.0 to 1.0, where 1.0 = 100%)
/// - search: optional search string (license ID prefix like "0x...")
#[server(GetPaginatedLicenses, "/api")]
pub async fn get_paginated_licenses(
    page: i32,
    page_size: i32,
    is_leased: Option<bool>,
    is_bound: Option<bool>,
    is_online: Option<bool>,
    uptime_min: Option<f64>,
    uptime_max: Option<f64>,
    search: Option<String>,
) -> Result<PaginatedLicenseResponse, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/functions/v1/licenses_get_licenses", unetwork::API_URL);

    let request_body = PaginatedLicenseRequest::new(page, page_size)
        .with_filters(is_leased, is_bound, is_online)
        .with_uptime_filter(uptime_min, uptime_max)
        .with_search(search);

    // Log the request
    eprintln!("=== UNETWORK API REQUEST ===");
    eprintln!("URL: {}", url);
    eprintln!("Request body: {}", serde_json::to_string_pretty(&request_body).unwrap_or_default());

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch paginated licenses: {}", e)))?;

    let status = response.status();

    // Get raw response text for logging
    let response_text = response.text().await
        .map_err(|e| ServerFnError::new(format!("Failed to read response: {}", e)))?;

    // Log the response
    eprintln!("=== UNETWORK API RESPONSE ===");
    eprintln!("Status: {}", status);
    eprintln!("Response body: {}", &response_text[..response_text.len().min(2000)]); // Limit to 2000 chars

    if !status.is_success() {
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            response_text
        )));
    }

    // Parse the response from the text we already have
    let items: Vec<PaginatedLicenseItem> = serde_json::from_str(&response_text)
        .map_err(|e| ServerFnError::new(format!("Failed to parse paginated licenses: {}", e)))?;

    // Get total count from first item (all items have the same total_count)
    let total_count = items.first().map(|i| i.total_count).unwrap_or(0);
    let total_pages = if total_count == 0 {
        1
    } else {
        ((total_count as f64) / (page_size as f64)).ceil() as i32
    };

    // Convert to LicenseDto
    let mut licenses: Vec<LicenseDto> = items
        .into_iter()
        .map(|item| {
            // Use API-provided is_leased and is_bound values directly
            let is_leased = item.is_leased.unwrap_or_else(|| item.lease_user_id.is_some());
            let is_bound = item.is_bound.unwrap_or(false);

            LicenseDto {
                id: item.id.clone(),
                license_id: item.id,
                node_id: item.node_id.unwrap_or_default(),
                alias: item.alias,
                owner_wallet_address: item.owner_wallet_address,
                device_id: item.device_id,
                device_name: item.device_name,
                uptime: item.uptime.unwrap_or(0.0),
                is_online: item.is_online.unwrap_or(false),
                is_leased,
                is_bound,
                lease_share_percentage: item.lease_share_percentage.unwrap_or(0.0),
                lease_min_uptime_percentage: item.lease_min_uptime_percentage.unwrap_or(0.0),
                lease_from: item.lease_from,
                lease_to: item.lease_to,
                activation_start_at: item.activation_start_at,
                activation_end_at: item.activation_end_at,
                validation_last_success_at: item.validation_last_success_at,
                is_on_marketplace: false,
                is_on_uno_marketplace: false,
                uno_share: None,
                ulo_share: None,
                agent_share: None,
            }
        })
        .collect();

    // Enrich with marketplace data from ScyllaDB
    if let Some(pool) = get_db() {
        let repo = LicenseRepository::new(pool);
        let license_ids: Vec<String> = licenses.iter().map(|l| l.license_id.clone()).collect();

        match repo.get_marketplace_data_batch(&license_ids).await {
            Ok(marketplace_data) => {
                let mut enriched_count = 0;
                for license in &mut licenses {
                    if let Some(data) = marketplace_data.get(&license.license_id) {
                        license.is_on_marketplace = data.is_on_marketplace;
                        license.is_on_uno_marketplace = data.is_on_uno_marketplace;
                        if data.is_on_marketplace || data.is_on_uno_marketplace {
                            enriched_count += 1;
                        }
                    }
                }
                if enriched_count > 0 {
                    eprintln!("Enriched {} licenses with marketplace data", enriched_count);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to fetch marketplace data from ScyllaDB: {}", e);
            }
        }
        // Also enrich with split data from ScyllaDB
        match repo.get_split_data_batch(&license_ids).await {
            Ok(split_data) => {
                for license in &mut licenses {
                    if let Some(data) = split_data.get(&license.license_id) {
                        license.uno_share = Some(data.uno_share);
                        license.ulo_share = Some(data.ulo_share);
                        license.agent_share = Some(data.agent_share);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to fetch split data from ScyllaDB: {}", e);
            }
        }
    } else {
        eprintln!("Warning: No database connection available for marketplace enrichment");
    }

    Ok(PaginatedLicenseResponse {
        licenses,
        total_count,
        page,
        page_size,
        total_pages,
    })
}

/// Search for a license by ID
/// Returns the license if found, or None if not found
#[server(SearchLicenseById, "/api")]
pub async fn search_license_by_id(license_id: String) -> Result<Option<LicenseDto>, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/functions/v1/licenses_get_licenses", unetwork::API_URL);

    // Create request with search parameter - no filter constraints to find ALL licenses
    let request_body = PaginatedLicenseRequest::new(1, 100)
        .with_search(Some(license_id.clone()));

    eprintln!("=== LICENSE SEARCH REQUEST ===");
    eprintln!("URL: {}", url);
    eprintln!("Search ID: {}", license_id);
    eprintln!("Request body: {}", serde_json::to_string_pretty(&request_body).unwrap_or_default());

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to search license: {}", e)))?;

    let status = response.status();
    let response_text = response.text().await
        .map_err(|e| ServerFnError::new(format!("Failed to read response: {}", e)))?;

    eprintln!("=== LICENSE SEARCH RESPONSE ===");
    eprintln!("Status: {}", status);
    eprintln!("Response: {}", &response_text[..response_text.len().min(2000)]);

    if !status.is_success() {
        return Err(ServerFnError::new(format!("API error {}: {}", status.as_u16(), response_text)));
    }

    let items: Vec<PaginatedLicenseItem> = serde_json::from_str(&response_text)
        .map_err(|e| ServerFnError::new(format!("Failed to parse response: {}", e)))?;

    // Return the first matching license, if any
    Ok(items.into_iter().next().map(|item| {
        let is_leased = item.is_leased.unwrap_or_else(|| item.lease_user_id.is_some());
        let is_bound = item.is_bound.unwrap_or(false);

        LicenseDto {
            id: item.id.clone(),
            license_id: item.id,
            node_id: item.node_id.unwrap_or_default(),
            alias: item.alias,
            owner_wallet_address: item.owner_wallet_address,
            device_id: item.device_id,
            device_name: item.device_name,
            uptime: item.uptime.unwrap_or(0.0),
            is_online: item.is_online.unwrap_or(false),
            is_leased,
            is_bound,
            lease_share_percentage: item.lease_share_percentage.unwrap_or(0.0),
            lease_min_uptime_percentage: item.lease_min_uptime_percentage.unwrap_or(0.0),
            lease_from: item.lease_from,
            lease_to: item.lease_to,
            activation_start_at: item.activation_start_at,
            activation_end_at: item.activation_end_at,
            validation_last_success_at: item.validation_last_success_at,
            is_on_marketplace: false,
            is_on_uno_marketplace: false,
            uno_share: None,
            ulo_share: None,
            agent_share: None,
        }
    }))
}

/// Fetch ALL licenses from Unetwork API by paginating through all pages
/// Used when client-side filtering needs access to the full dataset
#[server(GetAllLicensesForFiltering, "/api")]
pub async fn get_all_licenses_for_filtering() -> Result<Vec<LicenseDto>, ServerFnError> {
    let jwt_token = get_jwt_token()?;
    let client = crate::api::http_client::get_client();
    let url = format!("{}/functions/v1/licenses_get_licenses", ember_multichain::siwe::unetwork::API_URL);

    const PAGE_SIZE: i32 = 100; // API max
    let mut all_licenses: Vec<LicenseDto> = Vec::new();
    let mut current_page = 1;
    let mut total_pages = 1;

    loop {
        let request_body = PaginatedLicenseRequest::new(current_page, PAGE_SIZE);

        let response = client
            .post(&url)
            .header("apikey", ember_multichain::siwe::unetwork::API_KEY)
            .header("Authorization", format!("Bearer {}", jwt_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to fetch licenses page {}: {}", current_page, e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ServerFnError::new(format!(
                "API error {}: {}",
                status.as_u16(),
                error_text
            )));
        }

        let items: Vec<PaginatedLicenseItem> = response
            .json()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to parse licenses page {}: {}", current_page, e)))?;

        // On first page, calculate total pages
        if current_page == 1 {
            let total_count = items.first().map(|i| i.total_count).unwrap_or(0);
            total_pages = if total_count == 0 {
                1
            } else {
                ((total_count as f64) / (PAGE_SIZE as f64)).ceil() as i32
            };
        }

        // Convert to LicenseDto and add to collection
        for item in items {
            // Use API-provided is_leased and is_bound values directly
            let is_leased = item.is_leased.unwrap_or_else(|| item.lease_user_id.is_some());
            let is_bound = item.is_bound.unwrap_or(false);

            all_licenses.push(LicenseDto {
                id: item.id.clone(),
                license_id: item.id,
                node_id: item.node_id.unwrap_or_default(),
                alias: item.alias,
                owner_wallet_address: item.owner_wallet_address,
                device_id: item.device_id,
                device_name: item.device_name,
                uptime: item.uptime.unwrap_or(0.0),
                is_online: item.is_online.unwrap_or(false),
                is_leased,
                is_bound,
                lease_share_percentage: item.lease_share_percentage.unwrap_or(0.0),
                lease_min_uptime_percentage: item.lease_min_uptime_percentage.unwrap_or(0.0),
                lease_from: item.lease_from,
                lease_to: item.lease_to,
                activation_start_at: item.activation_start_at,
                activation_end_at: item.activation_end_at,
                validation_last_success_at: item.validation_last_success_at,
                is_on_marketplace: false,
                is_on_uno_marketplace: false,
                uno_share: None,
                ulo_share: None,
                agent_share: None,
            });
        }

        // Move to next page or break
        if current_page >= total_pages {
            break;
        }
        current_page += 1;
    }

    // Enrich with marketplace data from ScyllaDB
    if let Some(pool) = get_db() {
        let repo = LicenseRepository::new(pool);
        let license_ids: Vec<String> = all_licenses.iter().map(|l| l.license_id.clone()).collect();

        match repo.get_marketplace_data_batch(&license_ids).await {
            Ok(marketplace_data) => {
                for license in &mut all_licenses {
                    if let Some(data) = marketplace_data.get(&license.license_id) {
                        license.is_on_marketplace = data.is_on_marketplace;
                        license.is_on_uno_marketplace = data.is_on_uno_marketplace;
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to fetch marketplace data: {}", e);
            }
        }

        // Also enrich with split data from ScyllaDB
        match repo.get_split_data_batch(&license_ids).await {
            Ok(split_data) => {
                for license in &mut all_licenses {
                    if let Some(data) = split_data.get(&license.license_id) {
                        license.uno_share = Some(data.uno_share);
                        license.ulo_share = Some(data.ulo_share);
                        license.agent_share = Some(data.agent_share);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to fetch split data: {}", e);
            }
        }
    }

    Ok(all_licenses)
}

/// Fetch all licenses from Unetwork API
#[server(ListLicenses, "/api")]
pub async fn list_licenses() -> Result<Vec<LicenseDto>, ServerFnError> {
    let jwt_token = get_jwt_token()?;
    let config = UnetworkConfig::new(&jwt_token);
    let client = Arc::new(UnetworkClient::new(config));
    let service = UnetworkLicenseService::new(client);

    // Fetch all license groups (which contain full license details)
    let groups = service
        .get_all_license_groups()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch licenses: {}", e)))?;

    // Flatten all licenses from all groups
    let licenses: Vec<LicenseDto> = groups
        .into_iter()
        .flat_map(|group| group.licenses)
        .map(|l| {
            let device_name = l.device_name.clone();
            let device_id = l.device_id.clone();
            let alias = l.alias.clone();
            let owner_wallet_address = l.owner_wallet_address.clone();
            LicenseDto {
                id: l.id.clone(),
                license_id: l.id.clone(),
                node_id: l.node_id.clone().unwrap_or_default(),
                alias,
                owner_wallet_address,
                device_id,
                device_name,
                uptime: l.uptime.unwrap_or(0.0),
                is_online: l.is_online.unwrap_or(false),
                is_leased: l.is_leased(),
                is_bound: l.is_bound(),
                lease_share_percentage: l.lease_share_percentage.unwrap_or(0.0),
                lease_min_uptime_percentage: l.lease_min_uptime_percentage.unwrap_or(0.0),
                lease_from: l.lease_from.map(|dt| dt.to_rfc3339()),
                lease_to: l.lease_to.map(|dt| dt.to_rfc3339()),
                activation_start_at: l.activation_start_at.map(|dt| dt.to_rfc3339()),
                activation_end_at: l.activation_end_at.map(|dt| dt.to_rfc3339()),
                validation_last_success_at: l.validation_last_success_at.map(|dt| dt.to_rfc3339()),
                is_on_marketplace: false,
                is_on_uno_marketplace: false,
                uno_share: None,
                ulo_share: None,
                agent_share: None,
            }
        })
        .collect();

    Ok(licenses)
}

/// Get license summary statistics from Unetwork API
#[server(GetLicensesSummary, "/api")]
pub async fn get_licenses_summary() -> Result<LicensesSummaryDto, ServerFnError> {
    let jwt_token = get_jwt_token()?;
    let config = UnetworkConfig::new(&jwt_token);
    let client = Arc::new(UnetworkClient::new(config));
    let service = UnetworkLicenseService::new(client);

    // Get statistics from the service
    let stats = service
        .get_statistics()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch license statistics: {}", e)))?;

    Ok(LicensesSummaryDto {
        total_licenses: stats.total_licenses as i64,
        online_count: stats.online_count as i64,
        offline_count: (stats.total_licenses - stats.online_count) as i64,
        leased_count: stats.leased_count as i64,
        bound_count: stats.bound_count as i64,
        avg_uptime: stats.average_uptime,
        unique_agents: 0, // Not tracked at API level
        total_groups: stats.total_groups as i64,
    })
}

/// Get a single license by ID using Edge Functions API
#[server(GetLicenseById, "/api")]
pub async fn get_license_by_id(license_id: String) -> Result<Option<LicenseDto>, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/functions/v1/licenses_get_licenses", unetwork::API_URL);

    // Search through pages until we find the license
    // API maximum take is 100
    let page_size = 100;
    let mut page = 1;
    let max_pages = 25; // Safety limit (covers 2500 licenses)

    while page <= max_pages {
        let skip = (page - 1) * page_size;
        // Don't filter by isLeased/isGrouped/isBound - we want ALL licenses
        let request_body = serde_json::json!({
            "role": "uno",
            "page": page,
            "pageSize": page_size,
            "skip": skip,
            "take": page_size
        });

        let response = client
            .post(&url)
            .header("apikey", unetwork::API_KEY)
            .header("Authorization", format!("Bearer {}", jwt_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to fetch licenses: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ServerFnError::new(format!("API error: {}", error_text)));
        }

        let items: Vec<PaginatedLicenseItem> = response
            .json()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to parse response: {}", e)))?;

        // Check if we found the license
        if let Some(item) = items.iter().find(|item| item.id == license_id) {
            // Use API-provided is_leased and is_bound values directly
            let is_leased = item.is_leased.unwrap_or_else(|| item.lease_user_id.is_some());
            let is_bound = item.is_bound.unwrap_or(false);

            return Ok(Some(LicenseDto {
                id: item.id.clone(),
                license_id: item.id.clone(),
                node_id: item.node_id.clone().unwrap_or_default(),
                alias: item.alias.clone(),
                owner_wallet_address: item.owner_wallet_address.clone(),
                device_id: item.device_id.clone(),
                device_name: item.device_name.clone(),
                uptime: item.uptime.unwrap_or(0.0),
                is_online: item.is_online.unwrap_or(false),
                is_leased,
                is_bound,
                lease_share_percentage: item.lease_share_percentage.unwrap_or(0.0),
                lease_min_uptime_percentage: item.lease_min_uptime_percentage.unwrap_or(0.0),
                lease_from: item.lease_from.clone(),
                lease_to: item.lease_to.clone(),
                activation_start_at: item.activation_start_at.clone(),
                activation_end_at: item.activation_end_at.clone(),
                validation_last_success_at: item.validation_last_success_at.clone(),
                is_on_marketplace: false,
                is_on_uno_marketplace: false,
                uno_share: None,
                ulo_share: None,
                agent_share: None,
            }));
        }

        // If no more items, stop searching
        if items.is_empty() || items.len() < page_size as usize {
            break;
        }

        page += 1;
    }

    Ok(None)
}

/// Get online licenses only
#[server(GetOnlineLicenses, "/api")]
pub async fn get_online_licenses() -> Result<Vec<LicenseDto>, ServerFnError> {
    let jwt_token = get_jwt_token()?;
    let config = UnetworkConfig::new(&jwt_token);
    let client = Arc::new(UnetworkClient::new(config));
    let service = UnetworkLicenseService::new(client);

    let licenses = service
        .get_online_licenses()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch online licenses: {}", e)))?;

    Ok(licenses
        .into_iter()
        .map(|l| {
            let device_name = l.device_name.clone();
            let device_id = l.device_id.clone();
            let alias = l.alias.clone();
            let owner_wallet_address = l.owner_wallet_address.clone();
            LicenseDto {
                id: l.id.clone(),
                license_id: l.id.clone(),
                node_id: l.node_id.clone().unwrap_or_default(),
                alias,
                owner_wallet_address,
                device_id,
                device_name,
                uptime: l.uptime.unwrap_or(0.0),
                is_online: l.is_online.unwrap_or(false),
                is_leased: l.is_leased(),
                is_bound: l.is_bound(),
                lease_share_percentage: l.lease_share_percentage.unwrap_or(0.0),
                lease_min_uptime_percentage: l.lease_min_uptime_percentage.unwrap_or(0.0),
                lease_from: l.lease_from.map(|dt| dt.to_rfc3339()),
                lease_to: l.lease_to.map(|dt| dt.to_rfc3339()),
                activation_start_at: l.activation_start_at.map(|dt| dt.to_rfc3339()),
                activation_end_at: l.activation_end_at.map(|dt| dt.to_rfc3339()),
                validation_last_success_at: l.validation_last_success_at.map(|dt| dt.to_rfc3339()),
                is_on_marketplace: false,
                is_on_uno_marketplace: false,
                uno_share: None,
                ulo_share: None,
                agent_share: None,
            }
        })
        .collect())
}

/// Get UNO licenses summary from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/licenses_get_uno_licenses_summary
#[server(GetUnoLicensesSummary, "/api")]
pub async fn get_uno_licenses_summary() -> Result<UnoLicensesSummaryDto, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    // Debug: Log token length (not the token itself for security)
    eprintln!("DEBUG: JWT token length: {}", jwt_token.len());

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/licenses_get_uno_licenses_summary", unetwork::API_URL);
    eprintln!("DEBUG: API URL: {}", url);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch UNO licenses summary: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    let summaries: Vec<UnoLicensesSummaryDto> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse UNO licenses summary: {}", e)))?;

    Ok(summaries.into_iter().next().unwrap_or_default())
}

/// Get allocations summary from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/rewards_get_allocations_summary
#[server(GetAllocationsSummary, "/api")]
pub async fn get_allocations_summary() -> Result<AllocationsSummaryDto, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/rewards_get_allocations_summary", unetwork::API_URL);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch allocations summary: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    let summaries: Vec<AllocationsSummaryDto> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse allocations summary: {}", e)))?;

    Ok(summaries.into_iter().next().unwrap_or_default())
}

/// Helper to get JWT token from environment
#[cfg(feature = "ssr")]
fn get_jwt_token() -> Result<String, ServerFnError> {
    let token = std::env::var("API_TOKEN")
        .or_else(|_| std::env::var("UNITY_JWT_TOKEN"))
        .map_err(|_| {
            ServerFnError::new(
                "No JWT token: set API_TOKEN or UNITY_JWT_TOKEN environment variable",
            )
        })?;

    if token.is_empty() {
        return Err(ServerFnError::new("JWT token is empty"));
    }

    Ok(token)
}

// ========================================
// Server Function Registration
// ========================================

/// Force license server functions to be registered
/// Call this from main.rs to ensure inventory picks up the server functions
// ========================================
// License Details API (for detail page)
// ========================================

use crate::api::types::{
    AllocationsSummaryApi, UptimeAnalyticsPoint, LeaseResponse,
    LicenseSettingsResponse, WalletSettingsResponse,
};

/// Get allocation summary for a specific license from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/rewards_get_allocations_summary with licenseId
#[server(GetLicenseAllocationSummary, "/api")]
pub async fn get_license_allocation_summary(
    license_id: String
) -> Result<AllocationsSummaryApi, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/rewards_get_allocations_summary", unetwork::API_URL);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({ "licenseId": license_id }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch allocation summary: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    // Response is an array with single item or empty
    let summaries: Vec<AllocationsSummaryApi> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse allocation summary: {}", e)))?;

    Ok(summaries.into_iter().next().unwrap_or_default())
}

/// Get uptime analytics for a specific license from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/license_analytics_get_by_license
#[server(GetLicenseUptimeAnalytics, "/api")]
pub async fn get_license_uptime_analytics(
    license_id: String,
    start_date: String,
    end_date: String,
) -> Result<Vec<UptimeAnalyticsPoint>, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/license_analytics_get_by_license", unetwork::API_URL);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({
            "licenseId": license_id,
            "startDate": start_date,
            "endDate": end_date
        }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch uptime analytics: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    let analytics: Vec<UptimeAnalyticsPoint> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse uptime analytics: {}", e)))?;

    Ok(analytics)
}

/// Get 7-day uptime history for multiple licenses in parallel
/// Returns a HashMap of license_id -> Vec<f64> (uptime values 0.0-1.0)
#[server(GetBatchUptimeHistory, "/api")]
pub async fn get_batch_uptime_history(
    license_ids: Vec<String>,
) -> Result<std::collections::HashMap<String, Vec<f64>>, ServerFnError> {
    use std::collections::HashMap;

    if license_ids.is_empty() {
        return Ok(HashMap::new());
    }

    // Calculate last 7 days
    let end = chrono::Utc::now().date_naive();
    let start = end - chrono::Duration::days(7);
    let start_str = start.format("%Y-%m-%d").to_string();
    let end_str = end.format("%Y-%m-%d").to_string();

    // Fetch uptime history for each license in parallel
    let futures: Vec<_> = license_ids.iter().map(|id| {
        let id = id.clone();
        let start = start_str.clone();
        let end = end_str.clone();
        async move {
            let result = get_license_uptime_analytics(id.clone(), start, end).await;
            (id, result)
        }
    }).collect();

    let results = futures_util::future::join_all(futures).await;

    let mut history_map = HashMap::new();
    for (license_id, result) in results {
        if let Ok(analytics) = result {
            // Extract just the uptime values (already 0.0-1.0)
            let uptimes: Vec<f64> = analytics.iter().map(|p| p.uptime).collect();
            history_map.insert(license_id, uptimes);
        }
    }

    Ok(history_map)
}

/// Get license settings from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/licenses_get_license_settings
#[server(GetLicenseSettings, "/api")]
pub async fn get_license_settings(
    license_id: String
) -> Result<LicenseSettingsResponse, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/licenses_get_license_settings", unetwork::API_URL);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({ "licenseId": license_id }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch license settings: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    // Response is an array with single item or empty
    let settings: Vec<LicenseSettingsResponse> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse license settings: {}", e)))?;

    Ok(settings.into_iter().next().unwrap_or_default())
}

/// Get lease details by license ID from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/get_lease_id_by_license
#[server(GetLeaseByLicense, "/api")]
pub async fn get_lease_by_license(
    license_id: String
) -> Result<Option<LeaseResponse>, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/get_lease_id_by_license", unetwork::API_URL);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({ "licenseId": license_id }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch lease details: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    // Response is an array - may be empty if no lease
    let leases: Vec<LeaseResponse> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse lease details: {}", e)))?;

    Ok(leases.into_iter().next())
}

/// Get wallet settings from Unetwork RPC endpoint
/// Calls: POST /rest/v1/rpc/wallet_settings_get
#[server(GetWalletSettings, "/api")]
pub async fn get_wallet_settings() -> Result<WalletSettingsResponse, ServerFnError> {
    let jwt_token = get_jwt_token()?;

    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/wallet_settings_get", unetwork::API_URL);

    let response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch wallet settings: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API error {}: {}",
            status.as_u16(),
            error_text
        )));
    }

    // Response is an array with single item or empty
    let settings: Vec<WalletSettingsResponse> = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse wallet settings: {}", e)))?;

    Ok(settings.into_iter().next().unwrap_or_default())
}

/// Get paginated rewards for a license from local ScyllaDB
#[server(GetLicensePaginatedRewards, "/api")]
pub async fn get_license_paginated_rewards(
    license_id: String,
    page: u32,
    limit: u32,
) -> Result<crate::models::entity::PaginatedResult<crate::models::entity::RewardEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::repository::RewardRepository;
    use crate::repository::traits::RewardRepositoryTrait;
    use crate::models::entity::{ListRewardsByLicenseParams, PaginationParams};

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?;

    let repo = RewardRepository::new(pool);

    let params = ListRewardsByLicenseParams {
        license_id,
        pagination: PaginationParams { page, limit },
    };

    repo.list_rewards_by_license_id(params)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Publish licenses to marketplace response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishToMarketplaceResponse {
    pub published: i32,
    pub failed: i32,
    pub errors: Vec<String>,
}

/// Publish licenses to marketplace
/// Makes selected licenses available for claiming on uno-app
#[server(PublishToMarketplace, "/api")]
pub async fn publish_to_marketplace(
    license_ids: Vec<String>,
) -> Result<PublishToMarketplaceResponse, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::MarketplaceService;

    if license_ids.is_empty() {
        return Ok(PublishToMarketplaceResponse {
            published: 0,
            failed: 0,
            errors: vec![],
        });
    }

    let pool = get_db().ok_or_else(|| ServerFnError::new("Database not available"))?;
    let service = MarketplaceService::new(pool)
        .map_err(|e| ServerFnError::new(format!("Failed to create marketplace service: {}", e)))?;

    let result = service.publish_to_marketplace(license_ids).await
        .map_err(|e| ServerFnError::new(format!("Failed to publish to marketplace: {}", e)))?;

    Ok(PublishToMarketplaceResponse {
        published: result.published,
        failed: result.failed,
        errors: result.errors,
    })
}

/// Get marketplace licenses with live claim status from uno-app
/// Syncs claim statuses from uno-app on every call for fresh data
#[server(GetMarketplaceLicenses, "/api")]
pub async fn get_marketplace_licenses() -> Result<Vec<crate::models::entity::LicenseEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::MarketplaceService;

    let pool = get_db().ok_or_else(|| ServerFnError::new("Database not available"))?;
    let service = MarketplaceService::new(pool)
        .map_err(|e| ServerFnError::new(format!("Failed to create marketplace service: {}", e)))?;

    // Sync claim statuses from uno-app and return enriched licenses
    service.sync_and_get_licenses().await
        .map_err(|e| ServerFnError::new(format!("Failed to sync marketplace licenses: {}", e)))
}

/// Unpublish licenses from marketplace
/// Removes selected licenses from the public marketplace
#[server(UnpublishFromMarketplace, "/api")]
pub async fn unpublish_from_marketplace(
    license_ids: Vec<String>,
) -> Result<usize, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::MarketplaceService;

    if license_ids.is_empty() {
        return Ok(0);
    }

    let pool = get_db().ok_or_else(|| ServerFnError::new("Database not available"))?;
    let service = MarketplaceService::new(pool)
        .map_err(|e| ServerFnError::new(format!("Failed to create marketplace service: {}", e)))?;

    service.unpublish_from_marketplace(license_ids).await
        .map_err(|e| ServerFnError::new(format!("Failed to unpublish from marketplace: {}", e)))
}

/// Response from polling marketplace status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollMarketplaceResponse {
    pub fetched: usize,
    pub updated: usize,
}

/// Poll marketplace for claimed licenses
/// Fetches claimed license statuses from uno-app and updates local records
#[server(PollMarketplaceStatus, "/api")]
pub async fn poll_marketplace_status() -> Result<PollMarketplaceResponse, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::MarketplaceService;

    let pool = get_db().ok_or_else(|| ServerFnError::new("Database not available"))?;
    let service = MarketplaceService::new(pool)
        .map_err(|e| ServerFnError::new(format!("Failed to create marketplace service: {}", e)))?;

    let result = service.poll_and_sync().await
        .map_err(|e| ServerFnError::new(format!("Failed to poll marketplace status: {}", e)))?;

    Ok(PollMarketplaceResponse {
        fetched: result.fetched,
        updated: result.updated,
    })
}

// ========================================
// Bulk License Settings API
// ========================================

/// Request for bulk saving license settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkSaveSettingsRequest {
    pub license_ids: Vec<String>,
    pub share_percentage: f64,
    pub lease_duration_months: i32,
    pub uptime_requirement: f64,
    pub uno_marketplace: bool,
    pub unetwork_marketplace: bool,
    pub referral_code: Option<String>,
}

/// Response from bulk save license settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkSaveSettingsResponse {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<(String, String)>,
}

/// Bulk save license settings to Unetwork API and local ScyllaDB
///
/// Calls:
/// - Unetwork API: POST /rest/v1/rpc/licenses_bulk_save_license_settings
/// - Local ScyllaDB: update_marketplace_status_extended for each license
#[server(BulkSaveLicenseSettings, "/api")]
pub async fn bulk_save_license_settings(
    request: BulkSaveSettingsRequest,
) -> Result<BulkSaveSettingsResponse, ServerFnError> {
    use crate::db::get_db;
    use crate::repository::LicenseRepository;
    use crate::repository::traits::LicenseRepositoryTrait;

    let jwt_token = get_jwt_token()?;
    let client = crate::api::http_client::get_client();
    let url = format!("{}/rest/v1/rpc/licenses_bulk_save_license_settings", unetwork::API_URL);

    // Build payload for Unetwork API
    let api_payload = serde_json::json!({
        "license_ids": request.license_ids,
        "leaseDefaultSharePercentage": request.share_percentage,
        "leaseDefaultDurationMonths": request.lease_duration_months,
        "leaseDefaultMinUptimePercentage": request.uptime_requirement,
        "marketplaceOn": request.unetwork_marketplace
    });

    eprintln!("=== BULK SAVE LICENSE SETTINGS ===");
    eprintln!("URL: {}", url);
    eprintln!("Payload: {}", serde_json::to_string_pretty(&api_payload).unwrap_or_default());

    // Call Unetwork API
    let api_response = client
        .post(&url)
        .header("apikey", unetwork::API_KEY)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .header("Content-Profile", "public")
        .json(&api_payload)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to call Unetwork API: {}", e)))?;

    let api_status = api_response.status();
    let api_text = api_response.text().await.unwrap_or_default();

    eprintln!("API Response Status: {}", api_status);
    eprintln!("API Response: {}", &api_text[..api_text.len().min(500)]);

    if !api_status.is_success() {
        return Err(ServerFnError::new(format!(
            "Unetwork API error {}: {}",
            api_status.as_u16(),
            api_text
        )));
    }

    // Now persist to local ScyllaDB (optional - licenses may not exist locally yet)
    let total = request.license_ids.len();
    let mut local_updated = 0;
    let mut local_skipped = 0;

    if let Some(pool) = get_db() {
        let repo = LicenseRepository::new(pool);

        // Determine marketplace status based on settings
        let status = if request.uno_marketplace || request.unetwork_marketplace {
            Some("unclaimed")
        } else {
            None
        };

        for license_id in &request.license_ids {
            match repo.update_marketplace_status_extended(
                license_id,
                request.unetwork_marketplace,
                request.uno_marketplace,
                status,
                request.referral_code.as_deref(),
            ).await {
                Ok(_) => local_updated += 1,
                Err(e) => {
                    // License doesn't exist locally - this is OK, Unetwork API was already updated
                    eprintln!("Note: Local DB update skipped for {}: {}", license_id, e);
                    local_skipped += 1;
                }
            }
        }
    } else {
        eprintln!("Warning: Local database not available, skipping local updates");
        local_skipped = total;
    }

    // All succeeded because Unetwork API call was successful
    let succeeded = total;
    let failed = 0;
    let errors: Vec<(String, String)> = Vec::new();

    eprintln!("=== BULK SAVE COMPLETE ===");
    eprintln!("Total: {}, Succeeded: {}, Failed: {}", total, succeeded, failed);

    Ok(BulkSaveSettingsResponse {
        total,
        succeeded,
        failed,
        errors,
    })
}

#[cfg(feature = "ssr")]
pub fn register_license_server_fns() {
    use server_fn::ServerFn;

    // Reference the URL to ensure the server function is linked
    println!("Registering license server functions:");
    println!("  ListLicenses: {}", ListLicenses::url());
    println!("  GetLicensesSummary: {}", GetLicensesSummary::url());
    println!("  GetLicenseById: {}", GetLicenseById::url());
    println!("  GetOnlineLicenses: {}", GetOnlineLicenses::url());
    println!("  GetUnoLicensesSummary: {}", GetUnoLicensesSummary::url());
    println!("  GetAllocationsSummary: {}", GetAllocationsSummary::url());
    println!("  GetPaginatedLicenses: {}", GetPaginatedLicenses::url());
    println!("  GetAllLicensesForFiltering: {}", GetAllLicensesForFiltering::url());
    println!("  GetLicenseAllocationSummary: {}", GetLicenseAllocationSummary::url());
    println!("  GetLicenseUptimeAnalytics: {}", GetLicenseUptimeAnalytics::url());
    println!("  GetLicenseSettings: {}", GetLicenseSettings::url());
    println!("  GetLeaseByLicense: {}", GetLeaseByLicense::url());
    println!("  GetWalletSettings: {}", GetWalletSettings::url());
    println!("  GetLicensePaginatedRewards: {}", GetLicensePaginatedRewards::url());
    println!("  PublishToMarketplace: {}", PublishToMarketplace::url());
    println!("  GetMarketplaceLicenses: {}", GetMarketplaceLicenses::url());
    println!("  UnpublishFromMarketplace: {}", UnpublishFromMarketplace::url());
    println!("  BulkSaveLicenseSettings: {}", BulkSaveLicenseSettings::url());
}
