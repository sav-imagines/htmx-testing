use std::{thread::sleep, time::Duration};

use axum::{extract::{State, WebSocketUpgrade, ws::WebSocket}, response::{Html, Response}};

use crate::AppState;

#[axum::debug_handler]
pub async fn meows(State(state): State<AppState>) -> axum::response::Html<String> {
    let counter = state.data.lock().expect("Mutex was poisoned");
    Html(format!(include_str!("../static/meow_text.html"), *counter))
}

#[axum::debug_handler]
pub async fn handle_meows_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| meows_update_socket(socket, state))
}

async fn meows_update_socket(mut socket: WebSocket, state: AppState) {
    let mut last_count = get_count(&state);
    // TODO: implement passive waiting
    loop {
        let current_count = get_count(&state);
        if last_count != current_count {
            let content = format!(include_str!("../static/meow_button.html"), current_count);
            let result = socket.send(axum::extract::ws::Message::text(content)).await;
            if result.is_err() {
                panic!("{}", result.err().unwrap());
            }
            last_count = current_count;
        }
        sleep(Duration::from_millis(10));
    }
}

fn get_count(state: &AppState) -> u64 {
    *state.data.lock().expect("Mutex was poisoned")
}
