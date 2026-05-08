use tensor::Tensor;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Gradients {
    pub d_input: Tensor,
    pub params: Vec<(String, Tensor)>,
}

impl Gradients {
    pub fn new(d_input: Tensor) -> Self {
        Self { d_input, params: Vec::new() }
    }

    pub fn with_params(mut self, params: Vec<(String, Tensor)>) -> Self {
        self.params = params;
        self
    }
}

pub trait Module: Send + Sync {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn backward(&self, input: &Tensor, grad_output: &Tensor) -> Gradients;
    fn parameters(&self) -> Vec<Tensor> { Vec::new() }
    fn set_parameter(&mut self, _name: &str, _tensor: Tensor) {}
}

pub struct Sequential {
    layers: Vec<Box<dyn Module>>,
}

impl Sequential {
    pub fn new() -> Self { Self { layers: Vec::new() } }
    
    pub fn add<M: Module + 'static>(mut self, layer: M) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        self.layers.iter().fold(input.clone(), |x, layer| layer.forward(&x))
    }

    pub fn backward(&self, x: &Tensor, grad_output: &Tensor) -> Gradients {
        let mut grad = grad_output.clone();
        let mut current_input = x.clone();
        let mut param_grads = Vec::new();
        
        for layer in self.layers.iter().rev() {
            let g = layer.backward(&current_input, &grad);
            grad = g.d_input;
            param_grads.extend(g.params);
            current_input = layer.forward(x);
        }
        Gradients::new(grad).with_params(param_grads)
    }

    pub fn parameters(&self) -> Vec<Tensor> {
        self.layers.iter().flat_map(|l| l.parameters()).collect()
    }

    pub fn train(&mut self, x: &Tensor, y: &Tensor, lr: f32, epochs: usize) {
        for _ in 0..epochs {
            let pred = self.forward(x);
            let d_loss = Self::mse_loss_grad(&pred, y);
            let grads = self.backward(x, &d_loss);
            self.apply_gradients(grads, lr);
        }
    }

    pub fn predict(&self, x: &Tensor) -> Tensor {
        self.forward(x)
    }

    fn mse_loss_grad(pred: &Tensor, target: &Tensor) -> Tensor {
        let n = pred.data().len() as f32;
        let diff = (pred - target).unwrap();
        diff.map(|x| 2.0 * x / n)
    }

    fn apply_gradients(&mut self, grads: Gradients, lr: f32) {
        let mut grad_idx = 0;
        for layer in self.layers.iter_mut() {
            let params = layer.parameters();
            for (i, (name, _)) in params.iter().enumerate() {
                if let Some((_, g)) = grads.params.get(grad_idx) {
                    let p = grads.params.iter()
                        .filter(|(n, _)| n == name)
                        .nth(0)
                        .map(|(_, t)| t)
                        .unwrap_or(&params[i]);
                    let updated = (&params[i] - &g.map(|x| x * lr)).unwrap();
                    layer.set_parameter(name, updated);
                }
                grad_idx += 1;
            }
        }
    }
}

impl Default for Sequential { fn default() -> Self { Self::new() } }

#[derive(Clone)]
pub struct Linear {
    weight: Tensor,
    bias: Tensor,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let scale = (6.0 / (in_features + out_features) as f32).sqrt();
        let weight = Tensor::normal(vec![in_features, out_features], 0.0, scale).unwrap();
        let bias = Tensor::zeros(vec![out_features]).unwrap();
        Self { weight, bias }
    }
}

impl Module for Linear {
    fn forward(&self, input: &Tensor) -> Tensor {
        let out = input.matmul(&self.weight).unwrap();
        (&out + &self.bias).unwrap()
    }

    fn backward(&self, input: &Tensor, grad_output: &Tensor) -> Gradients {
        let dL_dx = grad_output.matmul(&self.weight.transpose().unwrap()).unwrap();
        let batch = input.data().len() / self.weight.shape()[0];
        let dL_dW = compute_dL_dW(input, grad_output, batch);
        let dL_db = grad_output.clone();
        Gradients::new(dL_dx).with_params(vec![
            ("weight".to_string(), dL_dW),
            ("bias".to_string(), dL_db),
        ])
    }

    fn parameters(&self) -> Vec<Tensor> {
        vec![self.weight.clone(), self.bias.clone()]
    }

    fn set_parameter(&mut self, name: &str, tensor: Tensor) {
        match name {
            "weight" => self.weight = tensor,
            "bias" => self.bias = tensor,
            _ => {}
        }
    }
}

fn compute_dL_dW(input: &Tensor, grad: &Tensor, batch: usize) -> Tensor {
    let in_f = input.data().len() / batch;
    let out_f = grad.data().len() / batch;
    let mut result = vec![0.0f32; in_f * out_f];
    
    for o in 0..out_f {
        for i in 0..in_f {
            for b in 0..batch {
                result[i * out_f + o] += input.data()[b * in_f + i] * grad.data()[b * out_f + o];
            }
        }
    }
    Tensor::build(result, vec![in_f, out_f]).unwrap()
}

#[derive(Clone)]
pub struct ReLU;

impl Module for ReLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.map(|_, x| x.max(0.0))
    }
    fn backward(&self, input: &Tensor, grad_output: &Tensor) -> Gradients {
        let mask = input.map(|_, x| if x > 0.0 { 1.0 } else { 0.0 });
        let dL_dx = (&mask * grad_output).unwrap();
        Gradients::new(dL_dx)
    }
}
