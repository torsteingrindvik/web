use stpl::Render;

use crate::html::base;

pub struct Data {
    name: String,
}

impl Data {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub fn page(data: &Data) -> impl Render {
    use stpl::html::*;

    let content = (
        h1.class("main")("Welcome!!"),
        p(format!("Hi, {}!", data.name)),
        ul((0..100)
            .map(|n| li.class("super-li")(format!("The thing is: {}, square is {}", n, n * n)))
            .collect::<Vec<_>>()),
    );

    base::base(&base::Data::new("Some cool title"), content)
}
