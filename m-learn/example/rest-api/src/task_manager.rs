use crate::models::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use nn::{Sequential, Linear, ReLU, Tanh, Loss, Module};
use tensor::Tensor;
use genetic::{GeneticAlgorithm, sbx_crossover, mutation};
use rand::seq::SliceRandom;
use tracing::info;

pub enum ModelState {
    Regression {
        model: Mutex<Sequential>,
        loss_fn: Mutex<Loss>,
        lr: f32,
        x_train: Vec<f32>,
        x_val: Vec<f32>,
        y_train: Vec<f32>,
        y_val: Vec<f32>,
        x_mean: f32,
        x_std: f32,
        y_mean: f32,
        y_std: f32,
        history: Vec<EpochRecord>,
        y_val_true: Vec<f64>,
        y_val_pred: Vec<f64>,
    },
    Genetic {
        ga: Mutex<GeneticAlgorithm>,
        function: GeneticFunction,
    },
}

unsafe impl Send for ModelState {}
unsafe impl Sync for ModelState {}

pub struct Task {
    pub id: Uuid,
    pub algorithm: AlgorithmType,
    pub regression_fn: Option<RegressionFunction>,
    pub genetic_fn: Option<GeneticFunction>,
    pub x_min: f64,
    pub x_max: f64,
    pub model: ModelState,
    pub total_epochs: usize,
    pub best_fitness: Option<f64>,
    pub created_at: u64,
}

