use axum::{
    extract::{Request, State},
    middleware::{from_fn, Next, from_fn_with_state},
    response::{IntoResponse, Response},
    routing::get,
    Router
};
use sqlx::PgPool;
use std::sync::Arc;


pub struct AppState {
    pub db_pool: PgPool
}

pub async fn provide_pool() -> PgPool {
    PgPool::connect(
        "postgres://nucccc:password@localhost:5432/hello1"
    )
    .await
    .expect("failed to connect to postgres")
}

async fn hello() -> impl IntoResponse {
    "hello"
}

async fn has_state(
    State(app_state): State<Arc<AppState>>
) -> impl IntoResponse {
    "has_state"
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

async fn wannabe_middleware_with_state(
    State(app_state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    println!("passing through middleware with state");
    next.run(request).await
}

#[tokio::main]
async fn main() {
    println!("starting app...");

    let db_pool = provide_pool().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await
        .unwrap();

    let state = Arc::new(AppState{
        db_pool: db_pool.clone()
    });

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/has_state", get(has_state))
        .layer(from_fn(wannabe_middleware))
        .layer(from_fn_with_state(state.clone(), wannabe_middleware_with_state))
        .route("/world", get(world))
        .with_state(state);

    let server = axum::serve(listener, app);

    server.await.unwrap();
}
