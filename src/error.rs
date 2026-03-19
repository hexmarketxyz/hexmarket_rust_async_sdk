//! SDK error types.

#[derive(Debug, thiserror::Error)]
pub enum HexSdkError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Invalid API secret")]
    InvalidSecret,

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Missing API credentials — call client.set_credentials() first")]
    MissingCredentials,

    #[error("{0}")]
    Other(String),
}
