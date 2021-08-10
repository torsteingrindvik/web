use std::path::{Path, PathBuf};

use async_std::fs;
use chrono::{DateTime, Utc};
use color_eyre::{eyre::eyre, eyre::Context, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

/// A blog post overview, pointing to the actual blog post.
#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    /// The blog title.
    pub title: String,

    /// An image related to the blog post.
    pub image_url: String,

    /// The url to the post itself.
    pub blog_url: String,

    /// When the blog post was written.
    pub published_at: DateTime<Utc>,
}

/// Given a markdown file, render it as HTML.
/// The path is expected to be the file only, e.g. `blog.md`,
/// without any parent folder(s).
pub async fn render_blog(blog: &Path) -> Result<String> {
    let blog_path = PathBuf::from(format!(
        "{}/{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "blog",
        blog.to_string_lossy()
    ));

    let md = fs::read_to_string(&blog_path).await?;
    let parser = pulldown_cmark::Parser::new(&md);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    Ok(html)
}

/// Get blog posts (equals markdown files in the blog dir)
pub async fn get_blogs() -> Result<Vec<Blog>> {
    let blogs_path = concat!(env!("CARGO_MANIFEST_DIR"), "/blog");

    let mut md_files = vec![];

    let mut dir = fs::read_dir(blogs_path)
        .await
        .wrap_err("Could not read dir")?;
    while let Some(res) = dir.next().await {
        let entry = res.wrap_err("Could not read entry")?;

        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "md" {
                let created = path
                    .metadata()
                    .await
                    .wrap_err("Could not get metadata")?
                    .created()
                    .wrap_err("Could not find creation time")?;

                let stem = path
                    .file_stem()
                    .ok_or_else(|| eyre!("No stem"))?
                    .to_string_lossy()
                    .to_string();
                md_files.push((path, created, stem));
            }
        }
    }

    Ok(md_files
        .into_iter()
        .map(|(_path, created, stem)| {
            let blog_name = stem
                .chars()
                .enumerate()
                .map(|(index, char)| {
                    if index == 0 {
                        char.to_ascii_uppercase()
                    } else {
                        char
                    }
                })
                .collect::<String>();
            Blog::new(
                &blog_name,
                "/static/hackernews.png",
                &format!("/blog/{}.md", stem),
                created.into(),
            )
        })
        .collect())
}

impl Blog {
    /// Create a new blog post overview.
    pub fn new(title: &str, image_url: &str, blog_url: &str, published_at: DateTime<Utc>) -> Self {
        Self {
            title: title.to_string(),
            image_url: image_url.to_string(),
            blog_url: blog_url.to_string(),
            published_at,
        }
    }
}
