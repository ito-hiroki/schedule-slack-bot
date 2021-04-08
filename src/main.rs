use actix_web::{post, web, App, HttpResponse, HttpServer, ResponseError};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct SlackPayload {
    token: String,
    team_id: String,
    team_domain: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: String,
    response_url: String,
    trigger_id: String,
    api_app_id: String,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed to SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

#[post("/thisweek")]
async fn set_schedule(params: web::Form<SlackPayload>) -> Result<HttpResponse, MyError> {
    println!("{:?}", params);
    Ok(HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body(format!("Bot received this data: {}", params.text)))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("schedule.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schedule (
            id  TEXT PRIMARY KEY,
            thisweek TEXT NOT NULL,
            nextweel TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table `schedule`.");

    HttpServer::new(move || App::new().service(set_schedule).data(pool.clone()))
        .bind("localhost:3000")?
        .run()
        .await?;
    Ok(())
}
