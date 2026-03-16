use axum::{Json, extract::State};
use serde_json::json;

use crate::{AppState, db};

pub async fn get_stats(State(state): State<AppState>) -> Json<serde_json::Value> {
    return Json(json!({
        "error": null,
        "stats": {
            "cached": {
                "sent": db::get_total_sent_levels(&state.connection).await,
                "not_sent": state.not_sent.lock().await.len()
            }
        }
    }));
}