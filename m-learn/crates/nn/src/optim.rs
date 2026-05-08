use tensor::Tensor;
use crate::Module;

pub struct SGD {
    lr: f32,
}

impl SGD {
    pub fn new(lr: f32) -> Self {
        Self { lr }
    }
    
    pub fn step(&self, layers: &mut [&mut dyn Module]) {
        let lr = self.lr;
        for layer in layers.iter_mut() {
            layer.update(lr);
        }
    }
}

pub fn sgd(lr: f32) -> SGD {
    SGD::new(lr)
}

pub fn mse_loss(pred: &Tensor, target: &Tensor) -> f32 {
    let diff = (pred - target).unwrap();
    let squared = (&diff * &diff).unwrap();
    squared.sum() / pred.data().len() as f32
}

pub fn mse_loss_grad(pred: &Tensor, target: &Tensor) -> Tensor {
    let n = pred.data().len() as f32;
    let diff = (pred - target).unwrap();
    diff.map_with(n, |_, x, n| 2.0 * x / n)
}
