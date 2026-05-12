use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;
use uuid::Uuid;
use crate::models::*;
use crate::task_manager::TaskManager;

pub async fn create_train(
    State(tm): State<TaskManager>,
    Json(req): Json<CreateTrainRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    info!("POST /train | algorithm={:?} regression_fn={:?} genetic_fn={:?} lr={}", req.algorithm, req.regression_fn, req.genetic_fn, req.learning_rate);
    let id = tm.create(req).map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    info!("POST /train | created task {}", id);
    Ok(Json(serde_json::json!({ "task_id": id })))
}

pub async fn train_step(
    State(tm): State<TaskManager>,
    Path(task_id): Path<Uuid>,
    Json(req): Json<StepRequest>,
) -> Result<Json<TrainStatusResponse>, (axum::http::StatusCode, String)> {
    info!("POST /train/{}/step | epochs={}", task_id, req.epochs);
    let start = std::time::Instant::now();
    tm.step(task_id, req.epochs).map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;
    let elapsed = start.elapsed();
    let status = tm.get_status(task_id).map_err(|e| (axum::http::StatusCode::NOT_FOUND, e))?;
    info!("POST /train/{}/step | done in {:.2?} | total_epochs={} loss={:?}", task_id, elapsed, status.total_epochs, status.best_fitness);
    Ok(Json(status))
}

pub async fn train_status(
    State(tm): State<TaskManager>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<TrainStatusResponse>, (axum::http::StatusCode, String)> {
    info!("GET /train/{}/status", task_id);
    let status = tm.get_status(task_id).map_err(|e| (axum::http::StatusCode::NOT_FOUND, e))?;
    Ok(Json(status))
}

pub async fn train_stop(
    State(tm): State<TaskManager>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    info!("POST /train/{}/stop", task_id);
    tm.stop(task_id).map_err(|e| (axum::http::StatusCode::NOT_FOUND, e))?;
    Ok(Json(serde_json::json!({ "status": "stopped" })))
}
