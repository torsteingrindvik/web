use chrono::{DateTime, Utc};
use color_eyre::Result;
use serde::{Deserialize, Serialize};

/// A news article related to space flight.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    /// An identifier of the news article.
    pub id: usize,

    /// Article title.
    pub title: String,

    /// Article URL.
    pub url: String,

    /// An image related to the space news.
    pub image_url: String,

    /// The news site originating the article.
    pub news_site: String,

    /// A summary of the article.
    pub summary: String,

    /// When the article was published.
    pub published_at: DateTime<Utc>,
}

/// Get the latest news articles.
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
