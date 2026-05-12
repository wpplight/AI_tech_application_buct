use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;
use uuid::Uuid;
use crate::task_manager::TaskManager;

pub async fn inference(
    State(tm): State<TaskManager>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    info!("GET /inference/{}", task_id);
    let start = std::time::Instant::now();
    let resp = tm.get_inference(task_id).map_err(|e| (axum::http::StatusCode::NOT_FOUND, e))?;
    let elapsed = start.elapsed();
    info!("GET /inference/{} | done in {:.2?}", task_id, elapsed);
    Ok(Json(serde_json::to_value(resp).unwrap()))
}
