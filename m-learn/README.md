# M-Learn - Machine Learning Library in Rust

A modular Rust library for machine learning with tensor operations, neural networks, genetic algorithms, parallel computation, and visualization.

## 🚀 Quick Start

### Neural Network Training

```rust
use nn::{Sequential, Linear, ReLU, Loss, Module};
use tensor::Tensor;

fn main() {
    let mut model = Sequential::new();
    model.add(Linear::new(1, 64));
    model.add(ReLU::new());
    model.add(Linear::new(64, 64));
    model.add(ReLU::new());
    model.add(Linear::new(64, 1));

    let mut loss_fn = Loss::mse();

    for epoch in 0..1000 {
        let input = Tensor::build(vec![x], vec![1, 1]).unwrap();
        let target = Tensor::build(vec![y], vec![1, 1]).unwrap();

        let pred = model.forward(&input);
        let loss_result = loss_fn.criterion(&pred, &target);
        model.backward(&loss_result);
        model.update(0.001);
    }
}
```

### Genetic Algorithm Optimization

```rust
use genetic::{GeneticAlgorithm, sbx_crossover, mutation};

let fitness_fn = |genes: &[f32]| {
    let x = genes[0];
    let y = genes[1];
    -(x*x + y*y)  // minimize
};

let mut ga = GeneticAlgorithm::new(2, fitness_fn)
    .population_size(200)
    .crossover(sbx_crossover(15.0))
    .set_mutation(0.01, mutation::uniform_mutation(-50.0, 50.0))
    .randomize();

for _ in 0..500 {
    ga.step();
}

let best = ga.best_chromosome();
```

## 📦 Project Structure

```
m-learn/
├── crates/
│   ├── tensor/       # Tensor operations and data structures
│   ├── optim/       # Global thread pool and parallel execution
│   ├── draw/        # Plotting and visualization (2D/3D)
│   ├── formula/     # Mathematical formulas (distributions, etc.)
│   ├── nn/          # Neural network layers and training
│   └── genetic/     # Genetic algorithm optimization
├── example/
│   ├── tensor-example/           # Tensor usage examples
│   ├── draw-example/             # Plotting examples
│   ├── formula-draw-example/     # Formula + Tensor + Plot examples
│   ├── regression-example/       # Regression analysis examples
│   ├── genetic-example/          # Genetic algorithm examples
│   └── ackley-example/          # Ackley function optimization
└── docs/                       # Design documents
```

## 🧠 Neural Network (nn crate)

### Architecture Design

The neural network crate implements a PyTorch-like API:

**1. Module Trait** - Unified interface for all neural network components
```rust
pub trait Module: Send + Sync {
    fn forward(&mut self, x: &Tensor) -> Tensor;
    fn backward(&mut self, grad: &LossResult) -> Tensor;
    fn update(&mut self, lr: f32);
    fn parameters(&self) -> Vec<Tensor>;
}
```

**2. Layers**
- **Linear**: Fully connected layer with Xavier initialization
- **ReLU**: Rectified Linear Unit activation
- **Tanh**: Hyperbolic tangent activation
- **Sequential**: Container for chaining layers

**3. Loss Functions**
- **LossFunction trait**: Standard interface for all loss functions
- **LossResult**: Result object containing loss value and gradient
- **MSE Loss**: Mean Squared Error implementation

### Usage Example: Fitting sin(x)

```rust
use nn::{Sequential, Linear, Tanh, Loss, Module};
use tensor::Tensor;

let mut model = Sequential::new();
model.add(Linear::new(1, 16));
model.add(Tanh::new());
model.add(Linear::new(16, 1));

let mut loss_fn = Loss::mse();

for epoch in 0..500 {
    let x = (epoch as f32) * 0.01;
    let y = x.sin();

    let input = Tensor::build(vec![x], vec![1, 1]).unwrap();
    let target = Tensor::build(vec![y], vec![1, 1]).unwrap();

    let pred = model.forward(&input);
    let loss_result = loss_fn.criterion(&pred, &target);
    model.backward(&loss_result);
    model.update(0.01);
}
```

## 🧬 Genetic Algorithm (genetic crate)

### Purpose

A lightweight, configurable genetic algorithm library designed for real-valued optimization problems.

### Key Features

