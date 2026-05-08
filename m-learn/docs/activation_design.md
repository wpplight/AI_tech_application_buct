# 激活函数模块设计

## 1. 概述

本模块参考 PyTorch 的 `torch.nn` 激活函数接口，为 m-learn 提供统一的激活函数实现。

## 2. 设计目标

- 提供与 PyTorch 风格一致的 API 接口
- 支持 in-place 操作以节省内存
- 支持多维 Tensor 输入
- 自动利用 Rayon 进行并行计算
- 提供激活函数的一阶导数（用于反向传播）

## 3. API 设计

### 3.1 基础 Trait 定义

```rust
/// 激活函数的 trait，所有激活函数必须实现此接口
pub trait Activation: Send + Sync {
    /// 前向传播：计算激活值
    fn forward(&self, input: &Tensor) -> Tensor;

    /// 原地前向传播：不分配新内存
    fn forward_inplace(&self, input: &mut Tensor);

    /// 获取激活函数名称
    fn name(&self) -> &str;

    /// 获取激活函数的配置参数
    fn config(&self) -> ActivationConfig;
}

/// 激活函数配置信息
#[derive(Debug, Clone)]
pub struct ActivationConfig {
    pub name: String,
    pub inplace: bool,
    pub extra_params: HashMap<String, f32>,
}
```

### 3.2 激活函数实现

#### 3.2.1 ReLU

```rust
/// ReLU: max(0, x)
/// API 风格: torch.nn.ReLU(inplace=False)
#[derive(Clone)]
pub struct ReLU {
    inplace: bool,
}

impl ReLU {
    pub fn new() -> Self {
        Self { inplace: false }
    }

    pub fn inplace(mut self, inplace: bool) -> Self {
        self.inplace = inplace;
        self
    }
}

impl Activation for ReLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        if self.inplace {
            let mut result = input.clone();
            self.forward_inplace(&mut result);
            result
        } else {
            input.map(|_, x| x.max(0.0))
        }
    }

    fn forward_inplace(&self, input: &mut Tensor) {
        for val in input.get_data_mut() {
            *val = val.max(0.0);
        }
    }

    fn name(&self) -> &str {
        "ReLU"
    }

    fn config(&self) -> ActivationConfig {
        ActivationConfig {
            name: self.name().to_string(),
            inplace: self.inplace,
            extra_params: HashMap::new(),
        }
    }
}
```

#### 3.2.2 LeakyReLU

```rust
/// LeakyReLU: x if x > 0 else negative_slope * x
/// API 风格: torch.nn.LeakyReLU(negative_slope=0.01, inplace=False)
#[derive(Clone)]
pub struct LeakyReLU {
    negative_slope: f32,
    inplace: bool,
}

impl LeakyReLU {
    pub fn new() -> Self {
        Self {
            negative_slope: 0.01,
            inplace: false,
        }
    }

    pub fn negative_slope(mut self, value: f32) -> Self {
        self.negative_slope = value;
        self
    }

    pub fn inplace(mut self, inplace: bool) -> Self {
        self.inplace = inplace;
        self
    }
}

impl Activation for LeakyReLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        let slope = self.negative_slope;
        input.map(move |_, x| if x > 0.0 { x } else { slope * x })
    }

    fn name(&self) -> &str {
        "LeakyReLU"
    }

    fn config(&self) -> ActivationConfig {
        let mut params = HashMap::new();
        params.insert("negative_slope".to_string(), self.negative_slope);
        ActivationConfig {
            name: self.name().to_string(),
            inplace: self.inplace,
            extra_params: params,
        }
    }
}
```

#### 3.2.3 Sigmoid

