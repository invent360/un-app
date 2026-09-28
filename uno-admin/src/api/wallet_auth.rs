//! Wallet Authentication Service for Unetwork
//!
//! This module provides Web3/SIWE authentication using ember-multichain for wallet operations.
//! It allows authentication with locally stored wallets without requiring browser extensions.

#[cfg(feature = "ssr")]
use ember_multichain::{
    siwe::{SiweMessage, build_unetwork_siwe_message, unetwork},
    ChainConfig, ChainType, ChainId, EvmClient, WalletTrait,
};

use serde::{Deserialize, Serialize};

// ============================================================================
// API TYPES
// ============================================================================

/// Request payload for Web3 authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web3AuthRequest {
    pub chain: String,
    pub message: String,
    pub signature: String,
}

/// Response from Web3 authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web3AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub expires_at: u64,
    pub refresh_token: String,
    pub user: UnetworkUser,
}

/// Unetwork user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnetworkUser {
    pub id: String,
    #[serde(default)]
    pub aud: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub last_sign_in_at: Option<String>,
    #[serde(default)]
    pub app_metadata: AppMetadata,
    #[serde(default)]
    pub user_metadata: UserMetadata,
    #[serde(default)]
    pub identities: Vec<Identity>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppMetadata {
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserMetadata {
    #[serde(default)]
    pub custom_claims: CustomClaims,
    #[serde(default)]
    pub email_verified: bool,
    #[serde(default)]
    pub phone_verified: bool,
    #[serde(default)]
    pub sub: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomClaims {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub chain: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub statement: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Identity {
    #[serde(default)]
    pub identity_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub provider: String,
}

/// Wallet settings from Unetwork
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletSettings {
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(rename = "licenseLeaseDefaultSharePercentage")]
    pub license_lease_default_share_percentage: f64,
    #[serde(rename = "licenseLeaseDefaultDurationMonths")]
    pub license_lease_default_duration_months: i32,
    #[serde(rename = "licenseLeaseDefaultMinUptimePercentage")]
    pub license_lease_default_min_uptime_percentage: f64,
    #[serde(rename = "licenseMarketplaceOn")]
    pub license_marketplace_on: bool,
}

/// Error type for wallet authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAuthError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for WalletAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for WalletAuthError {}

// ============================================================================
// WALLET AUTH SERVICE (SSR ONLY)
// ============================================================================

#[cfg(feature = "ssr")]
pub mod service {
    use super::*;
    use reqwest::Client;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// World Mobile Chain RPC URL
    const WMT_RPC_URL: &str = "https://worldmobilechain-mainnet.g.alchemy.com/public";

    /// Wallet authentication service
    ///
    /// Manages wallet operations and authentication with the Unetwork API.
    pub struct WalletAuthService {
        client: Client,
        evm_client: Arc<RwLock<Option<EvmClient>>>,
    }

    impl WalletAuthService {
        /// Create a new wallet auth service
        pub fn new() -> Self {
            Self {
                client: Client::new(),
                evm_client: Arc::new(RwLock::new(None)),
            }
        }

        /// Initialize EVM client for World Mobile Chain
        fn create_evm_client() -> EvmClient {
            let config = ChainConfig::new(
                ChainType::WorldMobile,
                ChainId::WorldMobile(unetwork::CHAIN_ID),
                WMT_RPC_URL.to_string(),
            );
            EvmClient::new(config)
        }

        /// Import a wallet from private key
        ///
        /// # Arguments
        /// * `private_key` - The private key (with or without 0x prefix)
        ///
        /// # Returns
        /// The wallet address
        pub async fn import_wallet(&self, private_key: &str) -> Result<String, WalletAuthError> {
            let evm_client = Self::create_evm_client();

            let credentials = evm_client
                .import_from_private_key(private_key)
                .await
                .map_err(|e| WalletAuthError {
                    code: "IMPORT_FAILED".to_string(),
                    message: format!("Failed to import wallet: {}", e),
                })?;

            let address = credentials.address.clone();

            // Store the client for later use
            *self.evm_client.write().await = Some(evm_client);

            Ok(address)
        }

        /// Import a wallet from mnemonic phrase
        ///
        /// # Arguments
        /// * `mnemonic` - The 12 or 24 word mnemonic phrase
        /// * `derivation_path` - Optional derivation path (defaults to m/44'/60'/0'/0/0)
        ///
        /// # Returns
        /// The wallet address
        pub async fn import_wallet_from_mnemonic(
            &self,
            mnemonic: &str,
            derivation_path: Option<&str>,
        ) -> Result<String, WalletAuthError> {
            let evm_client = Self::create_evm_client();

            let credentials = evm_client
                .import_from_mnemonic(mnemonic, derivation_path)
                .await
                .map_err(|e| WalletAuthError {
                    code: "IMPORT_FAILED".to_string(),
                    message: format!("Failed to import wallet from mnemonic: {}", e),
                })?;

            let address = credentials.address.clone();

            // Store the client for later use
            *self.evm_client.write().await = Some(evm_client);

            Ok(address)
        }

        /// Create a new wallet
        ///
        /// # Returns
        /// Tuple of (address, mnemonic)
        pub async fn create_wallet(&self) -> Result<(String, String), WalletAuthError> {
            let evm_client = Self::create_evm_client();

            let credentials = evm_client
                .create_wallet()
                .await
                .map_err(|e| WalletAuthError {
                    code: "CREATE_FAILED".to_string(),
                    message: format!("Failed to create wallet: {}", e),
                })?;

            let address = credentials.address.clone();
            let mnemonic = credentials.mnemonic().unwrap_or_default().to_string();

            // Store the client for later use
            *self.evm_client.write().await = Some(evm_client);

            Ok((address, mnemonic))
        }

        /// Get the current wallet address
        pub async fn get_address(&self) -> Result<String, WalletAuthError> {
            let guard = self.evm_client.read().await;
            let client = guard.as_ref().ok_or_else(|| WalletAuthError {
                code: "NO_WALLET".to_string(),
                message: "No wallet loaded".to_string(),
            })?;

            client.get_address().await.map_err(|e| WalletAuthError {
                code: "ADDRESS_ERROR".to_string(),
                message: format!("Failed to get address: {}", e),
            })
        }

        /// Generate a SIWE message for authentication
        ///
        /// # Arguments
        /// * `nonce` - Optional server-generated nonce (recommended for security)
        ///
        /// # Returns
        /// The SIWE message string ready for signing
        pub async fn generate_siwe_message(
            &self,
            nonce: Option<&str>,
        ) -> Result<String, WalletAuthError> {
            let address = self.get_address().await?;
            let siwe = build_unetwork_siwe_message(&address, nonce);
            Ok(siwe.to_message())
        }

        /// Sign a message with the loaded wallet
        ///
        /// # Arguments
        /// * `message` - The message to sign
        ///
        /// # Returns
        /// The signature as a 0x-prefixed hex string
        pub async fn sign_message(&self, message: &str) -> Result<String, WalletAuthError> {
            let guard = self.evm_client.read().await;
            let client = guard.as_ref().ok_or_else(|| WalletAuthError {
                code: "NO_WALLET".to_string(),
                message: "No wallet loaded".to_string(),
            })?;

            client.sign_message(message).await.map_err(|e| WalletAuthError {
                code: "SIGN_FAILED".to_string(),
                message: format!("Failed to sign message: {}", e),
            })
        }

        /// Authenticate with Unetwork using the loaded wallet
        ///
        /// This generates a SIWE message, signs it, and exchanges it for a JWT token.
        ///
        /// # Arguments
        /// * `nonce` - Optional server-generated nonce
        ///
        /// # Returns
        /// The authentication response containing the JWT token
        pub async fn authenticate(
            &self,
            nonce: Option<&str>,
        ) -> Result<Web3AuthResponse, WalletAuthError> {
            // 1. Generate SIWE message
            let message = self.generate_siwe_message(nonce).await?;

            // 2. Sign the message
            let signature = self.sign_message(&message).await?;

            // 3. Exchange for token
            self.exchange_signature_for_token(&message, &signature).await
        }

        /// Exchange a signed SIWE message for a JWT token
        ///
        /// # Arguments
        /// * `message` - The SIWE message that was signed
        /// * `signature` - The signature from signing the message
        ///
        /// # Returns
        /// The authentication response containing the JWT token
        pub async fn exchange_signature_for_token(
            &self,
            message: &str,
            signature: &str,
        ) -> Result<Web3AuthResponse, WalletAuthError> {
            let url = format!("{}/auth/v1/token?grant_type=web3", unetwork::API_URL);

            let request = Web3AuthRequest {
                chain: "ethereum".to_string(),
                message: message.to_string(),
                signature: signature.to_string(),
            };

            let response = self
                .client
                .post(&url)
                .header("apikey", unetwork::API_KEY)
                .header("Authorization", format!("Bearer {}", unetwork::API_KEY))
                .header("Content-Type", "application/json")
                .header("x-client-info", "ember-multichain/0.1.0")
                .json(&request)
                .send()
                .await
                .map_err(|e| WalletAuthError {
                    code: "NETWORK_ERROR".to_string(),
                    message: format!("Request failed: {}", e),
                })?;

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(WalletAuthError {
                    code: format!("API_ERROR_{}", status.as_u16()),
                    message: format!("Authentication failed: {}", error_text),
                });
            }

            response.json::<Web3AuthResponse>().await.map_err(|e| WalletAuthError {
                code: "PARSE_ERROR".to_string(),
                message: format!("Failed to parse response: {}", e),
            })
        }

        /// Refresh an access token
        ///
        /// # Arguments
        /// * `refresh_token` - The refresh token from a previous authentication
        ///
        /// # Returns
        /// New authentication response with fresh tokens
        pub async fn refresh_token(
            &self,
            refresh_token: &str,
        ) -> Result<Web3AuthResponse, WalletAuthError> {
            let url = format!("{}/auth/v1/token?grant_type=refresh_token", unetwork::API_URL);

            let body = serde_json::json!({
                "refresh_token": refresh_token
            });

            let response = self
                .client
                .post(&url)
                .header("apikey", unetwork::API_KEY)
                .header("Authorization", format!("Bearer {}", unetwork::API_KEY))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| WalletAuthError {
                    code: "NETWORK_ERROR".to_string(),
                    message: format!("Request failed: {}", e),
                })?;

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(WalletAuthError {
                    code: format!("API_ERROR_{}", status.as_u16()),
                    message: format!("Token refresh failed: {}", error_text),
                });
            }

            response.json::<Web3AuthResponse>().await.map_err(|e| WalletAuthError {
                code: "PARSE_ERROR".to_string(),
                message: format!("Failed to parse response: {}", e),
            })
        }

        /// Get wallet settings from Unetwork
        ///
        /// # Arguments
        /// * `access_token` - The JWT access token
        ///
        /// # Returns
        /// Wallet settings
        pub async fn get_wallet_settings(
            &self,
            access_token: &str,
        ) -> Result<Vec<WalletSettings>, WalletAuthError> {
            let url = format!("{}/rest/v1/rpc/wallet_settings_get", unetwork::API_URL);

            let response = self
                .client
                .post(&url)
                .header("apikey", unetwork::API_KEY)
                .header("Authorization", format!("Bearer {}", access_token))
                .header("Content-Type", "application/json")
                .header("Content-Profile", "public")
                .json(&serde_json::json!({}))
                .send()
                .await
                .map_err(|e| WalletAuthError {
                    code: "NETWORK_ERROR".to_string(),
                    message: format!("Request failed: {}", e),
                })?;

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(WalletAuthError {
                    code: format!("API_ERROR_{}", status.as_u16()),
                    message: format!("Failed to get wallet settings: {}", error_text),
                });
            }

            response.json::<Vec<WalletSettings>>().await.map_err(|e| WalletAuthError {
                code: "PARSE_ERROR".to_string(),
                message: format!("Failed to parse response: {}", e),
            })
        }

        /// Clear the loaded wallet
        pub async fn clear_wallet(&self) {
            *self.evm_client.write().await = None;
        }

        /// Check if a wallet is loaded
        pub async fn has_wallet(&self) -> bool {
            self.evm_client.read().await.is_some()
        }
    }

    impl Default for WalletAuthService {
        fn default() -> Self {
            Self::new()
        }
    }
}

// Re-export service for SSR
#[cfg(feature = "ssr")]
pub use service::WalletAuthService;
