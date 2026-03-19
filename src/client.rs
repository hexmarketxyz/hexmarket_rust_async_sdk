//! HexMarket API client.

use serde::de::DeserializeOwned;

use crate::auth::{self, ApiCredentials, L2Headers};
use crate::error::HexSdkError;

/// Configuration for the HexMarket client.
#[derive(Debug, Clone)]
pub struct HexClientConfig {
    /// API base URL, e.g. `https://api.hexmarket.io` or `http://localhost:8080`.
    pub api_url: String,
}

/// HexMarket API client.
///
/// # Example
///
/// ```no_run
/// use hexmarket_sdk::{HexClient, HexClientConfig, ApiCredentials};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = HexClient::new(HexClientConfig {
///     api_url: "https://api.hexmarket.io".into(),
/// });
///
/// // Public endpoints (no auth needed)
/// let events = client.list_events(&Default::default()).await?;
///
/// // Authenticated endpoints
/// client.set_credentials(
///     "your-solana-pubkey",
///     ApiCredentials {
///         api_key: "your-api-key".into(),
///         secret: "your-base64url-secret".into(),
///         passphrase: "your-passphrase".into(),
///     },
/// );
///
/// let balance = client.get_balance().await?;
/// # Ok(())
/// # }
/// ```
pub struct HexClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    credentials: std::sync::RwLock<Option<(String, ApiCredentials)>>,
}

impl HexClient {
    /// Create a new client.
    pub fn new(config: HexClientConfig) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http,
            base_url: config.api_url.trim_end_matches('/').to_string(),
            credentials: std::sync::RwLock::new(None),
        }
    }

    /// Set API credentials for L2-authenticated endpoints.
    pub fn set_credentials(&self, pubkey: &str, creds: ApiCredentials) {
        *self.credentials.write().unwrap() = Some((pubkey.to_string(), creds));
    }

    /// Clear stored credentials.
    pub fn clear_credentials(&self) {
        *self.credentials.write().unwrap() = None;
    }

    /// Build a full URL from a path.
    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Build L2 auth headers for the current credentials.
    pub(crate) fn l2_headers(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<L2Headers, HexSdkError> {
        let guard = self.credentials.read().unwrap();
        let (pubkey, creds) = guard.as_ref().ok_or(HexSdkError::MissingCredentials)?;
        auth::build_l2_headers(creds, pubkey, method, path, body)
    }

    /// Get the stored public key.
    pub(crate) fn require_pubkey(&self) -> Result<String, HexSdkError> {
        let guard = self.credentials.read().unwrap();
        let (pubkey, _) = guard.as_ref().ok_or(HexSdkError::MissingCredentials)?;
        Ok(pubkey.clone())
    }

    /// Parse a response, returning an error for non-2xx status codes.
    pub(crate) async fn parse<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, HexSdkError> {
        let status = resp.status();
        if !status.is_success() {
            let message = resp.text().await.unwrap_or_else(|_| status.to_string());
            return Err(HexSdkError::Api {
                status: status.as_u16(),
                message,
            });
        }
        resp.json::<T>()
            .await
            .map_err(|e| HexSdkError::InvalidResponse(e.to_string()))
    }
}
