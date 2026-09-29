//! Error types for the application

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error response structure for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: String,
}

impl ErrorResponse {
    pub fn new<S: Into<String>>(message: S, code: S) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
        }
    }
}

/// Application error type following rubix pattern
#[derive(Debug, Error, Serialize, Deserialize, PartialEq, Clone)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    DBError(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("License unavailable: {0}")]
    LicenseUnavailable(String),

    #[error("Import error: {0}")]
    ImportError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl AppError {
    pub fn error_response(&self) -> ErrorResponse {
        let code = match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::DBError(_) => "DATABASE_ERROR",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::LicenseUnavailable(_) => "LICENSE_UNAVAILABLE",
            AppError::ImportError(_) => "IMPORT_ERROR",
            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            AppError::ConfigError(_) => "CONFIG_ERROR",
        };
        ErrorResponse::new(self.to_string(), code.to_string())
    }
}

#[cfg(feature = "ssr")]
mod ssr_impl {
    use super::*;
    use actix_web::{error, http::StatusCode, HttpResponse};

    impl error::ResponseError for AppError {
        fn status_code(&self) -> StatusCode {
            match self {
                AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
                AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                AppError::Forbidden(_) => StatusCode::FORBIDDEN,
                AppError::NotFound(_) => StatusCode::NOT_FOUND,
                AppError::Conflict(_) => StatusCode::CONFLICT,
                AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
                AppError::LicenseUnavailable(_) => StatusCode::CONFLICT,
                AppError::ImportError(_) => StatusCode::BAD_REQUEST,
                AppError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }

        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(self.status_code()).json(self.error_response())
        }
    }

    impl From<sqlx::Error> for AppError {
        fn from(err: sqlx::Error) -> Self {
            AppError::DBError(err.to_string())
        }
    }

    impl From<actix_web::error::Error> for AppError {
        fn from(err: actix_web::error::Error) -> Self {
            AppError::InternalServerError(err.to_string())
        }
    }
}

/// API error response (alias for ErrorResponse for backwards compatibility)
pub type ApiError = ErrorResponse;
