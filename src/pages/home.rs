use axum::{extract::State, response::Html};

use crate::AppState;

#[axum::debug_handler]
pub async fn meow(State(state): State<AppState>) -> axum::response::Html<String> {
    let mut counter = state.data.lock().expect("Mutex was poisoned");
    *counter += 1;
    println!("{counter}");
    Html(format!(include_str!("meow_text.html"), counter))
}

#[axum::debug_handler]
pub async fn meows(State(state): State<AppState>) -> axum::response::Html<String> {
    Html(get_count_text(state))
}

pub async fn home(State(state): State<AppState>) -> axum::response::Html<String> {
    Html(homepage(get_count_text(state)).to_owned())
}

fn get_count_text(state: AppState) -> String {
    let counter = state.data.lock().expect("Mutex was poisoned");
    format!(include_str!("meow_text.html"), *counter)
}

fn homepage(count: String) -> String {
    format!(include_str!("index.html"), count)
}
