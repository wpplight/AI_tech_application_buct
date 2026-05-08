# 神经网络模块设计

## 1. 概述

参考 PyTorch 的 `torch.nn` 模块，设计一个 Rust 版本的神经网络库。

## 2. 设计原则

- **类型安全**: 利用 Rust 的类型系统确保网络构建正确性
- **零成本抽象**: 保持高性能，无额外运行时开销
- **可组合性**: 层与层之间可以灵活组合
- **自动微分**: 支持反向传播和梯度计算
- **并行计算**: 利用 Rayon 进行高效并行计算
- **张量层优化**: 并行计算下沉到张量层，层代码保持简洁

## 2.1 并行计算架构

并行计算已在 `tensor` crate 的矩阵乘法层面实现，神经网络层直接受益，无需在层内部重复优化。

```
┌─────────────────────────────────────────────────────────────────────┐
│                        神经网络层 (nn crate)                          │
│                                                                     │
│  pub struct Linear { weight: Tensor, bias: Option<Tensor> }         │
│                                                                     │
│  fn forward(&self, input: &Tensor) -> Tensor {                      │
│      let output = input.matmul(&self.weight).unwrap();              │
│      // 简单调用，内部自动并行优化                                      │
│      if let Some(ref bias) = self.bias {                            │
│          (output + bias).unwrap()                                   │
│      } else {                                                      │
│          output                                                    │
│      }                                                             │
│  }                                                                 │
└─────────────────────────────────────────────────────────────────────┘
                                ↓
┌─────────────────────────────────────────────────────────────────────┐
│                     张量运算 (tensor crate)                           │
│                                                                     │
│  matmul 自动选择最优并行策略:                                          │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │  1D × 2D (向量 × 矩阵)                                        │ │
│  │  vec(768) × mat(768, 256) → vec(256)                        │ │
│  │  按输出维度智能分块，任务数 = min(256, 线程数)                   │ │
│  └───────────────────────────────────────────────────────────────┘ │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │  2D × 2D (矩阵 × 矩阵)                                        │ │
│  │  mat(m, k) × mat(k, n) → mat(m, n)                          │ │
│  │  按输出行智能分块，任务数 = min(m, 线程数)                      │ │
│  └───────────────────────────────────────────────────────────────┘ │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │  N-D × N-D (批量矩阵乘法)                                     │ │
│  │  自动处理 batch 维度，多 batch 并行                            │ │
│  └───────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                                ↓
┌─────────────────────────────────────────────────────────────────────┐
│                    并行调度器 (optim crate)                          │
│                                                                     │
│  GlobalPool API:                                                    │
│  - num_threads(): 获取可用线程数                                     │
│  - optimal_num_chunks(total): 计算最优分块数                         │
│  - parallel_chunk_size(total): 计算分块大小                          │
│  - should_parallelize(total): 判断是否值得并行                       │
│                                                                     │
│  智能阈值:                                                          │
│  - PARALLEL_THRESHOLD = 100,000 元素                               │
│  - 小于阈值使用串行 tiled 计算，避免调度开销                          │
└─────────────────────────────────────────────────────────────────────┘
```

### 张量并行的优势

| 优化层级 | 优点 | 缺点 |
|----------|------|------|
| **张量层（推荐）** | ✅ 统一优化，所有层受益<br>✅ 代码简洁，职责清晰<br>✅ 自动适配硬件 | 需要 tensor 实现完善 |
| 层内部 | ✅ 针对特定层可定制 | ❌ 代码重复<br>❌ 难以维护<br>❌ 难以适配新硬件 |

### Linear 层的并行收益

```rust
// 前向传播: y = xW^T + b
// 输入: x [batch, in_features]
// 权重: W [out_features, in_features]  
// 输出: y [batch, out_features]

// 张量乘法: [batch, in] × [out, in]^T = [batch, out]
// 并行维度: batch × out 元素
// 
// 示例: batch=32, in=768, out=256
// - 输出空间: 32 × 256 = 8,192 个元素
// - 自动分配到可用线程
```

### 全连接层实现（简洁版）

```rust
/// 线性变换: y = xW^T + b
/// API 风格: torch.nn.Linear(in_features, out_features, bias=True)
#[derive(Clone)]
pub struct Linear {
    weight: Tensor,              // [out_features, in_features]
    bias: Option<Tensor>,       // [out_features]
    in_features: usize,
    out_features: usize,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let scale = (6.0 / (in_features + out_features) as f32).sqrt();
        
        let weight = Tensor::uniform(
            vec![out_features, in_features],
            -scale,
            scale,
        ).unwrap();

        let bias = Some(Tensor::zeros(vec![out_features]).unwrap());

        Self {
            weight,
            bias,
            in_features,
            out_features,
        }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        // 矩阵乘法在 tensor 层自动并行
        let output = input.matmul(&self.weight).unwrap();
        
        match &self.bias {
            Some(bias) => (output + bias).unwrap(),
            None => output,
        }
    }
}
```

### 自动并行的场景覆盖

| 场景 | 输入形状 | 权重形状 | 输出形状 | 并行策略 |
|------|---------|---------|---------|----------|
| 简单全连接 | [batch, in] | [out, in] | [batch, out] | 按 batch×out 并行 |
| 单样本 | [in] | [out, in] | [out] | 按 out 并行 |
| 批归一化后 | [batch, seq, in] | [out, in] | [batch, seq, out] | 按 batch×seq×out 并行 |
| 多头注意力 | [batch, heads, seq, dim] | [out, dim] | [batch, heads, seq, out] | 复杂 batch 并行 |

### 与 PyTorch 对比

| 特性 | PyTorch | m-learn (本设计) |
|------|---------|------------------|
| 并行实现 | C++/CUDA kernel | Rust + Rayon |
| 矩阵乘法 | cuBLAS/mkldnn | 张量层 matmul |
| 并行粒度 | Kernel 级别 | 元素/行级别 |
| 任务调度 | CUDA Stream | Rayon ThreadPool |
| 配置方式 | 环境变量/CUDA API | GlobalPool API |

## 3. 核心架构

```
┌─────────────────────────────────────────────────────────────┐
│                        Module                                │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   Sequential                         │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐            │   │
│  │  │  Linear │→ │ LayerNorm│→ │   GELU  │→ ...       │   │
│  │  │  Layer  │  │         │  │         │            │   │
│  │  └─────────┘  └─────────┘  └─────────┘            │   │
│  └─────────────────────────────────────────────────────┘   │
│                          ↓                                   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   ModuleList                          │   │
│  │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐           │   │
│  │  │Layer 1│ │Layer 2│ │Layer 3│ │Layer N│  ...     │   │
│  │  └───────┘ └───────┘ └───────┘ └───────┘           │   │
│  └─────────────────────────────────────────────────────┘   │
│                          ↓                                   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   ModuleDict                          │   │
│  │  {"encoder": Encoder, "decoder": Decoder, ...}      │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## 4. 核心 Trait 定义

### 4.1 Module Trait

```rust
use crate::tensor::Tensor;
use std::collections::HashMap;

