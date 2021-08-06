use color_eyre::Result;
use serde::{Deserialize, Serialize};

/// Television programs.
#[derive(Debug)]
pub struct Programs(pub Vec<Program>);

async fn fetch() -> Result<Vec<JsonProgram>> {
    Ok(
        reqwest::get("http://psapi-granitt-prod-ne.cloudapp.net/medium/tv/recentlypublishedprograms?maxnumber=20")
            .await?
            .json()
            .await?,
    )
}

impl From<JsonProgram> for Program {
    fn from(json_program: JsonProgram) -> Self {
        let most_fitting_picture = json_program
            .image
            .web_images
            .iter()
            .min_by_key(|image| (image.pixel_width as i64 - 600).abs())
            .expect("Need at least one image");

        Self {
            title: json_program
                .main_title
                .unwrap_or_else(|| "(No main title)".to_string()),
            image: most_fitting_picture.image_url.to_string(),
            description: json_program
                .description
                .unwrap_or_else(|| "(No description)".to_string()),
            datelike: json_program
                .episode_number_or_date
                .unwrap_or_else(|| "(No date)".to_string()),
        }
    }
}

impl Programs {
    /// Fetch television programs.
    pub async fn new() -> Result<Self> {
        let json_programs = fetch().await?;
        dbg!(&json_programs);

        let programs = json_programs.into_iter().map(Into::into).collect();

        Ok(Self(programs))
    }
}

/// A single television program.
#[derive(Debug)]
pub struct Program {
    /// The title of the program.
    pub title: String,

    /// An image related to the program.
    pub image: String,
    
    /// A description of the program.
    pub description: String,

    /// A date or time the program was aired.
    /// Not reliable as a proper datetime- some times it is given as an actual date,
    /// at other times only the air time.
    pub datelike: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonProgram {
    id: Option<String>,
    main_title: Option<String>,
    episode_title: Option<String>,
    image: Image,
    description: Option<String>,
    episode_number_or_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebImage {
    image_url: String,
    pixel_width: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    web_images: Vec<WebImage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api() {
        let api = Programs::new().await.unwrap();
        dbg!(api);
    }
}
