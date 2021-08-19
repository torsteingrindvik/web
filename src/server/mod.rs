use std::path::PathBuf;

use crate::{
    apis::{
        blog,
        general_api::{render_apis, GeneralApi},
        hackernews, nrk, space_flight,
    },
    html::render,
};
use rocket::{
    fs::{relative, FileServer},
    get,
    http::{Cookie, CookieJar},
    response::{content, Debug},
    routes,
};

type Result<T, E = Debug<color_eyre::Report>> = std::result::Result<T, E>;

#[get("/")]
fn get_root(cookies: &CookieJar<'_>) -> Result<content::Html<String>> {
    const COOKIE: &str = "times_visited";
    let visited = cookies
        .get(COOKIE)
        .and_then(|n| n.value().parse().ok())
        .unwrap_or(0);

    let response = render::render_content(
        "Home",
        format!(
            "Thanks for stopping by, just like you have {} times before",
            visited
        ),
    )?;

    cookies.add(Cookie::new(COOKIE, (visited + 1).to_string()));

    Ok(content::Html(response))
}

#[get("/spacenews")]
async fn get_spacenews() -> Result<content::Html<String>> {
    let news = space_flight::fetch().await?;
    let apis: Vec<GeneralApi> = news.into_iter().map(Into::into).collect();

    let response = render::render_content("Space News", render_apis(&apis))?;

    Ok(content::Html(response))
}

#[get("/hackernews")]
async fn get_hackernews() -> Result<content::Html<String>> {
    let hn = hackernews::HackerNews::new().await?;
    let apis: Vec<GeneralApi> = hn.0.into_iter().map(Into::into).collect();

    let response = render::render_content("HackerNews", render_apis(&apis))?;

    Ok(content::Html(response))
}

#[get("/nrk")]
async fn get_nrk() -> Result<content::Html<String>> {
    let programs = nrk::Programs::new().await?;
    let apis: Vec<GeneralApi> = programs.0.into_iter().map(Into::into).collect();

    let response = render::render_content("NRK", render_apis(&apis))?;

    Ok(content::Html(response))
}

#[get("/blog")]
async fn get_blogs() -> Result<content::Html<String>> {
    let blogs = blog::get_blogs().await?;
    let blogs: Vec<GeneralApi> = blogs.into_iter().map(Into::into).collect();

    let response = render::render_content("Blog", render_apis(&blogs))?;
    Ok(content::Html(response))
}

#[get("/blog/<post>")]
async fn get_blog(post: PathBuf) -> Result<content::Html<String>> {
    let content = blog::render_blog(&post).await?;

    let post_name = post
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Blog post");

    let content = render::div_wrap("blog-post card-bg-border", content)?;
    let response = render::render_content(post_name, content)?;

    Ok(content::Html(response))
}

/// Start the web server- never returns.
pub fn serve() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                get_root,
                get_blogs,
                get_blog,
                get_spacenews,
                get_hackernews,
                get_nrk
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
}
