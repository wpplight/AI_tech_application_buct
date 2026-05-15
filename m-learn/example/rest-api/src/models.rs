use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AlgorithmType {
    Regression,
    Genetic,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RegressionFunction {
    Linear,
    Quadratic,
    Sinusoidal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum GeneticFunction {
    RastriginVariant,
    Ackley,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTrainRequest {
    pub algorithm: AlgorithmType,
    #[serde(default)]
    pub regression_fn: Option<RegressionFunction>,
    #[serde(default)]
    pub genetic_fn: Option<GeneticFunction>,
    #[serde(default = "default_lr")]
    pub learning_rate: f64,
    #[serde(default = "default_noise")]
    pub noise: f64,
    #[serde(default)]
    pub x_min: Option<f64>,
    #[serde(default)]
    pub x_max: Option<f64>,
}

fn default_lr() -> f64 { 0.01 }
fn default_noise() -> f64 { 0.1 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpochRecord {
    pub epoch: usize,
    pub train_loss: f64,
    pub val_loss: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingHistory {
    pub task_id: uuid::Uuid,
    pub records: Vec<EpochRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecallResponse {
    pub y_true: Vec<f64>,
    pub y_pred: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepRequest {
    pub epochs: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainStatusResponse {
    pub task_id: uuid::Uuid,
    pub algorithm: AlgorithmType,
    pub total_epochs: usize,
    pub is_running: bool,
    pub best_fitness: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegressionInference {
    pub x_curve: Vec<f64>,
    pub y_curve: Vec<f64>,
    pub x_min: f64,
    pub x_max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genetic1DInference {
    pub x_range: Vec<f64>,
    pub y_true: Vec<f64>,
    pub best_gene: f64,
    pub best_fitness: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genetic2DInference {
    pub x_grid: Vec<f64>,
    pub y_grid: Vec<f64>,
    pub fitness_grid: Vec<f64>,
    pub best_gene_x: f64,
    pub best_gene_y: f64,
    pub best_fitness: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InferenceResponse {
    Regression(RegressionInference),
    Genetic1D(Genetic1DInference),
    Genetic2D(Genetic2DInference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListItem {
    pub task_id: uuid::Uuid,
    pub algorithm: AlgorithmType,
    pub regression_fn: Option<RegressionFunction>,
    pub genetic_fn: Option<GeneticFunction>,
    pub total_epochs: usize,
    pub best_fitness: Option<f64>,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskListItem>,
}
