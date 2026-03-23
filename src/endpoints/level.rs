use axum::{Json, extract::{Path, State}, response::{IntoResponse, Response}};
use axum::http::StatusCode;
use serde_json::json;

use crate::{AppState, db};

#[utoipa::path(
    get,
    path = "/level/{id}",
    summary = "Check if a level has been sent",
    description = "Check whether a Geometry Dash level has been sent or not",
    tag = "Levels",
    params(
        ("id" = u32, description = "A level ID to check")
    ),
    responses(
        (status = 200, description = "Level sent state successfully retrieved",
            body = serde_json::Value,
            example = json!({
                "error": null,
                "sent": true
            })
        ),
        (status = 400, description = "Invalid level ID",
            body = serde_json::Value,
            example = json!({
                "error": "Invalid level ID",
                "sent": null
            })
        ),
        (status = 500, description = "Internal server error",
            body = serde_json::Value,
            example = json!({
                "error": "Unknown Error",
                "sent": null
            })
        )
    )
)]

// Check if a level has been sent or not
pub async fn check_level(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    // If the level level_id isn't an integer or is less than/equal to 0, return 400
    let level_id: u32 = match id.trim().parse() {
        Ok(level_id) if level_id > 0 => level_id,
        _ => return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid level ID", "sent": null })),
        ).into_response(),
    };

    // If the permanent cache for sent levels has the level, return true directly
    if db::is_level_sent(&state.connection, level_id).await {
        return (
            StatusCode::OK,
            state.sent_cache_headers,
            Json(json!({ "error": null, "sent": true })),
        ).into_response();
    }

    // If the temporary cache for not sent levels has the level, return false directly
    if state.not_sent.contains_key(&level_id) {
        return (
            StatusCode::OK,
            state.not_sent_cache_headers,
            Json(json!({ "error": null, "sent": false })),
        ).into_response();
    }

    // Make a request to SendDB to know if the level's sent or not
    let url: String = format!("{}{}", state.api_endpoint_url, level_id);
    match state.client.get(&url).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(body) => {
                // If the sends object isn't empty then it's sent
                let has_sends = body["sends"]
                    .as_array()
                    .map(|array| !array.is_empty())
                    .unwrap_or(false);

                if has_sends {
                    // Cache the level permanently
                    db::add_sent_level(&state.connection, level_id).await;
                    state.not_sent.remove(&level_id);

                    return (
                        StatusCode::OK,
                        state.sent_cache_headers,
                        Json(json!({ "error": null, "sent": true })),
                    ).into_response();
                } else {
                    // Cache the level temporarily
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;
                    state.not_sent.insert(level_id, timestamp);

                    return (
                        StatusCode::OK,
                        state.not_sent_cache_headers,
                        Json(json!({ "error": null, "sent": false })),
                    ).into_response();
                }
            }
            Err(error) => {
                eprintln!("Empty body error: {error}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Unknown Error", "sent": null })),
                ).into_response();
            }
        },
        Err(error) => {
            eprintln!("Response error: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Unknown Error", "sent": null })),
            ).into_response();
        }
    }
}
