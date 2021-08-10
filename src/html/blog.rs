use stpl::Render;

use crate::html::base;

/// Create a new blog site with some data.
pub fn html(data: &Data) -> impl Render {
    use stpl::html::*;

    let content = (
        h1.class("main")("Welcome!!"),
        p(format!("Hi, {}!", data.name)),
        ul((0..100)
            .map(|n| li.class("super-li")(format!("The thing is: {}, square is {}", n, n * n)))
            .collect::<Vec<_>>()),
    );

    base::base("Blog", content)
}
