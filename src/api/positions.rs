use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::Position;

impl HexClient {
    /// Get all positions for the authenticated user (requires L2 auth).
    pub async fn get_positions(&self) -> Result<Vec<Position>, HexSdkError> {
        let pubkey = self.require_pubkey()?;
        let path = format!("/api/v1/positions?user={}", pubkey);
        let headers = self.l2_headers("GET", &path, None)?;
        let resp = headers
            .apply(self.http.get(self.url(&path)))
            .send()
            .await?;

        self.parse(resp).await
    }
}
