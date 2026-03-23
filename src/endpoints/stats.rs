use axum::{Json, extract::State};
use serde_json::json;

use crate::{AppState, db};

// Get statistics/information about the server
#[utoipa::path(
    get,
    path = "/stats",
    summary = "Get server statistics",
    description = "Get statistics/information about the server",
    tag = "Statistics",
    responses(
        (status = 200, description = "Statistics/information successfully retrieved",
            body = serde_json::Value,
            example = json!({
                "error": null,
                "stats": {
                    "version": "1.2.0",
                    "cached": {
                        "sent": 1234567890,
                        "not_sent": 1234567890
                    }
                }
            })
        )
    )
)]

pub async fn get_stats(State(state): State<AppState>) -> Json<serde_json::Value> {
    return Json(json!({
        "error": null,
        "stats": {
            "version": env!("CARGO_PKG_VERSION"),
            "cached": {
                "sent": db::get_total_sent_levels(&state.connection).await,
                "not_sent": state.not_sent.len()
            }
        }
    }));
}