**Crossover Operators**:
- `single_point_crossover` - Single-point crossover
- `two_point_crossover` - Two-point crossover
- `sbx_crossover(eta)` - SBX crossover (recommended for real optimization)
- `arithmetic_crossover(alpha)` - Arithmetic crossover

**Mutation Operators**:
- `uniform_mutation(min, max)` - Uniform mutation
- `gaussian_mutation(sigma)` - Gaussian mutation
- `polynomial_mutation(eta, min, max)` - Polynomial mutation

**Selection Operators**:
- `tournament_selection(k)` - Tournament selection
- `roulette_selection()` - Roulette wheel selection

**Special Mechanisms**:
- Elite protection: Best chromosomes always preserved
- Gene bounds: Constrain genes to specified ranges
- Optional mutation: Increase population diversity

### API Example

```rust
use genetic::{GeneticAlgorithm, sbx_crossover, mutation};

// Create and configure genetic algorithm
let mut ga = GeneticAlgorithm::new(2, fitness_fn)
    .population_size(200)
    .tournament_size(5)
    .crossover(sbx_crossover(15.0))
    .set_mutation(0.01, mutation::uniform_mutation(-50.0, 50.0))
    .uniform_bounds(-50.0, 50.0)
    .elite_protect(true)
    .randomize();

// Run optimization
for _ in 0..500 {
    ga.step();
}

let best = ga.best_chromosome();
let best_fitness = ga.best_fitness();
```

### Benchmark Functions

**Rastrigin Variant** (maximize):
```rust
fn objective(x: f32, y: f32) -> f32 {
    let r2 = x * x + y * y;
    let r = r2.sqrt();
    0.5 - (r - 0.5).sin() / (1.0 + 0.001 * r2).powi(2)
}
```

**Ackley Function** (minimize):
```rust
fn ackley(x: f32) -> f32 {
    let pi = std::f32::consts::PI;
    x * x - 10.0 * (2.0 * pi * x).cos() + 10.0
}
```

## 🔢 Tensor (tensor crate)

**Purpose**: Multi-dimensional array operations for ML and scientific computing

**Key Features:**
- Create tensors from data or using macros
- Element-wise mathematical operations (+, -, *, /, pow)
- Multi-dimensional indexing (1D, 2D, 3D, 4D)
- Slicing and mutation
- Element-wise mapping with position awareness (`map()`)
- Utility functions (zeros, arange, rand, randn, sum, reshape, display)

## ⚡ Optim (optim crate)

**Purpose**: High-performance parallel execution with thread pool management

**Key Features:**
- Global singleton Rayon thread pool
- Parallel iterator operations (par_iter, par_iter_mut)
- Batch processing (par_batches)
- Task execution (execute)

**Performance**: Automatic speedup (3-5x) for operations on >=100,000 elements

## 📊 Draw (draw crate)

**Purpose**: Tensor data visualization and plotting

**Key Features:**
- SVG export (2D charts)
- PNG export (3D charts)
- Interactive window display (minifb)
- 2D line plotting
- 3D surface and point cloud plotting
- Configurable: titles, labels, ranges, ticks, colors

**Example: Plotting Convergence Curve**
```rust
draw::plot(
    &draw::PlotConfig::new()
        .title("Genetic Algorithm Convergence")
        .xlabel("Generation")
        .ylabel("Fitness")
        .show_window(false)
        .export("output/genetic/convergence.svg"),
    &[&tensor_points],
    &["Maximum"]
)?;
```

## 📐 Formula (formula crate)

**Purpose**: Mathematical functions for data analysis and ML

**Key Features:**
- Normal distribution (probability density function)
- Extensible design for adding new formulas
- f32 precision with IEEE 754 compliance

---

## 🛠️ Getting Started

### Build and Test

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test

# Run specific examples
cargo run -p tensor-example
cargo run -p draw-example
cargo run -p nn-example
cargo run -p genetic-example
cargo run -p regression-example
```

### Dependencies

The project uses these key dependencies:
- **rayon**: Parallel iterator library
- **plotters**: 2D chart rendering
- **polyscope-rs**: 3D visualization
- **minifb**: Window display
- **dejavu**: Font rendering
- **once_cell**: Lazy static initialization
- **num_cpus**: CPU core detection

All dependencies are in `Cargo.lock`.
