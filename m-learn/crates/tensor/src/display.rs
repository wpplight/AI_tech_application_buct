use crate::tensor::Tensor;
impl Tensor {
    pub fn display(&self) {
        let result = self.format_nested();
        println!("{}", result);
    }

    // 格式化张量为嵌套数组字符串
    fn format_nested(&self) -> String {
        let shape = self.shape();
        let data = self.data();

        if shape.is_empty() {
            // 标量
            return format!(
                "{}",
                data.get(0)
                    .map(|&v| v.to_string())
                    .unwrap_or_else(|| "None".to_string())
            );
        }
        self.format_recursive(data, shape)
    }

    // 递归格式化函数
    fn format_recursive(&self, data: &[f32], shape: &[usize]) -> String {
        if shape.len() == 1 {
            // 一维数组：基础情况
            let elements: Vec<String> = data.iter().map(|&v| v.to_string()).collect();
            return format!("[{}]", elements.join(", "));
        }

        // 多维数组：递归处理
        let outer_dim = shape[0];
        let remaining_shape = &shape[1..];
        let block_size: usize = remaining_shape.iter().product();

        let mut blocks = Vec::new();
        for i in 0..outer_dim {
            let start = i * block_size;
            let end = start + block_size;
            let block = &data[start..end];
            blocks.push(self.format_recursive(block, remaining_shape));
        }

        format!("[{}]", blocks.join(", "))
    }

    pub fn print_info(&self) {
        println!(
            "Tensor(shape={:?}, numel={})",
            self.shape(),
            self.numel()
        );
        self.display();
    }

    pub fn numel(&self) -> usize {
        self.shape().iter().product()
    }
    
    pub fn sum(&self) -> f32 {
        self.data().iter().sum()
    }
    
    pub fn sum_axis(&self, axis: usize) -> Tensor {
        let shape = self.shape();
        let ndim = shape.len();
        
        if axis >= ndim {
            panic!("Axis {} out of range for tensor with {} dimensions", axis, ndim);
        }
        
        let inner_size: usize = shape[axis + 1..].iter().product();
        let outer_size: usize = shape[..axis].iter().product();
        let dim_size = shape[axis];
        
        let data = self.data();
        let mut result = Vec::with_capacity(outer_size * inner_size);
        
        for outer_idx in 0..outer_size {
            for inner_idx in 0..inner_size {
                let mut sum = 0.0f32;
                for d in 0..dim_size {
                    let idx = outer_idx * dim_size * inner_size + d * inner_size + inner_idx;
                    sum += data[idx];
                }
                result.push(sum);
            }
        }
        
        let mut new_shape = shape.to_vec();
        new_shape.remove(axis);
        
        if new_shape.len() == 0 {
            new_shape.push(1);
        }
        
        Tensor::build(result, new_shape).unwrap()
    }
}
