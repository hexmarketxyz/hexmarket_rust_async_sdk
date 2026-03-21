use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::{Order, PlaceOrderParams, PlaceOrderResponse};

impl HexClient {
    /// Place a new order (requires L2 auth).
    pub async fn place_order(&self, params: &PlaceOrderParams) -> Result<PlaceOrderResponse, HexSdkError> {
        let path = "/api/v1/orders";
        let body = serde_json::to_string(params)
            .map_err(|e| HexSdkError::Other(e.to_string()))?;

        let headers = self.l2_headers("POST", path, None)?;
        let resp = headers
            .apply(self.http.post(self.url(path)))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Cancel an order by ID (requires L2 auth).
    pub async fn cancel_order(&self, order_id: &str) -> Result<serde_json::Value, HexSdkError> {
        let path = format!("/api/v1/orders/{}", order_id);
        let headers = self.l2_headers("DELETE", &path, None)?;
        let resp = headers
            .apply(self.http.delete(self.url(&path)))
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Cancel all open orders, optionally filtered by market or event (requires L2 auth).
    pub async fn cancel_all_orders(
        &self,
        market_id: Option<&str>,
        event_id: Option<&str>,
    ) -> Result<serde_json::Value, HexSdkError> {
        let mut path = "/api/v1/orders".to_string();
        let mut params = Vec::new();
        if let Some(mid) = market_id {
            params.push(format!("market_id={}", mid));
        }
        if let Some(eid) = event_id {
            params.push(format!("event_id={}", eid));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        let headers = self.l2_headers("DELETE", &path, None)?;
        let resp = headers
            .apply(self.http.delete(self.url(&path)))
            .send()
            .await?;

        self.parse(resp).await
    }

    /// List open orders for the authenticated user (requires L2 auth).
    pub async fn get_open_orders(
        &self,
        outcome_id: Option<&str>,
    ) -> Result<Vec<Order>, HexSdkError> {
        let pubkey = self.require_pubkey()?;
        let mut path = format!("/api/v1/orders?user={}&status=open", pubkey);
        if let Some(oid) = outcome_id {
            path.push_str(&format!("&outcome_id={}", oid));
        }
        let headers = self.l2_headers("GET", &path, None)?;
        let resp = headers
            .apply(self.http.get(self.url(&path)))
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Place multiple orders in a single batch (requires L2 auth).
    /// All orders must belong to the same market.
    pub async fn batch_place_orders(
        &self,
        market_id: &str,
        orders: &[PlaceOrderParams],
    ) -> Result<serde_json::Value, HexSdkError> {
        let path = "/api/v1/orders/batch";
        let body = serde_json::json!({
            "market_id": market_id,
            "orders": orders,
        });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| HexSdkError::Other(e.to_string()))?;

        let headers = self.l2_headers("POST", path, None)?;
        let resp = headers
            .apply(self.http.post(self.url(path)))
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Cancel multiple orders in a single batch (requires L2 auth).
    /// All orders must belong to the same market.
    pub async fn batch_cancel_orders(
        &self,
        market_id: &str,
        order_ids: &[&str],
    ) -> Result<serde_json::Value, HexSdkError> {
        let path = "/api/v1/orders/batch";
        let body = serde_json::json!({
            "market_id": market_id,
            "order_ids": order_ids,
        });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| HexSdkError::Other(e.to_string()))?;

        let headers = self.l2_headers("DELETE", path, None)?;
        let resp = headers
            .apply(self.http.delete(self.url(path)))
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// List closed (filled/cancelled) orders for the authenticated user.
    pub async fn get_closed_orders(
        &self,
        outcome_id: Option<&str>,
    ) -> Result<Vec<Order>, HexSdkError> {
        let pubkey = self.require_pubkey()?;
        let mut path = format!("/api/v1/orders?user={}&status=closed", pubkey);
        if let Some(oid) = outcome_id {
            path.push_str(&format!("&outcome_id={}", oid));
        }
        let headers = self.l2_headers("GET", &path, None)?;
        let resp = headers
            .apply(self.http.get(self.url(&path)))
            .send()
            .await?;

        self.parse(resp).await
    }
}
