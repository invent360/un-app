//! Ember Multichain - Stub Package
//!
//! This is a minimal stub providing the types and constants needed for compilation.
//! The full ember-multichain crate should be vendored or referenced via git.
//!
//! To set up the real crate:
//! 1. Clone the ember-multichain repository
//! 2. Copy or symlink to deps/ember-multichain
//! 3. Or update Cargo.toml to use a git dependency

pub mod siwe;

use async_trait::async_trait;
use thiserror::Error;

/// Chain configuration
#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub chain_type: ChainType,
    pub chain_id: ChainId,
    pub rpc_url: String,
}

impl ChainConfig {
    pub fn new(chain_type: ChainType, chain_id: ChainId, rpc_url: String) -> Self {
        Self {
            chain_type,
            chain_id,
            rpc_url,
        }
    }
}

/// Chain types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainType {
    Ethereum,
    WorldMobile,
}

/// Chain ID variants
#[derive(Debug, Clone, Copy)]
pub enum ChainId {
    Ethereum(u64),
    WorldMobile(u64),
}

/// EVM client for wallet operations
pub struct EvmClient {
    config: ChainConfig,
    credentials: Option<WalletCredentials>,
}

/// Wallet credentials
#[derive(Debug, Clone)]
pub struct WalletCredentials {
    pub address: String,
    mnemonic: Option<String>,
}

impl WalletCredentials {
    pub fn mnemonic(&self) -> Option<&str> {
        self.mnemonic.as_deref()
    }
}

/// Wallet errors
#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Wallet not loaded")]
    NotLoaded,
    #[error("Import failed: {0}")]
    ImportFailed(String),
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    #[error("Network error: {0}")]
    NetworkError(String),
}

impl EvmClient {
    pub fn new(config: ChainConfig) -> Self {
        Self {
            config,
            credentials: None,
        }
    }

    pub async fn import_from_private_key(&self, _private_key: &str) -> Result<WalletCredentials, WalletError> {
        // Stub: In real implementation, this would derive address from private key
        Ok(WalletCredentials {
            address: "0x0000000000000000000000000000000000000000".to_string(),
            mnemonic: None,
        })
    }

    pub async fn import_from_mnemonic(&self, _mnemonic: &str, _derivation_path: Option<&str>) -> Result<WalletCredentials, WalletError> {
        // Stub: In real implementation, this would derive address from mnemonic
        Ok(WalletCredentials {
            address: "0x0000000000000000000000000000000000000000".to_string(),
            mnemonic: Some("stub mnemonic phrase".to_string()),
        })
    }

    pub async fn create_wallet(&self) -> Result<WalletCredentials, WalletError> {
        // Stub: In real implementation, this would generate a new wallet
        Ok(WalletCredentials {
            address: "0x0000000000000000000000000000000000000000".to_string(),
            mnemonic: Some("stub mnemonic phrase for new wallet".to_string()),
        })
    }

    pub async fn get_address(&self) -> Result<String, WalletError> {
        self.credentials
            .as_ref()
            .map(|c| c.address.clone())
            .ok_or(WalletError::NotLoaded)
    }

    pub async fn sign_message(&self, _message: &str) -> Result<String, WalletError> {
        // Stub: In real implementation, this would sign with the private key
        Ok("0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string())
    }
}

/// Wallet trait for common operations
#[async_trait]
pub trait WalletTrait {
    async fn get_address(&self) -> Result<String, WalletError>;
    async fn sign_message(&self, message: &str) -> Result<String, WalletError>;
}

#[async_trait]
impl WalletTrait for EvmClient {
    async fn get_address(&self) -> Result<String, WalletError> {
        self.get_address().await
    }

    async fn sign_message(&self, message: &str) -> Result<String, WalletError> {
        self.sign_message(message).await
    }
}
