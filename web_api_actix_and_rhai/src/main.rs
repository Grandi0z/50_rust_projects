use actix_web::{HttpServer, get, App, web::Path, Responder};
use rhai::Engine;
use std::io::Result;

#[get("/multiply/{num1}/{num2}")]


#[actix_web::main]
fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(multiply)
        .service(add)
        .service(remove)
        .service(divide)
    })
    .bind(("127.0.0.1", 8080))?
    .unwrap()
    .run()
    .await?
}
