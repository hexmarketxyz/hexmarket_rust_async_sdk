use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketStatus {
    Active,
    Paused,
    ResolutionProposed,
    Resolved,
    Voided,
    Unlisted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    Binary,
    Categorical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeResult {
    Unresolved,
    Yes,
    No,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outcome {
    pub id: uuid::Uuid,
    pub market_id: uuid::Uuid,
    pub label: String,
    #[serde(default)]
    pub label_translations: Option<HashMap<String, String>>,
    pub sort_order: i32,
    pub outcome_index: i32,
    pub mint: Option<String>,
    pub price: Option<Decimal>,
    pub question: String,
    #[serde(default)]
    pub question_translations: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub status: String,
    pub outcome: Option<String>,
    pub volume_24h: Option<Decimal>,
    pub total_volume: Option<Decimal>,
    pub liquidity: Option<Decimal>,
    pub close_time: Option<DateTime<Utc>>,
    pub resolution_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: uuid::Uuid,
    pub event_id: uuid::Uuid,
    pub title: String,
    #[serde(default)]
    pub title_translations: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub icon_url: Option<String>,
    pub market_type: String,
    pub status: String,
    pub start_time: Option<DateTime<Utc>>,
    pub close_time: Option<DateTime<Utc>>,
    pub resolution_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub sort_order: i32,
    pub onchain_market_id: Option<i32>,
    pub pubkey: Option<String>,
    pub vault_pubkey: Option<String>,
    pub collateral_mint: Option<String>,
    pub num_outcomes: i32,
    pub price_increment: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: uuid::Uuid,
    pub slug: String,
    pub label: String,
    #[serde(default)]
    pub label_translations: Option<HashMap<String, String>>,
    pub parent_id: Option<uuid::Uuid>,
    pub sort_order: i32,
    pub icon_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagDetail {
    #[serde(flatten)]
    pub tag: Tag,
    pub children: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HexEvent {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub title_translations: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub icon_url: Option<String>,
    pub status: String,
    pub close_time: Option<DateTime<Utc>>,
    pub resolution_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub is_archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListItem {
    #[serde(flatten)]
    pub event: HexEvent,
    pub outcomes: Vec<Outcome>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDetail {
    #[serde(flatten)]
    pub event: HexEvent,
    pub outcomes: Vec<Outcome>,
    pub markets: Vec<Market>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceSnapshot {
    pub id: uuid::Uuid,
    pub outcome_id: uuid::Uuid,
    pub price: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub captured_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub user_pubkey: String,
    pub outcome_id: uuid::Uuid,
    pub quantity: i64,
    pub avg_price: Option<Decimal>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBalance {
    pub user_pubkey: String,
    pub usdc_balance: i64,
    pub locked_usdc: i64,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultBalance {
    pub user: String,
    pub vault_pubkey: String,
    pub usdc_balance: u64,
}
