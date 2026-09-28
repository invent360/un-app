//! Variant service for business logic

use async_trait::async_trait;
use crate::types::{LicenseVariant, AppError, PaginationParams, PaginatedResponse, VariantStatus};
use crate::server::repositories::DynVariantRepository;
use super::traits::VariantService;

/// Variant service implementation
#[derive(Clone)]
pub struct VariantServiceImpl {
    variant_repo: DynVariantRepository,
}

impl VariantServiceImpl {
    pub fn new(variant_repo: DynVariantRepository) -> Self {
        Self { variant_repo }
    }
}

#[async_trait]
impl VariantService for VariantServiceImpl {
    /// Get all available variants (active with remaining licenses)
    async fn get_available_variants(&self) -> Result<Vec<LicenseVariant>, AppError> {
        let variants = self.variant_repo.get_active().await?;

        // Filter to only those with remaining licenses
        let available: Vec<LicenseVariant> = variants
            .into_iter()
            .filter(|v| v.remaining() > 0)
            .collect();

        Ok(available)
    }

    /// Get available variants with pagination
    async fn get_available_variants_paginated(
        &self,
        params: &PaginationParams,
    ) -> Result<PaginatedResponse<LicenseVariant>, AppError> {
        let response = self.variant_repo.get_active_paginated(params).await?;

        // Filter to only those with remaining licenses
        let available: Vec<LicenseVariant> = response.data
            .into_iter()
            .filter(|v| v.remaining() > 0)
            .collect();

        // Note: Total count may differ due to filtering. For accurate pagination,
        // the repository should handle the remaining > 0 filter in SQL.
        Ok(PaginatedResponse {
            data: available,
            pagination: response.pagination,
        })
    }

    /// Get all active variants (including depleted)
    async fn get_all_variants(&self) -> Result<Vec<LicenseVariant>, AppError> {
        self.variant_repo.get_active().await
    }