/// 所有神经网络模块的基 trait
pub trait Module: Send + Sync {
    /// 前向传播
    fn forward(&self, input: &Tensor) -> Tensor;

    /// 获取所有参数
    fn parameters(&self) -> Vec<Tensor>;

    /// 获取所有参数的名称和引用
    fn named_parameters(&self) -> HashMap<String, Tensor>;

    /// 获取模块名称
    fn name(&self) -> String;

    /// 应用函数到所有参数
    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync;

    /// 克隆（需要手动实现 Clone 以支持不同层类型）
    fn clone_module(&self) -> Box<dyn Module>;
}

/// 默认实现
impl<T: Module> Module for Box<T> {
    fn forward(&self, input: &Tensor) -> Tensor {
        (**self).forward(input)
    }

    fn parameters(&self) -> Vec<Tensor> {
        (**self).parameters()
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        (**self).named_parameters()
    }

    fn name(&self) -> String {
        (**self).name()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        (**self).apply(f)
    }

    fn clone_module(&self) -> Box<dyn Module> {
        (**self).clone_module()
    }
}
```

## 5. 层实现

> ⚠️ **设计原则**: 并行计算已在张量层优化，层代码保持简洁，直接调用 `tensor.matmul()` 即可自动并行。

### 5.1 Linear（全连接层）

```rust
/// 线性变换: y = xW^T + b
/// API 风格: torch.nn.Linear(in_features, out_features, bias=True)
#[derive(Clone)]
pub struct Linear {
    weight: Tensor,              // [out_features, in_features]
    bias: Option<Tensor>,       // [out_features]
    in_features: usize,
    out_features: usize,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let scale = (6.0 / (in_features + out_features) as f32).sqrt();
        let weight = Tensor::uniform(vec![out_features, in_features], -scale, scale).unwrap();
        let bias = Tensor::zeros(vec![out_features]).ok();
        Self { weight, bias, in_features, out_features }
    }

    pub fn new_no_bias(in_features: usize, out_features: usize) -> Self {
        let scale = (6.0 / (in_features + out_features) as f32).sqrt();
        let weight = Tensor::uniform(vec![out_features, in_features], -scale, scale).unwrap();
        Self { weight, bias: None, in_features, out_features }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        // 并行计算在 tensor.matmul() 内部自动完成
        let output = input.matmul(&self.weight).unwrap();
        match &self.bias {
            Some(bias) => (output + bias).unwrap(),
            None => output,
        }
    }

    // ... 其他方法保持不变 ...
}

impl Module for Linear {
    fn forward(&self, input: &Tensor) -> Tensor {
        self.forward(input)
    }
    fn parameters(&self) -> Vec<Tensor> {
        match &self.bias {
            Some(bias) => vec![self.weight.clone(), bias.clone()],
            None => vec![self.weight.clone()],
        }
    }
    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut params = HashMap::new();
        params.insert("weight".to_string(), self.weight.clone());
        if let Some(ref bias) = self.bias {
            params.insert("bias".to_string(), bias.clone());
        }
        params
    }
    fn name(&self) -> String {
        format!("Linear({}, {})", self.in_features, self.out_features)
    }
    fn apply<F>(&mut self, f: F) where F: Fn(&mut Tensor) + Send + Sync {
        f(&mut self.weight);
        if let Some(ref mut bias) = self.bias { f(bias); }
    }
    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}
```

### 5.2 LayerNorm

```rust
/// 层归一化: y = (x - mean) / sqrt(var + eps) * gamma + beta
/// API 风格: torch.nn.LayerNorm(normalized_shape, eps=1e-5, elementwise_affine=True)
#[derive(Clone)]
pub struct LayerNorm {
    normalized_shape: Vec<usize>,
    eps: f32,
    weight: Option<Tensor>,
    bias: Option<Tensor>,
}

impl LayerNorm {
    pub fn new(normalized_shape: Vec<usize>) -> Self {
        Self::with_affine(normalized_shape, true)
    }

    pub fn with_affine(normalized_shape: Vec<usize>, elementwise_affine: bool) -> Self {
        let weight = if elementwise_affine {
            Some(Tensor::ones(normalized_shape.clone()).ok().unwrap())
        } else {
            None
        };

        let bias = if elementwise_affine {
            Some(Tensor::zeros(normalized_shape.clone()).ok().unwrap())
        } else {
            None
        };

        Self {
            normalized_shape,
            eps: 1e-5,
            weight,
            bias,
        }
    }

    pub fn eps(mut self, eps: f32) -> Self {
        self.eps = eps;
        self
    }
}

impl Module for LayerNorm {
    fn forward(&self, input: &Tensor) -> Tensor {
        let shape = input.shape();
        let ndim = shape.len();
        let normalized_shape_len = self.normalized_shape.len();
        
        // 计算最后 normalized_shape_len 个维度的均值和方差
        let mut result = input.clone();
        
        // TODO: 实现完整的 LayerNorm 计算
        // 需要计算每个样本的均值和方差，然后归一化
        
        result
    }

    fn parameters(&self) -> Vec<Tensor> {
        let mut params = Vec::new();
        if let Some(ref w) = self.weight {
            params.push(w.clone());
        }
        if let Some(ref b) = self.bias {
            params.push(b.clone());
        }
        params
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut params = HashMap::new();
        if let Some(ref w) = self.weight {
            params.insert("weight".to_string(), w.clone());
        }
        if let Some(ref b) = self.bias {
            params.insert("bias".to_string(), b.clone());
        }
        params
    }

    fn name(&self) -> String {
        format!("LayerNorm({:?})", self.normalized_shape)
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        if let Some(ref mut w) = self.weight {
            f(w);
        }
        if let Some(ref mut b) = self.bias {
            f(b);
        }
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}
```

### 5.3 Dropout

```rust
/// Dropout: 在训练时随机置零元素，推理时不做处理
/// API 风格: torch.nn.Dropout(p=0.5, inplace=False)
#[derive(Clone)]
pub struct Dropout {
    p: f32,
    inplace: bool,
    training: bool,
}

impl Dropout {
    pub fn new(p: f32) -> Self {
        assert!(p >= 0.0 && p <= 1.0, "Dropout probability must be in [0, 1]");
        Self {
            p,
            inplace: false,
            training: true,
        }
    }