#[derive(Clone)]
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Task>>>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create(&self, req: CreateTrainRequest) -> Result<Uuid, String> {
        let id = Uuid::new_v4();

        let (model, x_min, x_max) = match req.algorithm {
            AlgorithmType::Regression => {
                let func = req.regression_fn.unwrap_or(RegressionFunction::Linear);
                let mut rng = rand::rng();
                let n = 200;
                let (x_min_val, x_max_val) = match func {
                    RegressionFunction::Linear => (0.0, 10.0),
                    RegressionFunction::Quadratic => (0.0, 10.0),
                    RegressionFunction::Sinusoidal => (0.0, 6.28),
                };
                let x_min_req = req.x_min.unwrap_or(x_min_val as f64) as f32;
                let x_max_req = req.x_max.unwrap_or(x_max_val as f64) as f32;
                let actual_x_min = x_min_req as f64;
                let actual_x_max = x_max_req as f64;

                let (raw_x, raw_y) = match func {
                    RegressionFunction::Linear => generate_linear_data(&mut rng, n, x_min_req, x_max_req, req.noise),
                    RegressionFunction::Quadratic => generate_quadratic_data(&mut rng, n, x_min_req, x_max_req, req.noise),
                    RegressionFunction::Sinusoidal => generate_sin_data(&mut rng, n, x_min_req, x_max_req, req.noise),
                };
                let x_mean = mean(&raw_x);
                let x_std = std_dev(&raw_x, x_mean);
                let y_min_val = raw_y.iter().cloned().fold(f32::INFINITY, f32::min);
                let y_max_val = raw_y.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let x_norm_all = normalize(&raw_x, x_mean, x_std);
                let y_norm_all = raw_y.iter().map(|&y| (y - y_min_val) / (y_max_val - y_min_val).max(1e-8)).collect::<Vec<_>>();

                info!("CREATE | fn={:?}, x=[{:.2},{:.2}], y=[{:.2},{:.2}], y_min={:.4}, y_max={:.4}", 
                    func, raw_x[0], raw_x[raw_x.len()-1], raw_y[0], raw_y[raw_y.len()-1], y_min_val, y_max_val);

                let mut indices: Vec<usize> = (0..n).collect();
                indices.shuffle(&mut rng);

                let split = (n as f32 * 0.8).round() as usize;
                let (train_idx, val_idx) = indices.split_at(split);

                let x_norm_train: Vec<f32> = train_idx.iter().map(|&i| x_norm_all[i]).collect();
                let x_norm_val: Vec<f32> = val_idx.iter().map(|&i| x_norm_all[i]).collect();
                let y_norm_train: Vec<f32> = train_idx.iter().map(|&i| y_norm_all[i]).collect();
                let y_norm_val: Vec<f32> = val_idx.iter().map(|&i| y_norm_all[i]).collect();
                let raw_y_val: Vec<f32> = val_idx.iter().map(|&i| raw_y[i]).collect();

                let y_val_true: Vec<f64> = raw_y_val.iter().map(|&v| v as f64).collect();
                let y_val_pred: Vec<f64> = y_norm_val.iter().map(|_| 0.0).collect();

                let mut seq = Sequential::new();
                match func {
                    RegressionFunction::Linear => { seq.add(Linear::new(1, 1)); }
                    RegressionFunction::Quadratic => {
                        seq.add(Linear::new(1, 32));
                        seq.add(ReLU::new());
                        seq.add(Linear::new(32, 32));
                        seq.add(ReLU::new());
                        seq.add(Linear::new(32, 1));
                    }
                    RegressionFunction::Sinusoidal => {
                        seq.add(Linear::new(1, 32));
                        seq.add(Tanh::new());
                        seq.add(Linear::new(32, 32));
                        seq.add(Tanh::new());
                        seq.add(Linear::new(32, 1));
                    }
                }

                (ModelState::Regression {
                    model: Mutex::new(seq),
                    loss_fn: Mutex::new(Loss::mse()),
                    lr: req.learning_rate as f32,
                    x_train: x_norm_train,
                    x_val: x_norm_val,
                    y_train: y_norm_train,
                    y_val: y_norm_val,
                    x_mean,
                    x_std,
                    y_mean: y_min_val,
                    y_std: y_max_val,
                    history: Vec::new(),
                    y_val_true,
                    y_val_pred,
                }, actual_x_min, actual_x_max)
            }
            AlgorithmType::Genetic => {
                let func = req.genetic_fn.unwrap_or(GeneticFunction::RastriginVariant);
                let ga = match func {
                    GeneticFunction::RastriginVariant => {
                        let fitness_fn = |genes: &[f32]| {
                            let x = genes[0];
                            let y = genes[1];
                            0.5 - (x * x + y * y).sqrt().sin() / (1.0 + 0.001 * (x * x + y * y)).powi(2)
                        };
                        GeneticAlgorithm::new(2, fitness_fn)
                            .population_size(200)
                            .tournament_size(10)
                            .crossover(sbx_crossover(15.0))
                            .set_mutation(0.01, mutation::uniform_mutation(-50.0, 50.0))
                            .uniform_bounds(-50.0, 50.0)
                            .elite_protect(true)
                            .randomize()
                    }
                    GeneticFunction::Ackley => {
                        let fitness_fn = |genes: &[f32]| {
                            let x = genes[0];
                            let val = x * x - 10.0 * (2.0 * std::f32::consts::PI * x).cos() + 10.0;
                            if val.is_nan() || val.is_infinite() { f32::MIN } else { -val }
                        };
                        GeneticAlgorithm::new(1, fitness_fn)
                            .population_size(200)
                            .tournament_size(10)
                            .crossover(sbx_crossover(15.0))
                            .set_mutation(0.01, mutation::uniform_mutation(-5.12, 5.12))
                            .uniform_bounds(-5.12, 5.12)
                            .elite_protect(true)
                            .randomize()
                    }
                };

                (ModelState::Genetic {
                    ga: Mutex::new(ga),
                    function: func,
                }, 0.0, 0.0)
            }
        };

        let task = Task {
            id,
            algorithm: req.algorithm,
            regression_fn: req.regression_fn,
            genetic_fn: req.genetic_fn,
            x_min,
            x_max,
            model,
            total_epochs: 0,
            best_fitness: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(id, Arc::new(Mutex::new(task)));
        Ok(id)
    }

    pub fn step(&self, id: Uuid, epochs: usize) -> Result<(), String> {
        let tasks = self.tasks.lock().unwrap();
        let task_arc = tasks.get(&id).ok_or("Task not found")?.clone();
        let mut task = task_arc.lock().unwrap();
        let base_epoch = task.total_epochs;

        let new_fitness = match &mut task.model {
            ModelState::Regression { model, loss_fn, lr, x_train, y_train, x_val, y_val, x_mean: _, x_std: _, y_mean, y_std, history, y_val_true, y_val_pred } => {
                let mut model = model.lock().unwrap();
                let mut loss_fn = loss_fn.lock().unwrap();

                for e in 0..epochs {
                    let mut train_order: Vec<usize> = (0..x_train.len()).collect();
                    let mut rng = rand::rng();
                    train_order.shuffle(&mut rng);

                    let mut train_loss_sum = 0.0f32;
                    for &i in &train_order {
                        let x = Tensor::build(vec![x_train[i]], vec![1, 1]).map_err(|e| e.to_string())?;
                        let y = Tensor::build(vec![y_train[i]], vec![1, 1]).map_err(|e| e.to_string())?;
                        let pred = model.forward(&x);
                        let loss_result = loss_fn.criterion(&pred, &y);
                        train_loss_sum += loss_result.loss;
                        model.backward(&loss_result);
                        model.update(*lr);
                    }
                    let train_loss = train_loss_sum / x_train.len() as f32;

                    let mut val_loss_sum = 0.0f32;
                    for i in 0..x_val.len() {
                        let x = Tensor::build(vec![x_val[i]], vec![1, 1]).unwrap();
                        let y = Tensor::build(vec![y_val[i]], vec![1, 1]).unwrap();
                        let pred = model.forward(&x);
                        let loss_result = loss_fn.criterion(&pred, &y);
                        val_loss_sum += loss_result.loss;
                        y_val_pred[i] = (pred.data()[0] * (*y_std - *y_mean) + *y_mean) as f64;
                    }
                    let val_loss = val_loss_sum / x_val.len() as f32;

                    let epoch_idx = base_epoch + e + 1;
                    let y_pred_min = y_val_pred.iter().cloned().fold(f64::INFINITY, f64::min);
                    let y_pred_max = y_val_pred.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let y_true_min = y_val_true.iter().cloned().fold(f64::INFINITY, f64::min);
                    let y_true_max = y_val_true.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    if epoch_idx == 1 || epoch_idx == epochs || epoch_idx % 100 == 0 {
                        info!("STEP | epoch={}, train={:.6}, val={:.6}, y_pred=[{:.2},{:.2}], y_true=[{:.2},{:.2}]",
                            epoch_idx, train_loss, val_loss, y_pred_min, y_pred_max, y_true_min, y_true_max);
                    }

                    history.push(EpochRecord {
                        epoch: epoch_idx,
                        train_loss: train_loss as f64,
                        val_loss: val_loss as f64,
                    });

                    if e == epochs - 1 || e % 10 == 0 {
                        info!("STEP | epoch={}, train_loss={:.6}, val_loss={:.6}", epoch_idx, train_loss, val_loss);
                    }
                }

                let last = history.last().map(|r| r.train_loss).unwrap_or(0.0);
                Some(last)
            }
            ModelState::Genetic { ga, .. } => {
                let mut ga = ga.lock().unwrap();
                for _ in 0..epochs {
                    ga.step();
                }
                Some(ga.best_fitness() as f64)
            }
        };

        task.best_fitness = new_fitness;
        task.total_epochs += epochs;
        Ok(())
    }

    pub fn get_status(&self, id: Uuid) -> Result<TrainStatusResponse, String> {
        let tasks = self.tasks.lock().unwrap();
        let task_arc = tasks.get(&id).ok_or("Task not found")?.clone();
        let task = task_arc.lock().unwrap();

        Ok(TrainStatusResponse {
            task_id: task.id,
            algorithm: task.algorithm,
            total_epochs: task.total_epochs,
            is_running: false,
            best_fitness: task.best_fitness,
        })
    }

    pub fn get_inference(&self, id: Uuid) -> Result<InferenceResponse, String> {
        let tasks = self.tasks.lock().unwrap();
        let task_arc = tasks.get(&id).ok_or("Task not found")?.clone();
        let task = task_arc.lock().unwrap();

        match &task.model {
            ModelState::Regression { model, x_mean, x_std, y_mean, y_std, .. } => {
                let mut model = model.lock().unwrap();
                let n = 100usize;
                let x_orig_min = task.x_min as f32;
                let x_orig_max = task.x_max as f32;
                let step = (x_orig_max - x_orig_min) / n as f32;

                let x_curve: Vec<f64> = (0..=n)
                    .map(|i| (x_orig_min + step * i as f32) as f64)
                    .collect();
                let y_curve: Vec<f64> = x_curve.iter().map(|&xi| {
                    let x_norm = (xi as f32 - x_mean) / x_std;
                    let input = Tensor::build(vec![x_norm], vec![1, 1]).unwrap();
                    let y_norm = model.forward(&input).data()[0];
                    (y_norm * (y_std - y_mean) + y_mean) as f64
                }).collect();

                info!("INFERENCE | x_curve range=[{:.2},{:.2}], y_curve sample at end: x={:.2}, y={:.2}", 
                    x_curve.first().unwrap_or(&0.0), x_curve.last().unwrap_or(&0.0),
                    x_curve.get(x_curve.len()-5).unwrap_or(&0.0), y_curve.get(y_curve.len()-5).unwrap_or(&0.0));

                Ok(InferenceResponse::Regression(RegressionInference {
                    x_curve,
                    y_curve,
                    x_min: task.x_min,
                    x_max: task.x_max,
                }))
            }
            ModelState::Genetic { ga, function } => {
                let ga = ga.lock().unwrap();
                let best_gene = ga.best_chromosome();
                let best_fitness = ga.best_fitness();

                match function {
                    GeneticFunction::Ackley => {
                        let min = -5.12f32;
                        let max = 5.12f32;
                        let steps = 200;
                        let x_range: Vec<f64> = (0..steps)
                            .map(|i| (min + (max - min) * i as f32 / (steps as f32 - 1.0)) as f64)
                            .collect();
                        let y_true: Vec<f64> = x_range.iter().map(|&x| {
                            let x = x as f32;
                            (x * x - 10.0 * (2.0 * std::f32::consts::PI * x).cos() + 10.0) as f64
                        }).collect();
                        Ok(InferenceResponse::Genetic1D(Genetic1DInference {
                            x_range,
                            y_true,
                            best_gene: best_gene[0] as f64,
                            best_fitness: (-best_fitness) as f64,
                        }))
                    }
                    GeneticFunction::RastriginVariant => {
                        let min = -50.0f32;
                        let max = 50.0f32;
                        let steps = 50;
                        let mut x_grid = Vec::with_capacity(steps * steps);
                        let mut y_grid = Vec::with_capacity(steps * steps);
                        let mut fitness_grid = Vec::with_capacity(steps * steps);
                        for i in 0..steps {
                            for j in 0..steps {
                                let x = min + (max - min) * i as f32 / (steps as f32 - 1.0);
                                let y = min + (max - min) * j as f32 / (steps as f32 - 1.0);
                                let r2 = x * x + y * y;
                                let r = r2.sqrt();
                                let val = 0.5 - (r - 0.5).sin() / (1.0 + 0.001 * r2).powi(2);
                                x_grid.push(x as f64);
                                y_grid.push(y as f64);
                                fitness_grid.push(val as f64);
                            }
                        }
                        Ok(InferenceResponse::Genetic2D(Genetic2DInference {
                            x_grid,
                            y_grid,
                            fitness_grid,
                            best_gene_x: best_gene[0] as f64,
                            best_gene_y: best_gene[1] as f64,
                            best_fitness: best_fitness as f64,
                        }))
                    }
                }
            }
        }
    }

    pub fn list_tasks(&self) -> Vec<TaskListItem> {
        let tasks = self.tasks.lock().unwrap();
        tasks.values().map(|task_arc| {
            let task = task_arc.lock().unwrap();
            TaskListItem {
                task_id: task.id,
                algorithm: task.algorithm,
                regression_fn: task.regression_fn,
                genetic_fn: task.genetic_fn,
                total_epochs: task.total_epochs,
                best_fitness: task.best_fitness,
                created_at: task.created_at,
            }
        }).collect()
    }

    pub fn get_history(&self, id: Uuid) -> Result<TrainingHistory, String> {
        let tasks = self.tasks.lock().unwrap();
        let task_arc = tasks.get(&id).ok_or("Task not found")?.clone();
        let task = task_arc.lock().unwrap();

        match &task.model {
            ModelState::Regression { history, .. } => Ok(TrainingHistory {
                task_id: task.id,
                records: history.clone(),
            }),
            ModelState::Genetic { .. } => Err("History not available for genetic tasks".to_string()),
        }
    }

    pub fn get_recall(&self, id: Uuid) -> Result<RecallResponse, String> {
        let tasks = self.tasks.lock().unwrap();
        let task_arc = tasks.get(&id).ok_or("Task not found")?.clone();
        let task = task_arc.lock().unwrap();

        match &task.model {
            ModelState::Regression { y_val_true, y_val_pred, .. } => Ok(RecallResponse {
                y_true: y_val_true.clone(),
                y_pred: y_val_pred.clone(),
            }),
            ModelState::Genetic { .. } => Err("Recall not available for genetic tasks".to_string()),
        }
    }

    pub fn stop(&self, id: Uuid) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&id);
        Ok(())
    }
}

