#[derive(Debug, Clone)]
pub enum TensorError {
    InvalidShape,
    IncompatibleShapes,
    IndexError { dim: usize, max: usize },
    SliceError(String),
}

impl std::fmt::Display for TensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TensorError::InvalidShape => write!(f, "Invalid shape"),
            TensorError::IncompatibleShapes => write!(f, "Incompatible shapes"),
            TensorError::IndexError { dim, max } => {
                write!(f, "Index {} out of bounds (max: {})", dim, max)
            }
            TensorError::SliceError(msg) => write!(f, "Slice error: {}", msg),
        }
    }
}

impl std::error::Error for TensorError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    data: Vec<f32>,
    shape: Vec<usize>,
}

impl Tensor {
    pub(crate) fn new_internal(data: Vec<f32>, shape: Vec<usize>) -> Self {
        Self { data, shape }
    }

    pub fn build(data: Vec<f32>, shape: Vec<usize>) -> Result<Self, TensorError> {
        if data.len() != shape.iter().product::<usize>() {
            Err(TensorError::InvalidShape)
        } else {
            Ok(Self::new_internal(data, shape))
        }
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn get_data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    pub(crate) fn get_mut_shape(&mut self) -> &mut Vec<usize> {
        &mut self.shape
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<f32> {
        &mut self.data
    }
    
    pub fn broadcast(&self, target_shape: &[usize]) -> Result<Tensor, TensorError> {
        let self_shape = &self.shape;
        
        if self_shape.is_empty() || target_shape.is_empty() {
            return Err(TensorError::IncompatibleShapes);
        }
        
        if self_shape.len() > target_shape.len() {
            return Err(TensorError::IncompatibleShapes);
        }
        
        let padded_self_shape: Vec<usize> = vec![1; target_shape.len() - self_shape.len()]
            .into_iter()
            .chain(self_shape.iter().cloned())
            .collect();
        
        for (s, t) in padded_self_shape.iter().zip(target_shape.iter()) {
            if *s != 1 && *s != *t {
                return Err(TensorError::IncompatibleShapes);
            }
        }
        
        if padded_self_shape == *target_shape {
            return Ok(self.clone());
        }
        
        let total_elements: usize = target_shape.iter().product();
        let mut result = Vec::with_capacity(total_elements);
        
        for idx in 0..total_elements {
            let mut remaining = idx;
            let mut src_idx = 0;
            let mut src_stride = 1;
            
            for dim in 0..target_shape.len() {
                let dim_size = target_shape[target_shape.len() - 1 - dim];
                let pos_in_dim = remaining % dim_size;
                remaining /= dim_size;
                
                let src_dim = padded_self_shape.len() - 1 - dim;
                let src_dim_size = padded_self_shape[src_dim];
                
                let src_pos = if src_dim_size == 1 { 0 } else { pos_in_dim };
                src_idx += src_pos * src_stride;
                src_stride *= src_dim_size;
            }
            
            result.push(self.data[src_idx]);
        }
        
        Tensor::build(result, target_shape.to_vec())
    }
    
    pub fn squeeze(&self) -> Tensor {
        let new_shape: Vec<usize> = self.shape.iter()
            .filter(|&&dim| dim != 1)
            .cloned()
            .collect();
        
        if new_shape.is_empty() {
            // 所有维度都是1，返回标量（如果只有一个元素）
            if self.data.len() == 1 {
                return Tensor::build(vec![self.data[0]], vec![1]).unwrap();
            }
            // 多个元素，保持第一个维度
            return Tensor::build(self.data.clone(), vec![self.data.len()]).unwrap();
        }
        
        if new_shape == self.shape {
            return self.clone();
        }
        
        Tensor::build(self.data.clone(), new_shape).unwrap()
    }
    
    pub fn normalize_shape(&self) -> Tensor {
        if self.shape.len() == 1 {
            let mut new_shape = vec![1];
            new_shape.extend(self.shape.iter());
            Tensor::build(self.data.clone(), new_shape).unwrap()
        } else {
            self.clone()
        }
    }
}
