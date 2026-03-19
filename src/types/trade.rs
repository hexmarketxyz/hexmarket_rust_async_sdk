use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: uuid::Uuid,
    pub outcome_id: uuid::Uuid,
    pub maker_order_id: uuid::Uuid,
    pub taker_order_id: uuid::Uuid,
    pub maker_pubkey: String,
    pub taker_pubkey: String,
    pub outcome: String,
    pub side: String,
    pub price: Decimal,
    pub quantity: i64,
    pub maker_fee: i64,
    pub taker_fee: i64,
    pub settlement_status: String,
    pub settlement_tx: Option<String>,
    pub settled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub outcome_id: uuid::Uuid,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedOrderBookLevel {
    pub price: Decimal,
    pub quantity: i64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedOrderBook {
    pub outcome_id: uuid::Uuid,
    pub bids: Vec<MergedOrderBookLevel>,
    pub asks: Vec<MergedOrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}
