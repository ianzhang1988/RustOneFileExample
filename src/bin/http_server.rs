use actix_web::{web, middleware, App, HttpRequest, HttpServer, Responder, HttpResponse, body::{Body}, web::{Bytes, post, Query} };

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Person {
    age: u8,
    id: u32,
}

async fn handler(bytes: Bytes, query: Query<Person>) -> impl Responder {
    let name = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish())?;
    Ok::<String, HttpResponse>(format!("Hello, {}!\nYou are user #{} and are {} years old.\n", name, query.id, query.age))
}

async fn handler_json(bytes: Bytes) -> impl Responder {
    let json_data = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish())?;

    Ok::<String, HttpResponse>(format!("JSON data, {}!\n"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/name", post().to(handler))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}