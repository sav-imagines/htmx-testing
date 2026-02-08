use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};

use crate::AppState;

#[axum::debug_handler]
pub async fn handle_meows_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| meows_update_socket(socket, state))
}

async fn meows_update_socket(mut socket: WebSocket, state: AppState) {
    let mut receiver = state.data_channel.1.clone();
    // TODO: implement passive waiting
    loop {
        let content = format!(
            include_str!("../static/meow_button.html"),
            *receiver.borrow_and_update()
        );
        let result = socket.send(axum::extract::ws::Message::text(content)).await;
        if result.is_err() {
            // it was abandoned
            return;
        }
        match receiver.changed().await {
            Err(_) => return,
            _ => continue,
        }
    }
}