    pub fn inplace(mut self, inplace: bool) -> Self {
        self.inplace = inplace;
        self
    }

    pub fn eval(&mut self) {
        self.training = false;
    }

    pub fn train(&mut self) {
        self.training = true;
    }

    pub fn is_training(&self) -> bool {
        self.training
    }
}

impl Module for Dropout {
    fn forward(&self, input: &Tensor) -> Tensor {
        if !self.training || self.p == 0.0 {
            return input.clone();
        }

        let p_keep = 1.0 - self.p;
        
        input.map(|_, x| {
            if rand::random::<f32>() < p_keep {
                x / p_keep
            } else {
                0.0
            }
        })
    }

    fn parameters(&self) -> Vec<Tensor> {
        Vec::new()
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        HashMap::new()
    }

    fn name(&self) -> String {
        format!("Dropout(p={})", self.p)
    }

    fn apply<F>(&mut self, _f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        // Dropout 没有参数
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}
```

### 5.4 Embedding

```rust
/// 词嵌入层
/// API 风格: torch.nn.Embedding(num_embeddings, embedding_dim, padding_idx=None)
#[derive(Clone)]
pub struct Embedding {
    weight: Tensor,
    num_embeddings: usize,
    embedding_dim: usize,
    padding_idx: Option<usize>,
}

impl Embedding {
    pub fn new(num_embeddings: usize, embedding_dim: usize) -> Self {
        let weight = Tensor::normal(
            vec![num_embeddings, embedding_dim],
            0.0,
            1.0 / (embedding_dim as f32).sqrt(),
        ).unwrap();

        Self {
            weight,
            num_embeddings,
            embedding_dim,
            padding_idx: None,
        }
    }

    pub fn from_pretrained(weight: Tensor) -> Self {
        let shape = weight.shape();
        assert!(shape.len() == 2, "Pretrained weight must be 2D");
        let num_embeddings = shape[0];
        let embedding_dim = shape[1];

        Self {
            weight,
            num_embeddings,
            embedding_dim,
            padding_idx: None,
        }
    }

    pub fn forward(&self, indices: &[usize]) -> Tensor {
        // indices: [batch_size, seq_len]
        // output: [batch_size, seq_len, embedding_dim]
        
        let batch_size = indices.len();
        let seq_len = if indices.is_empty() { 0 } else { 1 }; // 简化处理
        
        let mut result = Vec::with_capacity(batch_size * self.embedding_dim);
        
        for &idx in indices {
            if let Some(pad_idx) = self.padding_idx {
                if idx == pad_idx {
                    // 返回零向量
                    result.extend(vec![0.0f32; self.embedding_dim]);
                } else {
                    // 获取嵌入向量
                    for j in 0..self.embedding_dim {
                        result.push(self.weight[[idx, j]]);
                    }
                }
            } else {
                for j in 0..self.embedding_dim {
                    result.push(self.weight[[idx, j]]);
                }
            }
        }
        
        Tensor::build(result, vec![batch_size, self.embedding_dim]).unwrap()
    }
}

impl Module for Embedding {
    fn forward(&self, input: &Tensor) -> Tensor {
        // input: [batch_size, seq_len] 的索引
        // 需要特殊处理，暂时返回占位
        unimplemented!("Embedding forward requires index tensor handling")
    }

    fn parameters(&self) -> Vec<Tensor> {
        vec![self.weight.clone()]
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut params = HashMap::new();
        params.insert("weight".to_string(), self.weight.clone());
        params
    }

    fn name(&self) -> String {
        format!("Embedding({}, {})", self.num_embeddings, self.embedding_dim)
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        f(&mut self.weight);
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}
```

## 6. 容器模块

### 6.1 Sequential

```rust
/// 顺序容器 - 按顺序执行各层
/// API 风格: torch.nn.Sequential(*layers)
pub struct Sequential {
    layers: Vec<Box<dyn Module>>,
    names: Vec<String>,
}

impl Sequential {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn add(mut self, layer: Box<dyn Module>, name: Option<String>) -> Self {
        self.layers.push(layer);
        self.names.push(name.unwrap_or_else(|| format!("layer_{}", self.layers.len())));
        self
    }

