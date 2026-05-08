use tensor::Tensor;
use crate::{Module, Gradients};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Trainer<M: Module + ApplyGradients> {
    model: M,
    lr: f32,
}

impl<M: Module + ApplyGradients> Trainer<M> {
    pub fn new(model: M, lr: f32) -> Self {
        Self { model, lr }
    }

    pub fn train_step(&mut self, input: &Tensor, target: &Tensor) -> f32 {
        let pred = self.model.forward(input);
        let loss = Self::mse_loss(&pred, target);
        let d_loss = Self::mse_loss_grad(&pred, target);
        let grads = self.model.backward(&d_loss);
        self.model.apply_gradients(grads, self.lr);
        loss
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        self.model.forward(input)
    }

    fn mse_loss(pred: &Tensor, target: &Tensor) -> f32 {
        let diff = (pred - target).unwrap();
        let squared = (&diff * &diff).unwrap();
        squared.sum() / pred.data().len() as f32
    }

    fn mse_loss_grad(pred: &Tensor, target: &Tensor) -> Tensor {
        let n = pred.data().len() as f32;
        let diff = (pred - target).unwrap();
        diff.map_with(n, |_, x, n| 2.0 * x / n)
    }
}

pub trait ApplyGradients {
    fn apply_gradients(&mut self, grads: Gradients, lr: f32);
}

impl ApplyGradients for crate::layers::Linear {
    fn apply_gradients(&mut self, grads: Gradients, lr: f32) {
        for (name, grad) in grads.params {
            let scaled = grad.map_with(lr, |_, x, lr| x * lr);
            if name == "weight" {
                let current = (*self.weight()).clone();
                let updated = (&current - &scaled).unwrap();
                self.set_weight(updated);
            } else if name == "bias" {
                let current = self.bias().unwrap();
                let updated = (&current - &scaled).unwrap();
                self.set_bias(updated);
            }
        }
    }
}

impl ApplyGradients for crate::layers::ReLULayer {
    fn apply_gradients(&mut self, _grads: Gradients, _lr: f32) {
        // ReLU has no parameters, nothing to update
    }
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
