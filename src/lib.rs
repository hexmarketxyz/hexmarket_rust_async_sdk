//! # HexMarket Rust SDK
//!
//! Rust client for the HexMarket prediction market API.
//! Designed for quantitative trading and market making.
//!
//! ## Quick Start
//!
//! ```no_run
//! use hexmarket_sdk_async::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 1. Create client
//! let client = HexClient::new(HexClientConfig {
//!     api_url: "https://api.hexmarket.xyz".into(),
//! });
//!
//! // 2. Browse markets (no auth needed)
//! let events = client.list_events(&Default::default()).await?;
//! for event in &events {
//!     println!("{}: {} outcomes", event.event.title, event.outcomes.len());
//! }
//!
//! // 3. Set up authentication
//! client.set_credentials(
//!     "YourSolanaPubkey",
//!     ApiCredentials {
//!         api_key: "your-api-key".into(),
//!         secret: "your-base64url-secret".into(),
//!         passphrase: "your-passphrase".into(),
//!     },
//! );
//!
//! // 4. Check balance
//! let balance = client.get_balance().await?;
//! println!("USDC: {} (locked: {})", balance.usdc_balance, balance.locked_usdc);
//!
//! // 5. Get orderbook
//! let book = client.get_orderbook("outcome-uuid").await?;
//! println!("Best bid: {:?}, Best ask: {:?}", book.bids.first(), book.asks.first());
//!
//! // 6. Place an order (requires Ed25519 wallet signature)
//! use rust_decimal_macros::dec;
//!
//! let nonce = auth::generate_nonce();
//! let order_msg = auth::build_order_message(
//!     "outcome-uuid", "buy", "0.55", 10, nonce,
//! );
//! // Sign `order_msg` with your Ed25519 keypair, then:
//! // let signature = auth::ed25519_sign(&keypair, &order_msg);
//! // let resp = client.place_order(&PlaceOrderParams { ... }).await?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Authentication
//!
//! HexMarket uses two authentication layers:
//!
//! - **L1 (wallet)**: Ed25519 signature for API key creation. Format: `Bearer {pubkey}.{timestamp}.{sig}`
//! - **L2 (HMAC)**: API-key HMAC-SHA256 for all trading endpoints. Uses `HEX-*` headers.
//!
//! For quant trading, you typically:
//! 1. Create API credentials once via the web UI or L1 auth
//! 2. Store `{ api_key, secret, passphrase }` securely
//! 3. Pass them to `client.set_credentials()` — the SDK handles HMAC signing automatically
//!
//! ## Order Signatures
//!
//! Each order must include an Ed25519 wallet signature over the canonical order message.
//! This is required for on-chain settlement. Use [`auth::build_order_message`] to construct
//! the message, sign it with your keypair, and include the base58 signature in [`PlaceOrderParams`].

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod types;

// Re-export key types at crate root for convenience.
pub use auth::ApiCredentials;
pub use client::{HexClient, HexClientConfig};
pub use error::HexSdkError;
pub use types::*;
