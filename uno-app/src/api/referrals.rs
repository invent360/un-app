//! Referral-related server functions

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::types::ValidateReferralResponse;

/// Response for referral application submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralApplicationResponse {
    pub success: bool,
    pub message: String,
}

impl ReferralApplicationResponse {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
        }
    }
}

/// Validate a referral code
///
/// Checks if the referral code exists and is active (can be used for claims).
/// Returns the referral's username if valid for display purposes.
#[server(ValidateReferralCode, "/api")]
pub async fn validate_referral_code(code: String) -> Result<ValidateReferralResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    // Trim and normalize the code
    let code = code.trim().to_uppercase();

    if code.is_empty() {
        return Ok(ValidateReferralResponse::invalid("Referral code cannot be empty"));
    }

    if code.len() < 3 || code.len() > 20 {
        return Ok(ValidateReferralResponse::invalid("Invalid referral code format"));
    }

    let factory: Data<ServiceFactory> = extract().await?;

    // Check if the referral exists and is active
    let referral = factory.referral_repository
        .get_active_by_code(&code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match referral {
        Some(r) => Ok(ValidateReferralResponse::valid(r.username)),
        None => {
            // Check if code exists but is not active
            let exists = factory.referral_repository
                .get_by_code(&code)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

            if exists.is_some() {
                Ok(ValidateReferralResponse::invalid("This referral code is no longer active"))
            } else {
                Ok(ValidateReferralResponse::invalid("Referral code not found"))
            }
        }
    }
}

/// Get referral ID by code (internal use for claim processing)
///
/// Returns the referral ID if the code is valid and active, None otherwise.
#[server(GetReferralIdByCode, "/api")]
pub async fn get_referral_id_by_code(code: String) -> Result<Option<i32>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let code = code.trim().to_uppercase();

    if code.is_empty() {
        return Ok(None);
    }

    let factory: Data<ServiceFactory> = extract().await?;

    let referral = factory.referral_repository
        .get_active_by_code(&code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(referral.map(|r| r.id))
}

/// Submit a referral application
///
/// Creates a new referral with pending status. Validates all fields
/// and checks for duplicate email/code before creating.
#[server(SubmitReferralApplication, "/api")]
pub async fn submit_referral_application(
    username: String,
    email: String,
    country_code: String,
    referral_code: String,
) -> Result<ReferralApplicationResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    // Validate and normalize inputs
    let username = username.trim().to_string();
    let email = email.trim().to_lowercase();
    let country_code = country_code.trim().to_uppercase();
    let referral_code = referral_code.trim().to_uppercase();

    // Validation
    if username.is_empty() || username.len() < 2 || username.len() > 100 {
        return Ok(ReferralApplicationResponse::error("Username must be between 2 and 100 characters"));
    }

    if !email.contains('@') || email.len() < 5 || email.len() > 255 {
        return Ok(ReferralApplicationResponse::error("Please enter a valid email address"));
    }

    if country_code.len() != 2 {
        return Ok(ReferralApplicationResponse::error("Please select a valid country"));
    }

    if referral_code.is_empty() || referral_code.len() < 3 || referral_code.len() > 20 {
        return Ok(ReferralApplicationResponse::error("Referral code must be between 3 and 20 characters"));
    }

    // Only allow alphanumeric characters in referral code
    if !referral_code.chars().all(|c| c.is_alphanumeric()) {
        return Ok(ReferralApplicationResponse::error("Referral code can only contain letters and numbers"));
    }

    let factory: Data<ServiceFactory> = extract().await?;

    // Check if email already exists
    let existing_email = factory.referral_repository
        .get_by_email(&email)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if existing_email.is_some() {
        return Ok(ReferralApplicationResponse::error("An application with this email already exists"));
    }

    // Check if referral code already exists
    let code_exists = factory.referral_repository
        .code_exists(&referral_code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if code_exists {
        return Ok(ReferralApplicationResponse::error("This referral code is already taken. Please choose another."));
    }

    // Create the referral application
    factory.referral_repository
        .create(&username, &email, &country_code, &referral_code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(ReferralApplicationResponse::success("Your application has been submitted successfully! We will review it and get back to you soon."))
}
