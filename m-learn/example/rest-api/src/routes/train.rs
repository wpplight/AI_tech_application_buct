use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;
use uuid::Uuid;
use crate::models::*;
use crate::task_manager::TaskManager;

pub async fn list_tasks(
    State(tm): State<TaskManager>,
) -> Json<TaskListResponse> {
    info!("GET /train");
    let tasks = tm.list_tasks();
    info!("GET /train | returning {} tasks", tasks.len());
    Json(TaskListResponse { tasks })
}

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

pub async fn train_history(
    State(tm): State<TaskManager>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<TrainingHistory>, (axum::http::StatusCode, String)> {
    info!("GET /train/{}/history", task_id);
    let history = tm.get_history(task_id).map_err(|e| (axum::http::StatusCode::NOT_FOUND, e))?;
    Ok(Json(history))
}

pub async fn train_recall(
    State(tm): State<TaskManager>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<RecallResponse>, (axum::http::StatusCode, String)> {
    info!("GET /train/{}/recall", task_id);
    let recall = tm.get_recall(task_id).map_err(|e| (axum::http::StatusCode::NOT_FOUND, e))?;
    Ok(Json(recall))
}
