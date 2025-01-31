use axum::{
    Router,
    response::IntoResponse,
    routing::get,
};

async fn hello() -> impl IntoResponse {
    "hello"
}

#[tokio::main]
async fn main() {
    println!("starting app...");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await
        .unwrap();

    let app = Router::new()
        .route("/hello", get(hello));

    let server = axum::serve(listener, app);

    server.await.unwrap();
}
