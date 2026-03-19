use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::{MergedOrderBook, OrderBook};

impl HexClient {
    /// Get the direct orderbook for an outcome.
    pub async fn get_orderbook(&self, outcome_id: &str) -> Result<OrderBook, HexSdkError> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/orderbook/{}", outcome_id)))
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Get the merged orderbook (direct + cross-outcome synthetic liquidity).
    pub async fn get_merged_orderbook(
        &self,
        outcome_id: &str,
    ) -> Result<MergedOrderBook, HexSdkError> {
        let resp = self
            .http
            .get(self.url(&format!(
                "/api/v1/orderbook/{}/merged",
                outcome_id
            )))
            .send()
            .await?;

        self.parse(resp).await
    }
}
