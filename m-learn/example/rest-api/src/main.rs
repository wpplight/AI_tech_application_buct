mod models;
mod task_manager;
mod routes;

use axum::{routing::{get, post}, Router};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing::info;
use task_manager::TaskManager;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rest_api=debug,tower_http=debug".into()),
        )
        .init();

    let tm = TaskManager::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/algorithms", get(algorithms))
        .route("/train", get(routes::train::list_tasks).post(routes::train::create_train))
        .route("/train/{task_id}/step", post(routes::train::train_step))
        .route("/train/{task_id}/status", get(routes::train::train_status))
        .route("/train/{task_id}/stop", post(routes::train::train_stop))
        .route("/train/{task_id}/history", get(routes::train::train_history))
        .route("/train/{task_id}/recall", get(routes::train::train_recall))
        .route("/inference/{task_id}", get(routes::inference::inference))
        .with_state(tm)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    info!("m-learn REST API listening on 0.0.0.0:8082");
    axum::serve(listener, app).await.unwrap();
}

async fn algorithms() -> axum::Json<serde_json::Value> {
    info!("GET /algorithms");
    axum::Json(serde_json::json!({
        "regression": ["linear", "quadratic", "sinusoidal"],
        "genetic": ["rastrigin_variant", "ackley"]
    }))
}
