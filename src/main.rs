#[macro_use]
extern crate nickel;
use nickel::{Nickel, QueryString, HttpRouter};

const INDEX: &str = include_str!("index.html");

fn main() {
    let mut server = Nickel::new();

    server.get(
        "/",
        middleware!{ |request|
                      let name = request.query()
                                        .get("name")
                                        .unwrap_or("TDC");

                      INDEX.replace("TDC", name)
        },
    );

    let port = std::env::var("PORT").unwrap_or("3000".into());
    let url = format!("0.0.0.0:{port}", port = port);
    println!("On {}", url);

    server
        .listen(url)
        .expect("Não conseguimos iniciar o servidor");
}
