use crate::models::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use nn::{Sequential, Linear, ReLU, Tanh, Loss, Module};
use tensor::Tensor;
use genetic::{GeneticAlgorithm, sbx_crossover, mutation};

pub enum ModelState {
    Regression {
        model: Mutex<Sequential>,
        loss_fn: Mutex<Loss>,
        lr: f32,
        x_train: Vec<f32>,
        y_train: Vec<f32>,
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
    pub model: ModelState,
    pub total_epochs: usize,
    pub best_fitness: Option<f64>,
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
        let model = match req.algorithm {
            AlgorithmType::Regression => {
                let func = req.regression_fn.unwrap_or(RegressionFunction::Linear);
                let mut rng = rand::rng();
                let n = 200;
                let (x_train, y_train) = match func {
                    RegressionFunction::Linear => generate_linear_data(&mut rng, n, 0.0, 10.0, req.noise),
                    RegressionFunction::Quadratic => generate_quadratic_data(&mut rng, n, 0.0, 10.0, req.noise),
                    RegressionFunction::Sinusoidal => generate_sin_data(&mut rng, n, 0.0, 6.28, req.noise),
                };

                let mut seq = Sequential::new();
                match func {
                    RegressionFunction::Linear => {
                        seq.add(Linear::new(1, 1));
                    }
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

                ModelState::Regression {
                    model: Mutex::new(seq),
                    loss_fn: Mutex::new(Loss::mse()),
                    lr: req.learning_rate as f32,
                    x_train,
                    y_train,
                }
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

                ModelState::Genetic {
                    ga: Mutex::new(ga),
                    function: func,
                }
            }
        };

        let task = Task {
            id,
            algorithm: req.algorithm,
            model,
            total_epochs: 0,
            best_fitness: None,
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(id, Arc::new(Mutex::new(task)));
        Ok(id)
    }

    pub fn step(&self, id: Uuid, epochs: usize) -> Result<(), String> {
        let tasks = self.tasks.lock().unwrap();
        let task_arc = tasks.get(&id).ok_or("Task not found")?.clone();
        let mut task = task_arc.lock().unwrap();

        let new_fitness = match &task.model {
            ModelState::Regression { model, loss_fn, lr, x_train, y_train } => {
                let mut model = model.lock().unwrap();
                let mut loss_fn = loss_fn.lock().unwrap();
                for _ in 0..epochs {
                    for i in 0..x_train.len() {
                        let x = Tensor::build(vec![x_train[i]], vec![1, 1]).map_err(|e| e.to_string())?;
                        let y = Tensor::build(vec![y_train[i]], vec![1, 1]).map_err(|e| e.to_string())?;
                        let pred = model.forward(&x);
                        let loss_result = loss_fn.criterion(&pred, &y);
                        model.backward(&loss_result);
                        model.update(*lr);
                    }
                }
                let mut total_loss = 0.0f32;
                for i in 0..x_train.len() {
                    let x = Tensor::build(vec![x_train[i]], vec![1, 1]).unwrap();
                    let y = Tensor::build(vec![y_train[i]], vec![1, 1]).unwrap();
                    let pred = model.forward(&x);
                    let loss_result = loss_fn.criterion(&pred, &y);
                    total_loss += loss_result.loss;
                }
                Some((total_loss / x_train.len() as f32) as f64)
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
            ModelState::Regression { model, x_train, y_train, .. } => {
                let mut model = model.lock().unwrap();
                let n = 100usize;
                let x_min = *x_train.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
                let x_max = *x_train.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&1.0);
                let step = (x_max - x_min) / n as f32;

                let x_curve: Vec<f64> = (0..n)
                    .map(|i| (x_min + step * i as f32) as f64)
                    .collect();
                let y_curve: Vec<f64> = x_curve.iter().map(|&xi| {
                    let input = Tensor::build(vec![xi as f32], vec![1, 1]).unwrap();
                    model.forward(&input).data()[0] as f64
                }).collect();

                let mut total_loss = 0.0f32;
                for i in 0..x_train.len() {
                    let x = Tensor::build(vec![x_train[i]], vec![1, 1]).unwrap();
                    let pred = model.forward(&x);
                    let diff = pred.data()[0] - y_train[i];
                    total_loss += diff * diff;
                }

                Ok(InferenceResponse::Regression(RegressionInference {
                    x_data: x_train.iter().map(|v| *v as f64).collect(),
                    y_data: y_train.iter().map(|v| *v as f64).collect(),
                    x_curve,
                    y_curve,
                    loss: (total_loss / x_train.len() as f32) as f64,
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

    pub fn stop(&self, id: Uuid) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&id);
        Ok(())
    }
}

fn generate_linear_data(rng: &mut impl rand::Rng, n: usize, x_min: f32, x_max: f32, noise: f64) -> (Vec<f32>, Vec<f32>) {
    let noise = noise as f32;
    let x: Vec<f32> = (0..n).map(|i| x_min + (x_max - x_min) * i as f32 / (n as f32 - 1.0)).collect();
    let y: Vec<f32> = x.iter().map(|&xi| {
        let n = rng.random_range(-noise..noise);
        2.0 * xi + 1.0 + n
    }).collect();
    (x, y)
}

fn generate_quadratic_data(rng: &mut impl rand::Rng, n: usize, x_min: f32, x_max: f32, noise: f64) -> (Vec<f32>, Vec<f32>) {
    let noise = noise as f32;
    let x: Vec<f32> = (0..n).map(|i| x_min + (x_max - x_min) * i as f32 / (n as f32 - 1.0)).collect();
    let y: Vec<f32> = x.iter().map(|&xi| {
        let n = rng.random_range(-noise..noise);
        xi * xi + n
    }).collect();
    (x, y)
}

fn generate_sin_data(rng: &mut impl rand::Rng, n: usize, x_min: f32, x_max: f32, noise: f64) -> (Vec<f32>, Vec<f32>) {
    let noise = noise as f32;
    let x: Vec<f32> = (0..n).map(|i| x_min + (x_max - x_min) * i as f32 / (n as f32 - 1.0)).collect();
    let y: Vec<f32> = x.iter().map(|&xi| {
        let n = rng.random_range(-noise..noise);
        xi.sin() + n
    }).collect();
    (x, y)
}
