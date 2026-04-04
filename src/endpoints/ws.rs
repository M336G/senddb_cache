use axum::{extract::{State, WebSocketUpgrade, ws::Message}, response::Response};
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, db};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WsRequest {
    Status,
    Stats,
    Level { ids: Vec<u32> }
}

pub async fn handle_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |mut socket| async move {
        while let Some(Ok(msg)) = socket.recv().await {
            let Message::Text(text) = msg else { continue };

            let response = match serde_json::from_str::<WsRequest>(&text) {
                Ok(WsRequest::Status) => {
                    json!({ "error": null })
                },

                Ok(WsRequest::Stats) => {
                    json!({
                        "error": null,
                        "stats": {
                            "version": env!("CARGO_PKG_VERSION"),
                            "cached": {
                                "sent": db::get_total_sent_levels(&state.connection).await,
                                "not_sent": state.not_sent.len()
                            }
                        }
                    })
                },

                Ok(WsRequest::Level { ids }) => {
                    if ids.is_empty() || ids.iter().any(|&id| id == 0) {
                        json!({ "error": "Invalid level ID(s)", "levels": null })
                    } else if ids.len() > 50 {
                        json!({ "error": "You cannot check more than 50 levels at a time!", "levels": null })
                    } else {
                        let mut levels = serde_json::Map::new();

                        for id in ids {
                            // Check if the permanent cache (for sent levels) contains the ID
                            let sent = if db::is_level_sent(&state.connection, id).await {
                                true
                            
                            // Or if the temporary cache (for not sent levels) contains it
                            } else if state.not_sent.contains_key(&id) {
                                false
                            
                            // If neither do, check its state via SendDB
                            } else {
                                let url = format!("{}{}", state.api_endpoint_url, id);
                                match state.client.get(&url).send().await {
                                    Ok(response) => match response.json::<serde_json::Value>().await {
                                        Ok(body) => {
                                            // If the sends object isn't empty then it's sent
                                            let has_sends = body["sends"]
                                                .as_array()
                                                .map(|array| !array.is_empty())
                                                .unwrap_or(false);

                                            // If it's sent add it to the permanent cache
                                            if has_sends {
                                                db::add_sent_level(&state.connection, id).await;
                                                state.not_sent.remove(&id);

                                            // If it's not still add it to the temporary cache
                                            } else {
                                                let timestamp = std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_secs() as i64;
                                                state.not_sent.insert(id, timestamp);
                                            }

                                            has_sends
                                        },
                                        
                                        Err(error) => {
                                            eprintln!("Empty body error (ws): {error:?}");
                                            levels.insert(id.to_string(), json!({ "error": "Unknown Error", "sent": null }));
                                            continue;
                                        }
                                    },

                                    Err(error) => {
                                        eprintln!("Response error (ws) {id}: {error:?}");
                                        levels.insert(id.to_string(), json!({ "error": "Unknown Error", "sent": null }));
                                        continue;
                                    }
                                }
                            };

                            levels.insert(id.to_string(), json!({ "error": null, "sent": sent }));
                        }

                        json!({ "error": null, "levels": levels })
                    }
                },

                Err(_) => json!({ "error": "Invalid request", "type": null })
            };

            if socket
                .send(Message::Text(response.to_string().into()))
                .await
                .is_err()
            {
                break;
            }
        }
    })
}