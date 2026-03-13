use axum::response::{IntoResponse, Redirect, Response};
use rand::RngExt;
use reqwest::StatusCode;

// Redirect to https://senddb.dev/ if the user tries accessing the API's root (or have
// a 1% chance to get soggied)
pub async fn root() -> Response {
    if rand::rng().random_range(1..=100) == 1 {
        let soggy: Vec<u8> = tokio::fs::read("assets/soggy.webp").await.unwrap();
        return (StatusCode::OK, [("Content-Type", "image/webp")], soggy).into_response();
    } else {
        Redirect::to("https://senddb.dev/").into_response()
    }
}