#![feature(proc_macro_hygiene, decl_macro)]
// TODO: Remove proc_macro_hygiene at some point. See https://github.com/rust-lang/rust/issues/54727
// TODO: Remove decl_macro. See https://github.com/rust-lang/rust/issues/39412

use anyhow::Result;
use md_to_html::markdown_to_html;
use rocket::{get, response::content::Html, response::Redirect, routes};
use rocket_contrib::serve::StaticFiles;
use std::fs;

mod md_to_html;

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    // uri! macro has resolve issues in this version
    // Redirect::to(uri!(index))
    Redirect::to("/static/favicon.ico")
}

#[get("/")]
fn root() -> Redirect {
    // uri! macro has resolve issues in this version
    // Redirect::to(uri!(index))
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

fn main() -> Result<()> {
    rocket::ignite()
        .mount("/", routes![root, markdown, favicon])
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();

    // Will never reach this point
    Ok(())
}
