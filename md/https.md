# Rocket+HTTPS setup

## Setup
Install `certbot`.
I have it in my `$PATH`.

## Dry-run
Run:

```bash
certbot certonly --manual --dry-run
```

We do a dry run to check that things work without actually "spending" an attempt towards our Let's Encrypt rate limit.

Certbot will tell you to create a file containing data `FOO`, available at domain `yourdomain.tld/.well-known/acme-challenge/BAR`,
where `FOO` and `BAR` are relatively long strings of data.

Don't continue until the next section is done.

## Rocket

The default setup uses address `localhost` and port `8000`, which will not work.
Therefore we use this configuration:

```toml
# Rocket.toml
[development]
address = "0.0.0.0"
port = 80
log = "normal"
```

Certbot will have asked you to make some data available at some endpoint.
Let's call the endpoint `FOO`, and the data `BAR`.

```rust
// src/main.rs
#![feature(decl_macro)]

use rocket::{get, routes};

#[get("/<endpoint>")]
fn acme_challenge(endpoint: String) -> String {
    match endpoint.as_ref() {
        "FOO" => "BAR".to_string(),
        _ => format!("Well I certainly did not expect THIS request: '{}'", endpoint),
    }
}

fn main() {
    rocket::ignite()
        .mount("/.well-known/acme-challenge/", routes![acme_challenge])
        .launch();
}
```

Where the match matches the actual `FOO` path specified by certbot, and `BAR` is the data the certbot asks for.


Spin up the project: `cargo run`.
Naturally `rocket` should be in the `Cargo.toml` dependency list, and (as of Q4 2020) a nightly compiler is required.

## Validate project

Try navigating to your server in your browser:

`yourdomain.tld/.well-known/acme-challenge/FOO`, should make `BAR` appear in your browser.

Anything else should return the other match message.

Now let certbot continue, if it works it will let you know:

> Waiting for verification...
>
> Cleaning up challenges
> 
> IMPORTANT NOTES:
>  - The dry run was successful.

## Generate certs

```bash
certbot certonly --manual
```

This will give you a new `FOO` and `BAR`, so replace those in the project when they are given.

Now it should work, and we get the message:

> Congratulations! Your certificate and chain have been saved at:
> /etc/letsencrypt/live/yourdomain.tld/fullchain.pem
>
> Your key file has been saved at:
> /etc/letsencrypt/live/yourdomain.tld/privkey.pem
>
> Your cert will expire on 20XX-YY-ZZ. To obtain a new or tweaked
> version of this certificate in the future, simply run certbot
> again. To non-interactively renew *all* of your certificates, run
> "certbot renew"

## Use certs

We need to tell our Rocket project to use TLS:

```toml
# Cargo.toml

# ...

[dependencies.rocket]
version = "0.4"
features = ["tls"]
```

Now we point to the files certbot gave us in the Rocket configuration:

```toml
# Rocket.toml
[development]
address = "0.0.0.0"
port = 443
log = "normal"

[global.tls]
certs = "private/cert.pem"
key = "private/key.pem"
```

The `private` folder is parallel to the project `src`.

Note that we changed our ports.

Let's make the cert and key available (assuming we currently are at the top level of our project):

```bash
ln -s /etc/letsencrypt/live/yourdomain.tld/cert.pem private/cert.pem
ln -s /etc/letsencrypt/live/yourdomain.tld/privkey.pem private/key.pem

# To see that our generated links are pointing to where we expect:
ls -l private
```

## Run

We're done! Simply: `cargo run`.

## Renew certs

TODO when first certs expire (2021-01).