```rust
/// Sigmoid: 1 / (1 + exp(-x))
/// API 风格: torch.nn.Sigmoid()
#[derive(Clone)]
pub struct Sigmoid;

impl Sigmoid {
    pub fn new() -> Self {
        Self
    }
}

impl Activation for Sigmoid {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.map(|_, x| 1.0 / (1.0 + (-x).exp()))
    }

    fn name(&self) -> &str {
        "Sigmoid"
    }

    fn config(&self) -> ActivationConfig {
        ActivationConfig {
            name: self.name().to_string(),
            inplace: false,
            extra_params: HashMap::new(),
        }
    }
}
```

#### 3.2.4 Tanh

```rust
/// Tanh: (exp(x) - exp(-x)) / (exp(x) + exp(-x))
/// API 风格: torch.nn.Tanh()
#[derive(Clone)]
pub struct Tanh;

impl Tanh {
    pub fn new() -> Self {
        Self
    }
}

impl Activation for Tanh {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.map(|_, x| x.tanh())
    }

    fn name(&self) -> &str {
        "Tanh"
    }

    fn config(&self) -> ActivationConfig {
        ActivationConfig {
            name: self.name().to_string(),
            inplace: false,
            extra_params: HashMap::new(),
        }
    }
}
```

#### 3.2.5 Softmax

```rust
/// Softmax: exp(x_i) / sum(exp(x_j))
/// API 风格: torch.nn.Softmax(dim=1)
#[derive(Clone)]
pub struct Softmax {
    dim: usize,
}

impl Softmax {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl Activation for Softmax {
    fn forward(&self, input: &Tensor) -> Tensor {
        let dim = self.dim;
        let shape = input.shape();
        let ndim = shape.len();

        if dim >= ndim {
            panic!("Softmax dimension {} out of range for tensor with {} dimensions", dim, ndim);
        }

        let stride = shape[dim..].iter().product::<usize>();
        let batch_size = shape[..dim].iter().product::<usize>();

        let mut result = Vec::with_capacity(input.data().len());

        for batch in 0..batch_size {
            let offset = batch * stride * shape[dim];
            let slice = &input.data()[offset..offset + stride * shape[dim]];

            let max_val = slice.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

            let exp_sum: f32 = slice
                .chunks(stride)
                .map(|chunk| chunk.iter().map(|&x| (x - max_val).exp()).sum::<f32>())
                .sum();

            for chunk in slice.chunks(stride) {
                for &x in chunk {
                    result.push((x - max_val).exp() / exp_sum);
                }
            }
        }

        Tensor::build(result, shape.to_vec()).unwrap()
    }

    fn name(&self) -> &str {
        "Softmax"
    }

    fn config(&self) -> ActivationConfig {
        let mut params = HashMap::new();
        params.insert("dim".to_string(), self.dim as f32);
        ActivationConfig {
            name: self.name().to_string(),
            inplace: false,
            extra_params: params,
        }
    }
}
```

#### 3.2.6 GELU

```rust
/// GELU: 0.5 * x * (1 + tanh(sqrt(2/pi) * (x + 0.044715 * x^3)))
/// API 风格: torch.nn.GELU(approximate='none')
#[derive(Clone)]
pub struct GELU {
    approximate: GELUApproximate,
}

#[derive(Clone)]
pub enum GELUApproximate {
    None,
    Tanh,
}

impl Default for GELU {
    fn default() -> Self {
        Self::new()
    }
}

impl GELU {
    pub fn new() -> Self {
        Self {
            approximate: GELUApproximate::None,
        }
    }

    pub fn approximate(mut self, mode: GELUApproximate) -> Self {
        self.approximate = mode;
        self
    }
}

impl Activation for GELU {
    fn forward(&self, input: &Tensor) -> Tensor {
        match self.approximate {
            GELUApproximate::None => {
                input.map(|_, x| {
                    let cdf = 0.5 * (1.0 + (x * (1.0 + 0.044715 * x * x)).tanh());
                    x * cdf
                })
            }
            GELUApproximate::Tanh => {
                input.map(|_, x| {
                    let x3 = x * x * x;
                    let inner = 0.7978845608028654 * (x + 0.044715 * x3);
                    0.5 * x * (1.0 + inner.tanh())
                })
            }
        }
    }

    fn name(&self) -> &str {
        "GELU"
    }

    fn config(&self) -> ActivationConfig {
        let mut params = HashMap::new();
        params.insert(
            "approximate".to_string(),
            match self.approximate {
                GELUApproximate::None => 0.0,
                GELUApproximate::Tanh => 1.0,
            },
        );
        ActivationConfig {
            name: self.name().to_string(),
            inplace: false,
            extra_params: params,
        }
    }
}
```

