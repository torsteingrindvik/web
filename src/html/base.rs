use stpl::Render;

pub struct Data {
    title: String,
}

impl Data {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

pub fn base<C: Render + 'static>(data: &Data, content: C) -> impl Render {
    use stpl::html::*;

    html((
        head((
            title(data.title.clone()),
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
        body(div.id("main")(content)),
    ))
}
