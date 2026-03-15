use axum::{Router, http::HeaderMap, routing::get};
use dotenv::dotenv;
use reqwest::{Client, ClientBuilder};
use sqlx::SqlitePool;
use std::{collections::HashMap, env, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

mod db;
mod endpoints;
use crate::endpoints::{check_level, root};

// Stores some "global variables" which will be used across the whole program
#[derive(Clone)]
struct AppState {
    connection: SqlitePool,
    client: Client,
    not_sent: Arc<Mutex<HashMap<i32, i64>>>,
    api_endpoint_url: String,
    sent_cache_headers: HeaderMap,
    not_sent_cache_headers: HeaderMap,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load from .env

    // Initialize all needed variables here
    let server_port: u16 = env::var("PORT")
        .ok()
        .and_then(|port: String| port.parse().ok())
        .unwrap_or(8273);

    let not_sent_expiration: u32 = env::var("NOT_SENT_EXPIRATION")
        .ok()
        .and_then(|expiration: String| expiration.parse().ok())
        .unwrap_or(5);
    println!("Expiration for not sent levels set to {not_sent_expiration} minutes");

    let sent_cache: u32 = env::var("SENT_CACHE")
        .ok()
        .and_then(|time: String| time.parse().ok())
        .unwrap_or(60);
    println!("Sent levels set to be cached for {sent_cache} minutes");

    let not_sent_cache: u32 = env::var("NOT_SENT_CACHE")
        .ok()
        .and_then(|time: String| time.parse().ok())
        .unwrap_or(5);
    println!("Not sent levels set to be cached for {not_sent_cache} minutes");

    let mut sent_cache_headers = HeaderMap::new();
    sent_cache_headers.insert(
        "Cache-Control",
        format!("public, max-age={}", sent_cache * 60)
            .parse()
            .unwrap(),
    );

    let mut not_sent_cache_headers = HeaderMap::new();
    not_sent_cache_headers.insert(
        "Cache-Control",
        format!("public, max-age={}", not_sent_cache * 60)
            .parse()
            .unwrap(),
    );

    let client_builder: ClientBuilder = Client::builder()
        .user_agent(format!("SendDBCache/{}", env!("CARGO_PKG_VERSION")));

    let client: Client = if let Ok(token) = env::var("ENDPOINT_TOKEN") {
        println!("Endpoint token set, using a bearer token for authentication");

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());

        client_builder.default_headers(headers)
    } else {
        client_builder
    }
    .build()
    .unwrap();

    let state: AppState = AppState {
        connection: db::open().await,
        client,
        api_endpoint_url: env::var("api_endpoint_url")
            .unwrap_or_else(|_| "https://api.senddb.dev/api/v1/level/".to_string()),
        not_sent: Arc::new(Mutex::new(HashMap::new())),
        sent_cache_headers,
        not_sent_cache_headers,
    };
    println!("SendDB API's endpoint URL set to {}", state.api_endpoint_url);

    // Routine that'll check for expired not sent levels (so that they are re-checked)
    // every minute
    let state_for_cleanup: AppState = state.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_mins(1)).await;
            let now: i64 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            state_for_cleanup
                .not_sent
                .lock()
                .await
                .retain(|_, timestamp: &mut i64| {
                    now - *timestamp < not_sent_expiration as i64 * 60
                });
        }
    });

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/level/{id}", get(check_level))
        .with_state(state);

    println!("Server running on http://0.0.0.0:{server_port}/");
    axum::serve(
        tokio::net::TcpListener::bind(format!("0.0.0.0:{server_port}"))
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
