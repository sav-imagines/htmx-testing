use axum::{extract::State, response::Html};

use crate::AppState;

#[axum::debug_handler]
pub async fn meow(State(state): State<AppState>) -> Result<(), ()> {
    let mut counter = state.data.lock().expect("Mutex was poisoned");
    *counter += 1;
    state.data_channel.0.send_replace(*counter);
    println!("{counter}");
    Ok(())
}

#[axum::debug_handler]
pub async fn reset(State(state): State<AppState>) -> Result<(), ()> {
    let mut counter = state.data.lock().expect("Mutex was poisoned");
    *counter = 0;
    state.data_channel.0.send_replace(*counter);
    println!("{counter}");
    Ok(())
}

pub async fn home(State(state): State<AppState>) -> axum::response::Html<String> {
    let counter = state.data.lock().expect("Mutex was poisoned");
    Html(homepage(format!(include_str!("../static/meow_text.html"), *counter)).to_owned())
}

fn homepage(count: String) -> String {
    format!(include_str!("../static/index.html"), count)
}
