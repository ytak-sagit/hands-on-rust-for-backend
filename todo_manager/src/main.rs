use actix_web::{get, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_hello).service(get_hello_name))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/hello")]
async fn get_hello() -> String {
    "Hello, world!".to_string()
}

#[get("/hello/{name}")]
async fn get_hello_name(name: web::Path<String>) -> String {
    format!("Hello, {name}!")
}
