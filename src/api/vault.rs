use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::VaultBalance;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TransactionResponse {
    pub transaction: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SubmitResponse {
    pub signature: String,
}

impl HexClient {
    /// Create a vault for the authenticated user (requires L2 auth).
    /// Returns a partially-signed transaction to co-sign and submit.
    pub async fn create_vault(&self) -> Result<TransactionResponse, HexSdkError> {
        let path = "/api/v1/vault/create";
        let headers = self.l2_headers("POST", path, None)?;
        let resp = headers
            .apply(self.http.post(self.url(path)))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Build a deposit transaction (requires L2 auth).
    /// `amount` is in USDC base units (6 decimals, e.g. 10_000_000 = 10 USDC).
    pub async fn deposit(&self, amount: u64) -> Result<TransactionResponse, HexSdkError> {
        let path = "/api/v1/vault/deposit";
        let body = serde_json::json!({ "amount": amount }).to_string();
        let headers = self.l2_headers("POST", path, None)?;
        let resp = headers
            .apply(self.http.post(self.url(path)))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Build a withdrawal transaction (requires L2 auth).
    /// `amount` is in USDC base units (6 decimals).
    pub async fn withdraw(&self, amount: u64) -> Result<TransactionResponse, HexSdkError> {
        let path = "/api/v1/vault/withdraw";
        let body = serde_json::json!({ "amount": amount }).to_string();
        let headers = self.l2_headers("POST", path, None)?;
        let resp = headers
            .apply(self.http.post(self.url(path)))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Submit a fully-signed transaction to Solana (requires L2 auth).
    pub async fn submit_transaction(
        &self,
        transaction_b64: &str,
    ) -> Result<SubmitResponse, HexSdkError> {
        let path = "/api/v1/vault/submit";
        let body = serde_json::json!({ "transaction": transaction_b64 }).to_string();
        let headers = self.l2_headers("POST", path, None)?;
        let resp = headers
            .apply(self.http.post(self.url(path)))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Get on-chain vault USDC balance (requires L2 auth).
    pub async fn get_vault_balance(&self) -> Result<VaultBalance, HexSdkError> {
        let pubkey = self.require_pubkey()?;
        let path = format!("/api/v1/vault/balance?user={}", pubkey);
        let headers = self.l2_headers("GET", &path, None)?;
        let resp = headers
            .apply(self.http.get(self.url(&path)))
            .send()
            .await?;

        self.parse(resp).await
    }
}