    /// Get variant by ID
    async fn get_variant(&self, id: i32) -> Result<Option<LicenseVariant>, AppError> {
        self.variant_repo.get_by_id(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use async_trait::async_trait;
    use chrono::Utc;
    use std::sync::Mutex;

    // Mock VariantRepository for testing
    struct MockVariantRepository {
        variants: Mutex<Vec<LicenseVariant>>,
    }

    impl MockVariantRepository {
        fn new() -> Self {
            Self {
                variants: Mutex::new(vec![]),
            }
        }

        fn with_variants(variants: Vec<LicenseVariant>) -> Self {
            Self {
                variants: Mutex::new(variants),
            }
        }
    }

    #[async_trait]
    impl crate::server::repositories::VariantRepository for MockVariantRepository {
        async fn get_active(&self) -> Result<Vec<LicenseVariant>, AppError> {
            let variants = self.variants.lock().unwrap();
            Ok(variants.iter().filter(|v| v.status == VariantStatus::Active).cloned().collect())
        }

        async fn get_active_paginated(&self, params: &crate::types::PaginationParams) -> Result<crate::types::PaginatedResponse<LicenseVariant>, AppError> {
            let variants = self.variants.lock().unwrap();
            let active: Vec<LicenseVariant> = variants.iter().filter(|v| v.status == VariantStatus::Active).cloned().collect();
            let total = active.len() as i64;
            let start = params.offset() as usize;
            let end = (start + params.limit() as usize).min(active.len());
            let data = if start < active.len() {
                active[start..end].to_vec()
            } else {
                vec![]
            };
            Ok(crate::types::PaginatedResponse::new(data, params, total))
        }

        async fn count_active(&self) -> Result<i64, AppError> {
            let variants = self.variants.lock().unwrap();
            Ok(variants.iter().filter(|v| v.status == VariantStatus::Active).count() as i64)
        }

        async fn get_by_id(&self, id: i32) -> Result<Option<LicenseVariant>, AppError> {
            let variants = self.variants.lock().unwrap();
            Ok(variants.iter().find(|v| v.id == id).cloned())
        }

        async fn get_by_share(&self, user_share: i32, operator_share: i32) -> Result<Option<LicenseVariant>, AppError> {
            let variants = self.variants.lock().unwrap();
            Ok(variants.iter()
                .find(|v| v.user_share_percentage == user_share && v.operator_share_percentage == operator_share)
                .cloned())
        }

        async fn upsert(&self, variant: &LicenseVariant) -> Result<LicenseVariant, AppError> {
            let mut variants = self.variants.lock().unwrap();
            let mut new_variant = variant.clone();
            new_variant.id = variants.len() as i32 + 1;
            variants.push(new_variant.clone());
            Ok(new_variant)
        }

        async fn increment_claimed(&self, variant_id: i32) -> Result<(), AppError> {
            let mut variants = self.variants.lock().unwrap();
            if let Some(v) = variants.iter_mut().find(|v| v.id == variant_id) {
                v.claimed_count += 1;
            }
            Ok(())
        }

        async fn update_status(&self, variant_id: i32, status: VariantStatus) -> Result<(), AppError> {
            let mut variants = self.variants.lock().unwrap();
            if let Some(v) = variants.iter_mut().find(|v| v.id == variant_id) {
                v.status = status;
            }
            Ok(())
        }
    }

    fn create_test_variant(id: i32, user_share: i32, total: i32, claimed: i32, status: VariantStatus) -> LicenseVariant {
        LicenseVariant {
            id,
            user_share_percentage: user_share,
            operator_share_percentage: 100 - user_share,
            lease_duration_months: 12,
            min_uptime_percentage: 95.0,
            total_quantity: total,
            claimed_count: claimed,
            min_monthly_earnings: Some(100.0),
            max_monthly_earnings: Some(500.0),
            display_name: Some(format!("{}:{}", user_share, 100 - user_share)),
            display_order: id,
            is_featured: id == 1,
            status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_get_available_variants_filters_depleted() {
        let variants = vec![
            create_test_variant(1, 60, 100, 50, VariantStatus::Active),  // 50 remaining
            create_test_variant(2, 70, 100, 100, VariantStatus::Active), // 0 remaining (depleted)
            create_test_variant(3, 80, 100, 30, VariantStatus::Active),  // 70 remaining
        ];

        let repo = Arc::new(MockVariantRepository::with_variants(variants));
        let service = VariantServiceImpl::new(repo as DynVariantRepository);

        let result = service.get_available_variants().await;
        assert!(result.is_ok());

        let available = result.unwrap();
        assert_eq!(available.len(), 2);
        assert!(available.iter().all(|v| v.remaining() > 0));
        assert!(available.iter().find(|v| v.id == 2).is_none()); // Depleted variant not included
    }

    #[tokio::test]
    async fn test_get_available_variants_filters_inactive() {
        let variants = vec![
            create_test_variant(1, 60, 100, 50, VariantStatus::Active),
            create_test_variant(2, 70, 100, 50, VariantStatus::Inactive),
            create_test_variant(3, 80, 100, 50, VariantStatus::Active),
        ];

        let repo = Arc::new(MockVariantRepository::with_variants(variants));
        let service = VariantServiceImpl::new(repo as DynVariantRepository);

        let result = service.get_available_variants().await;
        assert!(result.is_ok());

        let available = result.unwrap();
        assert_eq!(available.len(), 2);
        assert!(available.iter().find(|v| v.id == 2).is_none()); // Inactive variant not included
    }

    #[tokio::test]
    async fn test_get_all_variants_includes_depleted() {
        let variants = vec![
            create_test_variant(1, 60, 100, 50, VariantStatus::Active),
            create_test_variant(2, 70, 100, 100, VariantStatus::Active), // Depleted but still active
        ];

        let repo = Arc::new(MockVariantRepository::with_variants(variants));
        let service = VariantServiceImpl::new(repo as DynVariantRepository);

        let result = service.get_all_variants().await;
        assert!(result.is_ok());

        let all = result.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn test_get_variant_found() {
        let variants = vec![
            create_test_variant(1, 60, 100, 50, VariantStatus::Active),
            create_test_variant(2, 70, 100, 30, VariantStatus::Active),
        ];

        let repo = Arc::new(MockVariantRepository::with_variants(variants));
        let service = VariantServiceImpl::new(repo as DynVariantRepository);

        let result = service.get_variant(1).await;
        assert!(result.is_ok());

        let variant = result.unwrap();
        assert!(variant.is_some());
        assert_eq!(variant.unwrap().id, 1);
    }

    #[tokio::test]
    async fn test_get_variant_not_found() {
        let repo = Arc::new(MockVariantRepository::new());
        let service = VariantServiceImpl::new(repo as DynVariantRepository);

        let result = service.get_variant(999).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_variant_remaining_calculation() {
        let variant = create_test_variant(1, 60, 100, 30, VariantStatus::Active);
        assert_eq!(variant.remaining(), 70);
    }

    #[test]
    fn test_variant_progress_calculation() {
        let variant = create_test_variant(1, 60, 100, 25, VariantStatus::Active);
        assert!((variant.progress() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_variant_progress_zero_total() {
        let mut variant = create_test_variant(1, 60, 0, 0, VariantStatus::Active);
        variant.total_quantity = 0;
        assert_eq!(variant.progress(), 0.0);
    }

    #[test]
    fn test_variant_is_available() {
        let active_with_remaining = create_test_variant(1, 60, 100, 50, VariantStatus::Active);
        assert!(active_with_remaining.is_available());

        let active_depleted = create_test_variant(2, 60, 100, 100, VariantStatus::Active);
        assert!(!active_depleted.is_available());

        let inactive_with_remaining = create_test_variant(3, 60, 100, 50, VariantStatus::Inactive);
        assert!(!inactive_with_remaining.is_available());
    }

    #[test]
    fn test_variant_split_display() {
        let variant = create_test_variant(1, 60, 100, 50, VariantStatus::Active);
        assert_eq!(variant.split_display(), "60:40");

        let variant2 = create_test_variant(2, 75, 100, 50, VariantStatus::Active);
        assert_eq!(variant2.split_display(), "75:25");
    }
}
