use std::sync::{Arc, Mutex};

use axum::{
    Router,
    routing::{get, post},
    serve,
};

mod pages;

#[tokio::main]
async fn main() {
    let state = AppState {
        data: Arc::new(Mutex::new(0u64)),
    };

    let app = Router::new()
        .route("/", get(pages::home::home))
        .route("/meow", post(pages::home::meow))
        .route("/meows", get(pages::home::meows))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<u64>>,
}
