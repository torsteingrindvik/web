# Rust-based home page

This website is used as a simple hobby project,
in order to test various Rust related technologies.

## Server

[Rocket](https://docs.rs/rocket/0.5.0-rc.1/rocket/index.html) is used to serve the website.
As of now, version 0.5.0-rc1 is used since it brings async support.

Some features employed:

* Sync requests
* Async requests
* Cookies
* Error handling via [color-eyre](TODO)

## HTML

Instead of using the "usual" HTML templating libraries, [stpl](https://docs.rs/stpl/0.5.0/stpl/) is used.
This is done to get a feel of writing native Rust instead of HTML.

It would be fun to try a 100% Rust based solution, but nothing similar is found for CSS yet.

## Cards

Mention:

* General API
* Reqwest