fn sample_noise(rng: &mut impl rand::Rng, noise: f32) -> f32 {
    if noise <= 0.0 { 0.0 } else { rng.random_range(-noise..noise) }
}

fn mean(data: &[f32]) -> f32 {
    data.iter().sum::<f32>() / data.len() as f32
}

fn std_dev(data: &[f32], m: f32) -> f32 {
    let v = data.iter().map(|x| (x - m).powi(2)).sum::<f32>() / data.len() as f32;
    v.sqrt().max(1e-8)
}

fn normalize(data: &[f32], m: f32, s: f32) -> Vec<f32> {
    data.iter().map(|x| (x - m) / s).collect()
}

fn generate_linear_data(rng: &mut impl rand::Rng, n: usize, x_min: f32, x_max: f32, noise: f64) -> (Vec<f32>, Vec<f32>) {
    let noise = noise as f32;
    let x: Vec<f32> = (0..n).map(|i| x_min + (x_max - x_min) * i as f32 / (n as f32 - 1.0)).collect();
    let y: Vec<f32> = x.iter().map(|&xi| {
        2.0 * xi + 1.0 + sample_noise(rng, noise)
    }).collect();
    (x, y)
}

fn generate_quadratic_data(rng: &mut impl rand::Rng, n: usize, x_min: f32, x_max: f32, noise: f64) -> (Vec<f32>, Vec<f32>) {
    let noise = noise as f32;
    let x: Vec<f32> = (0..n).map(|i| x_min + (x_max - x_min) * i as f32 / (n as f32 - 1.0)).collect();
    let y: Vec<f32> = x.iter().map(|&xi| {
        xi * xi + sample_noise(rng, noise)
    }).collect();
    (x, y)
}

fn generate_sin_data(rng: &mut impl rand::Rng, n: usize, x_min: f32, x_max: f32, noise: f64) -> (Vec<f32>, Vec<f32>) {
    let noise = noise as f32;
    let x: Vec<f32> = (0..n).map(|i| x_min + (x_max - x_min) * i as f32 / (n as f32 - 1.0)).collect();
    let y: Vec<f32> = x.iter().map(|&xi| {
        xi.sin() + sample_noise(rng, noise)
    }).collect();
    (x, y)
}
