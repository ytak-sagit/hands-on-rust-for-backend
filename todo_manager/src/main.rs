use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{Pool, Row, Sqlite, SqlitePool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    initialize_db(&pool).await.unwrap();

    // NOTE: サーバーに組み込みたい変数を web::Data に詰め込む
    // NOTE: move を付けることで、クロージャ内からアクセスできるようになる（この場合は get_todo() 等の中から pool にアクセスできる）
    HttpServer::new(move || {
        App::new()
            .service(get_hello_name)
            .service(get_todo)
            .service(to_done_todo)
            .service(create_todo)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn initialize_db(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // tasks テーブル作成
    sqlx::query("CREATE TABLE tasks (task TEXT)")
        .execute(pool)
        .await
        .unwrap();

    // 初期データ挿入
    sqlx::query("INSERT INTO tasks (task) VALUES ('タスク1')")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tasks (task) VALUES ('タスク2')")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tasks (task) VALUES ('タスク3')")
        .execute(pool)
        .await
        .unwrap();

    Ok(())
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
struct TodoTemplate {
    tasks: Vec<String>,
}

#[get("/")]
async fn get_todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    let rows = sqlx::query("SELECT task FROM tasks;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let tasks = rows
        .iter()
        .map(|row| row.get::<String, _>("task"))
        .collect();
    let todo = TodoTemplate { tasks };
    todo.to_response()
}

#[derive(serde::Deserialize)]
struct DoneTask {
    id: String,
}

#[post("/done")]
async fn to_done_todo(pool: web::Data<SqlitePool>, form: web::Form<DoneTask>) -> HttpResponse {
    let done_task = form.into_inner();
    sqlx::query("DELETE FROM tasks WHERE task = ?")
        .bind(done_task.id) // NOTE: 以降、参照しないので、& を付けなくてもコンパイルエラーにならない
        .execute(pool.as_ref())
        .await
        .unwrap();
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[derive(serde::Deserialize)]
struct CreateTask {
    task: String,
}

#[post("/create")]
async fn create_todo(pool: web::Data<SqlitePool>, form: web::Form<CreateTask>) -> HttpResponse {
    let create_task = form.into_inner();
    sqlx::query("INSERT INTO tasks (task) VALUES (?)")
        .bind(create_task.task)
        .execute(pool.as_ref())
        .await
        .unwrap();
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
