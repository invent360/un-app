//! HTTP test client with session and authentication management

use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;

/// HTTP client for test requests with session management
pub struct TestClient {
    client: Client,
    base_url: String,
    session_token: Option<String>,
    admin_api_key: Option<String>,
}

impl TestClient {
    /// Create a new test client for the given base URL
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .cookie_store(true)
                .build()
                .expect("Failed to create HTTP client"),
            base_url: base_url.to_string(),
            session_token: None,
            admin_api_key: None,
        }
    }

    /// Create a client with admin authentication
    pub fn with_admin_key(base_url: &str, api_key: &str) -> Self {
        Self {
            client: Client::builder()
                .cookie_store(true)
                .build()
                .expect("Failed to create HTTP client"),
            base_url: base_url.to_string(),
            session_token: None,
            admin_api_key: Some(api_key.to_string()),
        }
    }

    /// Set the session token for authenticated requests
    pub fn set_session(&mut self, token: &str) {
        self.session_token = Some(token.to_string());
    }

    /// Clear the session token
    pub fn clear_session(&mut self) {
        self.session_token = None;
    }

    /// Set admin API key
    pub fn set_admin_key(&mut self, key: &str) {
        self.admin_api_key = Some(key.to_string());
    }

    /// Build full URL from path
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Make a GET request
    pub async fn get(&self, path: &str) -> Response {
        let mut request = self.client.get(self.url(path));

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("GET request failed")
    }

    /// Make a GET request with query parameters
    pub async fn get_with_query<T: Serialize + ?Sized>(&self, path: &str, query: &T) -> Response {
        let mut request = self.client.get(self.url(path)).query(query);

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("GET request failed")
    }

    /// Make a POST request with JSON body
    pub async fn post<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Response {
        let mut request = self.client.post(self.url(path)).json(body);

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("POST request failed")
    }

    /// Make a POST request without body
    pub async fn post_empty(&self, path: &str) -> Response {
        let mut request = self.client.post(self.url(path));

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("POST request failed")
    }

    /// Make a PUT request with JSON body
    pub async fn put<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Response {
        let mut request = self.client.put(self.url(path)).json(body);

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("PUT request failed")
    }

    /// Make a PATCH request with JSON body
    pub async fn patch<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Response {
        let mut request = self.client.patch(self.url(path)).json(body);

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("PATCH request failed")
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Response {
        let mut request = self.client.delete(self.url(path));

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("DELETE request failed")
    }

    /// Make a DELETE request with JSON body
    pub async fn delete_with_body<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Response {
        let mut request = self.client.delete(self.url(path)).json(body);

        if let Some(ref token) = self.session_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(ref api_key) = self.admin_api_key {
            request = request.header("X-API-Key", api_key);
        }

        request.send().await.expect("DELETE request failed")
    }

    /// Create multiple concurrent clients for stress testing
    pub fn create_concurrent_clients(base_url: &str, count: usize) -> Vec<Self> {
        (0..count)
            .map(|_| Self::new(base_url))
            .collect()
    }
}

/// Request body builders for common test scenarios
pub mod bodies {
    use serde_json::{json, Value};

    /// Build a license claim request body
    pub fn claim_license(lease_code: &str, device_id: Option<&str>) -> Value {
        json!({
            "lease_code": lease_code,
            "device_id": device_id
        })
    }

    /// Build a license reservation request body
    pub fn reserve_license(user_id: &str, country_code: Option<&str>) -> Value {
        let mut body = json!({
            "user_id": user_id
        });
        if let Some(cc) = country_code {
            body["country_code"] = json!(cc);
        }
        body
    }

    /// Build an admin import request body
    pub fn import_licenses(codes: &[&str]) -> Value {
        let licenses: Vec<Value> = codes
            .iter()
            .map(|code| json!({ "lease_code": code }))
            .collect();
        json!({ "licenses": licenses })
    }

    /// Build a consent grant request body
    pub fn grant_consent(consent_type: &str, granted: bool) -> Value {
        json!({
            "consent_type": consent_type,
            "granted": granted
        })
    }

    /// Build a session create request body
    pub fn create_session(user_id: &str, device_info: Option<&str>) -> Value {
        let mut body = json!({
            "user_id": user_id
        });
        if let Some(info) = device_info {
            body["device_info"] = json!(info);
        }
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_builders() {
        let claim = bodies::claim_license("CODE-123", Some("device-1"));
        assert_eq!(claim["lease_code"], "CODE-123");
        assert_eq!(claim["device_id"], "device-1");

        let reserve = bodies::reserve_license("user-1", Some("US"));
        assert_eq!(reserve["user_id"], "user-1");
        assert_eq!(reserve["country_code"], "US");

        let import = bodies::import_licenses(&["CODE-1", "CODE-2"]);
        assert!(import["licenses"].is_array());
    }

    #[test]
    fn test_concurrent_clients_creation() {
        let clients = TestClient::create_concurrent_clients("http://localhost:8080", 5);
        assert_eq!(clients.len(), 5);
    }
}
