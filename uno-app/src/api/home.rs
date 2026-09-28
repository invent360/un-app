//! Server functions for home page
//!
//! Fetches home page content from the schema-driven CMS (content_items table).
//! Supports preview mode via signed tokens.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::server::utils::url_converter::convert_storage_urls;

/// Home page section data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HomeSection {
    pub section_type: String,
    pub display_order: i32,
    pub is_visible: bool,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Hero section highlight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroHighlight {
    pub text: String,
    #[serde(default)]
    pub icon: Option<String>,
}

/// CTA button data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtaButton {
    pub text: String,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub style: Option<String>,
}

/// Hero section specific data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeroData {
    #[serde(default)]
    pub headline: String,
    #[serde(default)]
    pub headline_highlight: String,
    #[serde(default)]
    pub subheadline: String,
    #[serde(default)]
    pub cta_button: Option<CtaButton>,
    #[serde(default)]
    pub highlights: Vec<HeroHighlight>,
    #[serde(default)]
    pub media: Vec<String>,
}

/// Step data for How It Works section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HowItWorksStep {
    pub order: i32,
    #[serde(default)]
    pub icon: String,
    pub title: String,
    pub description: String,
}

/// Download button data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadButton {
    pub platform: String,
    pub link: String,
    #[serde(default = "default_true")]
    pub is_visible: bool,
}

fn default_true() -> bool { true }

/// How It Works section specific data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HowItWorksData {
    #[serde(default)]
    pub steps: Vec<HowItWorksStep>,
    #[serde(default)]
    pub download_buttons: Vec<DownloadButton>,
}

/// Earnings tier data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsTier {
    pub name: String,
    pub min_earnings: f64,
    pub max_earnings: f64,
    #[serde(default = "default_period")]
    pub period: String,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub is_popular: bool,
}

fn default_period() -> String { "month".to_string() }

/// Earnings section specific data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EarningsData {
    #[serde(default)]
    pub tiers: Vec<EarningsTier>,
    #[serde(default)]
    pub disclaimer: String,
}

/// Testimonial data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonial {
    pub quote: String,
    pub author_name: String,
    #[serde(default)]
    pub author_location: Option<String>,
    #[serde(default)]
    pub author_avatar: Option<String>,
    #[serde(default = "default_rating")]
    pub rating: i32,
    #[serde(default)]
    pub is_featured: bool,
}

fn default_rating() -> i32 { 5 }

/// Testimonials section specific data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestimonialsData {
    #[serde(default)]
    pub testimonials: Vec<Testimonial>,
    #[serde(default)]
    pub api_endpoint: Option<String>,
}

/// FAQ item for home page display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeFaqItem {
    pub id: i32,
    pub category: String,
    pub question: String,
    pub answer: String,
    pub is_featured: bool,
}

/// Response structure for home page API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomePageResponse {
    /// Page title
    pub title: String,
    /// List of sections
    pub sections: Vec<HomeSection>,
    /// Whether in preview mode
    pub is_preview: bool,
    /// FAQ items to display
    #[serde(default)]
    pub faqs: Vec<HomeFaqItem>,
}

