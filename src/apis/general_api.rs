use serde::{Deserialize, Serialize};
use stpl::{html::*, Render};

use crate::util;

use super::{hackernews, nrk, space_flight};

// TODO: Create a trait for this instead.

/// A generalized API able to be displayed in a common way.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralApi {
    img: String,
    title: String,
    content: String,
    time: String,
}

impl GeneralApi {
    /// Get renderable HTML of the contents.
    pub fn html(&self) -> impl Render {
        div.class("api")((
            div.class("header")((
                h2.class("title")(self.title.to_string()),
                p.class("subtitle")(self.time.to_string()),
            )),
            img.class("image")
                .src(self.img.to_string())
                .alt("api image"),
            p.class("content")(self.content.clone()),
        ))
    }
}

/// Collect several `GeneralApi`s into a collection.
pub fn render_apis(apis: &[GeneralApi]) -> impl Render {
    apis.iter().map(|apis| apis.html()).collect::<Vec<_>>()
}

impl From<space_flight::News> for GeneralApi {
    fn from(news: space_flight::News) -> Self {
        Self {
            img: news.image_url,
            title: news.title,
            content: news.summary,
            time: util::date_display(&news.published_at),
        }
    }
}

impl From<hackernews::Story> for GeneralApi {
    fn from(story: hackernews::Story) -> Self {
        Self {
            img: "/static/hackernews.png".to_string(),
            title: format!("{} (score: {})", story.title, story.score),
            content: util::sanitize_html(
                &story.comments[0]
                    .text
                    .to_string()
                    .chars()
                    .take(200)
                    .collect::<String>(),
            ),
            time: util::date_display(&story.time),
        }
    }
}

impl From<nrk::Program> for GeneralApi {
    fn from(program: nrk::Program) -> Self {
        Self {
            img: program.image,
            title: program.title,
            content: program.description,
            time: program.datelike,
        }
    }
}
