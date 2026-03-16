use axum::response::{IntoResponse, Response};
use rand::RngExt;
use reqwest::StatusCode;

// Check if the server's running
#[utoipa::path(
    get,
    path = "/",
    summary = "Check the server's running state",
    description = "Check if the server is up and running",
    tag = "Health",
    responses(
        (status = 200, description = "The server is up and running",
            body = String,
            example = "
_//     _//           _// _//          _//
_//     _//           _// _//          _//
_//     _//   _//     _// _//   _//    _//
_////// _// _/   _//  _// _// _//  _// _/ 
_//     _//_///// _// _// _//_//    _//_/ 
_//     _//_/         _// _// _//  _//    
_//     _//  _////   _///_///   _//    _//
        "
        )
    )
)]

// With a 1% chance of being soggied!!
pub async fn health_check() -> Response {
    if rand::rng().random_range(1..=100) == 1 {
        let soggy: Vec<u8> = tokio::fs::read("assets/soggy.webp").await.unwrap();
        return (StatusCode::OK, [("Content-Type", "image/webp")], soggy).into_response();
    } else {
        return (StatusCode::OK, "
_//     _//           _// _//          _//
_//     _//           _// _//          _//
_//     _//   _//     _// _//   _//    _//
_////// _// _/   _//  _// _// _//  _// _/ 
_//     _//_///// _// _// _//_//    _//_/ 
_//     _//_/         _// _// _//  _//    
_//     _//  _////   _///_///   _//    _//
        ").into_response();
    }
}