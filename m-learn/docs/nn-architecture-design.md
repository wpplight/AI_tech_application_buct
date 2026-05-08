# Neural Network Architecture Design v2

## 支持嵌套模型的统一架构

### 1. Module Trait - 核心接口

```rust
pub trait Module: Send + Sync {
    /// 前向传播
    fn forward(&self, x: &Tensor) -> Tensor;
    
    /// 反向传播
    fn backward(&self, grad: &Tensor) -> Tensor;
    
    /// 更新参数
    fn update(&mut self, lr: f32);
    
    /// 获取所有参数
    fn parameters(&self) -> Vec<Tensor>;
    
    /// 获取参数名称 (用于调试)
    fn named_parameters(&self) -> Vec<(&'static str, Tensor)>;
}
```

### 2. 用户自定义模型示例

```rust
// 用户可以定义自己的复杂模型
pub struct ResNetBlock {
    conv1: Linear,
    conv2: Linear,
    shortcut: Option<Linear>,
    relu: ReLU,
}

impl ResNetBlock {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        Self {
            conv1: Linear::new(in_features, out_features),
            conv2: Linear::new(out_features, out_features),
            shortcut: if in_features != out_features {
                Some(Linear::new(in_features, out_features))
            } else {
                None
            },
            relu: ReLU::new(),
        }
    }
}

impl Module for ResNetBlock {
    fn forward(&self, x: &Tensor) -> Tensor {
        let identity = x.clone();
        
        let out = self.conv1.forward(x);
        let out = self.relu.forward(&out);
        let out = self.conv2.forward(&out);
        
        // Shortcut connection
        let out = if let Some(shortcut) = &self.shortcut {
            (&out + &shortcut.forward(&identity)).unwrap()
        } else {
            (&out + &identity).unwrap()
        };
        
        self.relu.forward(&out)
    }
    
    fn backward(&self, grad: &Tensor) -> Tensor {
        // 反向传播逻辑
        todo!()
    }
    
    fn update(&mut self, lr: f32) {
        self.conv1.update(lr);
        self.conv2.update(lr);
        if let Some(s) = &mut self.shortcut {
            s.update(lr);
        }
    }
    
    fn parameters(&self) -> Vec<Tensor> {
        let mut p = self.conv1.parameters();
        p.extend(self.conv2.parameters());
        if let Some(s) = &self.shortcut {
            p.extend(s.parameters());
        }
        p
    }
}
```

### 3. Sequential 容器 - 组合模型

```rust
pub struct Sequential {
    modules: Vec<Box<dyn Module>>,
}

impl Sequential {
    pub fn new() -> Self {
        Self { modules: vec![] }
    }
    
    /// 添加模块
    pub fn add<M: Module + 'static>(mut self, module: M) -> Self {
        self.modules.push(Box::new(module));
        self
    }
    
    /// 前向传播
    pub fn forward(&self, x: &Tensor) -> Tensor {
        self.modules.iter().fold(x.clone(), |acc, m| m.forward(&acc))
    }
    
    /// 反向传播
    pub fn backward(&self, grad: &Tensor) {
        let mut current = grad.clone();
        for module in self.modules.iter().rev() {
            current = module.backward(&current);
        }
    }
    
    /// 更新所有参数
    pub fn update(&mut self, lr: f32) {
        for module in self.modules.iter_mut() {
            module.update(lr);
        }
    }
    
    /// 获取所有参数
    pub fn parameters(&self) -> Vec<Tensor> {
        self.modules.iter().flat_map(|m| m.parameters()).collect()
    }
}
```

### 4. 完整使用示例

```rust
// 定义一个简单的 MLP
let mlp = Sequential::new()
    .add(Linear::new(784, 256))
    .add(ReLU::new())
    .add(Linear::new(256, 128))
    .add(ReLU::new())
    .add(Linear::new(128, 10));

// 定义一个 ResNet Block
let res_block = ResNetBlock::new(256, 256);

// 组合成更大的网络
let network = Sequential::new()
    .add(mlp)                    // 添加子网络
    .add(res_block)              // 添加 ResNet 块
    .add(Linear::new(256, 10)); // 添加最后一层

// 一键训练
network.train(&x_train, &y_train, 0.01, 100);

// 一键预测
let pred = network.predict(&x_test);
```

