//! Client credential registry.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::error::AuthError;

/// Client credentials.
#[derive(Debug, Clone)]
pub struct ClientCredentials {
    /// Client identifier.
    pub client_id: String,
    /// Secret key for HMAC signing.
    pub secret_key: Vec<u8>,
    /// Roles/permissions for this client.
    pub roles: Vec<String>,
    /// Whether this client is active.
    pub active: bool,
}

impl ClientCredentials {
    /// Create new client credentials.
    pub fn new(client_id: impl Into<String>, secret_key: &[u8]) -> Self {
        Self {
            client_id: client_id.into(),
            secret_key: secret_key.to_vec(),
            roles: Vec::new(),
            active: true,
        }
    }

    /// Add roles to the credentials.
    pub fn with_roles(mut self, roles: &[&str]) -> Self {
        self.roles = roles.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Check if the client has a specific role.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

/// Thread-safe registry for client credentials.
///
/// In production, this would typically be backed by a database.
/// This implementation provides an in-memory registry for development
/// and testing, or for cases where credentials are loaded from config.
pub struct ClientRegistry {
    clients: RwLock<HashMap<String, ClientCredentials>>,
}

impl ClientRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new client.
    pub fn register(&self, client_id: &str, secret_key: &[u8], roles: &[&str]) {
        let credentials = ClientCredentials::new(client_id, secret_key).with_roles(roles);
        self.clients
            .write()
            .unwrap()
            .insert(client_id.to_string(), credentials);
    }

    /// Register credentials directly.
    pub fn register_credentials(&self, credentials: ClientCredentials) {
        self.clients
            .write()
            .unwrap()
            .insert(credentials.client_id.clone(), credentials);
    }

    /// Get client credentials by ID.
    pub fn get(&self, client_id: &str) -> Result<ClientCredentials, AuthError> {
        self.clients
            .read()
            .unwrap()
            .get(client_id)
            .filter(|c| c.active)
            .cloned()
            .ok_or_else(|| AuthError::UnknownClient(client_id.to_string()))
    }

    /// Check if a client exists and is active.
    pub fn exists(&self, client_id: &str) -> bool {
        self.clients
            .read()
            .unwrap()
            .get(client_id)
            .is_some_and(|c| c.active)
    }

    /// Deactivate a client.
    pub fn deactivate(&self, client_id: &str) -> bool {
        if let Some(creds) = self.clients.write().unwrap().get_mut(client_id) {
            creds.active = false;
            true
        } else {
            false
        }
    }

    /// Reactivate a client.
    pub fn activate(&self, client_id: &str) -> bool {
        if let Some(creds) = self.clients.write().unwrap().get_mut(client_id) {
            creds.active = true;
            true
        } else {
            false
        }
    }

    /// Remove a client from the registry.
    pub fn remove(&self, client_id: &str) -> bool {
        self.clients.write().unwrap().remove(client_id).is_some()
    }

    /// List all client IDs.
    pub fn list_clients(&self) -> Vec<String> {
        self.clients.read().unwrap().keys().cloned().collect()
    }

    /// Get the number of registered clients.
    pub fn len(&self) -> usize {
        self.clients.read().unwrap().len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.clients.read().unwrap().is_empty()
    }
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get() {
        let registry = ClientRegistry::new();
        registry.register("client1", b"secret1", &["admin"]);

        let creds = registry.get("client1").unwrap();
        assert_eq!(creds.client_id, "client1");
        assert!(creds.has_role("admin"));
    }

    #[test]
    fn test_unknown_client() {
        let registry = ClientRegistry::new();
        let result = registry.get("unknown");
        assert!(matches!(result, Err(AuthError::UnknownClient(_))));
    }

    #[test]
    fn test_deactivate() {
        let registry = ClientRegistry::new();
        registry.register("client1", b"secret1", &[]);

        assert!(registry.exists("client1"));
        registry.deactivate("client1");
        assert!(!registry.exists("client1"));

        registry.activate("client1");
        assert!(registry.exists("client1"));
    }
}
