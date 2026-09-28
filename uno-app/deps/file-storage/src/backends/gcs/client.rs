//! Google Cloud Storage client implementation

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, error, info, warn};

use crate::config::GcsConfig;
use crate::error::{Result, StorageError};
use crate::traits::FileStorageClient;
use crate::types::{FileRetrievalResult, FileUploadResult, UploadError, UploadOptions};

use super::auth::{GcsAuth, ServiceAccountKey};
use super::signed_url;

/// GCS object metadata from API response
#[derive(Debug, Deserialize)]
struct GcsObject {
    name: String,
    #[serde(rename = "mediaLink")]
    #[allow(dead_code)]
    media_link: Option<String>,
    #[serde(rename = "selfLink")]
    #[allow(dead_code)]
    self_link: Option<String>,
}

/// GCS list response
#[derive(Debug, Deserialize)]
struct GcsListResponse {
    items: Option<Vec<GcsObject>>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

/// Google Cloud Storage client
pub struct GoogleCloudStorageClient {
    client: Client,
    auth: GcsAuth,
    config: GcsConfig,
}

impl GoogleCloudStorageClient {
    const STORAGE_API: &'static str = "https://storage.googleapis.com/storage/v1";
    const UPLOAD_API: &'static str = "https://storage.googleapis.com/upload/storage/v1";

    /// Create a new GCS client from configuration
    pub fn new(config: GcsConfig) -> Result<Self> {
        // Load service account key
        let service_account = if let Some(ref path) = config.service_account_path {
            ServiceAccountKey::from_file(path)?
        } else if let Some(ref json) = config.service_account_json {
            ServiceAccountKey::from_json(json)?
        } else {
            return Err(StorageError::ConfigError(
                "No service account credentials provided".into(),
            ));
        };

        let client = Client::builder()
            .connect_timeout(config.connection_timeout)
            .timeout(config.request_timeout)
            .pool_max_idle_per_host(10)
            .build()
            .map_err(|e| StorageError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        let auth = GcsAuth::new(service_account, client.clone());

        Ok(Self {
            client,
            auth,
            config,
        })
    }

    /// Generate a unique filename with resource prefix
    fn generate_filename(&self, resource_id: &str, original_name: &str, index: usize) -> String {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let extension = std::path::Path::new(original_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");

        format!("{}/{}_{}.{}", resource_id, index, timestamp, extension)
    }

    /// Get a display URL for an object (signed or public)
    fn get_display_url(&self, object_name: &str) -> String {
        match signed_url::generate_signed_url(
            &self.config.bucket,
            object_name,
            self.auth.service_account(),
            self.config.signed_url_expiration,
        ) {
            Ok(url) => url,
            Err(e) => {
                warn!("Failed to generate signed URL, falling back to public: {}", e);
                signed_url::generate_public_url(&self.config.bucket, object_name)
            }
        }
    }

    /// Upload a single file to GCS
    async fn upload_single_file(
        &self,
        object_name: &str,
        data: &[u8],
        mime_type: &str,
        access_token: &str,
    ) -> Result<String> {
        let upload_url = format!(
            "{}/b/{}/o?uploadType=media&name={}",
            Self::UPLOAD_API,
            self.config.bucket,
            urlencoding::encode(object_name)
        );

        let response = self
            .client
            .post(&upload_url)
            .bearer_auth(access_token)
            .header("Content-Type", mime_type)
            .header("Content-Length", data.len().to_string())
            .body(data.to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(StorageError::UploadError(format!(
                "GCS upload failed ({}): {}",
                status, error_text
            )));
        }

        let _obj: GcsObject = response.json().await?;
        let url = signed_url::generate_storage_url(&self.config.bucket, object_name);

        debug!("Uploaded file {} -> {}", object_name, url);
        Ok(url)
    }

    /// List objects in bucket by prefix
    async fn list_objects_by_prefix(
        &self,
        prefix: &str,
        access_token: &str,
    ) -> Result<Vec<GcsObject>> {
        let mut all_objects = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut url = format!(
                "{}/b/{}/o?prefix={}",
                Self::STORAGE_API,
                self.config.bucket,
                urlencoding::encode(prefix)
            );

            if let Some(token) = &page_token {
                url.push_str(&format!("&pageToken={}", token));
            }

            let response = self
                .client
                .get(&url)
                .bearer_auth(access_token)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(StorageError::DownloadError(format!(
                    "GCS list failed ({}): {}",
                    status, error_text
                )));
            }

            let list_response: GcsListResponse = response.json().await?;
            if let Some(items) = list_response.items {
                all_objects.extend(items);
            }

            if list_response.next_page_token.is_none() {
                break;
            }
            page_token = list_response.next_page_token;
        }

        Ok(all_objects)
    }

