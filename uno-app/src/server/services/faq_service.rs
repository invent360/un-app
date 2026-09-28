//! FAQ service for business logic

use crate::types::{FaqItem, FaqItemResponse, FaqCategory, FaqListResponse, FaqSearchParams, AppError};
use crate::server::repositories::DynFaqRepository;

/// FAQ service for FAQ operations
#[derive(Clone)]
pub struct FaqServiceImpl {
    faq_repo: DynFaqRepository,
}

impl FaqServiceImpl {
    pub fn new(faq_repo: DynFaqRepository) -> Self {
        Self { faq_repo }
    }

    /// Get all FAQ items with optional filtering
    pub async fn get_faqs(&self, params: FaqSearchParams) -> Result<FaqListResponse, AppError> {
        let locale = params.locale.as_deref().unwrap_or("en");

        // Get items based on filters
        let items = if let Some(ref query) = params.query {
            if query.trim().is_empty() {
                self.get_items_by_category(params.category.as_deref()).await?
            } else {
                self.faq_repo.search(query).await?
            }
        } else if params.featured_only {
            self.faq_repo.get_featured().await?
        } else {
            self.get_items_by_category(params.category.as_deref()).await?
        };

        // Convert to localized response items
        let response_items: Vec<FaqItemResponse> = items
            .iter()
            .map(|item| FaqItemResponse::from_item(item, locale))
            .collect();

        // Get categories
        let categories = self.faq_repo.get_categories().await?;

        let total = response_items.len();

        Ok(FaqListResponse {
            items: response_items,
            categories,
            total,
        })
    }

    /// Helper to get items optionally filtered by category
    async fn get_items_by_category(&self, category: Option<&str>) -> Result<Vec<FaqItem>, AppError> {
        match category {
            Some(cat) if !cat.is_empty() => self.faq_repo.get_by_category(cat).await,
            _ => self.faq_repo.get_all().await,
        }
    }

    /// Search FAQ items
    pub async fn search_faqs(&self, query: &str, locale: Option<&str>) -> Result<Vec<FaqItemResponse>, AppError> {
        let locale = locale.unwrap_or("en");

        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let items = self.faq_repo.search(query).await?;

        let response_items: Vec<FaqItemResponse> = items
            .iter()
            .map(|item| FaqItemResponse::from_item(item, locale))
            .collect();

        Ok(response_items)
    }

    /// Get all FAQ categories
    pub async fn get_categories(&self) -> Result<Vec<FaqCategory>, AppError> {
        self.faq_repo.get_categories().await
    }

    /// Get featured FAQ items
    pub async fn get_featured(&self, locale: Option<&str>) -> Result<Vec<FaqItemResponse>, AppError> {
        let locale = locale.unwrap_or("en");

        let items = self.faq_repo.get_featured().await?;

        let response_items: Vec<FaqItemResponse> = items
            .iter()
            .map(|item| FaqItemResponse::from_item(item, locale))
            .collect();

        Ok(response_items)
    }

    /// Get a single FAQ item by ID
    pub async fn get_faq_by_id(&self, id: i32, locale: Option<&str>) -> Result<Option<FaqItemResponse>, AppError> {
        let locale = locale.unwrap_or("en");

        let item = self.faq_repo.get_by_id(id).await?;

        Ok(item.map(|i| FaqItemResponse::from_item(&i, locale)))
    }
}
