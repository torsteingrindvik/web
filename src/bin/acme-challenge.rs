#![feature(proc_macro_hygiene, decl_macro)]
// TODO: Remove proc_macro_hygiene at some point. See https://github.com/rust-lang/rust/issues/54727
// TODO: Remove decl_macro. See https://github.com/rust-lang/rust/issues/39412

use anyhow::Result;
use rocket::{
    config::{Config, Environment},
    get, routes,
};

#[get("/<foo>")]
fn acme_challenge(foo: String) -> String {
    match foo.as_ref() {
        env!("ACME_ENDPOINT") => env!("ACME_DATA").to_string(),
        _ => format!("Well I certainly did not expect THIS request: '{}'", foo),
    }
}

/// Custom binary only used to serve ACME challenges
fn main() -> Result<()> {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(80)
        .finalize()?;

    rocket::custom(config)
        .mount("/.well-known/acme-challenge/", routes![acme_challenge])
        .launch();

    // Will never reach this point
    Ok(())
}
