use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/echo", post(echo));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Rust server!"
}

async fn hello() -> Json<Greeting> {
    Json(Greeting {
        message: "Hello".to_string(),
    })
}

async fn echo(Json(payload): Json<Greeting>) -> Json<Greeting> {
    Json(payload)
}

#[derive(Serialize, Deserialize)]
struct Greeting {
    message: String,
}
