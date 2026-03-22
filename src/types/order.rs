use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "buy"),
            Side::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Limit => write!(f, "limit"),
            OrderType::Market => write!(f, "market"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

impl std::fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeInForce::Gtc => write!(f, "gtc"),
            TimeInForce::Ioc => write!(f, "ioc"),
            TimeInForce::Fok => write!(f, "fok"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: uuid::Uuid,
    pub outcome_id: uuid::Uuid,
    pub user_pubkey: String,
    pub side: String,
    pub order_type: String,
    pub time_in_force: String,
    pub price: Decimal,
    pub quantity: i64,
    pub filled_quantity: i64,
    pub remaining_quantity: i64,
    pub status: String,
    pub nonce: i64,
    pub signature: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expired_at: Option<DateTime<Utc>>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderParams {
    pub outcome_id: String,
    pub side: Side,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub price: Decimal,
    pub quantity: u64,
    pub nonce: u64,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaceOrderResponse {
    pub order_id: String,
    pub status: String,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    pub order_id: String,
    pub status: String,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelledOrderRef {
    pub order_id: String,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponse {
    pub cancelled_count: usize,
    pub status: String,
    pub orders: Vec<CancelledOrderRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchPlaceResult {
    pub index: usize,
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchPlaceResponse {
    pub results: Vec<BatchPlaceResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelResult {
    pub order_id: String,
    pub client_order_id: Option<String>,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelResponse {
    pub results: Vec<BatchCancelResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchUpdateResponse {
    pub cancel_results: Vec<BatchCancelResult>,
    pub place_results: Vec<BatchPlaceResult>,
}
