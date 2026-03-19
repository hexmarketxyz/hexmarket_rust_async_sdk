use crate::client::HexClient;
use crate::error::HexSdkError;
use crate::types::{EventDetail, EventListItem, Tag, TagDetail};

/// Query parameters for listing events.
#[derive(Debug, Default)]
pub struct ListEventsParams {
    pub tag: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl HexClient {
    /// List events (no auth required).
    pub async fn list_events(
        &self,
        params: &ListEventsParams,
    ) -> Result<Vec<EventListItem>, HexSdkError> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref t) = params.tag {
            query.push(("tag", t.clone()));
        }
        if let Some(ref s) = params.status {
            query.push(("status", s.clone()));
        }
        if let Some(l) = params.limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(o) = params.offset {
            query.push(("offset", o.to_string()));
        }

        let resp = self
            .http
            .get(self.url("/api/v1/events"))
            .query(&query)
            .send()
            .await?;

        self.parse(resp).await
    }

    /// Get event detail by slug.
    pub async fn get_event(&self, slug: &str) -> Result<EventDetail, HexSdkError> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/events/{}", slug)))
            .send()
            .await?;

        self.parse(resp).await
    }

    /// List top-level tags.
    pub async fn list_tags(&self) -> Result<Vec<Tag>, HexSdkError> {
        let resp = self.http.get(self.url("/api/v1/tags")).send().await?;
        self.parse(resp).await
    }

    /// Get a tag by slug with its children.
    pub async fn get_tag(&self, slug: &str) -> Result<TagDetail, HexSdkError> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/tags/{}", slug)))
            .send()
            .await?;

        self.parse(resp).await
    }
}
