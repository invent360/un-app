//! CSV import service for Unetwork license data

use csv::Reader;
use chrono::{DateTime, Utc, NaiveDateTime};
use std::io::Read;
use std::collections::HashMap;

use crate::types::{License, LicenseVariant, CsvLicenseRow, ImportResult, AppError, VariantStatus};
use crate::server::repositories::{DynLicenseRepository, DynVariantRepository};

/// CSV import service for license data
#[derive(Clone)]
pub struct CsvImportServiceImpl {
    license_repo: DynLicenseRepository,
    variant_repo: DynVariantRepository,
}

impl CsvImportServiceImpl {
    pub fn new(
        license_repo: DynLicenseRepository,
        variant_repo: DynVariantRepository,
    ) -> Self {
        Self {
            license_repo,
            variant_repo,
        }
    }

    /// Import licenses from CSV data
    pub async fn import<R: Read>(&self, reader: R) -> Result<ImportResult, AppError> {
        let mut csv_reader = Reader::from_reader(reader);
        let mut result = ImportResult {
            total_rows: 0,
            imported: 0,
            skipped: 0,
            errors: Vec::new(),
            variants_created: 0,
        };

        // Track variants to create/update
        let mut variant_counts: HashMap<i32, i32> = HashMap::new();

        for row_result in csv_reader.deserialize::<CsvLicenseRow>() {
            result.total_rows += 1;

            match row_result {
                Ok(row) => {
                    // Only import leased licenses
                    if row.is_leased != Some(true) {
                        result.skipped += 1;
                        continue;
                    }

                    // Parse the CSV row into a License
                    match self.parse_license_row(&row) {
                        Ok(license) => {
                            // Insert the license
                            if let Err(e) = self.license_repo.insert(&license).await {
                                result.errors.push(format!("Row {}: {}", result.total_rows, e));
                                result.skipped += 1;
                                continue;
                            }

                            // Track variant count
                            if let Some(share) = license.lease_share_percentage {
                                let share_int = share as i32;
                                *variant_counts.entry(share_int).or_insert(0) += 1;
                            }

                            result.imported += 1;
                        }
                        Err(e) => {
                            result.errors.push(format!("Row {}: {}", result.total_rows, e));
                            result.skipped += 1;
                        }
                    }
                }
                Err(e) => {
                    result.errors.push(format!("Row {}: Parse error: {}", result.total_rows, e));
                    result.skipped += 1;
                }
            }
        }

        // Create/update variants based on imported licenses
        for (user_share, count) in variant_counts {
            let operator_share = 100 - user_share;

            let variant = LicenseVariant {
                id: 0,
                user_share_percentage: user_share,
                operator_share_percentage: operator_share,
                lease_duration_months: 12,
                min_uptime_percentage: 75.0,
                total_quantity: count,
                claimed_count: 0,
                min_monthly_earnings: calculate_min_earnings(user_share),
                max_monthly_earnings: calculate_max_earnings(user_share),
                display_name: Some(format!("{}:{} Split", user_share, operator_share)),
                display_order: 100 - user_share,
                is_featured: user_share >= 60,
                status: VariantStatus::Active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            if let Err(e) = self.variant_repo.upsert(&variant).await {
                result.errors.push(format!("Failed to create variant {}:{}: {}", user_share, operator_share, e));
            } else {
                result.variants_created += 1;
            }
        }

        Ok(result)
    }

    /// Parse a CSV row into a License struct
    fn parse_license_row(&self, row: &CsvLicenseRow) -> Result<License, AppError> {
        Ok(License {
            id: row.id.clone(),
            node_id: row.node_id.clone(),
            owner_wallet_address: row.owner_wallet_address.clone(),
            device_name: row.device_name.clone(),
            activation_start_at: parse_optional_datetime(&row.activation_start_at),
            activation_end_at: parse_optional_datetime(&row.activation_end_at),
            is_active: row.is_active.unwrap_or(true),
            is_leased: row.is_leased.unwrap_or(false),
            lease_from: parse_optional_datetime(&row.lease_from),
            lease_to: parse_optional_datetime(&row.lease_to),
            lease_share_percentage: row.lease_share_percentage,
            lease_min_uptime_percentage: row.lease_min_uptime_percentage,
            uptime: row.uptime,
            imported_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

/// Parse an optional datetime string
fn parse_optional_datetime(s: &Option<String>) -> Option<DateTime<Utc>> {
    s.as_ref().and_then(|s| {
        if s.is_empty() {
            return None;
        }
        // Try ISO 8601 format first
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .ok()
            .or_else(|| {
                // Try common formats
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .map(|ndt| ndt.and_utc())
                    .ok()
            })
            .or_else(|| {
                NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                    .map(|ndt| ndt.and_utc())
                    .ok()
            })
    })
}

/// Calculate minimum monthly earnings based on user share percentage
fn calculate_min_earnings(user_share: i32) -> Option<f64> {
    let base_min = 3.0;
    Some(base_min * (user_share as f64 / 100.0))
}

/// Calculate maximum monthly earnings based on user share percentage
fn calculate_max_earnings(user_share: i32) -> Option<f64> {
    let base_max = 15.0;
    Some(base_max * (user_share as f64 / 100.0))
}
