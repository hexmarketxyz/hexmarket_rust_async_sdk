//! L1 (wallet) and L2 (API key HMAC) authentication for HexMarket.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::HexSdkError;

/// Stored API credentials returned by `POST /auth/api-key`.
#[derive(Debug, Clone)]
pub struct ApiCredentials {
    pub api_key: String,
    pub secret: String,
    pub passphrase: String,
}

/// L2 headers to attach to authenticated API requests.
#[derive(Debug, Clone)]
pub struct L2Headers {
    pub address: String,
    pub api_key: String,
    pub passphrase: String,
    pub timestamp: String,
    pub signature: String,
}

impl L2Headers {
    /// Insert L2 headers into a `reqwest::RequestBuilder`.
    pub fn apply(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder
            .header("HEX-ADDRESS", &self.address)
            .header("HEX-API-KEY", &self.api_key)
            .header("HEX-PASSPHRASE", &self.passphrase)
            .header("HEX-TIMESTAMP", &self.timestamp)
            .header("HEX-SIGNATURE", &self.signature)
    }
}

// ---------------------------------------------------------------------------
// L2 HMAC signing
// ---------------------------------------------------------------------------

/// Build the HMAC-SHA256 payload: `{timestamp}{METHOD}{path}[{body}]`.
pub fn build_l2_signing_payload(
    timestamp: u64,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> String {
    let mut payload = format!("{}{}{}", timestamp, method, path);
    if let Some(b) = body {
        payload.push_str(b);
    }
    payload
}

/// Sign a payload with the API secret using HMAC-SHA256.
/// Returns a base64url-encoded (no padding) signature.
pub fn sign_l2(secret_b64: &str, payload: &str) -> Result<String, HexSdkError> {
    let secret_bytes = URL_SAFE_NO_PAD
        .decode(secret_b64)
        .or_else(|_| {
            // Try with padding
            base64::engine::general_purpose::URL_SAFE
                .decode(secret_b64)
        })
        .map_err(|_| HexSdkError::InvalidSecret)?;

    let mut mac =
        Hmac::<Sha256>::new_from_slice(&secret_bytes).map_err(|_| HexSdkError::InvalidSecret)?;
    mac.update(payload.as_bytes());
    let result = mac.finalize();

    Ok(URL_SAFE_NO_PAD.encode(result.into_bytes()))
}

/// Build L2 authentication headers for an API request.
pub fn build_l2_headers(
    creds: &ApiCredentials,
    pubkey: &str,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<L2Headers, HexSdkError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = build_l2_signing_payload(timestamp, method, path, body);
    let signature = sign_l2(&creds.secret, &payload)?;

    Ok(L2Headers {
        address: pubkey.to_string(),
        api_key: creds.api_key.clone(),
        passphrase: creds.passphrase.clone(),
        timestamp: timestamp.to_string(),
        signature,
    })
}

// ---------------------------------------------------------------------------
// L1 wallet auth token
// ---------------------------------------------------------------------------

const AUTH_MESSAGE_PREFIX: &str = "hexmarket:auth\n";

/// Build the auth message bytes: `hexmarket:auth\n{timestamp}`.
pub fn build_auth_message(timestamp: u64) -> Vec<u8> {
    format!("{}{}", AUTH_MESSAGE_PREFIX, timestamp).into_bytes()
}

/// Build a signed auth token: `{pubkey}.{timestamp}.{signature_b58}`.
///
/// `sign_fn` should produce an Ed25519 signature over the given message bytes.
pub fn build_auth_token(
    pubkey: &str,
    timestamp: u64,
    signature_b58: &str,
) -> String {
    format!("{}.{}.{}", pubkey, timestamp, signature_b58)
}

// ---------------------------------------------------------------------------
// Order message signing
// ---------------------------------------------------------------------------

/// Build the canonical order message for wallet signing.
/// Must match the server-side `build_order_message` exactly.
pub fn build_order_message(
    outcome_id: &str,
    side: &str,
    price: &str,
    quantity: u64,
    nonce: u64,
) -> Vec<u8> {
    format!(
        "hexmarket:place_order\noutcome_id:{}\nside:{}\nprice:{}\nquantity:{}\nnonce:{}",
        outcome_id, side, price, quantity, nonce
    )
    .into_bytes()
}

/// Build the message for API key creation/derivation.
/// Format: `hexmarket:create_api_key\n{nonce}`
pub fn build_api_key_message(nonce: u32) -> Vec<u8> {
    format!("hexmarket:create_api_key\n{}", nonce).into_bytes()
}

/// Generate a nonce for replay protection.
/// Uses timestamp in ms * 1000 + random suffix.
pub fn generate_nonce() -> u64 {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    ms * 1000 + (rand::random::<u64>() % 1000)
}

/// Current Unix timestamp in seconds.
pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// ---------------------------------------------------------------------------
// Ed25519 signing helpers
// ---------------------------------------------------------------------------

/// Sign a message with an Ed25519 keypair, returning the base58-encoded signature.
pub fn ed25519_sign(keypair: &ed25519_dalek::SigningKey, message: &[u8]) -> String {
    use ed25519_dalek::Signer;
    let sig = keypair.sign(message);
    bs58::encode(sig.to_bytes()).into_string()
}

/// Get the base58-encoded public key from an Ed25519 signing key.
pub fn pubkey_b58(keypair: &ed25519_dalek::SigningKey) -> String {
    bs58::encode(keypair.verifying_key().to_bytes()).into_string()
}
