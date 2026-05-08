use tensor::Tensor;
use std::ops::Deref;

pub struct LossResult {
    pub loss: f32,
    grad: Tensor,
    batch_size: usize,
}

impl LossResult {
    pub fn new(loss: f32, grad: Tensor, batch_size: usize) -> Self {
        Self { loss, grad, batch_size }
    }
    
    pub fn grad(&self) -> &Tensor {
        &self.grad
    }
    
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
    
    pub fn grad_mean(&self) -> Tensor {
        let batch = self.batch_size;
        if batch <= 1 {
            self.grad.clone()
        } else {
            let sum_axis_result = self.grad.sum_axis(0);
            let mean_data = sum_axis_result.map(move |_, v| v / batch as f32);
            
            // 如果原始梯度是 [batch, features]，返回 [features]（压缩batch维度）
            if self.grad.shape().len() == 2 {
                mean_data
            } else {
                mean_data
            }
        }
    }
}

impl Deref for LossResult {
    type Target = Tensor;
    
    fn deref(&self) -> &Self::Target {
        &self.grad
    }
}

pub trait LossFunction: Send + Sync {
    fn criterion(&mut self, pred: &Tensor, target: &Tensor) -> LossResult;
}

pub struct MSELoss {
    output_cache: Option<Tensor>,
    target_cache: Option<Tensor>,
}

impl MSELoss {
    pub fn new() -> Self {
        Self {
            output_cache: None,
            target_cache: None,
        }
    }
}

impl Default for MSELoss {
    fn default() -> Self {
        Self::new()
    }
}

impl LossFunction for MSELoss {
    fn criterion(&mut self, pred: &Tensor, target: &Tensor) -> LossResult {
        self.output_cache = Some(pred.clone());
        self.target_cache = Some(target.clone());
        
        let pred_shape = pred.shape();
        let batch_size = if pred_shape.len() == 2 { pred_shape[0] } else { 1 };
        
        let diff = (pred - target).unwrap();
        let squared = diff.map(|_, x| x * x);
        
        // Loss 是所有样本的平均
        let loss = squared.sum() / batch_size as f32;
        
        // 梯度也除以 batch_size，这样得到的是平均梯度
        let scale = 2.0 / batch_size as f32;
        let grad = diff.map(move |_, x| x * scale);
        
        // 返回梯度（已经是平均后的）
        LossResult::new(loss, grad, batch_size)
    }
}

pub enum Loss {
    MSE(MSELoss),
}

impl Loss {
    pub fn mse() -> Self {
        Loss::MSE(MSELoss::new())
    }
    
    pub fn criterion(&mut self, pred: &Tensor, target: &Tensor) -> LossResult {
        match self {
            Loss::MSE(loss) => loss.criterion(pred, target),
        }
    }
}

impl LossFunction for Loss {
    fn criterion(&mut self, pred: &Tensor, target: &Tensor) -> LossResult {
        match self {
            Loss::MSE(loss) => loss.criterion(pred, target),
        }
    }
}
