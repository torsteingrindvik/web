use color_eyre::Result;
use serde::{Deserialize, Serialize};

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
            title: json_program.main_title,
            image: most_fitting_picture.image_url.to_string(),
            description: json_program.series_description,
            datelike: json_program.episode_number_or_date,
        }
    }
}

impl Programs {
    pub async fn new() -> Result<Self> {
        let json_programs = fetch().await?;
        dbg!(&json_programs);

        let programs = json_programs.into_iter().map(Into::into).collect();

        Ok(Self(programs))
    }
}

#[derive(Debug)]
pub struct Program {
    pub title: String,
    pub image: String,
    pub description: String,
    pub datelike: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonProgram {
    id: String,
    main_title: String,
    episode_title: String,
    image: Image,
    series_description: String,
    episode_number_or_date: String,
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
