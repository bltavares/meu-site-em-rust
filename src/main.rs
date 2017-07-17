#[macro_use]
extern crate nickel;
use nickel::{Nickel, HttpRouter};

const INDEX: &str = include_str!("index.html");

fn main() {
    let mut server = Nickel::new();
    server.get("/", middleware!(INDEX));

    let port = std::env::var("PORT").unwrap_or("3000".into());
    let url = format!("0.0.0.0:{port}", port = port);

    println!("On {}", url);
    server.listen(url)
          .expect("Não conseguimos iniciar o servidor");
}
