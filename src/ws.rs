//! WebSocket clients for real-time market data and user events.
//!
//! Two WebSocket endpoints are available:
//!
//! - **`/ws/market`** — public market data (order books, trades, prices).
//!   Subscribe by outcome (asset) IDs, no authentication required.
//! - **`/ws/user`** — private user events (order fills, cancellations).
//!   Requires L2 API key authentication.
//!
//! # Example — Market WebSocket
//!
//! ```no_run
//! use hexmarket_sdk_async::ws::HexMarketWs;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let (ws, mut rx) = HexMarketWs::connect("wss://api.hexmarket.xyz/ws/market").await?;
//!
//! ws.subscribe(vec!["outcome-id-1".into(), "outcome-id-2".into()]).await?;
//!
//! while let Some(event) = rx.recv().await {
//!     println!("event_type={}, asset_id={}", event.event_type, event.asset_id);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Example — User WebSocket
//!
//! ```no_run
//! use hexmarket_sdk_async::ws::HexUserWs;
//! use hexmarket_sdk_async::ApiCredentials;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let creds = ApiCredentials {
//!     api_key: "your-api-key".into(),
//!     secret: "your-secret".into(),
//!     passphrase: "your-passphrase".into(),
//! };
//!
//! let (ws, mut rx) = HexUserWs::connect("wss://api.hexmarket.xyz/ws/user", creds, vec![]).await?;
//!
//! while let Some(event) = rx.recv().await {
//!     println!("{:?}", event);
//! }
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};

use crate::auth::ApiCredentials;

// ---------------------------------------------------------------------------
// Market WebSocket
// ---------------------------------------------------------------------------

/// A market data event received from `/ws/market`.
#[derive(Debug, Clone, Deserialize)]
pub struct MarketEvent {
    pub event_type: String,
    pub asset_id: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Handle to a connected market WebSocket. Use [`Self::subscribe`] /
/// [`Self::unsubscribe`] to manage subscriptions.
pub struct HexMarketWs {
    tx: mpsc::Sender<String>,
    _task: tokio::task::JoinHandle<()>,
}

impl HexMarketWs {
    /// Connect to the market WebSocket endpoint.
    /// Returns a handle and a receiver for incoming [`MarketEvent`]s.
    pub async fn connect(
        url: &str,
    ) -> Result<(Self, mpsc::Receiver<MarketEvent>), Box<dyn std::error::Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(url).await?;
        let (mut sink, mut stream) = ws_stream.split();

        let (event_tx, event_rx) = mpsc::channel::<MarketEvent>(256);
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(64);

        let task = tokio::spawn(async move {
            let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(10));

            loop {
                tokio::select! {
                    // Outgoing commands
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            Some(msg) => {
                                if sink.send(Message::Text(msg.into())).await.is_err() {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    // Incoming messages
                    msg = stream.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                let text = text.to_string();
                                if text == "PONG" {
                                    continue;
                                }
                                if let Ok(event) = serde_json::from_str::<MarketEvent>(&text) {
                                    if event_tx.send(event).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Some(Ok(Message::Close(_))) | None => break,
                            _ => continue,
                        }
                    }
                    // Ping heartbeat
                    _ = ping_interval.tick() => {
                        if sink.send(Message::Text("PING".into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Ok((Self { tx: cmd_tx, _task: task }, event_rx))
    }

    /// Subscribe to market events for the given outcome (asset) IDs.
    pub async fn subscribe(&self, asset_ids: Vec<String>) -> Result<(), mpsc::error::SendError<String>> {
        let msg = serde_json::json!({
            "assets_ids": asset_ids,
            "type": "market"
        });
        self.tx.send(msg.to_string()).await
    }

    /// Dynamically subscribe to additional asset IDs.
    pub async fn subscribe_more(&self, asset_ids: Vec<String>) -> Result<(), mpsc::error::SendError<String>> {
        let msg = serde_json::json!({
            "operation": "subscribe",
            "assets_ids": asset_ids
        });
        self.tx.send(msg.to_string()).await
    }

    /// Unsubscribe from asset IDs.
    pub async fn unsubscribe(&self, asset_ids: Vec<String>) -> Result<(), mpsc::error::SendError<String>> {
        let msg = serde_json::json!({
            "operation": "unsubscribe",
            "assets_ids": asset_ids
        });
        self.tx.send(msg.to_string()).await
    }
}

// ---------------------------------------------------------------------------
// User WebSocket
// ---------------------------------------------------------------------------

/// An event received from `/ws/user`.
#[derive(Debug, Clone, Deserialize)]
pub struct UserEvent {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Handle to a connected user WebSocket.
pub struct HexUserWs {
    tx: mpsc::Sender<String>,
    _task: tokio::task::JoinHandle<()>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserAuthMessage<'a> {
    auth: UserAuthPayload<'a>,
    #[serde(rename = "type")]
    msg_type: &'static str,
    markets: &'a [String],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserAuthPayload<'a> {
    api_key: &'a str,
    secret: &'a str,
    passphrase: &'a str,
}

impl HexUserWs {
    /// Connect to the user WebSocket endpoint with L2 API key credentials.
    /// Returns a handle and a receiver for incoming [`UserEvent`]s.
    pub async fn connect(
        url: &str,
        credentials: ApiCredentials,
        markets: Vec<String>,
    ) -> Result<(Self, mpsc::Receiver<UserEvent>), Box<dyn std::error::Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(url).await?;
        let (mut sink, mut stream) = ws_stream.split();

        // Send auth message immediately
        let auth_msg = serde_json::to_string(&UserAuthMessage {
            auth: UserAuthPayload {
                api_key: &credentials.api_key,
                secret: &credentials.secret,
                passphrase: &credentials.passphrase,
            },
            msg_type: "user",
            markets: &markets,
        })?;
        sink.send(Message::Text(auth_msg.into())).await?;

        let (event_tx, event_rx) = mpsc::channel::<UserEvent>(256);
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(64);

        let task = tokio::spawn(async move {
            let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(10));

            loop {
                tokio::select! {
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            Some(msg) => {
                                if sink.send(Message::Text(msg.into())).await.is_err() {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    msg = stream.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                let text = text.to_string();
                                if text == "PONG" {
                                    continue;
                                }
                                if let Ok(event) = serde_json::from_str::<UserEvent>(&text) {
                                    if event_tx.send(event).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Some(Ok(Message::Close(_))) | None => break,
                            _ => continue,
                        }
                    }
                    _ = ping_interval.tick() => {
                        if sink.send(Message::Text("PING".into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Ok((Self { tx: cmd_tx, _task: task }, event_rx))
    }

    /// Dynamically subscribe to additional markets.
    pub async fn subscribe_markets(&self, markets: Vec<String>) -> Result<(), mpsc::error::SendError<String>> {
        let msg = serde_json::json!({
            "operation": "subscribe",
            "markets": markets
        });
        self.tx.send(msg.to_string()).await
    }

    /// Dynamically unsubscribe from markets.
    pub async fn unsubscribe_markets(&self, markets: Vec<String>) -> Result<(), mpsc::error::SendError<String>> {
        let msg = serde_json::json!({
            "operation": "unsubscribe",
            "markets": markets
        });
        self.tx.send(msg.to_string()).await
    }
}
