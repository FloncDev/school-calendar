use axum::{http::StatusCode, routing::get, Router};
use edulink::{EduLink, TokenJSON};
use std::fs;

pub mod edulink;
pub mod models;

#[tokio::main]
async fn main() {
    // Check auth token
    let session = match fs::read_to_string("./.token.json") {
        Ok(data) => match serde_json::from_str::<TokenJSON>(&data) {
            Ok(json) => EduLink::from_json(json).await,
            Err(_) => EduLink::new().await,
        },
        Err(_) => EduLink::new().await,
    };

    println!("{:#?}", session);

    let app = Router::new().route("/", get(read_root));

    println!("Running app on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn read_root() -> StatusCode {
    StatusCode::OK
}
