use actix_web::{post, web, App, HttpResponse, HttpServer, ResponseError};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
struct Week {
    text: String,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed to SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

#[post("/")]
async fn set_schedule(params: web::Form<Week>) -> Result<HttpResponse, MyError> {
    println!("{}", params.text);
    Ok(HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body(format!("Bot received this data: {}", params.text)))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(set_schedule))
        .bind("localhost:3000")?
        .run()
        .await?;
    Ok(())
}
