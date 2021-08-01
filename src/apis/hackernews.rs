use chrono::{DateTime, TimeZone, Utc};
use color_eyre::Result;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HackerNews(pub Vec<Story>);

async fn top_stories() -> Result<Vec<Item>> {
    Ok(
        reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .await?
            .json()
            .await?,
    )
}

async fn produce_story(item: Item) -> Result<Story> {
    let json_story = JsonStory::new(&item).await?;
    let story = Story::from_json_story(json_story).await?;

    Ok(story)
}

impl HackerNews {
    pub async fn new() -> Result<Self> {
        let story_items = top_stories().await?;
        dbg!("Got top stories");

        let stories = story_items
            .into_iter()
            .take(10)
            .map(produce_story)
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|story| story.ok())
            .collect::<Vec<_>>();

        Ok(Self(stories))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub text: String,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            text: "<problem getting comment>".to_string(),
        }
    }
}

impl Comment {
    pub async fn new(item: &Item) -> Result<Self> {
        fetch_item(item).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub title: String,
    pub url: String,
    pub comments: Vec<Comment>,
    pub time: DateTime<Utc>,
    pub score: usize,
}

impl Story {
    async fn from_json_story(json_story: JsonStory) -> Result<Self> {
        let mut comments = vec![];

        for item in json_story.kids.iter().take(1) {
            comments.push(Comment::new(item).await.unwrap_or_default());
        }

        Ok(Self {
            title: json_story.title,
            url: json_story.url,
            comments,
            time: Utc.timestamp(json_story.time, 0),
            score: json_story.score,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonStory {
    title: String,
    url: String,
    kids: Vec<Item>,
    time: i64,
    score: usize,
}

async fn fetch_item<T: for<'de> serde::Deserialize<'de>>(item: &Item) -> Result<T> {
    dbg!("Fetching item: {:?}", item);

    Ok(reqwest::get(format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json",
        item.0
    ))
    .await?
    .json()
    .await?)
}

impl JsonStory {
    pub async fn new(item: &Item) -> Result<Self> {
        fetch_item(item).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api() {
        let api = HackerNews::new().await.unwrap();
        dbg!(api);
    }
}
