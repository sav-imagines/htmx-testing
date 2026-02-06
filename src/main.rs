use axum::{Router, routing::get, serve};

mod pages;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(pages::home::home))
        .route("/meow/{num}", get(pages::home::meow));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
