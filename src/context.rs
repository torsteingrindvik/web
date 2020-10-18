use serde::Serialize;

#[derive(Serialize)]
pub struct LinkableContent {
    pub description: &'static str,
    pub link: &'static str,
}

#[derive(Serialize)]
pub struct Card {
    pub title: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub contents: Vec<LinkableContent>,
    pub content_bg: (f32, f32, f32, f32),
}

impl Default for Card {
    fn default() -> Self {
        Self {
            title: "TODO",
            description: "TODO",
            image: "TODO",
            contents: vec![],
            content_bg: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Serialize)]
pub struct CardsContext {
    pub title: &'static str,
    pub cards: Vec<Card>,
    pub body_class: &'static str,
    pub parent: &'static str,
}

impl Default for CardsContext {
    fn default() -> Self {
        Self {
            title: "torste.in",
            cards: vec![],
            body_class: "grid-3-row-1-col",
            parent: "layout",
        }
    }
}

#[derive(Serialize)]
pub struct HtmlContext {
    pub title: String,
    pub content: String,
    pub parent: &'static str,
}

impl Default for HtmlContext {
    fn default() -> Self {
        Self {
            title: String::from("HTML document"),
            content: "<p>My content</p>".to_string(),
            parent: "layout",
        }
    }
}

#[derive(Serialize)]
pub struct TokeiContext {
    pub output: String,
    pub parent: &'static str,
}

impl Default for TokeiContext {
    fn default() -> Self {
        Self {
            output: String::from(""),
            parent: "layout",
        }
    }
}

#[derive(Serialize)]
pub struct ListContext {
    pub links: Vec<String>,
    pub parent: &'static str,
}

impl Default for ListContext {
    fn default() -> Self {
        Self {
            links: vec![],
            parent: "layout",
        }
    }
}
