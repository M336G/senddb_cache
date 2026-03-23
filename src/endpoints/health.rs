use axum::{extract::State, response::{IntoResponse, Response}};
use rand::RngExt;
use reqwest::StatusCode;

use crate::AppState;

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
pub async fn health_check(State(state): State<AppState>) -> Response {
    if rand::rng().random_range(1..=100) == 1 {
        return (StatusCode::OK, [("Content-Type", "image/webp")], state.soggy_image.as_ref().clone()).into_response();
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