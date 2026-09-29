//! SIWE (Sign-In With Ethereum) module
//!
//! Provides EIP-4361 Sign-In With Ethereum message construction and verification.

pub mod unetwork;

use serde::{Deserialize, Serialize};

/// SIWE Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiweMessage {
    pub domain: String,
    pub address: String,
    pub statement: Option<String>,
    pub uri: String,
    pub version: String,
    pub chain_id: u64,
    pub nonce: String,
    pub issued_at: String,
    pub expiration_time: Option<String>,
}

impl SiweMessage {
    /// Convert the SIWE message to the EIP-4361 message format
    pub fn to_message(&self) -> String {
        let mut msg = format!(
            "{} wants you to sign in with your Ethereum account:\n{}\n",
            self.domain, self.address
        );

        if let Some(ref statement) = self.statement {
            msg.push_str(&format!("\n{}\n", statement));
        }

        msg.push_str(&format!(
            "\nURI: {}\nVersion: {}\nChain ID: {}\nNonce: {}\nIssued At: {}",
            self.uri, self.version, self.chain_id, self.nonce, self.issued_at
        ));

        if let Some(ref exp) = self.expiration_time {
            msg.push_str(&format!("\nExpiration Time: {}", exp));
        }

        msg
    }
}

/// Build a SIWE message for Unetwork authentication
pub fn build_unetwork_siwe_message(address: &str, nonce: Option<&str>) -> SiweMessage {
    use chrono::Utc;

    let now = Utc::now();
    let nonce = nonce.map(|s| s.to_string()).unwrap_or_else(|| {
        // Generate a random nonce if not provided
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{:x}", timestamp)
    });

    SiweMessage {
        domain: unetwork::DOMAIN.to_string(),
        address: address.to_string(),
        statement: Some(unetwork::STATEMENT.to_string()),
        uri: unetwork::URI.to_string(),
        version: "1".to_string(),
        chain_id: unetwork::CHAIN_ID,
        nonce,
        issued_at: now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        expiration_time: None,
    }
}