### 5. PyTorch 风格的用户定义模型

```rust
// 用户可以定义任何模型，只要实现 Module trait

pub struct MyModel {
    encoder: Sequential,
    backbone: ResNetBlock,
    head: Sequential,
}

impl MyModel {
    pub fn new() -> Self {
        Self {
            encoder: Sequential::new()
                .add(Linear::new(784, 256))
                .add(ReLU::new()),
            backbone: ResNetBlock::new(256, 256),
            head: Sequential::new()
                .add(Linear::new(256, 10)),
        }
    }
}

impl Module for MyModel {
    fn forward(&self, x: &Tensor) -> Tensor {
        let x = self.encoder.forward(x);
        let x = self.backbone.forward(&x);
        self.head.forward(&x)
    }
    
    fn backward(&self, grad: &Tensor) -> Tensor {
        let g = self.head.backward(grad);
        self.backbone.backward(&g)
    }
    
    fn update(&mut self, lr: f32) {
        self.encoder.update(lr);
        self.backbone.update(lr);
        self.head.update(lr);
    }
    
    fn parameters(&self) -> Vec<Tensor> {
        let mut p = self.encoder.parameters();
        p.extend(self.backbone.parameters());
        p.extend(self.head.parameters());
        p
    }
}
```

### 6. 架构图

```
┌─────────────────────────────────────────────────────────────┐
│                      MyModel                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                    Encoder                          │    │
│  │  Linear(784→256) → ReLU → Linear(256→256)         │    │
│  └─────────────────────────────────────────────────────┘    │
│                           │                                 │
│                           ▼                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   Backbone                           │    │
│  │  ResNetBlock: conv1 → relu → conv2 → + → relu     │    │
│  └─────────────────────────────────────────────────────┘    │
│                           │                                 │
│                           ▼                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                      Head                           │    │
│  │  Linear(256→10)                                    │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### 7. 文件结构

```
crates/nn/src/
├── lib.rs
├── module.rs           # Module trait 定义
├── layers/
│   ├── mod.rs
│   ├── linear.rs       # Linear 层
│   ├── relu.rs         # ReLU 层
│   └── ...
├── container/
│   ├── mod.rs
│   ├── sequential.rs   # Sequential 容器
│   └── module_list.rs  # ModuleList 容器
└── losses.rs           # 损失函数
```

### 8. 实现顺序

1. **Module Trait** - 定义核心接口
2. **Linear Layer** - 基础层实现
3. **ReLU Layer** - 激活函数
4. **Sequential Container** - 组合容器
5. **用户示例** - 展示如何自定义模型

### 9. 关键设计决策

#### 为什么用 `Box<dyn Module>`?
- 支持异构模块集合（不同类型）
- 运行时多态

#### 为什么要求 `Send + Sync`?
- 支持并行训练
- 支持 GPU 加速（未来）

#### 为什么参数用 `Vec<Tensor>`?
- 简单直接
- 易于序列化和反序列化

### 10. 扩展功能

#### ModuleList - 类似 nn.ModuleList
```rust
pub struct ModuleList {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleList {
    pub fn push<M: Module + 'static>(&mut self, module: M) {
        self.modules.push(Box::new(module));
    }
    
    pub fn get(&self, index: usize) -> &dyn Module {
        &*self.modules[index]
    }
}
```

#### 预定义模型
```rust
pub struct MLP {
    layers: Sequential,
}

impl MLP {
    pub fn new(sizes: &[usize]) -> Self {
        let mut layers = Sequential::new();
        for i in 0..sizes.len() - 1 {
            layers = layers.add(Linear::new(sizes[i], sizes[i + 1]));
            if i < sizes.len() - 2 {
                layers = layers.add(ReLU::new());
            }
        }
        Self { layers }
    }
}

impl Module for MLP {
    fn forward(&self, x: &Tensor) -> Tensor {
        self.layers.forward(x)
    }
    // ... 实现其他方法，委托给 self.layers
}
```
