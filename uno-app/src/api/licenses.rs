//! License-related server functions

use leptos::prelude::*;
use crate::types::{LicenseVariant, ClaimResponse, ClaimPageData, SplitType};

/// Get all available license split options
#[server(GetVariants, "/api")]
pub async fn get_variants() -> Result<Vec<LicenseVariant>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Get availability for each split type
    let split_availability = factory.license_service.get_available_splits()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Convert to LicenseVariant for UI compatibility
    let variants: Vec<LicenseVariant> = split_availability
        .into_iter()
        .map(|sa| LicenseVariant::from_split_with_counts(sa.split_type, sa.available, sa.claimed, sa.total))
        .collect();

    Ok(variants)
}

/// Claim a license by split type (auto-assigns an available license)
#[server(ClaimBySplitType, "/api")]
pub async fn claim_by_split_type(
    split_type: SplitType,
    device_id: Option<String>,
    referral_code: Option<String>,
) -> Result<ClaimResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // First, claim the license
    let response = factory.license_service.claim_by_split_type(split_type, device_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If claim was successful and a referral code was provided, link the referral
    if response.success {
        if let Some(code) = referral_code {
            let code = code.trim().to_uppercase();
            if !code.is_empty() {
                // Get the active referral by code
                if let Ok(Some(referral)) = factory.referral_repository.get_active_by_code(&code).await {
                    // Get the license_id from the response
                    if let Some(ref license_id) = response.license_id {
                        // Set the referral_id on the claim
                        if let Err(e) = factory.claim_repository.set_referral(license_id, referral.id).await {
                            // Log the error but don't fail the claim
                            tracing::warn!("Failed to set referral on claim: {}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(response)
}

/// Claim a license by lease code
#[server(ClaimLicense, "/api")]
pub async fn claim_license(
    lease_code: String,
    device_id: Option<String>,
) -> Result<ClaimResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::types::ClaimRequest;

    let factory: Data<ServiceFactory> = extract().await?;

    let request = ClaimRequest {
        lease_code,
        device_id,
    };

    factory.license_service.claim_license(request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get license data by lease code
#[server(GetClaimData, "/api")]
pub async fn get_claim_data(lease_code: String) -> Result<ClaimPageData, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    let data = factory.license_service.get_license_by_code(&lease_code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match data {
        Some(claim_data) => Ok(claim_data),
        None => Err(ServerFnError::new("License not found")),
    }
}

/// Reserve a license by split type (returns license info without marking as claimed)
/// This is phase 1 of the two-phase claim process.
#[server(ReserveBySplitType, "/api")]
pub async fn reserve_by_split_type(
    split_type: SplitType,
    referral_code: Option<String>,
) -> Result<ClaimResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Get an unclaimed license without marking it as claimed
    let response = factory.license_service.reserve_by_split_type(split_type)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If reservation was successful and a referral code was provided, store it for later
    // The referral will be linked when the claim is confirmed
    if response.success {
        if let Some(code) = referral_code {
            let code = code.trim().to_uppercase();
            if !code.is_empty() {
                // Validate the referral code exists
                if let Ok(Some(_)) = factory.referral_repository.get_active_by_code(&code).await {
                    // Referral code is valid - it will be linked during confirm_claim
                    tracing::info!("Valid referral code provided during reservation: {}", code);
                }
            }
        }
    }

    Ok(response)
}

/// Reserve the next available license (any split type).
/// Returns license info without marking as claimed.
/// This is the simplified claim flow - phase 1.
#[server(ReserveNextLicense, "/api")]
pub async fn reserve_next_license(
    referral_code: Option<String>,
) -> Result<ClaimResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Get the next available unclaimed license (any split type)
    let response = factory.license_service.reserve_next_available()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If reservation was successful and a referral code was provided, validate it
    if response.success {
        if let Some(code) = referral_code {
            let code = code.trim().to_uppercase();
            if !code.is_empty() {
                if let Ok(Some(_)) = factory.referral_repository.get_active_by_code(&code).await {
                    tracing::info!("Valid referral code provided during reservation: {}", code);
                }
            }
        }
    }

    Ok(response)
}

/// Check if any licenses are available for claiming.
#[server(CheckLicenseAvailability, "/api")]
pub async fn check_license_availability() -> Result<bool, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    factory.license_service.has_available_licenses()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Confirm a license claim by license ID (marks the license as claimed)
/// This is phase 2 of the two-phase claim process - called when user copies the key.
#[server(ConfirmLicenseClaim, "/api")]
pub async fn confirm_license_claim(
    license_id: String,
    device_id: Option<String>,
    referral_code: Option<String>,
) -> Result<ClaimResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Confirm the claim (marks the license as claimed)
    let response = factory.license_service.confirm_claim(&license_id, device_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If claim was successful and a referral code was provided, link the referral
    if response.success {
        if let Some(code) = referral_code {
            let code = code.trim().to_uppercase();
            if !code.is_empty() {
                // Get the active referral by code
                if let Ok(Some(referral)) = factory.referral_repository.get_active_by_code(&code).await {
                    // Set the referral_id on the claim
                    if let Err(e) = factory.claim_repository.set_referral(&license_id, referral.id).await {
                        // Log the error but don't fail the claim
                        tracing::warn!("Failed to set referral on claim: {}", e);
                    }
                }
            }
        }
    }

    Ok(response)
}
