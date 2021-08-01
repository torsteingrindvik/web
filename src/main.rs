use hiho_templates::server;

#[rocket::launch]
fn run() -> _ {
    server::serve()
}
