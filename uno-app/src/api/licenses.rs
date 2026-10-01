//! License-related server functions
//!
//! Implements atomic two-phase claim flow:
//! 1. Reserve: Get a license with session token, validate referral
//! 2. Confirm: Claim with session verification, immutable referral attribution

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
        user_id: None,
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

// ============================================================================
// ATOMIC CLAIM FUNCTIONS (New Two-Phase Flow)
// ============================================================================
// These functions provide atomic reservation with session tokens and
// immutable referral attribution.

/// Response from atomic reservation
///
/// R5-05: Does NOT include lease_code - credential is revealed only at confirmation
/// when ownership is established.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AtomicReservationResponse {
    pub success: bool,
    pub message: Option<String>,
    pub license_id: Option<String>,
    /// R5-05: lease_code intentionally omitted - revealed only at confirm
    /// Session token required for confirmation - keep secure!
    pub session_token: Option<String>,
    /// When the reservation expires (2 minutes from now)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub split_type: Option<SplitType>,
    /// Whether the provided referral code was validated
    pub referral_validated: bool,
    /// R5-05: Remaining capacity after this reservation
    pub capacity_remaining: Option<i64>,
}

/// Atomically reserve a license with session binding (Phase 1 - New)
///
/// This is the secure version that:
/// - Atomically locks a license within a transaction
/// - Returns a session token required for confirmation
/// - Validates referral code (but doesn't attribute yet)
/// - Expires after 2 minutes if not confirmed
///
/// Use `atomic_confirm_claim` with the session_token to complete the claim.
#[server(AtomicReserveLicense, "/api")]
pub async fn atomic_reserve_license(
    split_type: Option<SplitType>,
    referral_code: Option<String>,
) -> Result<AtomicReservationResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Use atomic reserve
    let response = factory.license_service.atomic_reserve(split_type, referral_code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // R5-05: lease_code intentionally omitted - revealed only at confirm
    Ok(AtomicReservationResponse {
        success: response.success,
        message: response.message,
        license_id: response.license_id,
        session_token: response.session_token,
        expires_at: response.expires_at,
        split_type: response.split_type,
        referral_validated: response.referral_validated,
        capacity_remaining: response.capacity_remaining,
    })
}

/// Atomically confirm a reservation and claim the license (Phase 2 - New)
///
/// This is the secure version that:
/// - Requires the session token from reservation
/// - F2: Verifies owner_id matches the user who made the reservation
/// - Verifies the reservation hasn't expired
/// - Atomically sets claimed=true AND referral_id in one transaction
/// - Referral attribution is immutable (once set, cannot be changed)
/// - Idempotent - calling again returns success if already claimed by same session
#[server(AtomicConfirmClaim, "/api")]
pub async fn atomic_confirm_claim(
    license_id: String,
    session_token: String,
    device_id: Option<String>,
    referral_code: Option<String>,
) -> Result<ClaimResponse, ServerFnError> {
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;

    let factory: Data<ServiceFactory> = extract().await?;
    let req: HttpRequest = extract().await?;

    // F2: Try to get authenticated user ID for owner verification
    // During onboarding, users may not be fully authenticated yet - in that case
    // we use "anonymous" and rely on session_token as primary authorization
    let owner_id = match get_authenticated_user(&req) {
        Ok(auth_user) => auth_user.id,
        Err(_) => "anonymous".to_string(),
    };

    // Get referral ID if code provided
    let referral_id = if let Some(code) = referral_code {
        let code = code.trim().to_uppercase();
        if !code.is_empty() {
            match factory.referral_repository.get_active_by_code(&code).await {
                Ok(Some(referral)) => Some(referral.id),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };

    // Use atomic confirm - this does everything in one transaction
    // F2: Now includes owner_id for verification against reservation's user_id
    factory.license_service.atomic_confirm(&license_id, &session_token, &owner_id, device_id, referral_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Release a reservation without claiming
///
/// Call this if the user abandons the claim flow before confirming.
/// This frees the license for other users immediately instead of waiting
/// for the 2-minute expiry.
#[server(ReleaseReservation, "/api")]
pub async fn release_reservation(
    license_id: String,
    session_token: String,
) -> Result<(), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    factory.license_service.release_reservation(&license_id, &session_token)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
