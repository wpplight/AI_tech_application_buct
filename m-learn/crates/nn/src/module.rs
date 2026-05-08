use tensor::Tensor;
use crate::loss::LossResult;

pub trait Module: Send + Sync {
    fn forward(&mut self, x: &Tensor) -> Tensor;
    fn backward(&mut self, grad: &LossResult) -> Tensor;
    fn update(&mut self, lr: f32);
    fn parameters(&self) -> Vec<Tensor>;
    fn parameters_mut(&mut self) -> Vec<&mut Tensor>;
}