#### 3.2.7 SiLU / Swish

```rust
/// SiLU (Swish): x * sigmoid(x)
/// API 风格: torch.nn.SiLU()
#[derive(Clone)]
pub struct SiLU;

impl SiLU {
    pub fn new() -> Self {
        Self
    }
}

impl Activation for SiLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.map(|_, x| {
            let sigmoid = 1.0 / (1.0 + (-x).exp());
            x * sigmoid
        })
    }

    fn name(&self) -> &str {
        "SiLU"
    }

    fn config(&self) -> ActivationConfig {
        ActivationConfig {
            name: self.name().to_string(),
            inplace: false,
            extra_params: HashMap::new(),
        }
    }
}
```

#### 3.2.8 Mish

```rust
/// Mish: x * tanh(softplus(x))
/// API 风格: torch.nn.Mish()
#[derive(Clone)]
pub struct Mish;

impl Mish {
    pub fn new() -> Self {
        Self
    }
}

impl Activation for Mish {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.map(|_, x| {
            let softplus = (1.0 + (-x).exp()).ln().max(x);
            let tanh_softplus = softplus.tanh();
            x * tanh_softplus
        })
    }

    fn name(&self) -> &str {
        "Mish"
    }

    fn config(&self) -> ActivationConfig {
        ActivationConfig {
            name: self.name().to_string(),
            inplace: false,
            extra_params: HashMap::new(),
        }
    }
}
```

#### 3.2.9 ELU

```rust
/// ELU: x if x > 0 else alpha * (exp(x) - 1)
/// API 风格: torch.nn.ELU(alpha=1.0, inplace=False)
#[derive(Clone)]
pub struct ELU {
    alpha: f32,
    inplace: bool,
}

impl ELU {
    pub fn new() -> Self {
        Self {
            alpha: 1.0,
            inplace: false,
        }
    }

    pub fn alpha(mut self, value: f32) -> Self {
        self.alpha = value;
        self
    }

    pub fn inplace(mut self, inplace: bool) -> Self {
        self.inplace = inplace;
        self
    }
}

impl Activation for ELU {
    fn forward(&self, input: &Tensor) -> Tensor {
        let alpha = self.alpha;
        input.map(move |_, x| {
            if x > 0.0 {
                x
            } else {
                alpha * ((x).exp() - 1.0)
            }
        })
    }

    fn name(&self) -> &str {
        "ELU"
    }

    fn config(&self) -> ActivationConfig {
        let mut params = HashMap::new();
        params.insert("alpha".to_string(), self.alpha);
        ActivationConfig {
            name: self.name().to_string(),
            inplace: self.inplace,
            extra_params: params,
        }
    }
}
```

#### 3.2.10 Hardswish

```rust
/// Hardswish: 0 if x <= -3, x if x >= 3, else x * (x + 3) / 6
/// API 风格: torch.nn.Hardswish(inplace=False)
#[derive(Clone)]
pub struct Hardswish {
    inplace: bool,
}

impl Hardswish {
    pub fn new() -> Self {
        Self { inplace: false }
    }

    pub fn inplace(mut self, inplace: bool) -> Self {
        self.inplace = inplace;
        self
    }
}

impl Activation for Hardswish {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.map(|_, x| {
            if x <= -3.0 {
                0.0
            } else if x >= 3.0 {
                x
            } else {
                x * (x + 3.0) / 6.0
            }
        })
    }

    fn name(&self) -> &str {
        "Hardswish"
    }

    fn config(&self) -> ActivationConfig {
        ActivationConfig {
            name: self.name().to_string(),
            inplace: self.inplace,
            extra_params: HashMap::new(),
        }
    }
}
```

