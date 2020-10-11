#![feature(proc_macro_hygiene, decl_macro)]
// TODO: Remove proc_macro_hygiene at some point. See https://github.com/rust-lang/rust/issues/54727
// TODO: Remove decl_macro. See https://github.com/rust-lang/rust/issues/39412

use anyhow::{anyhow, Result};
use md_to_html::markdown_to_html;
use rocket::{get, post, response::content::Html, response::Redirect, routes, Data, State};
use rocket_contrib::serve::StaticFiles;
use std::{fs, sync::Mutex};

mod db;
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
fn root() -> Redirect {
    Redirect::to("/index")
}

#[get("/<document>")]
fn markdown(document: String) -> Result<Html<String>> {
    dbg!(&document);

    let markdown_file_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/md");
    let markdown = &fs::read_to_string(&format!("{}/{}.md", markdown_file_dir, &document))?;

    let html = markdown_to_html(markdown, &document)?;

    Ok(Html(html))
}

// Note that format = "plain" means other Content-Type headers
// will result in a 404 (i.e. it has to be text/plain).
// This makes it more obvious that we expect a raw markdown document.
#[post("/<url>", format = "plain", data = "<data>")]
fn upload(url: String, data: Data, db: State<DbConn>) -> Result<Redirect> {
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
fn stored_document(slug: String, db: State<DbConn>) -> Result<Html<String>> {
    dbg!(&slug);

    let document = match db.lock() {
        Ok(locked) => locked.get(slug),
        Err(e) => Err(anyhow!("Lock error: {:?}", e.to_string())),
    }?;

    Ok(Html(document.html))
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
        .mount("/static", static_route)
        .manage(Mutex::new(db_conn))
        .launch();

    // Will never reach this point
    Ok(())
}
