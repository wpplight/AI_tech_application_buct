# 自动微分与反向传播设计

## 1. 核心概念

### 1.1 梯度本质

反向传播的本质是**链式法则**：

```
L = f(g(x))

∂L/∂x = ∂f/∂g · ∂g/∂x
```

对于神经网络：
- **前向传播**: `y = Wx + b`
- **反向传播**: 计算 `∂L/∂W`, `∂L/∂x`, `∂L/∂b`

### 1.2 核心公式

```rust
// 前向传播: y = x @ W^T + b
// 输入: x [batch, in], 权重: W [out, in], 偏置: b [out]
// 输出: y [batch, out]

// 反向传播 (核心!)
// dL_dy 是上游传来的梯度 [batch, out]
// 
// dL_dW = x.T @ dL_dy      // [in, batch] @ [batch, out] = [in, out]
// dL_db = sum(dL_dy, axis=0)  // [out]
// dL_dx = dL_dy @ W         // [batch, out] @ [out, in] = [batch, in]
```

## 2. 设计原则

### 2.1 三大核心方法

每个层必须实现：

```rust
pub trait Module: Send + Sync {
    fn forward(&self, input: &Tensor) -> Tensor;
    
    // 新增：反向传播
    fn backward(&self, grad_output: &Tensor) -> Gradients;
    
    // 新增：保存前向传播的中间结果（用于反向传播）
    fn cache(&self) -> Option<Box<dyn Any>>;
}

// 梯度容器
pub struct Gradients {
    pub d_input: Tensor,    // ∂L/∂输入
    pub params: Vec<(String, Tensor)>,  // 参数名 -> ∂L/∂参数
}
```

### 2.2 梯度流动

```
┌─────────────────────────────────────────────────────────────┐
│                     前向传播                               │
│                                                             │
│  x ──▶ Layer1 ──▶ Layer2 ──▶ Layer3 ──▶ y                   │
│              │          │          │                      │
│           cache1      cache2      cache3     保存中间结果      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                     反向传播                               │
│                                                             │
│  dL_dy ──▶ Layer3 ──▶ Layer2 ──▶ Layer1 ──▶ dL_dx        │
│           backward    backward   backward                    │
│              │          │          │                       │
│           dL_dW3     dL_dW2    dL_dW1      更新权重              │
└─────────────────────────────────────────────────────────────┘
```

## 3. 层实现规范

### 3.1 Linear 层

```rust
pub struct Linear {
    weight: Tensor,           // [in, out]
    bias: Option<Tensor>,    // [out]
    in_features: usize,
    out_features: usize,
}

pub struct LinearCache {
    input: Tensor,            // 保存前向传播的输入
}

impl Module for Linear {
    fn forward(&self, input: &Tensor) -> Tensor {
        let output = input.matmul(&self.weight).unwrap();
        
        // 添加 bias（支持广播）
        ...
        
        // 保存 cache 用于反向传播
        self.cache = Some(Box::new(LinearCache {
            input: input.clone(),
        }));
        
        output
    }
    
    fn backward(&self, grad_output: &Tensor) -> Gradients {
        let cache = self.cache.take()
            .expect("Must call forward() before backward()");
        let cache: LinearCache = *(cache.downcast().unwrap());
        
        // 1. dL_dW = x.T @ dL_dy
        // cache.input: [batch, in]
        // grad_output: [batch, out]
        // dL_dW: [in, out]
        let dL_dW = cache.input.transpose().unwrap()
            .matmul(grad_output).unwrap();
        
        // 2. dL_db = sum(dL_dy, axis=0)
        let dL_db = grad_output.sum_axis(0);  // [out]
        
        // 3. dL_dx = dL_dy @ W
        // grad_output: [batch, out]
        // self.weight: [in, out]
        // dL_dx: [batch, in]
        let dL_dx = grad_output.matmul(&self.weight.transpose().unwrap()).unwrap();
        
        Gradients {
            d_input: dL_dx,
            params: vec![
                ("weight".to_string(), dL_dW),
                ("bias".to_string(), dL_db),
            ],
        }
    }
    
    fn cache(&self) -> Option<Box<dyn Any>> {
        self.cache.take().map(|c| Box::new(*c))
    }
}
```

### 3.2 ReLU 层

```rust
pub struct ReLUCache {
    input: Tensor,           // 保存输入用于计算 mask
}

impl Module for ReLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        let output = input.map(|_, x| x.max(0.0));
        
        self.cache = Some(Box::new(ReLUCache {
            input: input.clone(),
        }));
        
        output
    }
    
    fn backward(&self, grad_output: &Tensor) -> Gradients {
        let cache = self.cache.take()
            .expect("Must call forward() before backward()");
        let cache: ReLUCache = *(cache.downcast().unwrap());
        
        // ReLU 的导数: dL_dx = dL_dy * mask
        // mask = 1 if input > 0 else 0
        let mask = cache.input.map(|_, x| if x > 0.0 { 1.0 } else { 0.0 });
        let dL_dx = (grad_output * mask).unwrap();
        
        Gradients {
            d_input: dL_dx,
            params: vec![],  // ReLU 没有参数
        }
    }
}
```

### 3.3 Sequential 容器

```rust
pub struct Sequential {
    layers: Vec<Box<dyn Module>>,
    caches: Vec<Option<Box<dyn Any>>>,  // 保存每层的 cache
}

impl Sequential {
    pub fn forward(&self, input: &Tensor) -> Tensor {
        let mut x = input.clone();
        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x
    }
    
    pub fn backward(&mut self, grad_output: &Tensor) -> Gradients {
        let mut grad = grad_output.clone();
        let mut param_grads = Vec::new();
        
        // 反向遍历各层
        for (i, layer) in self.layers.iter().rev().enumerate() {
            let layer_grad = layer.backward(&grad);
            grad = layer_grad.d_input;
            
            // 累加参数梯度
            for (name, param_grad) in layer_grad.params {
                param_grads.push((format!("{}.{}", i, name), param_grad));
            }
        }
        
        Gradients {
            d_input: grad,
            params: param_grads,
        }
    }
}
```

