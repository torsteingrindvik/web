use stpl::Render;

use crate::html::base;

/// Placeholder data.
pub struct Data {
    name: String,
}

impl Data {
    /// Create new data.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Create a new home page with some data.
pub fn page(data: &Data) -> impl Render {
    use stpl::html::*;

    let content = (
        h1.class("main")("Welcome!!"),
        p(format!("Hi, {}!", data.name)),
        ul((0..100)
            .map(|n| li.class("super-li")(format!("The thing is: {}, square is {}", n, n * n)))
            .collect::<Vec<_>>()),
    );

    base::base("Home page", content)
}
