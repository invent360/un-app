//! Service interface traits
//!
//! This module defines trait interfaces for all services, enabling:
//! - Dependency injection via trait objects
//! - Easy mocking for unit tests
//! - Clear service API contracts

use async_trait::async_trait;
use std::sync::Arc;
use crate::types::AppError;

/// Stats service trait for statistics operations
#[async_trait]
pub trait StatsService: Send + Sync {
    /// Get license statistics
    async fn get_license_stats(&self) -> Result<LicenseStats, AppError>;

    /// Get claim statistics for a time period
    async fn get_claim_stats(&self, period: StatsPeriod) -> Result<ClaimStats, AppError>;
}

/// Dynamic stats service type for dependency injection
pub type DynStatsService = Arc<dyn StatsService>;

/// Statistics period for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatsPeriod {
    Day,
    Week,
    Month,
    All,
}

/// License statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseStats {
    pub total: i64,
    pub claimed: i64,
    pub unclaimed: i64,
    pub expired: i64,
}

/// Claim statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClaimStats {
    pub total_claims: i64,
    pub claims_today: i64,
    pub claims_this_week: i64,
    pub claims_this_month: i64,
}

/// FAQ service trait
#[async_trait]
pub trait FaqService: Send + Sync {
    /// Get all FAQ items
    async fn get_all_faqs(&self) -> Result<Vec<FaqItem>, AppError>;

    /// Get FAQ by category
    async fn get_faqs_by_category(&self, category: &str) -> Result<Vec<FaqItem>, AppError>;

    /// Search FAQs
    async fn search_faqs(&self, query: &str) -> Result<Vec<FaqItem>, AppError>;
}

/// Dynamic FAQ service type
pub type DynFaqService = Arc<dyn FaqService>;

/// FAQ item structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FaqItem {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub category: String,
    pub order: i32,
}
