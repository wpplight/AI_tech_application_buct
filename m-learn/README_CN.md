# M-Learn - Rust 机器学习库

模块化的 Rust 机器学习库，包含张量运算、神经网络、遗传算法、并行计算和可视化功能。

## 🚀 快速开始

### 神经网络训练

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

### 遗传算法优化

```rust
use genetic::{GeneticAlgorithm, sbx_crossover, mutation};

let fitness_fn = |genes: &[f32]| {
    let x = genes[0];
    let y = genes[1];
    -(x*x + y*y)  // 求最小值
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

## 📦 项目结构

```
m-learn/
├── crates/
│   ├── tensor/       # 张量运算和数据结构
│   ├── optim/        # 全局线程池和并行计算
│   ├── draw/         # 绘图和可视化（2D/3D）
│   ├── formula/      # 数学公式（分布等）
│   ├── nn/           # 神经网络层和训练
│   └── genetic/      # 遗传算法优化
├── example/
│   ├── tensor-example/           # 张量使用示例
│   ├── draw-example/            # 绘图示例
│   ├── formula-draw-example/    # 公式+张量+绘图示例
│   ├── regression-example/      # 回归分析示例
│   ├── genetic-example/         # 遗传算法示例
│   └── ackley-example/          # Ackley函数优化
└── docs/                       # 设计文档
```

## 🧠 神经网络（nn crate）

### 架构设计

神经网络 crate 实现了类似 PyTorch 的 API：

**1. Module Trait** - 所有神经网络组件的统一接口
```rust
pub trait Module: Send + Sync {
    fn forward(&mut self, x: &Tensor) -> Tensor;
    fn backward(&mut self, grad: &LossResult) -> Tensor;
    fn update(&mut self, lr: f32);
    fn parameters(&self) -> Vec<Tensor>;
}
```

**2. 网络层**
- **Linear**: 全连接层，使用 Xavier 初始化
- **ReLU**: 线性整流单元激活函数
- **Tanh**: 双曲正切激活函数
- **Sequential**: 容器，用于链式组合各层

**3. 损失函数**
- **LossFunction trait**: 所有损失函数的标准接口
- **LossResult**: 结果对象，包含损失值和梯度
- **MSE Loss**: 均方误差损失实现

### 使用示例：拟合 sin(x)

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

## 🧬 遗传算法（genetic crate）

### 功能

轻量级、可配置的遗传算法库，专为实数优化问题设计。

### 主要特性

**预置交叉算法**:
- `single_point_crossover` - 单点交叉
- `two_point_crossover` - 双点交叉
- `sbx_crossover(eta)` - SBX交叉（实数优化推荐）
- `arithmetic_crossover(alpha)` - 算术交叉

**预置变异算法**:
- `uniform_mutation(min, max)` - 均匀变异
- `gaussian_mutation(sigma)` - 高斯变异
- `polynomial_mutation(eta, min, max)` - 多项式变异

**预置选择算法**:
- `tournament_selection(k)` - 锦标赛选择
- `roulette_selection()` - 轮盘赌选择

**特殊机制**:
- 精英保护：最佳染色体始终保留
- 基因范围约束：限制基因在指定范围内
- 可选变异：增加种群多样性

### API 示例

```rust
use genetic::{GeneticAlgorithm, sbx_crossover, mutation};

// 创建并配置遗传算法
let mut ga = GeneticAlgorithm::new(2, fitness_fn)
    .population_size(200)
    .tournament_size(5)
    .crossover(sbx_crossover(15.0))
    .set_mutation(0.01, mutation::uniform_mutation(-50.0, 50.0))
    .uniform_bounds(-50.0, 50.0)
    .elite_protect(true)
    .randomize();

// 运行优化
for _ in 0..500 {
    ga.step();
}

let best = ga.best_chromosome();
let best_fitness = ga.best_fitness();
```

### 基准测试函数示例

**Rastrigin变体函数** (求最大值):
```rust
fn objective(x: f32, y: f32) -> f32 {
    let r2 = x * x + y * y;
    let r = r2.sqrt();
    0.5 - (r - 0.5).sin() / (1.0 + 0.001 * r2).powi(2)
}
```

**Ackley函数** (求最小值):
```rust
fn ackley(x: f32) -> f32 {
    let pi = std::f32::consts::PI;
    x * x - 10.0 * (2.0 * pi * x).cos() + 10.0
}
```

## 🔢 张量（tensor crate）

**功能**：用于 ML 和科学计算的多维数组运算

**主要特性：**
- 从数据或宏创建张量
- 元素级数学运算（+、-、*、/、pow）
- 多维索引（1D、2D、3D、4D）
- 切片和修改
- 带位置感知的元素映射（`map()`）
- 工具函数（zeros、arange、rand、randn、sum、reshape、display）

## ⚡ 优化（optim crate）

**功能**：高性能并行执行和线程池管理

**主要特性：**
- 全局单例 Rayon 线程池
- 并行迭代器操作（par_iter、par_iter_mut）
- 批处理（par_batches）
- 任务执行（execute）

**性能**：对 >=100,000 元素的操作自动加速 3-5 倍

## 📊 绘图（draw crate）

**功能**：张量数据可视化和绘图

**主要特性：**
- SVG 导出（2D图表）
- PNG 导出（3D图表）
- 交互式窗口显示（minifb）
- 2D曲线绘图
- 3D曲面和点云绘图
- 可配置：标题、标签、范围、刻度、颜色

**示例：绘制收敛曲线**
```rust
draw::plot(
    &draw::PlotConfig::new()
        .title("遗传算法收敛曲线")
        .xlabel("Generation")
        .ylabel("Fitness")
        .show_window(false)
        .export("output/genetic/convergence.svg"),
    &[&tensor_points],
    &["最大值"]
)?;
```

## 📐 公式（formula crate）

**功能**：数据分析和机器学习的数学函数

**主要特性：**
- 正态分布（概率密度函数）
- 可扩展设计，易于添加新公式
- f32 精度，符合 IEEE 754 标准

---

## 🛠️ 快速开始

### 构建和测试

```bash
# 构建所有 crate
cargo build --workspace

# 运行测试
cargo test

# 运行特定示例
cargo run -p tensor-example
cargo run -p draw-example
cargo run -p nn-example
cargo run -p genetic-example
cargo run -p regression-example
```

### 依赖项

项目使用以下关键依赖：
- **rayon**：并行迭代器库
- **plotters**：2D图表渲染
- **polyscope-rs**：3D可视化
- **minifb**：窗口显示
- **dejavu**：字体渲染
- **once_cell**：延迟静态初始化
- **num_cpus**：CPU 核心检测

所有依赖都在 `Cargo.lock` 中。
