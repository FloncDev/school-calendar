use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use edulink::{EduLink, TokenJSON};
use icalendar::{Calendar, Component, Event, EventLike};
use std::fs;

pub mod edulink;
pub mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(read_root))
        .route("/homeworks", get(get_homeworks));

    println!("Running app on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_edulink() -> EduLink {
    match fs::read_to_string("./.token.json") {
        Ok(data) => match serde_json::from_str::<TokenJSON>(&data) {
            Ok(json) => EduLink::from_json(json).await,
            Err(_) => EduLink::new().await,
        },
        Err(_) => EduLink::new().await,
    }
}

async fn read_root() -> StatusCode {
    StatusCode::OK
}

async fn get_homeworks() -> impl IntoResponse {
    let mut session = get_edulink().await;

    let homeworks = session.get_homework().await;
    let mut calendar = Calendar::new();

    for homework in homeworks {
        calendar.push(
            Event::new()
                .summary(format!("{} Homework", homework.subject).as_str())
                .description(
                    format!("{}\n\nSet by: {}", homework.activity, homework.set_by).as_str(),
                )
                .all_day(homework.due)
                .done(),
        );
    }

    format!("{}", calendar)
}
