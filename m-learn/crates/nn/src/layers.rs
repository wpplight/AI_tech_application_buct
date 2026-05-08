use tensor::Tensor;
use crate::loss::LossResult;
use crate::Module;

#[derive(Clone)]
pub struct Linear {
    weight: Tensor,
    bias: Tensor,
    input_cache: Tensor,
    weight_grad: Option<Tensor>,
    bias_grad: Option<Tensor>,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let scale = (2.0 / (in_features + out_features) as f32).sqrt();
        Self {
            weight: Tensor::normal(vec![in_features, out_features], 0.0, scale).unwrap(),
            bias: Tensor::zeros(vec![out_features]).unwrap(),
            input_cache: Tensor::zeros(vec![0]).unwrap(),
            weight_grad: None,
            bias_grad: None,
        }
    }
}

impl Module for Linear {
    fn forward(&mut self, x: &Tensor) -> Tensor {
        self.input_cache = x.clone();
        
        let x_normalized = x.normalize_shape();
        let out = x_normalized.matmul(&self.weight).unwrap();
        
        let out_shape = out.shape();
        let bias_shape = self.bias.shape();
        
        if out_shape == bias_shape {
            (&out + &self.bias).unwrap()
        } else if out_shape.len() == 2 && bias_shape.len() == 1 {
            let batch = out_shape[0];
            let out_features = bias_shape[0];
            let bias_data = self.bias.data();
            let out_data = out.data();
            
            let mut result = Vec::with_capacity(batch * out_features);
            for b in 0..batch {
                for f in 0..out_features {
                    result.push(out_data[b * out_features + f] + bias_data[f]);
                }
            }
            
            Tensor::build(result, vec![batch, out_features]).unwrap()
        } else {
            panic!("Unsupported shape combination: {:?} + {:?}", out_shape, bias_shape);
        }
    }
    
    fn backward(&mut self, grad: &LossResult) -> Tensor {
        let grad_tensor = grad.grad();
        
        // dL/dx = grad @ W^T
        let d_l_dx = grad_tensor.matmul(&self.weight.transpose().unwrap()).unwrap();
        
        // dL/dW = input^T @ grad
        // 注意：这个 matmul 已经对 batch 维度求和了！
        let input_transposed = self.input_cache.transpose().unwrap();
        let d_l_dw = input_transposed.matmul(&grad_tensor).unwrap();
        self.weight_grad = Some(d_l_dw);
        
        // dL/db = sum over batch（需要对 batch 求和）
        let bias_grad = grad_tensor.sum_axis(0).squeeze();
        self.bias_grad = Some(bias_grad);
        
        d_l_dx
    }
    
    fn update(&mut self, lr: f32) {
        if let Some(w_grad) = &self.weight_grad {
            let update = w_grad.map(move |_, x| x * lr);
            self.weight = (&self.weight - &update).unwrap();
        }
        if let Some(b_grad) = &self.bias_grad {
            let update = b_grad.map(move |_, x| x * lr);
            self.bias = (&self.bias - &update).unwrap();
        }
    }
    
    fn parameters(&self) -> Vec<Tensor> {
        vec![self.weight.clone(), self.bias.clone()]
    }
    
    fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        vec![&mut self.weight, &mut self.bias]
    }
}

#[derive(Clone)]
pub struct ReLU {
    mask: Tensor,
}

impl ReLU {
    pub fn new() -> Self {
        Self { mask: Tensor::zeros(vec![0]).unwrap() }
    }
}

impl Default for ReLU {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for ReLU {
    fn forward(&mut self, x: &Tensor) -> Tensor {
        self.mask = x.map(|_, v| if v > 0.0 { 1.0 } else { 0.0 });
        x.map(|_, v| v.max(0.0))
    }
    
    fn backward(&mut self, grad: &LossResult) -> Tensor {
        let grad_tensor = grad.grad();
        
        (&self.mask * grad_tensor).unwrap()
    }
    
    fn update(&mut self, _lr: f32) {}
    
    fn parameters(&self) -> Vec<Tensor> {
        vec![]
    }
    
    fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        vec![]
    }
}

#[derive(Clone)]
pub struct Tanh {
    output_cache: Tensor,
}

impl Tanh {
    pub fn new() -> Self {
        Self {
            output_cache: Tensor::zeros(vec![0]).unwrap(),
        }
    }
}

impl Default for Tanh {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for Tanh {
    fn forward(&mut self, x: &Tensor) -> Tensor {
        self.output_cache = x.map(|_, v| v.tanh());
        self.output_cache.clone()
    }
    
    fn backward(&mut self, grad: &LossResult) -> Tensor {
        let grad_tensor = grad.grad();
        
        let tanh_grad = self.output_cache.map(|_, v| 1.0 - v * v);
        (&tanh_grad * grad_tensor).unwrap()
    }
    
    fn update(&mut self, _lr: f32) {}
    
    fn parameters(&self) -> Vec<Tensor> {
        vec![]
    }
    
    fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        vec![]
    }
}
