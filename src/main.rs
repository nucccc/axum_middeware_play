use axum::{
    extract::Request, middleware::{from_fn, Next}, response::{IntoResponse, Response}, routing::get, Router
};

async fn hello() -> impl IntoResponse {
    "hello"
}

async fn world() -> impl IntoResponse {
    "world"
}

async fn wannabe_middleware(
    request: Request,
    next: Next
) -> Response {
    println!("passing through middleware");
    next.run(request).await
}

#[tokio::main]
async fn main() {
    println!("starting app...");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await
        .unwrap();

    let app = Router::new()
        .route("/hello", get(hello))
        .layer(from_fn(wannabe_middleware))
        .route("/world", get(world));

    let server = axum::serve(listener, app);

    server.await.unwrap();
}
