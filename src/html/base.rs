use stpl::Render;

use crate::util;

/// Add some renderable content to the base HTML.
pub fn base<C: Render + 'static>(data: &str, content: C) -> impl Render {
    use stpl::html::*;

    let nav_items = vec![
        "nrk".to_string(),
        "spacenews".to_string(),
        "hackernews".to_string(),
    ];

    html((
        head((
            title(data.to_string()),
            meta.name("description")
                .content("This is a template test: Using the `stpl` crate."),
            meta.name("viewport")
                .content("width=device-width, initial-scale=1.0"),
            link.rel("shortcut icon")
                .type_("image/x-icon")
                .href("/static/favicon.ico"),
            link.rel("stylesheet")
                .type_("text/css")
                .href("/static/style.css"),
        )),
        body((
            nav.id("nav-bar")(ul(nav_items
                .into_iter()
                .map(|nav_item| {
                    li((
                        div.class("nav-emoji")(format!("{}", util::random_emoij())),
                        a.href(format!("/{}", nav_item))(nav_item),
                    ))
                })
                .collect::<Vec<_>>())),
            div.id("main")(content),
        )),
    ))
}
