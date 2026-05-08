use tensor::Tensor;
use crate::Module;
use crate::loss::LossResult;

pub struct Sequential {
    layers: Vec<Box<dyn Module>>,
}

impl Sequential {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
        }
    }
    
    pub fn add(&mut self, layer: impl Module + 'static) {
        self.layers.push(Box::new(layer));
    }
}

impl Module for Sequential {
    fn forward(&mut self, x: &Tensor) -> Tensor {
        let mut input = x.clone();
        for layer in &mut self.layers {
            input = layer.forward(&input);
        }
        input
    }
    
    fn backward(&mut self, grad: &LossResult) -> Tensor {
        let mut grad_tensor = grad.grad().clone();
        let batch_size = grad.batch_size();
        
        for layer in self.layers.iter_mut().rev() {
            grad_tensor = layer.backward(&LossResult::new(0.0, grad_tensor, batch_size));
        }
        
        grad_tensor
    }
    
    fn update(&mut self, lr: f32) {
        for layer in &mut self.layers {
            layer.update(lr);
        }
    }
    
    fn parameters(&self) -> Vec<Tensor> {
        let mut params = Vec::new();
        for layer in &self.layers {
            params.extend(layer.parameters());
        }
        params
    }
    
    fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        let mut params = Vec::new();
        for layer in &mut self.layers {
            params.extend(layer.parameters_mut());
        }
        params
    }
}

pub struct SequentialBuilder {
    seq: Sequential,
}

impl SequentialBuilder {
    pub fn new() -> Self {
        Self {
            seq: Sequential::new(),
        }
    }
    
    pub fn add(mut self, layer: impl Module + 'static) -> Self {
        self.seq.add(layer);
        self
    }
    
    pub fn build(self) -> Sequential {
        self.seq
    }
}
