use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::Trade;

/// Query parameters for listing trades.
#[derive(Debug, Default)]
pub struct ListTradesParams {
    pub outcome_id: Option<String>,
    pub user: Option<String>,
    pub limit: Option<i64>,
}

impl HexClient {
    /// List trades (no auth required).
    pub async fn list_trades(&self, params: &ListTradesParams) -> Result<Vec<Trade>, HexSdkError> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref oid) = params.outcome_id {
            query.push(("outcome_id", oid.clone()));
        }
        if let Some(ref u) = params.user {
            query.push(("user", u.clone()));
        }
        if let Some(l) = params.limit {
            query.push(("limit", l.to_string()));
        }

        let resp = self
            .http
            .get(self.url("/api/v1/trades"))
            .query(&query)
            .send()
            .await?;

        self.parse(resp).await
    }
}
