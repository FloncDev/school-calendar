use axum::{http::StatusCode, routing::get, Router};
use edulink::EduLink;
use std::fs;

pub mod edulink;
pub mod models;

#[tokio::main]
async fn main() {
    // Check auth token
    match fs::read_to_string("./.token.json") {
        Ok(data) => match serde_json::from_str::<serde_json::Value>(&data) {
            Ok(json) => {
                println!("{}", json);
            }
            Err(_) => {
                EduLink::new();
            }
        },
        Err(_) => {
            EduLink::new();
        }
    }

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
