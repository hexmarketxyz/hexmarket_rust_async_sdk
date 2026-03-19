use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::Outcome;

/// Query parameters for listing markets/outcomes.
#[derive(Debug, Default)]
pub struct ListMarketsParams {
    pub status: Option<String>,
    pub category: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl HexClient {
    /// List outcomes (paginated).
    pub async fn list_markets(&self, params: &ListMarketsParams) -> Result<Vec<Outcome>, HexSdkError> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = params.status {
            query.push(("status", s.clone()));
        }
        if let Some(ref c) = params.category {
            query.push(("category", c.clone()));
        }
        if let Some(l) = params.limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(o) = params.offset {
            query.push(("offset", o.to_string()));
        }

        let resp = self.http.get(self.url("/api/v1/markets"))
            .query(&query)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Get a single outcome by ID.
    pub async fn get_market(&self, outcome_id: &str) -> Result<Outcome, HexSdkError> {
        let resp = self.http.get(self.url(&format!("/api/v1/markets/{}", outcome_id)))
            .send()
            .await?;

        self.parse(resp).await
    }
}