    pub fn push(&mut self, layer: Box<dyn Module>) {
        self.layers.push(layer);
        self.names.push(format!("layer_{}", self.layers.len()));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Module> {
        let idx = self.names.iter().position(|n| n == name)?;
        Some(self.layers[idx].as_ref())
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Module> {
        let idx = self.names.iter().position(|n| n == name)?;
        Some(self.layers[idx].as_mut())
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}

impl Default for Sequential {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for Sequential {
    fn forward(&self, input: &Tensor) -> Tensor {
        self.layers.iter().fold(input.clone(), |x, layer| {
            layer.forward(&x)
        })
    }

    fn parameters(&self) -> Vec<Tensor> {
        self.layers.iter()
            .flat_map(|layer| layer.parameters())
            .collect()
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut result = HashMap::new();
        for (name, layer) in self.names.iter().zip(self.layers.iter()) {
            for (param_name, param) in layer.named_parameters() {
                result.insert(format!("{}.{}", name, param_name), param);
            }
        }
        result
    }

    fn name(&self) -> String {
        "Sequential".to_string()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        for layer in &mut self.layers {
            layer.apply(&f);
        }
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(Self {
            layers: self.layers.iter().map(|l| l.clone_module()).collect(),
            names: self.names.clone(),
        })
    }
}

/// Builder 模式方便创建
impl Sequential {
    pub fn builder() -> SequentialBuilder {
        SequentialBuilder::new()
    }
}

pub struct SequentialBuilder {
    layers: Vec<Box<dyn Module>>,
    names: Vec<String>,
}

impl SequentialBuilder {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn layer(mut self, name: impl Into<String>, layer: Box<dyn Module>) -> Self {
        self.names.push(name.into());
        self.layers.push(layer);
        self
    }

    pub fn linear(mut self, in_features: usize, out_features: usize) -> Self {
        self.layer(
            format!("linear_{}", self.layers.len()),
            Box::new(Linear::new(in_features, out_features)),
        )
    }

    pub fn relu(self) -> Self {
        self.layer(
            format!("relu_{}", self.layers.len()),
            Box::new(formula::activation::ReLU::new()),
        )
    }

    pub fn dropout(self, p: f32) -> Self {
        self.layer(
            format!("dropout_{}", self.layers.len()),
            Box::new(Dropout::new(p)),
        )
    }

    pub fn gelu(self) -> Self {
        self.layer(
            format!("gelu_{}", self.layers.len()),
            Box::new(formula::activation::GELU::new()),
        )
    }

    pub fn build(self) -> Sequential {
        Sequential {
            layers: self.layers,
            names: self.names,
        }
    }
}
```

### 6.2 ModuleList

```rust
/// ModuleList - 类似 Python 列表的容器
/// API 风格: torch.nn.ModuleList()
pub struct ModuleList {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleList {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn push(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub fn extend(&mut self, modules: Vec<Box<dyn Module>>) {
        self.modules.extend(modules);
    }

    pub fn get(&self, index: usize) -> Option<&dyn Module> {
        self.modules.get(index).map(|m| m.as_ref())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut dyn Module> {
        self.modules.get_mut(index).map(|m| m.as_mut())
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

impl Default for ModuleList {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for ModuleList {
    fn forward(&self, _input: &Tensor) -> Tensor {
        unimplemented!("ModuleList doesn't implement forward directly")
    }

    fn parameters(&self) -> Vec<Tensor> {
        self.modules.iter()
            .flat_map(|m| m.parameters())
            .collect()
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut result = HashMap::new();
        for (idx, module) in self.modules.iter().enumerate() {
            for (name, param) in module.named_parameters() {
                result.insert(format!("{}[{}]", name, idx), param);
            }
        }
        result
    }

    fn name(&self) -> String {
        "ModuleList".to_string()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        for module in &mut self.modules {
            module.apply(&f);
        }
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(Self {
            modules: self.modules.iter().map(|m| m.clone_module()).collect(),
        })
    }
}
```

### 6.3 ModuleDict

```rust
/// ModuleDict - 类似 Python 字典的容器
/// API 风格: torch.nn.ModuleDict()
pub struct ModuleDict {
    modules: HashMap<String, Box<dyn Module>>,
}

impl ModuleDict {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, module: Box<dyn Module>) {
        self.modules.insert(key, module);
    }

    pub fn get(&self, key: &str) -> Option<&dyn Module> {
        self.modules.get(key).map(|m| m.as_ref())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut dyn Module> {
        self.modules.get_mut(key).map(|m| m.as_mut())
    }

    pub fn keys(&self) -> Vec<&String> {
        self.modules.keys().collect()
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

impl Default for ModuleDict {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for ModuleDict {
    fn forward(&self, _input: &Tensor) -> Tensor {
        unimplemented!("ModuleDict doesn't implement forward directly")
    }

    fn parameters(&self) -> Vec<Tensor> {
        self.modules.values()
            .flat_map(|m| m.parameters())
            .collect()
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut result = HashMap::new();
        for (key, module) in &self.modules {
            for (name, param) in module.named_parameters() {
                result.insert(format!("{}.{}", key, name), param);
            }
        }
        result
    }

    fn name(&self) -> String {
        "ModuleDict".to_string()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        for module in self.modules.values_mut() {
            module.apply(&f);
        }
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(Self {
            modules: self.modules.iter()
                .map(|(k, v)| (k.clone(), v.clone_module()))
                .collect(),
        })
    }
}
```

## 7. 网络示例

### 7.1 MLP (多层感知机)

```rust
/// MLP 示例
pub struct MLP {
    layers: Sequential,
}

impl MLP {
    pub fn new(input_size: usize, hidden_sizes: &[usize], output_size: usize, dropout: f32) -> Self {
        let mut builder = Sequential::builder();
        
        let mut prev_size = input_size;
        for &hidden_size in hidden_sizes {
            builder = builder
                .linear(prev_size, hidden_size)
                .relu();
            
            if dropout > 0.0 {
                builder = builder.dropout(dropout);
            }
            
            prev_size = hidden_size;
        }
        
        builder = builder.linear(prev_size, output_size);
        
        Self {
            layers: builder.build(),
        }
    }
}

impl Module for MLP {
    fn forward(&self, input: &Tensor) -> Tensor {
        self.layers.forward(input)
    }

    fn parameters(&self) -> Vec<Tensor> {
        self.layers.parameters()
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        self.layers.named_parameters()
    }

    fn name(&self) -> String {
        "MLP".to_string()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        self.layers.apply(f);
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(Self {
            layers: *self.layers.clone_module().downcast().unwrap(),
        })
    }
}
```

### 7.2 Transformer Encoder Layer

```rust
/// Transformer Encoder Layer
pub struct TransformerEncoderLayer {
    self_attn: Option<Box<dyn Module>>,  // MultiHeadAttention
    linear1: Linear,
    linear2: Linear,
    norm1: LayerNorm,
    norm2: LayerNorm,
    dropout: Dropout,
    activation: Box<dyn Module>,  // GELU or ReLU
}

impl TransformerEncoderLayer {
    pub fn new(
        d_model: usize,
        nhead: usize,
        dim_feedforward: usize,
        dropout: f32,
    ) -> Self {
        Self {
            self_attn: None,  // 后续实现 Attention 后填充
            linear1: Linear::new(d_model, dim_feedforward),
            linear2: Linear::new(dim_feedforward, d_model),
            norm1: LayerNorm::new(vec![d_model]),
            norm2: LayerNorm::new(vec![d_model]),
            dropout: Dropout::new(dropout),
            activation: Box::new(formula::activation::GELU::new()),
        }
    }
}

impl Module for TransformerEncoderLayer {
    fn forward(&self, input: &Tensor) -> Tensor {
        // x = x + self.dropout(self.attention(x))
        // x = self.norm1(x)
        // x = x + self.dropout(self.linear2(self.activation(self.linear1(x))))
        // x = self.norm2(x)
        
        // 简化实现
        let x = input.clone();
        let x = self.norm1.forward(&x);
        let x = self.dropout.forward(&x);
        
        // Self-attention 后续实现
        let x = self.norm2.forward(&x);
        let x = self.activation.forward(&x);
        let x = self.linear1.forward(&x);
        let x = self.activation.forward(&x);
        let x = self.dropout.forward(&x);
        let x = self.linear2.forward(&x);
        
        x
    }

    fn parameters(&self) -> Vec<Tensor> {
        let mut params = Vec::new();
        params.extend(self.linear1.parameters());
        params.extend(self.linear2.parameters());
        params.extend(self.norm1.parameters());
        params.extend(self.norm2.parameters());
        params
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut params = HashMap::new();
        for (k, v) in self.linear1.named_parameters() {
            params.insert(format!("linear1.{}", k), v);
        }
        for (k, v) in self.linear2.named_parameters() {
            params.insert(format!("linear2.{}", k), v);
        }
        for (k, v) in self.norm1.named_parameters() {
            params.insert(format!("norm1.{}", k), v);
        }
        for (k, v) in self.norm2.named_parameters() {
            params.insert(format!("norm2.{}", k), v);
        }
        params
    }

    fn name(&self) -> String {
        "TransformerEncoderLayer".to_string()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        self.linear1.apply(&f);
        self.linear2.apply(&f);
        self.norm1.apply(&f);
        self.norm2.apply(&f);
    }

    fn clone_module(&self) -> Box<dyn Module> {
        unimplemented!()
    }
}
```

## 8. 工具函数

### 8.1 权重初始化

```rust
/// 权重初始化模块
pub mod init {
    use crate::tensor::Tensor;

    /// Xavier 均匀初始化
    pub fn xavier_uniform_(tensor: &mut Tensor, gain: f32) {
        let shape = tensor.shape();
        let (fan_in, fan_out) = if shape.len() == 2 {
            (shape[1] as f32, shape[0] as f32)
        } else {
            (shape.iter().product::<usize>() as f32, shape.iter().product::<usize>() as f32)
        };
        
        let std = gain * (2.0 / (fan_in + fan_out)).sqrt();
        let bound = std * (3.0_f32).sqrt();
        
        // 重新填充随机值
        *tensor = Tensor::uniform(shape.clone(), -bound, bound).unwrap();
    }

    /// Xavier 正态初始化
    pub fn xavier_normal_(tensor: &mut Tensor, gain: f32) {
        let shape = tensor.shape();
        let (fan_in, fan_out) = if shape.len() == 2 {
            (shape[1] as f32, shape[0] as f32)
        } else {
            (shape.iter().product::<usize>() as f32, shape.iter().product::<usize>() as f32)
        };
        
        let std = gain * (2.0 / (fan_in + fan_out)).sqrt();
        
        *tensor = Tensor::normal(shape.clone(), 0.0, std).unwrap();
    }

    /// Kaiming 均匀初始化（ReLU 推荐）
    pub fn kaiming_uniform_(tensor: &mut Tensor, a: f32, mode: &str) {
        let shape = tensor.shape();
        let fan = if mode == "fan_in" {
            shape[1] as f32
        } else {
            shape[0] as f32
        };
        
        let bound = (1.0 / fan).sqrt() * ((1.0 + a * a) as f32).sqrt();
        
        *tensor = Tensor::uniform(shape.clone(), -bound, bound).unwrap();
    }

    /// Kaiming 正态初始化
    pub fn kaiming_normal_(tensor: &mut Tensor, a: f32, mode: &str) {
        let shape = tensor.shape();
        let fan = if mode == "fan_in" {
            shape[1] as f32
        } else {
            shape[0] as f32
        };
        
        let std = (1.0 / fan).sqrt() * ((1.0 + a * a) as f32).sqrt();
        
        *tensor = Tensor::normal(shape.clone(), 0.0, std).unwrap();
    }
}
```

### 8.2 权重保存与加载

```rust
/// 模型序列化
pub mod serialization {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{Read, Write};

    #[derive(Debug)]
    pub struct ModelState {
        pub parameters: HashMap<String, Tensor>,
    }

    impl ModelState {
        pub fn new() -> Self {
            Self {
                parameters: HashMap::new(),
            }
        }

        pub fn insert(&mut self, name: String, tensor: Tensor) {
            self.parameters.insert(name, tensor);
        }

        pub fn save(&self, path: &str) -> std::io::Result<()> {
            let mut file = File::create(path)?;
            
            for (name, tensor) in &self.parameters {
                let shape = tensor.shape();
                let data = tensor.data();
                
                // 简单的二进制格式
                let name_bytes = name.as_bytes();
                file.write_all(&(name_bytes.len() as u32).to_le_bytes())?;
                file.write_all(name_bytes)?;
                file.write_all(&(shape.len() as u32).to_le_bytes())?;
                for &dim in shape {
                    file.write_all(&(dim as u32).to_le_bytes())?;
                }
                file.write_all(&(data.len() as u64).to_le_bytes())?;
                for &val in data {
                    file.write_all(&val.to_le_bytes())?;
                }
            }
            
            Ok(())
        }

        pub fn load(path: &str) -> std::io::Result<Self> {
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            
            let mut state = ModelState::new();
            let mut offset = 0;
            
            while offset < buffer.len() {
                // 读取名称
                let name_len = u32::from_le_bytes([
                    buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3]
                ]) as usize;
                offset += 4;
                
                let name = String::from_utf8_lossy(&buffer[offset..offset+name_len]).to_string();
                offset += name_len;
                
                // 读取形状
                let ndim = u32::from_le_bytes([
                    buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3]
                ]) as usize;
                offset += 4;
                
                let shape: Vec<usize> = (0..ndim).map(|_| {
                    let dim = u32::from_le_bytes([
                        buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3]
                    ]) as usize;
                    offset += 4;
                    dim
                }).collect();
                
                // 读取数据
                let data_len = u64::from_le_bytes([
                    buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
                    buffer[offset+4], buffer[offset+5], buffer[offset+6], buffer[offset+7]
                ]) as usize;
                offset += 8;
                
                let data: Vec<f32> = (0..data_len).map(|_| {
                    let val = f32::from_le_bytes([
                        buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3]
                    ]);
                    offset += 4;
                    val
                }).collect();
                
                let tensor = Tensor::build(data, shape).unwrap();
                state.insert(name, tensor);
            }
            
            Ok(state)
        }
    }
}
```

## 9. 模块结构

```
crates/
├── formula/                      # 数学公式库
│   ├── src/
│   │   ├── activation/          # 激活函数
│   │   │   ├── mod.rs
│   │   │   ├── relu.rs
│   │   │   ├── sigmoid.rs
│   │   │   ├── gelu.rs
│   │   │   └── ...
│   │   └── normal.rs
│   └── Cargo.toml
│
├── tensor/                       # 张量库
│   ├── src/
│   │   ├── tensor.rs
│   │   ├── operator.rs
│   │   └── ...
│   └── Cargo.toml
│
└── nn/                           # 新增：神经网络库
    ├── src/
    │   ├── lib.rs               # 模块入口
    │   ├── module.rs            # Module trait
    │   ├── sequential.rs         # Sequential 容器
    │   ├── module_list.rs        # ModuleList 容器
    │   ├── module_dict.rs        # ModuleDict 容器
    │   ├── layers/
    │   │   ├── mod.rs
    │   │   ├── linear.rs         # Linear 层
    │   │   ├── layer_norm.rs     # LayerNorm
    │   │   ├── dropout.rs        # Dropout
    │   │   ├── embedding.rs      # Embedding
    │   │   └── ...
    │   ├── init.rs               # 初始化函数
    │   ├── serialization.rs      # 模型保存/加载
    │   └── examples/
    │       ├── mlp.rs           # MLP 示例
    │       └── transformer.rs    # Transformer 示例
    └── Cargo.toml
```

## 10. 使用示例

### 10.1 构建 MLP

```rust
use nn::{Module, Sequential, Linear};
use formula::activation::{ReLU, GELU};

// 方式 1: 使用 builder
let model = Sequential::builder()
    .linear(784, 256)
    .relu()
    .linear(256, 128)
    .relu()
    .linear(128, 10)
    .build();

// 方式 2: 使用 add 方法
let model = Sequential::new()
    .add(Box::new(Linear::new(784, 256)), Some("fc1".to_string()))
    .add(Box::new(ReLU::new()), Some("relu1".to_string()))
    .add(Box::new(Linear::new(256, 128)), Some("fc2".to_string()))
    .add(Box::new(ReLU::new()), Some("relu2".to_string()))
    .add(Box::new(Linear::new(128, 10)), Some("fc3".to_string()));

// 前向传播
let input = Tensor::randn(vec![32, 784]);
let output = model.forward(&input);
println!("Output shape: {:?}", output.shape());

// 获取参数
let params = model.parameters();
println!("Number of parameters: {}", params.len());

// 权重初始化
use nn::init;
model.apply(|t| init::kaiming_uniform_(t, 0.01, "fan_in"));
```

### 10.2 保存和加载模型

```rust
use nn::serialization::{ModelState, serialization};

// 保存模型
let state = ModelState::new();
for (name, param) in model.named_parameters() {
    state.insert(name, param);
}
state.save("model.bin")?;

// 加载模型
let loaded_state = ModelState::load("model.bin")?;
```

### 10.3 自定义模块

```rust
use nn::{Module, module::Module};
use crate::tensor::Tensor;
use std::collections::HashMap;

pub struct MyCustomModule {
    linear: Linear,
    another_layer: Linear,
}

impl MyCustomModule {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        Self {
            linear: Linear::new(input_size, hidden_size),
            another_layer: Linear::new(hidden_size, output_size),
        }
    }
}

impl Module for MyCustomModule {
    fn forward(&self, input: &Tensor) -> Tensor {
        let x = self.linear.forward(input);
        self.another_layer.forward(&x)
    }

    fn parameters(&self) -> Vec<Tensor> {
        let mut params = Vec::new();
        params.extend(self.linear.parameters());
        params.extend(self.another_layer.parameters());
        params
    }

    fn named_parameters(&self) -> HashMap<String, Tensor> {
        let mut params = HashMap::new();
        for (k, v) in self.linear.named_parameters() {
            params.insert(format!("linear.{}", k), v);
        }
        for (k, v) in self.another_layer.named_parameters() {
            params.insert(format!("another_layer.{}", k), v);
        }
        params
    }

    fn name(&self) -> String {
        "MyCustomModule".to_string()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&mut Tensor) + Send + Sync {
        self.linear.apply(&f);
        self.another_layer.apply(&f);
    }

    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(Self {
            linear: self.linear.clone(),
            another_layer: self.another_layer.clone(),
        })
    }
}
```

## 11. 后续扩展

| 功能                 | 优先级 | 说明              |
| ------------------ | --- | --------------- |
| MultiHeadAttention | P1  | Transformer 核心  |
| Conv2d             | P1  | 图像任务            |
| BatchNorm          | P1  | 批归一化            |
| RNN/LSTM/GRU       | P2  | 序列任务            |
| Loss Functions     | P0  | 损失函数            |
| Optimizer          | P0  | 优化器 (SGD, Adam) |
| Gradient Tracking  | P0  | 反向传播支持          |

## 12. GlobalPool API（并行调度）

`optim` crate 提供的并行调度接口，用于张量运算的智能任务拆分。

### 12.1 API 列表

```rust
pub struct GlobalPool;

impl GlobalPool {
    /// 获取总线程数
    pub fn num_threads() -> usize { ... }
    
    /// 获取当前可用并行度
    pub fn available_parallelism() -> usize { ... }
    
    /// 判断是否应该并行化（元素数 >= 阈值）
    pub fn should_parallelize(total_items: usize) -> bool { ... }
    
    /// 计算最优分块大小
    pub fn parallel_chunk_size(total_items: usize) -> usize { ... }
    
    /// 计算最优 chunk 数量（不超过线程数）
    pub fn optimal_num_chunks(total_items: usize) -> usize { ... }
}
```

### 12.2 并行决策逻辑

```rust
// matvec_parallel: 向量 × 矩阵
fn matvec_parallel(a: &[f32], k: usize, b: &[f32], n: usize) -> Vec<f32> {
    let num_chunks = GlobalPool::optimal_num_chunks(n).min(256);
    
    if num_chunks <= 1 || n < 1000 {
        // 串行：小矩阵避免调度开销
        serial_compute(a, k, b, n)
    } else {
        // 并行：智能分块
        let chunk_size = (n + num_chunks - 1) / num_chunks;
        (0..num_chunks).into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = (start + chunk_size).min(n);
                compute_chunk(a, k, b, start, end)
            })
            .flatten_iter()
            .collect()
    }
}

// matmul_2d_row_parallel: 矩阵 × 矩阵
fn matmul_2d_row_parallel(a: &[f32], m: usize, k: usize, b: &[f32], n: usize) -> Vec<f32> {
    let num_chunks = GlobalPool::optimal_num_chunks(m).min(256);
    
    if num_chunks <= 1 || m * n < 10000 {
        // 串行：小矩阵避免调度开销
        serial_matmul(a, m, k, b, n)
    } else {
        // 并行：按行分块
        let chunk_size = (m + num_chunks - 1) / num_chunks;
        (0..num_chunks).into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = (start + chunk_size).min(m);
                compute_rows(a, k, b, start, end, n)
            })
            .flatten_iter()
            .collect()
    }
}
```

### 12.3 智能阈值

| 阈值 | 值 | 说明 |
|------|-----|------|
| `PARALLEL_THRESHOLD` | 100,000 | 元素数超过此值才考虑并行 |
| `n < 1000` | 1000 | 向量维度小于此值串行 |
| `m * n < 10000` | 10000 | 矩阵元素小于此值串行 |
| `num_chunks.min(256)` | 256 | 最大并行任务数限制 |

### 12.4 使用示例

```rust
use optim::GlobalPool;

// 检查是否值得并行
if GlobalPool::should_parallelize(500_000) {
    // 创建最优数量的并行任务
    let chunks = GlobalPool::optimal_num_chunks(500_000);
    let chunk_size = GlobalPool::parallel_chunk_size(500_000);
    
    println!("并行: {} 个 chunks, 每个 {} 个元素", chunks, chunk_size);
}

// 神经网络典型场景
// batch=32, seq=512, hidden=768
// attention: (32*512, 768) × (768, 768) = (16384, 768)
// 输出元素: 16384 × 768 = 12,582,912 > 阈值
// 自动并行分配到可用线程
```

### 12.5 与 Rayon 的协作

```rust
// GlobalPool 内部使用 Rayon ThreadPool
static GLOBAL_POOL: once_cell::sync::Lazy<ThreadPool> =
    once_cell::sync::Lazy::new(|| {
        ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())  // 自动检测 CPU 核心数
            .thread_name(|i| format!("rayon-{}", i))
            .build()
            .unwrap()
    });

// 任务提交示例
(0..num_chunks).into_par_iter()
    .map(|chunk_idx| { /* 计算逻辑 */ })
    .flatten_iter()
    .collect()
```

### 12.6 配置方式

```rust
// 全局配置（影响所有并行操作）
PoolConfig::default()
    .with_num_threads(8)                    // 固定线程数
    .with_thread_name_prefix("mlearn-worker");

// 查询当前配置
let info = GlobalPool::pool_info();
println!("线程数: {}", info.num_threads);
```

## 13. 开发任务列表

### 13.1 第一阶段：激活函数库 (P0)

> 目标：参考 PyTorch torch.nn.functional 接口，实现常用激活函数

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| A01 | ReLU | `F.relu` | P0 | 待开发 | y = max(0, x) |
| A02 | LeakyReLU | `F.leaky_relu` | P0 | 待开发 | y = x if x > 0 else slope * x |
| A03 | Sigmoid | `F.sigmoid` | P0 | 待开发 | y = 1 / (1 + exp(-x)) |
| A04 | Tanh | `F.tanh` | P0 | 待开发 | y = (exp(x) - exp(-x)) / (exp(x) + exp(-x)) |
| A05 | Softmax | `F.softmax` | P0 | 待开发 | y_i = exp(x_i) / sum(exp(x_j)) |
| A06 | GELU | `F.gelu` | P0 | 待开发 | y = 0.5 * x * (1 + tanh(sqrt(2/pi) * (x + 0.044715 * x^3))) |
| A07 | SiLU (Swish) | `F.silu` | P1 | 待开发 | y = x * sigmoid(x) |
| A08 | Mish | `F.mish` | P1 | 待开发 | y = x * tanh(softplus(x)) |
| A09 | ELU | `F.elu` | P2 | 待开发 | y = x if x > 0 else alpha * (exp(x) - 1) |
| A10 | Hardswish | `F.hardswish` | P2 | 待开发 | y = 0 if x <= -3, x if x >= 3, else x * (x + 3) / 6 |
| A11 | Hardtanh | `F.hardtanh` | P2 | 待开发 | y = min_val if x < min_val, max_val if x > max_val, else x |
| A12 | PReLU | `F.prelu` | P3 | 待开发 | 可学习负斜率的 LeakyReLU |

### 13.2 第一阶段：常用函数 (P0)

> 目标：实现常用的张量操作函数

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| F01 | Dropout | `F.dropout` | P0 | 待开发 | 训练时随机置零 |
| F02 | LayerNorm | `F.layer_norm` | P0 | 待开发 | 层归一化 |
| F03 | BatchNorm1d | `F.batch_norm` | P0 | 待开发 | 批归一化 (1D) |
| F04 | BatchNorm2d | `F.batch_norm` | P0 | 待开发 | 批归一化 (2D) |
| F05 | Embedding | `F.embedding` | P0 | 待开发 | 词嵌入查找表 |
| F06 | Flatten | `F.flatten` | P1 | 待开发 | 张量展平 |
| F07 | Reshape | `F.reshape` | P1 | 待开发 | 张量变形 |
| F08 | Transpose | `F.transpose` | P1 | 待开发 | 维度交换 |

### 13.3 第二阶段：全连接层网络 (P0)

> 目标：构建 MLP 和基础 FC 网络，支持 Sequential 容器

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| N01 | Module Trait | `nn.Module` | P0 | 待开发 | 基础模块接口 |
| N02 | Linear | `nn.Linear` | P0 | 待开发 | 线性全连接层 |
| N03 | Sequential | `nn.Sequential` | P0 | 待开发 | 顺序容器 |
| N04 | ModuleList | `nn.ModuleList` | P1 | 待开发 | 列表容器 |
| N05 | ModuleDict | `nn.ModuleDict` | P2 | 待开发 | 字典容器 |
| N06 | ParameterList | `nn.ParameterList` | P3 | 待开发 | 参数列表 |
| N07 | Dropout 层 | `nn.Dropout` | P0 | 待开发 | Dropout 层封装 |
| N08 | LayerNorm 层 | `nn.LayerNorm` | P0 | 待开发 | LayerNorm 层封装 |

### 13.4 第二阶段：损失函数 (P0)

> 目标：实现常用损失函数，支持训练

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| L01 | MSELoss | `nn.MSELoss` | P0 | 待开发 | 均方误差损失 |
| L02 | CrossEntropyLoss | `nn.CrossEntropyLoss` | P0 | 待开发 | 交叉熵损失 |
| L03 | BCELoss | `nn.BCELoss` | P1 | 待开发 | 二分类交叉熵 |
| L04 | L1Loss | `nn.L1Loss` | P1 | 待开发 | L1 损失 |
| L05 | SmoothL1Loss | `nn.SmoothL1Loss` | P2 | 待开发 | 平滑 L1 损失 |

### 13.5 第二阶段：优化器 (P0)

> 目标：实现常用优化器，支持参数更新

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| O01 | SGD | `torch.optim.SGD` | P0 | 待开发 | 随机梯度下降 |
| O02 | Adam | `torch.optim.Adam` | P0 | 待开发 | Adam 优化器 |
| O03 | AdamW | `torch.optim.AdamW` | P1 | 待开发 | Adam with Weight Decay |
| O04 | RMSprop | `torch.optim.RMSprop` | P2 | 待开发 | RMSprop 优化器 |

### 13.6 第二阶段：权重初始化 (P0)

| 编号 | 功能 | 说明 | 优先级 | 状态 |
|------|------|------|--------|------|
| I01 | Xavier Uniform | Xavier 均匀初始化 | P0 | 待开发 |
| I02 | Xavier Normal | Xavier 正态初始化 | P0 | 待开发 |
| I03 | Kaiming Uniform | Kaiming 均匀初始化 | P0 | 待开发 |
| I04 | Kaiming Normal | Kaiming 正态初始化 | P0 | 待开发 |
| I05 | Uniform | 均匀分布初始化 | P1 | 待开发 |
| I06 | Normal | 正态分布初始化 | P1 | 待开发 |
| I07 | Constant | 常数初始化 | P2 | 待开发 |
| I08 | Orthogonal | 正交初始化 | P3 | 待开发 |

### 13.7 第三阶段：卷积网络 (P1)

> 目标：实现 CNN 相关层

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| C01 | Conv1d | `nn.Conv1d` | P1 | 待开发 | 一维卷积 |
| C02 | Conv2d | `nn.Conv2d` | P1 | 待开发 | 二维卷积 |
| C03 | ConvTranspose1d | `nn.ConvTranspose1d` | P2 | 待开发 | 一维转置卷积 |
| C04 | ConvTranspose2d | `nn.ConvTranspose2d` | P2 | 待开发 | 二维转置卷积 |
| C05 | MaxPool1d | `nn.MaxPool1d` | P1 | 待开发 | 一维最大池化 |
| C06 | MaxPool2d | `nn.MaxPool2d` | P1 | 待开发 | 二维最大池化 |
| C07 | AvgPool1d | `nn.AvgPool1d` | P2 | 待开发 | 一维平均池化 |
| C08 | AvgPool2d | `nn.AvgPool2d` | P2 | 待开发 | 二维平均池化 |
| C09 | AdaptiveAvgPool2d | `nn.AdaptiveAvgPool2d` | P2 | 待开发 | 自适应平均池化 |
| C10 | BatchNorm2d | `nn.BatchNorm2d` | P1 | 待开发 | 2D 批归一化 |
| C11 | BatchNorm3d | `nn.BatchNorm3d` | P2 | 待开发 | 3D 批归一化 |

### 13.8 第三阶段：序列网络 (P1)

> 目标：实现 RNN/LSTM/GRU 等序列模型

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| S01 | RNN | `nn.RNN` | P1 | 待开发 | 循环神经网络 |
| S02 | LSTM | `nn.LSTM` | P1 | 待开发 | 长短期记忆网络 |
| S03 | GRU | `nn.GRU` | P1 | 待开发 | 门控循环单元 |
| S04 | RNNCell | `nn.RNNCell` | P2 | 待开发 | RNN 单元 |
| S05 | LSTMCell | `nn.LSTMCell` | P2 | 待开发 | LSTM 单元 |
| S06 | GRUCell | `nn.GRUCell` | P2 | 待开发 | GRU 单元 |
| S07 | EmbeddingBag | `nn.EmbeddingBag` | P2 | 待开发 | 打包词嵌入 |

### 13.9 第三阶段：注意力机制 (P1)

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| A101 | MultiheadAttention | `nn.MultiheadAttention` | P1 | 待开发 | 多头注意力 |
| A102 | scaled_dot_product_attention | `F.scaled_dot_product_attention` | P1 | 待开发 | Flash Attention |

### 13.10 第三阶段：Transformer (P1)

| 编号 | 功能 | PyTorch 对应 | 优先级 | 状态 | 说明 |
|------|------|--------------|--------|------|------|
| T01 | TransformerEncoder | `nn.TransformerEncoder` | P1 | 待开发 | Transformer 编码器 |
| T02 | TransformerDecoder | `nn.TransformerDecoder` | P1 | 待开发 | Transformer 解码器 |
| T03 | TransformerEncoderLayer | `nn.TransformerEncoderLayer` | P1 | 待开发 | 编码器层 |
| T04 | TransformerDecoderLayer | `nn.TransformerDecoderLayer` | P1 | 待开发 | 解码器层 |
| T05 | Transformer | `nn.Transformer` | P2 | 待开发 | 完整 Transformer |

### 13.11 第四阶段：工具和序列化 (P2)

| 编号 | 功能 | 说明 | 优先级 | 状态 |
|------|------|------|--------|------|
| U01 | 模型保存 | `torch.save` | P2 | 待开发 |
| U02 | 模型加载 | `torch.load` | P2 | 待开发 |
| U03 | 参数统计 | 参数量、梯度统计 | P2 | 待开发 |
| U04 | 模型克隆 | 深拷贝模型 | P2 | 待开发 |
| U05 | GPU 支持 | CUDA 支持 | P3 | 规划中 |
| U06 | 混合精度 | FP16/FP32 混合精度 | P3 | 规划中 |

---

## 开发优先级总结

```
┌─────────────────────────────────────────────────────────────────────┐
│                         第一阶段 (P0)                                 │
│  ┌─────────────────────────────────────────────────────────────┐     │
│  │ 激活函数 (A01-A06)  │  常用函数 (F01-F05)                │     │
│  │ 网络层 (N01-N03, N07-N08)  │  损失函数 (L01-L02)       │     │
│  │ 优化器 (O01-O02)  │  初始化 (I01-I04)                   │     │
│  └─────────────────────────────────────────────────────────────┘     │
│                          ↓                                          │
│                         第二阶段 (P1)                                 │
│  ┌─────────────────────────────────────────────────────────────┐     │
│  │ 进阶激活 (A07-A08)  │  进阶函数 (F06-F08)                │     │
│  │ 容器扩展 (N04-N06)  │  进阶损失 (L03-L05)               │     │
│  │ 进阶优化 (O03-O04)  │  初始化 (I05-I08)                 │     │
│  └─────────────────────────────────────────────────────────────┘     │
│                          ↓                                          │
│                         第三阶段 (P1-P2)                              │
│  ┌─────────────────────────────────────────────────────────────┐     │
│  │ CNN (C01-C11)  │  RNN/LSTM/GRU (S01-S07)                │     │
│  │ 注意力 (A101-A102)  │  Transformer (T01-T05)              │     │
│  └─────────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

## 快速开始

```rust
// 1. 创建 Sequential 网络
let model = Sequential::builder()
    .linear(784, 256)
    .relu()
    .linear(256, 128)
    .relu()
    .linear(128, 10)
    .build();

// 2. 前向传播
let input = Tensor::randn(vec![32, 784]);
let output = model.forward(&input);

// 3. 计算损失
let target = Tensor::randn(vec![32, 10]);
let loss = CrossEntropyLoss::forward(&output, &target);

// 4. 反向传播和优化
let gradients = loss.backward();
optimizer.step();
```