## 4. 激活函数注册表

```rust
/// 激活函数工厂，用于根据名称创建激活函数
pub struct ActivationRegistry;

impl ActivationRegistry {
    /// 从名称创建激活函数实例
    pub fn create(name: &str) -> Option<Box<dyn Activation>> {
        match name.to_lowercase().as_str() {
            "relu" => Some(Box::new(ReLU::new())),
            "sigmoid" => Some(Box::new(Sigmoid::new())),
            "tanh" => Some(Box::new(Tanh::new())),
            "softmax" => Some(Box::new(Softmax::new(1))),
            "gelu" => Some(Box::new(GELU::new())),
            "silu" | "swish" => Some(Box::new(SiLU::new())),
            "mish" => Some(Box::new(Mish::new())),
            "elu" => Some(Box::new(ELU::new())),
            "leakyrelu" => Some(Box::new(LeakyReLU::new())),
            "hardswish" => Some(Box::new(Hardswish::new())),
            _ => None,
        }
    }

    /// 获取所有支持的激活函数名称
    pub fn available() -> Vec<&'static str> {
        vec![
            "relu", "sigmoid", "tanh", "softmax", "gelu",
            "silu", "swish", "mish", "elu", "leakyrelu",
            "hardswish",
        ]
    }
}
```

## 5. 模块结构

```
crates/
├── formula/                    # 数学公式库
│   ├── src/
│   │   ├── lib.rs
│   │   ├── normal.rs          # 正态分布
│   │   └── activation/        # 新增：激活函数
│   │       ├── mod.rs         # 模块入口
│   │       ├── trait.rs       # Activation trait 定义
│   │       ├── relu.rs        # ReLU 系列
│   │       ├── sigmoid.rs     # Sigmoid 系列
│   │       ├── softmax.rs      # Softmax
│   │       ├── gelu.rs        # GELU
│   │       ├── silu.rs        # SiLU / Swish
│   │       ├── mish.rs        # Mish
│   │       ├── elu.rs         # ELU
│   │       └── registry.rs    # 注册表
│   └── Cargo.toml
```

## 6. 使用示例

### 6.1 基本使用

```rust
use formula::activation::{ReLU, Sigmoid, GELU};

// 创建激活函数
let relu = ReLU::new();
let sigmoid = Sigmoid::new();
let gelu = GELU::new().approximate(GELUApproximate::Tanh);

// 应用激活函数
let input = Tensor::randn(vec![32, 128]);
let output = relu.forward(&input);

// 使用注册表
let activation = ActivationRegistry::create("relu").unwrap();
let result = activation.forward(&input);
```

### 6.2 链式调用

```rust
// 在神经网络中使用
let x = Tensor::randn(vec![64, 256]);
let x = ReLU::new().inplace(true).forward(&x);
let x = GELU::new().approximate(GELUApproximate::Tanh).forward(&x);
let x = LayerNorm::new(256).forward(&x);
```

## 7. 实现优先级

| 优先级 | 激活函数 | 说明 |
|--------|----------|------|
| P0 | ReLU, Sigmoid, Tanh | 最基础，必需 |
| P1 | Softmax, GELU, SiLU | Transformer 必需 |
| P2 | LeakyReLU, ELU, Mish | 进阶激活函数 |
| P3 | Hardswish, Hardtanh | 移动端优化 |

## 8. 性能考虑

- **并行化**: 利用 Rayon 对所有元素操作进行并行计算
- **原地操作**: `inplace=true` 时复用输入内存
- **数值稳定性**: Softmax 等函数需要数值稳定化处理
- **SIMD**: 考虑使用 SIMD 加速核心计算
