#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
// TODO: Remove proc_macro_hygiene at some point. See https://github.com/rust-lang/rust/issues/54727
// TODO: Remove decl_macro. See https://github.com/rust-lang/rust/issues/39412

use anyhow::{anyhow, Result};
use context::{Card, CardsContext, HtmlContext, LinkableContent};
use helpers::html_helper;
use md_to_html::markdown_to_html;
use rocket::{catch, catchers, get, post, response::Redirect, routes, Data, Request, State};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::{fs, sync::Mutex};

mod context;
mod db;
mod helpers;
mod md_to_html;
mod util;

type DbConn = Mutex<db::DbConn>;

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    // uri! macro has resolve issues in this version
    // Redirect::to(uri!(index))
    Redirect::to("/static/favicon.ico")
}

#[get("/")]
fn root() -> Template {
    Template::render(
        "index",
        &CardsContext {
            cards: vec![
                Card {
                    title: "Web related ⌨️",
                    description: "Web related",
                    image: "autumn_forest",
                    contents: vec![LinkableContent {
                        description: "Setting up Rocket+HTTPS",
                        link: "https",
                    }],
                    content_bg: (1.0, 0.0, 0.0, 0.5),
                },
                Card {
                    title: "Tools 🧊",
                    description: "Let's create something",
                    image: "blue_water",
                    contents: vec![LinkableContent {
                        description: "Create a sharable Markdown render",
                        link: "html/upload",
                    }],
                    content_bg: (0.0, 1.0, 0.0, 0.5),
                },
                Card {
                    title: "Test 🔋",
                    description: "Test related WIP stuff",
                    image: "forest_heart",
                    contents: vec![
                        LinkableContent {
                            description: "TODO 3",
                            link: "invalid",
                        },
                        LinkableContent {
                            description: "TODO 4",
                            link: "invalid",
                        },
                    ],
                    content_bg: (0.0, 0.0, 1.0, 0.5),
                },
            ],
            ..CardsContext::default()
        },
    )
}

/// Serve dynamic Markdown-to-HTML sites wrapped in Handlebars single-card template
#[get("/<document>")]
fn markdown(document: String) -> Result<Template> {
    dbg!(&document);

    let markdown_file_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/md");
    let markdown = &fs::read_to_string(&format!("{}/{}.md", markdown_file_dir, &document))?;

    let html = markdown_to_html(markdown, &document)?;

    Ok(Template::render(
        "html",
        &HtmlContext {
            title: document,
            content: html,
            ..HtmlContext::default()
        },
    ))
}

/// Serve static HTML sites wrapped in Handlebars single-card template
#[get("/<static_html>")]
fn html(static_html: String) -> Result<Template> {
    dbg!(&static_html);

    let html_file_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/html");
    let html = fs::read_to_string(&format!("{}/{}.html", html_file_dir, &static_html))?;

    Ok(Template::render(
        "html",
        &HtmlContext {
            title: static_html,
            content: html,
            ..HtmlContext::default()
        },
    ))
}

// Note that format = "plain" means other Content-Type headers
// will result in a 404 (i.e. it has to be text/plain).
// This makes it more obvious that we expect a raw markdown document.
#[post("/<_url>", format = "plain", data = "<data>")]
fn upload(_url: String, data: Data, db: State<DbConn>) -> Result<Redirect> {
    // TODO: Pre-allocate with limit capacity
    let mut data_buf = vec![];
    if data.stream_to(&mut data_buf)? == 0 {
        return Err(anyhow!("No data uploaded"));
    }

    // The form (see static/upload.html) prepends "message="
    let markdown = String::from_utf8(data_buf)?.replace("message=", "");

    let slug = util::random_name()?;
    dbg!(&slug);

    let html = markdown_to_html(&markdown, &slug)?;

    let document = db::Document {
        slug: slug.clone(),
        html,
    };

    let _ = match db.lock() {
        Ok(locked) => locked.insert(&document),
        Err(e) => Err(anyhow!("Lock error: {:?}", e.to_string())),
    }?;

    Ok(Redirect::to(format!("/md/{}", slug)))
}

/// domain.tld/md/<document>
/// Does a lookup in the db for the given document,
/// and returns the stored HTML if successful.
/// Documents are stored via random "names", e.g.
/// domain.tld/md/coffee-struggle-838
#[get("/<slug>")]
fn stored_document(slug: String, db: State<DbConn>) -> Result<Template> {
    dbg!(&slug);

    let document = match db.lock() {
        Ok(locked) => locked.get(slug.clone()),
        Err(e) => Err(anyhow!("Lock error: {:?}", e.to_string())),
    }?;

    Ok(Template::render(
        "html",
        &HtmlContext {
            title: slug,
            content: document.html,
            ..HtmlContext::default()
        },
    ))
}

#[catch(500)]
fn internal_server_error(_req: &Request<'_>) -> Redirect {
    Redirect::to("/html/error_500")
}

#[catch(404)]
fn not_found(_req: &Request<'_>) -> Redirect {
    Redirect::to("/html/error_404")
}

fn main() -> Result<()> {
    println!("Starting web backend");

    let db_conn = db::init_and_open_db()?;
    println!("Database open");

    let static_route = StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));

    println!("Launching Rocket");
    rocket::ignite()
        .mount("/", routes![root, markdown, favicon])
        .mount("/md", routes![stored_document])
        .mount("/upload", routes![upload])
        .mount("/html", routes![html])
        .mount("/static", static_route)
        .register(catchers![internal_server_error, not_found])
        .manage(Mutex::new(db_conn))
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("html", Box::new(html_helper));
        }))
        .launch();

    // Will never reach this point
    Ok(())
}
