# Neural Network (nn)

Rust 实现的全连接神经网络库，提供常用层和容器。

## 设计理念

- **并行计算下沉到张量层**：层代码保持简洁，调用 `tensor.matmul()` 自动享受并行优化
- **类型安全**：使用 Rust 类型系统确保网络构建正确性
- **零成本抽象**：高性能，无额外运行时开销

## 快速开始

### 创建神经网络

```rust
use nn::{Sequential, Linear, ReLULayer, Module};
use tensor::Tensor;

// 方式 1: 使用 add 方法
let model = Sequential::new()
    .add("fc1", Linear::new(2, 10))
    .add("relu", ReLULayer::new())
    .add("fc2", Linear::new(10, 1));

// 前向传播
let input = Tensor::build(vec![3.0, 4.0], vec![2]).unwrap();
let output = model.forward(&input);
```

### 矩阵乘法并行

矩阵乘法自动选择最优并行策略：

| 场景 | 策略 |
|------|------|
| 小矩阵 (n < 64) | 串行 tiled 计算 |
| 大矩阵 (n >= 线程数 * 4) | 两步并行（转置 + 缓存友好乘法） |
| 批量 | 批量并行 |

## 可用层

### Linear（全连接层）

```rust
// 创建带 bias 的线性层
let fc = Linear::new(in_features, out_features);

// 创建不带 bias 的线性层
let fc = Linear::new_no_bias(in_features, out_features);

// 前向传播: y = x @ W^T + b
// 输入: [batch, in_features]
// 输出: [batch, out_features]
let output = fc.forward(&input);
```

### ReLULayer

```rust
let relu = ReLULayer::new();

// 前向传播: y = max(0, x)
let output = relu.forward(&input);
```

### Dropout

```rust
let dropout = Dropout::new(0.5);  // 50% dropout rate

// 训练模式（默认）
let output = dropout.forward(&input);

// 推理模式
dropout.eval();
let output = dropout.forward(&input);
```

### LayerNorm

```rust
let ln = LayerNorm::new(vec![hidden_size]);

// 前向传播: 归一化最后 hidden_size 个元素
let output = ln.forward(&input);
```

## Sequential 容器

### 创建方式

```rust
use nn::Sequential;

// 方式 1: add 方法
let model = Sequential::new()
    .add("fc1", Linear::new(784, 256))
    .add("relu1", ReLULayer::new())
    .add("fc2", Linear::new(256, 128))
    .add("relu2", ReLULayer::new())
    .add("fc3", Linear::new(128, 10));
```

### 访问层

```rust
// 按名称获取层
if let Some(layer) = model.get("fc1") {
    println!("Layer: {}", layer.name());
}
```

## 参数管理

### 获取所有参数

```rust
let params = model.parameters();
println!("Total parameters: {}", params.len());
```

### 获取命名参数

```rust
let named_params = model.named_parameters();
for (name, param) in &named_params {
    println!("{}: shape={:?}", name, param.shape());
}
```

## Module Trait

所有层都实现 `Module` trait：

```rust
pub trait Module: Send + Sync {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn parameters(&self) -> Vec<Tensor>;
    fn named_parameters(&self) -> HashMap<String, Tensor>;
    fn name(&self) -> String;
}
```

## 完整示例

```rust
use nn::{Sequential, Linear, ReLULayer, Module};
use tensor::Tensor;

fn main() {
    // 创建 MLP
    let model = Sequential::new()
        .add("fc1", Linear::new(2, 10))
        .add("relu", ReLULayer::new())
        .add("fc2", Linear::new(10, 1));

    // 打印参数
    let params = model.parameters();
    println!("Parameters: {}", params.len());
    for (i, p) in params.iter().enumerate() {
        println!("  Param {}: shape={:?}", i, p.shape());
    }

    // 前向传播
    let x = Tensor::build(vec![3.0, 4.0], vec![2]).unwrap();
    let y = model.forward(&x);
    println!("Output: {:?}", y.data());

    // 批量前向传播
    let batch_x = Tensor::build(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![3, 2]).unwrap();
    let batch_y = model.forward(&batch_x);
    println!("Batch output: {:?}", batch_y.data());
}
```

## 架构

```
┌─────────────────────────────────────────────────────────┐
│                      用户代码                             │
│  model.forward(&input)                                   │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                    Sequential 容器                        │
│  遍历各层，调用 forward()                                │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                     各个层 (Module)                      │
│  Linear: input.matmul(&weight) + bias                   │
│  ReLU: input.map(|_, x| x.max(0.0))                   │
│  Dropout: 随机置零                                     │
│  LayerNorm: 归一化                                      │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                  Tensor Operations                        │
│  matmul: 两步并行自动优化                               │
│  map: Rayon 并行化                                     │
└─────────────────────────────────────────────────────────┘
```

## 性能

- 矩阵乘法使用两步并行策略：
  1. 并行将 B 的列分块转换为列优先格式
  2. 使用缓存友好的乘法
- 激活函数使用 `tensor.map()` 自动并行
- 小张量使用串行计算避免调度开销

## 依赖

```toml
[dependencies]
tensor = { path = "../tensor" }
rand = "0.9"
```
