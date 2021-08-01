use crate::{
    apis::{
        general_api::{render_apis, GeneralApi},
        hackernews, nrk, space_flight,
    },
    html::render::render_content,
};
use rocket::{
    fs::{relative, FileServer},
    get,
    response::{content, Debug},
    routes,
};

type Result<T, E = Debug<color_eyre::Report>> = std::result::Result<T, E>;

#[get("/")]
async fn get_spacenews() -> Result<content::Html<String>> {
    let news = space_flight::fetch().await?;
    let apis: Vec<GeneralApi> = news.into_iter().map(Into::into).collect();

    let response = render_content(render_apis(&apis))?;

    Ok(content::Html(response))
}

#[get("/")]
async fn get_hackernews() -> Result<content::Html<String>> {
    let hn = hackernews::HackerNews::new().await?;
    let apis: Vec<GeneralApi> = hn.0.into_iter().map(Into::into).collect();

    let response = render_content(render_apis(&apis))?;

    Ok(content::Html(response))
}

#[get("/")]
async fn get_nrk() -> Result<content::Html<String>> {
    let programs = nrk::Programs::new().await?;
    let apis: Vec<GeneralApi> = programs.0.into_iter().map(Into::into).collect();

    let response = render_content(render_apis(&apis))?;

    Ok(content::Html(response))
}

#[get("/")]
fn get_root() -> Result<content::Html<String>> {
    let response = render_content("Thanks for stopping by")?;

    Ok(content::Html(response))
}

#[rocket::launch]
pub fn serve() -> _ {
    rocket::build()
        .mount("/", routes![get_root])
        .mount("/spacenews", routes![get_spacenews])
        .mount("/hackernews", routes![get_hackernews])
        .mount("/nrk", routes![get_nrk])
        .mount("/static", FileServer::from(relative!("static")))
}
