use chrono::{DateTime, Utc};
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub news_site: String,
    pub summary: String,
    pub published_at: DateTime<Utc>,
}

pub async fn fetch() -> Result<Vec<News>> {
    Ok(
        reqwest::get("https://api.spaceflightnewsapi.net/v3/articles")
            .await?
            .json()
            .await?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api() {
        let api = fetch().await.unwrap();
        dbg!(api);
    }
}
