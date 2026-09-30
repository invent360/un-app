//! Schema API client for CMS operations with uno-app.
//!
//! This module provides HMAC-authenticated client for managing content schemas
//! and schema-driven content items.

pub use super::schema_types::*;

// ========================================
// SSR-only types and implementation
// ========================================

#[cfg(feature = "ssr")]
pub use ssr_impl::*;

#[cfg(feature = "ssr")]
mod ssr_impl {
    use super::*;
    use chrono::Utc;
    use hmac::{Hmac, Mac};
    use serde::{Deserialize, Serialize};
    use sha2::Sha256;
    use thiserror::Error;
    use uuid::Uuid;

    type HmacSha256 = Hmac<Sha256>;

    /// Schema client error types
    #[derive(Debug, Error)]
    pub enum SchemaClientError {
        #[error("HTTP error: {0}")]
        Http(#[from] reqwest::Error),
        #[error("JSON error: {0}")]
        Json(#[from] serde_json::Error),
        #[error("API error: {message}")]
        Api { message: String, code: String },
        #[error("Configuration error: {0}")]
        Config(String),
    }

    /// Signed request wrapper for HMAC authentication
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SignedRequest<T> {
        pub client_id: String,
        pub timestamp: i64,
        pub nonce: String,
        pub signature: String,
        pub payload: T,
    }

    /// Schema client for managing content schemas and items
    pub struct SchemaClient {
        http_client: reqwest::Client,
        base_url: String,
        client_id: String,
        secret_key: Vec<u8>,
    }

    impl SchemaClient {
        /// Create a new client from environment variables
        pub fn from_env() -> Result<Self, SchemaClientError> {
            let base_url = std::env::var("UNO_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());

            // Support both ADMIN_* and UNO_* env var names
            let client_id = std::env::var("ADMIN_CLIENT_ID")
                .or_else(|_| std::env::var("UNO_CLIENT_ID"))
                .map_err(|_| {
                    SchemaClientError::Config(
                        "ADMIN_CLIENT_ID or UNO_CLIENT_ID not set".to_string(),
                    )
                })?;

            let secret_key = std::env::var("ADMIN_SECRET_KEY")
                .or_else(|_| std::env::var("UNO_SECRET_KEY"))
                .map_err(|_| {
                    SchemaClientError::Config(
                        "ADMIN_SECRET_KEY or UNO_SECRET_KEY not set".to_string(),
                    )
                })?;

            Ok(Self {
                http_client: reqwest::Client::new(),
                base_url,
                client_id,
                secret_key: secret_key.into_bytes(),
            })
        }

        /// Sign a request with HMAC-SHA256
        fn sign<T: Serialize + Clone>(
            &self,
            payload: &T,
        ) -> Result<SignedRequest<T>, SchemaClientError> {
            let timestamp = Utc::now().timestamp();
            let nonce = Uuid::new_v4().to_string();
            let payload_json = serde_json::to_string(payload)?;

            let message = format!(
                "{}:{}:{}:{}",
                self.client_id, timestamp, nonce, payload_json
            );

            let mut mac = HmacSha256::new_from_slice(&self.secret_key)
                .map_err(|e| SchemaClientError::Config(format!("Invalid key: {}", e)))?;

            mac.update(message.as_bytes());
            let signature = hex::encode(mac.finalize().into_bytes());

            Ok(SignedRequest {
                client_id: self.client_id.clone(),
                timestamp,
                nonce,
                signature,
                payload: payload.clone(),
            })
        }

        /// Parse API error response
        fn parse_error(&self, body: &str) -> SchemaClientError {
            if let Ok(err) = serde_json::from_str::<serde_json::Value>(body) {
                let message = err
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                let code = err
                    .get("code")
                    .and_then(|v| v.as_str())
                    .unwrap_or("UNKNOWN")
                    .to_string();
                SchemaClientError::Api { message, code }
            } else {
                SchemaClientError::Api {
                    message: body.to_string(),
                    code: "UNKNOWN".to_string(),
                }
            }
        }

        // ==========================================
        // Schema Operations (Public)
        // ==========================================

        /// Get all schemas
        pub async fn get_schemas(&self) -> Result<Vec<ContentSchema>, SchemaClientError> {
            let response = self
                .http_client
                .get(format!("{}/api/v1/schemas", self.base_url))
                .send()
                .await?;

            if response.status().is_success() {
                let result: SchemaListResponse = response.json().await?;
                Ok(result.schemas)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Get schema by ID
        pub async fn get_schema(&self, id: &str) -> Result<ContentSchema, SchemaClientError> {
            let response = self
                .http_client
                .get(format!("{}/api/v1/schemas/{}", self.base_url, id))
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        // ==========================================
        // Schema Operations (Admin - requires signing)
        // ==========================================

        /// Create a new schema
        pub async fn create_schema(
            &self,
            request: UpsertSchemaRequest,
        ) -> Result<SchemaOperationResponse, SchemaClientError> {
            let signed = self.sign(&request)?;

            let response = self
                .http_client
                .post(format!("{}/api/v1/admin/schemas", self.base_url))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Update an existing schema
        pub async fn update_schema(
            &self,
            id: &str,
            request: UpsertSchemaRequest,
        ) -> Result<SchemaOperationResponse, SchemaClientError> {
            let signed = self.sign(&request)?;

            let response = self
                .http_client
                .put(format!("{}/api/v1/admin/schemas/{}", self.base_url, id))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Delete a schema
        pub async fn delete_schema(&self, id: &str) -> Result<(), SchemaClientError> {
            let signed = self.sign(&serde_json::json!({}))?;

            let response = self
                .http_client
                .post(format!(
                    "{}/api/v1/admin/schemas/{}/delete",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(())
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        // ==========================================
        // Content Item Operations (Admin - requires signing)
        // ==========================================

        /// List content items with filters
        pub async fn list_items(
            &self,
            params: ContentItemListParams,
        ) -> Result<ContentItemListResponse, SchemaClientError> {
            let signed = self.sign(&params)?;

            let response = self
                .http_client
                .post(format!("{}/api/v1/admin/items/list", self.base_url))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Get content item by ID
        pub async fn get_item(
            &self,
            id: &str,
        ) -> Result<ContentItemDetailResponse, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({}))?;

            let response = self
                .http_client
                .post(format!("{}/api/v1/admin/items/{}/get", self.base_url, id))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Create a new content item
        pub async fn create_item(
            &self,
            request: UpsertContentItemRequest,
        ) -> Result<ContentItemOperationResponse, SchemaClientError> {
            let signed = self.sign(&request)?;

            let response = self
                .http_client
                .post(format!("{}/api/v1/admin/items", self.base_url))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Update an existing content item
        pub async fn update_item(
            &self,
            id: &str,
            request: UpsertContentItemRequest,
        ) -> Result<ContentItemOperationResponse, SchemaClientError> {
            let signed = self.sign(&request)?;

            let response = self
                .http_client
                .put(format!("{}/api/v1/admin/items/{}", self.base_url, id))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Delete a content item
        pub async fn delete_item(&self, id: &str) -> Result<(), SchemaClientError> {
            let signed = self.sign(&serde_json::json!({}))?;

            let response = self
                .http_client
                .post(format!(
                    "{}/api/v1/admin/items/{}/delete",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(())
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Publish a content item
        pub async fn publish_item(&self, id: &str) -> Result<ContentItem, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({}))?;

            let response = self
                .http_client
                .post(format!(
                    "{}/api/v1/admin/items/{}/publish",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Archive a content item
        pub async fn archive_item(&self, id: &str) -> Result<ContentItem, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({}))?;

            let response = self
                .http_client
                .post(format!(
                    "{}/api/v1/admin/items/{}/archive",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Get version history for a content item
        pub async fn get_item_versions(
            &self,
            id: &str,
        ) -> Result<Vec<ContentItemVersion>, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({}))?;

            let response = self
                .http_client
                .post(format!(
                    "{}/api/v1/admin/items/{}/versions",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                #[derive(Deserialize)]
                struct VersionsResponse {
                    versions: Vec<ContentItemVersion>,
                }
                let result: VersionsResponse = response.json().await?;
                Ok(result.versions)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Revert to a specific version
        pub async fn revert_item(
            &self,
            id: &str,
            version: i32,
        ) -> Result<ContentItem, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({ "version": version }))?;

            let response = self
                .http_client
                .post(format!(
                    "{}/api/v1/admin/items/{}/revert",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Update content item schedule
        pub async fn update_item_schedule(
            &self,
            id: &str,
            publish_at: Option<String>,
            unpublish_at: Option<String>,
        ) -> Result<ContentItem, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({
                "publish_at": publish_at,
                "unpublish_at": unpublish_at
            }))?;

            let response = self
                .http_client
                .put(format!(
                    "{}/api/v1/admin/items/{}/schedule",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }

        /// Update translation for a specific locale
        pub async fn update_item_translation(
            &self,
            id: &str,
            locale: &str,
            data: serde_json::Value,
            status: Option<&str>,
        ) -> Result<ContentItem, SchemaClientError> {
            let signed = self.sign(&serde_json::json!({
                "locale": locale,
                "data": data,
                "status": status
            }))?;

            let response = self
                .http_client
                .put(format!(
                    "{}/api/v1/admin/items/{}/translations",
                    self.base_url, id
                ))
                .json(&signed)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else {
                let body = response.text().await?;
                Err(self.parse_error(&body))
            }
        }
    }
}

// ========================================
// Leptos Server Functions
// ========================================

use leptos::prelude::*;
use server_fn::codec::PostUrl;

/// Fetch all schemas
#[server(GetSchemas, "/api", endpoint = "get_schemas")]
pub async fn get_schemas() -> Result<Vec<ContentSchema>, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .get_schemas()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Fetch a single schema by ID
#[server(GetSchema, "/api", endpoint = "get_schema")]
pub async fn get_schema(id: String) -> Result<ContentSchema, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .get_schema(&id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Create a new schema
#[server(CreateSchema, "/api", endpoint = "create_schema", input = PostUrl)]
pub async fn create_schema(
    id: String,
    name: String,
    name_plural: String,
    description: Option<String>,
    fields: String,   // JSON string of Vec<FieldDefinition>
    settings: String, // JSON string of SchemaSettings
) -> Result<SchemaOperationResponse, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    let fields_value: Vec<FieldDefinition> = serde_json::from_str(&fields)
        .map_err(|e| ServerFnError::new(format!("Invalid fields JSON: {}", e)))?;

    let settings_value: SchemaSettings = serde_json::from_str(&settings)
        .map_err(|e| ServerFnError::new(format!("Invalid settings JSON: {}", e)))?;

    let now = chrono::Utc::now();
    let request = UpsertSchemaRequest {
        id,
        name,
        name_plural,
        description,
        icon: None,
        fields: fields_value,
        settings: settings_value,
        version: 1, // New schemas start at version 1
        is_system: false,
        created_at: now,
        updated_at: now,
    };

    client
        .create_schema(request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Update an existing schema
#[server(UpdateSchema, "/api", endpoint = "update_schema", input = PostUrl)]
pub async fn update_schema(
    id: String,
    name: String,
    name_plural: String,
    description: Option<String>,
    fields: String,   // JSON string of Vec<FieldDefinition>
    settings: String, // JSON string of SchemaSettings
    version: i32,
    is_system: bool,
    created_at: String, // ISO 8601 string, will be parsed
) -> Result<SchemaOperationResponse, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    let fields_value: Vec<FieldDefinition> = serde_json::from_str(&fields)
        .map_err(|e| ServerFnError::new(format!("Invalid fields JSON: {}", e)))?;

    let settings_value: SchemaSettings = serde_json::from_str(&settings)
        .map_err(|e| ServerFnError::new(format!("Invalid settings JSON: {}", e)))?;

    let created_at_dt: chrono::DateTime<chrono::Utc> = created_at
        .parse()
        .map_err(|e| ServerFnError::new(format!("Invalid created_at: {}", e)))?;

    let request = UpsertSchemaRequest {
        id: id.clone(),
        name,
        name_plural,
        description,
        icon: None,
        fields: fields_value,
        settings: settings_value,
        version,
        is_system,
        created_at: created_at_dt,
        updated_at: chrono::Utc::now(),
    };

    client
        .update_schema(&id, request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Delete a schema
#[server(DeleteSchema, "/api", endpoint = "delete_schema")]
pub async fn delete_schema(id: String) -> Result<(), ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .delete_schema(&id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// List content items with filters
#[server(ListContentItems, "/api", endpoint = "list_content_items")]
pub async fn list_content_items(
    schema_id: Option<String>,
    status: Option<String>,
    search: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<ContentItemListResponse, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    let status_enum = status.and_then(|s| match s.as_str() {
        "draft" => Some(ContentItemStatus::Draft),
        "pending_review" => Some(ContentItemStatus::PendingReview),
        "approved" => Some(ContentItemStatus::Approved),
        "published" => Some(ContentItemStatus::Published),
        "scheduled" => Some(ContentItemStatus::Scheduled),
        "archived" => Some(ContentItemStatus::Archived),
        _ => None,
    });

    let params = ContentItemListParams {
        schema_id,
        status: status_enum,
        search,
        is_featured: None,
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(20),
    };

    client
        .list_items(params)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get a content item by ID with its schema
#[server(GetContentItem, "/api", endpoint = "get_content_item")]
pub async fn get_content_item(id: String) -> Result<ContentItemDetailResponse, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .get_item(&id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Save (create or update) a content item
///
/// This function automatically converts any base64 data URLs in the content
/// to GCS storage URLs before saving.
#[server(SaveContentItem, "/api", endpoint = "save_content_item", input = PostUrl)]
pub async fn save_content_item(
    id: Option<String>,
    schema_id: String,
    slug: Option<String>,
    data: String,                 // JSON string
    translations: Option<String>, // JSON string
    is_featured: Option<bool>,
    display_order: Option<i32>,
    change_summary: Option<String>,
    task_status: Option<String>, // Task status for task-type content
) -> Result<String, ServerFnError> {
    use crate::utils::base64_converter::{contains_base64_data, convert_base64_to_gcs};

    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut data_value: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| ServerFnError::new(format!("Invalid data JSON: {}", e)))?;

    // Convert any base64 data URLs to GCS storage URLs
    if contains_base64_data(&data_value) {
        tracing::info!("Content contains base64 data, converting to GCS...");

        let storage_client = file_storage::create_client_from_env()
            .map_err(|e| ServerFnError::new(format!("Failed to create storage client: {}", e)))?;

        let converted_count =
            convert_base64_to_gcs(&mut data_value, &schema_id, storage_client.as_ref())
                .await
                .map_err(|e| {
                    ServerFnError::new(format!("Failed to convert base64 to GCS: {}", e))
                })?;

        tracing::info!("Converted {} base64 images to GCS URLs", converted_count);
    }

    let translations_value = translations
        .map(|t| serde_json::from_str(&t))
        .transpose()
        .map_err(|e| ServerFnError::new(format!("Invalid translations JSON: {}", e)))?;

    let request = UpsertContentItemRequest {
        schema_id,
        slug,
        data: data_value,
        translations: translations_value,
        is_featured,
        display_order,
        change_summary,
        task_status,
    };

    let item = if let Some(item_id) = id {
        client.update_item(&item_id, request).await
    } else {
        client.create_item(request).await
    };

    item.map(|i| i.id)
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Delete a content item
#[server(DeleteContentItem, "/api", endpoint = "delete_content_item")]
pub async fn delete_content_item(id: String) -> Result<(), ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .delete_item(&id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Publish a content item
#[server(PublishContentItem, "/api", endpoint = "publish_content_item")]
pub async fn publish_content_item(id: String) -> Result<(), ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .publish_item(&id)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Archive a content item
#[server(ArchiveContentItem, "/api", endpoint = "archive_content_item")]
pub async fn archive_content_item(id: String) -> Result<(), ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .archive_item(&id)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get version history
#[server(GetContentItemVersions, "/api", endpoint = "get_content_item_versions")]
pub async fn get_content_item_versions(
    id: String,
) -> Result<Vec<ContentItemVersion>, ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .get_item_versions(&id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Revert to a specific version
#[server(RevertContentItem, "/api", endpoint = "revert_content_item")]
pub async fn revert_content_item(id: String, version: i32) -> Result<(), ServerFnError> {
    let client = SchemaClient::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    client
        .revert_item(&id, version)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Force schema client server functions to be registered
/// Call this from main.rs to ensure inventory picks up the server functions
#[cfg(feature = "ssr")]
pub fn register_schema_server_fns() {
    use server_fn::ServerFn;

    // Reference the URL to ensure the server function is linked
    println!("Registering schema server functions:");
    println!("  GetSchemas: {}", GetSchemas::url());
    println!("  GetSchema: {}", GetSchema::url());
    println!("  CreateSchema: {}", CreateSchema::url());
    println!("  UpdateSchema: {}", UpdateSchema::url());
    println!("  DeleteSchema: {}", DeleteSchema::url());
    println!("  ListContentItems: {}", ListContentItems::url());
    println!("  GetContentItem: {}", GetContentItem::url());
    println!("  SaveContentItem: {}", SaveContentItem::url());
    println!("  DeleteContentItem: {}", DeleteContentItem::url());
    println!("  PublishContentItem: {}", PublishContentItem::url());
    println!("  ArchiveContentItem: {}", ArchiveContentItem::url());
    println!(
        "  GetContentItemVersions: {}",
        GetContentItemVersions::url()
    );
    println!("  RevertContentItem: {}", RevertContentItem::url());
}