## 4. 优化器接口

### 4.1 梯度更新

```rust
pub trait Optimizer {
    fn step(&mut self, grads: &Gradients);
}

pub struct SGD {
    params: Vec<Tensor>,
    lr: f32,
}

impl SGD {
    pub fn new(params: Vec<Tensor>, lr: f32) -> Self {
        Self { params, lr }
    }
}

impl Optimizer for SGD {
    fn step(&mut self, grads: &Gradients) {
        // W = W - lr * dL_dW
        for (name, grad) in &grads.params {
            if let Some(param) = self.params.iter_mut().find(|p| p.name() == *name) {
                let update = (param * self.lr).unwrap();
                *param = (param - update).unwrap();
            }
        }
    }
}
```

### 4.2 训练循环

```rust
fn train_step(
    model: &mut Sequential,
    optimizer: &mut SGD,
    input: &Tensor,
    target: &Tensor,
) -> f32 {
    // 1. 前向传播
    let output = model.forward(input);
    
    // 2. 计算损失
    let loss = mse_loss(&output, target);
    
    // 3. 反向传播 (dL_dy = 1)
    let dL_dy = Tensor::ones(output.shape());
    let grads = model.backward(&dL_dy);
    
    // 4. 更新参数
    optimizer.step(&grads);
    
    loss
}
```

## 5. 损失函数

### 5.1 MSE Loss

```rust
pub fn mse_loss(pred: &Tensor, target: &Tensor) -> Tensor {
    let diff = (pred - target).unwrap();
    let squared = diff * &diff;
    let mean = squared.mean();
    mean
}

pub fn mse_loss_backward(pred: &Tensor, target: &Tensor) -> Tensor {
    // dL/dpred = 2 * (pred - target) / n
    let diff = (pred - target).unwrap();
    let n = pred.data().len() as f32;
    ((diff * 2.0).unwrap() / n).unwrap()
}
```

### 5.2 CrossEntropy Loss

```rust
pub fn cross_entropy_loss(pred: &Tensor, target: &Tensor) -> Tensor {
    // softmax + log + nll_loss
    let softmax_pred = pred.softmax(-1);
    let log_pred = softmax_pred.map(|_, x| x.max(1e-8).ln());
    let loss = (log_pred * target).unwrap().mean_axis(-1);
    -loss
}
```

## 6. 完整示例

```rust
use nn::{Sequential, Linear, ReLULayer, Module};
use tensor::Tensor;

fn main() {
    // 创建模型
    let model = Sequential::new()
        .add("fc1", Linear::new(2, 10))
        .add("relu", ReLULayer::new())
        .add("fc2", Linear::new(10, 1));

    // 创建优化器
    let params = model.parameters();
    let mut optimizer = SGD::new(params, 0.01);

    // 训练数据
    let x = Tensor::build(vec![3.0, 4.0], vec![2]).unwrap();
    let y = Tensor::build(vec![7.0], vec![1]).unwrap();

    // 训练循环
    for epoch in 0..1000 {
        // 前向
        let pred = model.forward(&x);
        
        // 损失
        let loss = mse_loss(&pred, &y);
        if epoch % 100 == 0 {
            println!("Epoch {}: Loss = {:?}", epoch, loss.data());
        }

        // 反向
        let dL_dy = mse_loss_backward(&pred, &y);
        let grads = model.backward(&dL_dy);

        // 更新
        optimizer.step(&grads);
    }
}
```

## 7. API 总结

### 7.1 Module Trait

```rust
pub trait Module: Send + Sync {
    // 前向传播
    fn forward(&self, input: &Tensor) -> Tensor;
    
    // 反向传播
    fn backward(&self, grad_output: &Tensor) -> Gradients;
    
    // 获取参数
    fn parameters(&self) -> Vec<Tensor>;
    fn named_parameters(&self) -> HashMap<String, Tensor>;
    
    // 缓存管理
    fn cache(&self) -> Option<Box<dyn Any>>;
}
```

### 7.2 Gradients 结构

```rust
pub struct Gradients {
    pub d_input: Tensor,              // ∂L/∂输入
    pub params: Vec<(String, Tensor)>, // (参数名, ∂L/∂参数)
}
```

### 7.3 核心公式速查

| 层 | 前向 | dL_dW | dL_dx |
|----|------|--------|--------|
| Linear | y = xW^T + b | x^T @ dL_dy | dL_dy @ W |
| ReLU | y = max(0, x) | - | dL_dy * mask |
| Softmax | y_i = exp(x_i)/Σ | 略 | 略 |
| Dropout | y = x * mask | - | dL_dy * mask |

## 8. 实现优先级

| 优先级 | 功能 | 说明 |
|--------|------|------|
| P0 | Module.backward() | 核心反向传播 |
| P0 | Linear.backward() | 线性层梯度 |
| P0 | Gradients 结构 | 梯度容器 |
| P0 | SGD 优化器 | 参数更新 |
| P1 | ReLU.backward() | 激活函数梯度 |
| P1 | MSE Loss | 损失函数 |
| P1 | Sequential.backward() | 容器反向传播 |
| P2 | CrossEntropy Loss | 交叉熵损失 |
| P2 | Adam 优化器 | 自适应学习率 |
| P3 | BatchNorm 层 | 带状态的层 |