    /// Delete an object by name
    async fn delete_object(&self, object_name: &str, access_token: &str) -> Result<()> {
        let url = format!(
            "{}/b/{}/o/{}",
            Self::STORAGE_API,
            self.config.bucket,
            urlencoding::encode(object_name)
        );

        let response = self
            .client
            .delete(&url)
            .bearer_auth(access_token)
            .send()
            .await?;

        // 404 is OK - object already deleted
        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(StorageError::DeleteError(format!(
                "GCS delete failed ({}): {}",
                status, error_text
            )));
        }

        Ok(())
    }

    /// Download an object's content
    async fn download_object(&self, object_name: &str, access_token: &str) -> Result<Vec<u8>> {
        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            Self::STORAGE_API,
            self.config.bucket,
            urlencoding::encode(object_name)
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            if status.as_u16() == 404 {
                return Err(StorageError::NotFound(format!(
                    "Object not found: {}",
                    object_name
                )));
            }
            let error_text = response.text().await.unwrap_or_default();
            return Err(StorageError::DownloadError(format!(
                "GCS download failed ({}): {}",
                status, error_text
            )));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Validate file against configuration
    fn validate_file(&self, data: &[u8], mime_type: &str, options: &Option<UploadOptions>) -> Result<()> {
        let max_size = options
            .as_ref()
            .and_then(|o| o.max_size)
            .unwrap_or(self.config.max_file_size);

        if data.len() as u64 > max_size {
            return Err(StorageError::FileTooLarge {
                size: data.len() as u64,
                limit: max_size,
            });
        }

        // Check allowed types
        let allowed_types = if let Some(opts) = options {
            if !opts.allowed_types.is_empty() {
                &opts.allowed_types
            } else {
                &self.config.allowed_mime_types
            }
        } else {
            &self.config.allowed_mime_types
        };

        if !allowed_types.is_empty() {
            let mime_lower = mime_type.to_lowercase();
            let is_allowed = allowed_types.iter().any(|t| {
                let t_lower = t.to_lowercase();
                if t_lower.ends_with("/*") {
                    // Wildcard match (e.g., "image/*")
                    let prefix = &t_lower[..t_lower.len() - 1];
                    mime_lower.starts_with(prefix)
                } else {
                    mime_lower == t_lower
                }
            });

            if !is_allowed {
                return Err(StorageError::InvalidFileType(format!(
                    "MIME type '{}' not allowed",
                    mime_type
                )));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl FileStorageClient for GoogleCloudStorageClient {
    async fn upload_files(
        &self,
        resource_id: &str,
        files: Vec<(Vec<u8>, String, String)>,
        options: Option<UploadOptions>,
    ) -> Result<FileUploadResult> {
        info!("Uploading {} files for resource {}", files.len(), resource_id);

        let access_token = self.auth.get_access_token().await?;

        let mut storage_urls = Vec::new();
        let mut display_urls = Vec::new();
        let mut errors = Vec::new();
        let mut total_bytes = 0u64;

        for (index, (data, filename, mime_type)) in files.into_iter().enumerate() {
            // Validate file
            if let Err(e) = self.validate_file(&data, &mime_type, &options) {
                errors.push(UploadError {
                    filename: filename.clone(),
                    error: e.to_string(),
                });
                continue;
            }

            let object_name = self.generate_filename(resource_id, &filename, index);

            match self
                .upload_single_file(&object_name, &data, &mime_type, &access_token)
                .await
            {
                Ok(storage_url) => {
                    info!("Successfully uploaded: {}", object_name);
                    total_bytes += data.len() as u64;
                    storage_urls.push(storage_url);
                    display_urls.push(self.get_display_url(&object_name));
                }
                Err(e) => {
                    let error_msg = format!("Failed to upload {}: {}", filename, e);
                    error!("{}", error_msg);
                    errors.push(UploadError {
                        filename,
                        error: e.to_string(),
                    });
                }
            }
        }

        let uploaded_count = storage_urls.len();
        info!(
            "Upload complete: {} successful, {} failed",
            uploaded_count,
            errors.len()
        );

        Ok(FileUploadResult {
            storage_urls,
            display_urls,
            uploaded_count,
            total_bytes,
            errors,
            thumbnail_urls: None,
        })
    }

    async fn upload_file(
        &self,
        resource_id: &str,
        bytes: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> Result<String> {
        let result = self
            .upload_files(
                resource_id,
                vec![(bytes, filename.to_string(), mime_type.to_string())],
                None,
            )
            .await?;

        if let Some(url) = result.storage_urls.first() {
            Ok(url.clone())
        } else if let Some(err) = result.errors.first() {
            Err(StorageError::UploadError(err.error.clone()))
        } else {
            Err(StorageError::UploadError("Unknown upload error".into()))
        }
    }

    async fn get_files(&self, resource_id: &str) -> Result<FileRetrievalResult> {
        info!("Retrieving files for resource {}", resource_id);

        let access_token = self.auth.get_access_token().await?;

        let prefix = format!("{}/", resource_id);
        let objects = self.list_objects_by_prefix(&prefix, &access_token).await?;

        let storage_urls: Vec<String> = objects
            .iter()
            .map(|obj| signed_url::generate_storage_url(&self.config.bucket, &obj.name))
            .collect();

        let display_urls: Vec<String> = objects
            .iter()
            .map(|obj| self.get_display_url(&obj.name))
            .collect();

        let count = display_urls.len();
        info!("Found {} files for resource {}", count, resource_id);

        Ok(FileRetrievalResult {
            display_urls,
            storage_urls,
            count,
        })
    }

    async fn get_file(&self, storage_url: &str) -> Result<Vec<u8>> {
        let (bucket, object_name) = self.parse_storage_url(storage_url).ok_or_else(|| {
            StorageError::InvalidUrl(format!("Invalid storage URL: {}", storage_url))
        })?;

        if bucket != self.config.bucket {
            return Err(StorageError::InvalidUrl(format!(
                "Bucket mismatch: expected {}, got {}",
                self.config.bucket, bucket
            )));
        }

        let access_token = self.auth.get_access_token().await?;
        self.download_object(&object_name, &access_token).await
    }

    async fn delete_files(&self, resource_id: &str) -> Result<()> {
        info!("Deleting files for resource {}", resource_id);

        let access_token = self.auth.get_access_token().await?;

        let prefix = format!("{}/", resource_id);
        let objects = self.list_objects_by_prefix(&prefix, &access_token).await?;

        let mut deleted = 0;
        let mut failed = 0;

        for obj in objects {
            match self.delete_object(&obj.name, &access_token).await {
                Ok(_) => {
                    debug!("Deleted object: {}", obj.name);
                    deleted += 1;
                }
                Err(e) => {
                    warn!("Failed to delete {}: {}", obj.name, e);
                    failed += 1;
                }
            }
        }

        info!(
            "Deleted {} files, {} failed for resource {}",
            deleted, failed, resource_id
        );

        if failed > 0 && deleted == 0 {
            return Err(StorageError::DeleteError(format!(
                "Failed to delete any files for resource {}",
                resource_id
            )));
        }

        Ok(())
    }

    async fn delete_file(&self, storage_url: &str) -> Result<()> {
        let (bucket, object_name) = self.parse_storage_url(storage_url).ok_or_else(|| {
            StorageError::InvalidUrl(format!("Invalid storage URL: {}", storage_url))
        })?;

        if bucket != self.config.bucket {
            return Err(StorageError::InvalidUrl(format!(
                "Bucket mismatch: expected {}, got {}",
                self.config.bucket, bucket
            )));
        }

        let access_token = self.auth.get_access_token().await?;
        self.delete_object(&object_name, &access_token).await
    }

    fn storage_to_display_urls(&self, storage_urls: &[String]) -> Vec<String> {
        storage_urls
            .iter()
            .map(|url| self.storage_to_display_url(url))
            .collect()
    }

    fn storage_to_display_url(&self, storage_url: &str) -> String {
        if let Some((_, object_name)) = self.parse_storage_url(storage_url) {
            self.get_display_url(&object_name)
        } else {
            // Return as-is if not a recognized storage URL
            storage_url.to_string()
        }
    }

    fn parse_storage_url(&self, storage_url: &str) -> Option<(String, String)> {
        signed_url::parse_storage_url(storage_url)
    }

    async fn is_configured(&self) -> bool {
        true
    }

    fn backend_name(&self) -> &'static str {
        "google_cloud_storage"
    }
}
