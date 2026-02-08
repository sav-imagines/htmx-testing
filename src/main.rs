use std::sync::{Arc, Mutex};
use crate::{api::meows::{handle_meows_ws, meows}, pages::home::{home, meow}};

use axum::{
    Router,
    routing::{any, get, post},
    serve,
};

mod pages;
mod api;

#[tokio::main]
async fn main() {
    let state = AppState {
        data: Arc::new(Mutex::new(0u64)),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/meow", post(meow))
        .route("/meows", get(meows))
        .route("/meow_ws", any(handle_meows_ws))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<u64>>,
}
