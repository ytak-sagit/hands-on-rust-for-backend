use actix_web::{get, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_hello_name).service(get_todo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[get("/hello/{name}")]
async fn get_hello_name(name: web::Path<String>) -> HttpResponse {
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    hello.to_response()
}

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {}

#[get("/")]
async fn get_todo() -> HttpResponse {
    let todo = TodoTemplate {};
    todo.to_response()
}
