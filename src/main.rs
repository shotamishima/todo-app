use actix_web::{get, App, HttpResponse, HttpServer, Responder, ResponseError};
use askama::Template;
use thiserror::Error;

// htmlに埋め込むためデータの構造体
struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for MyError {}


#[get("/")]
async fn index() -> Result<impl Responder, MyError> {
    // Ok(HttpResponse::Ok().body("Hello world!"))
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });
    let html = IndexTemplate {entries};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
       .content_type("text/html")
       .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
