extern crate iron;

use iron::prelude::*;
use iron::headers::ContentType;
use iron::modifiers;
use iron::status;

const INDEX: &str = include_str!("index.html");

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(
        (status::Ok, modifiers::Header(ContentType::html()), INDEX),
    ))
}

fn main() {
    let port = std::env::var("PORT").unwrap_or("3000".into());
    let url = format!("0.0.0.0:{port}", port = port);

    Iron::new(hello_world)
        .http(&url)
        .expect("Não conseguimos iniciar o servidor");

    println!("On {}", url);
}