/// Convert content item to home page response
#[cfg(feature = "ssr")]
fn content_item_to_home_response(
    item: &crate::types::ContentItem,
    locale: &str,
    is_preview: bool,
) -> HomePageResponse {
    let content = item.localize(locale);

    let title = content.get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Home")
        .to_string();

    // Extract sections from content
    let raw_sections = content.get("sections")
        .and_then(|s| s.as_array())
        .cloned()
        .unwrap_or_default();

    let sections: Vec<HomeSection> = raw_sections.into_iter().map(|section| {
        let section_type = section.get("section_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let display_order = section.get("display_order")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        let is_visible = section.get("is_visible")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let title = section.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let description = section.get("subtitle")
            .or_else(|| section.get("description"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Convert any storage URLs in the section data to display URLs
        let mut data = section.clone();
        convert_storage_urls(&mut data);

        HomeSection {
            section_type,
            display_order,
            is_visible,
            title,
            description,
            data,
        }
    }).collect();

    HomePageResponse {
        title,
        sections,
        is_preview,
        faqs: vec![],
    }
}

/// Inject testimonials from database into the testimonials section
#[cfg(feature = "ssr")]
async fn inject_testimonials(
    mut response: HomePageResponse,
    testimonial_repo: &crate::server::repositories::DynTestimonialRepository,
) -> HomePageResponse {
    // Fetch testimonials from database
    let testimonials = match testimonial_repo.get_active().await {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch testimonials: {}", e);
            return response;
        }
    };

    if testimonials.is_empty() {
        tracing::warn!("No testimonials found in database");
        return response;
    }

    tracing::info!("Fetched {} testimonials from database", testimonials.len());

    // Convert testimonials to JSON array
    let testimonials_json: Vec<serde_json::Value> = testimonials
        .iter()
        .map(|t| t.to_json())
        .collect();

    // Find testimonials section and inject the data
    for section in &mut response.sections {
        if section.section_type == "testimonials" {
            // Create new data with testimonials injected
            let mut new_data = section.data.clone();

            // If data has nested "data" object, inject there
            if let Some(nested_data) = new_data.get_mut("data") {
                if let Some(obj) = nested_data.as_object_mut() {
                    obj.insert("testimonials".to_string(), serde_json::json!(testimonials_json));
                }
            } else if let Some(obj) = new_data.as_object_mut() {
                // Otherwise inject at top level
                obj.insert("testimonials".to_string(), serde_json::json!(testimonials_json));
            }

            section.data = new_data;
            tracing::info!("Injected {} testimonials into section", testimonials_json.len());
            break;
        }
    }

    response
}

/// Inject FAQs from faq_items database table into the response
#[cfg(feature = "ssr")]
async fn inject_faqs(
    mut response: HomePageResponse,
    faq_service: &crate::server::services::FaqServiceImpl,
) -> HomePageResponse {
    use crate::types::FaqSearchParams;

    // Fetch all active FAQs from database
    let params = FaqSearchParams {
        query: None,
        category: None,
        locale: Some("en".to_string()),
        featured_only: false,
    };

    match faq_service.get_faqs(params).await {
        Ok(faq_response) => {
            let faqs: Vec<HomeFaqItem> = faq_response.items
                .into_iter()
                .map(|item| HomeFaqItem {
                    id: item.id,
                    category: item.category,
                    question: item.question,
                    answer: item.answer,
                    is_featured: item.is_featured,
                })
                .collect();

            tracing::info!("Fetched {} FAQs from database for home page", faqs.len());
            response.faqs = faqs;
        }
        Err(e) => {
            tracing::error!("Failed to fetch FAQs from database: {}", e);
        }
    }

    response
}

/// Server function to get home page with optional preview token
#[server(GetHomeWithPreview, "/api")]
pub async fn get_home_with_preview(
    locale: String,
    preview_token: Option<String>,
) -> Result<HomePageResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // If preview token provided, validate and fetch draft content
    if let Some(token) = preview_token {
        let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
            .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

        match uno_api::auth::validate_preview_token(&token, preview_secret.as_bytes()) {
            Ok(payload) => {
                // Verify schema matches
                if payload.schema_id != "home" {
                    tracing::warn!("Preview token schema mismatch: expected 'home', got '{}'", payload.schema_id);
                    return Err(ServerFnError::new("Invalid preview token: schema mismatch"));
                }

                // Parse content_id as UUID
                let content_id = uuid::Uuid::parse_str(&payload.content_id)
                    .map_err(|e| ServerFnError::new(format!("Invalid content ID: {}", e)))?;

                tracing::info!("Home preview mode: fetching draft content_id={}", content_id);

                // Fetch the specific content item by ID (includes draft/unpublished)
                let detail = factory.content_item_service
                    .get_by_id(content_id)
                    .await
                    .map_err(|e| ServerFnError::new(e.to_string()))?;

                if let Some(detail) = detail {
                    let response = content_item_to_home_response(&detail.item, &locale, true);
                    // Inject testimonials from database
                    let response = inject_testimonials(response, &factory.testimonial_repository).await;
                    // Inject FAQs from database
                    let response = inject_faqs(response, &factory.faq_service).await;
                    tracing::info!("Home preview: returning {} sections, {} FAQs", response.sections.len(), response.faqs.len());
                    return Ok(response);
                } else {
                    return Err(ServerFnError::new("Content not found for preview"));
                }
            }
            Err(e) => {
                tracing::warn!("Invalid home preview token: {:?}", e);
                // Fall through to normal published content fetch
            }
        }
    }

    // Fetch published home page content
    let items = factory.content_item_service
        .get_published_items_by_schema("home")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(item) = items.first() {
        let response = content_item_to_home_response(item, &locale, false);
        // Inject testimonials from database
        let response = inject_testimonials(response, &factory.testimonial_repository).await;
        // Inject FAQs from database
        let response = inject_faqs(response, &factory.faq_service).await;
        tracing::info!("Home page: returning {} sections, {} FAQs", response.sections.len(), response.faqs.len());
        Ok(response)
    } else {
        // No home page found - return empty response
        tracing::warn!("No home page content found, returning defaults");
        Ok(HomePageResponse {
            title: "Home".to_string(),
            sections: vec![],
            is_preview: false,
            faqs: vec![],
        })
    }
}

/// Register home server functions explicitly
#[cfg(feature = "ssr")]
pub fn register_home_server_fns() {
    use server_fn::ServerFn;
    println!("Registering home server functions:");
    println!("  GetHomeWithPreview: {}", GetHomeWithPreview::url());
}
