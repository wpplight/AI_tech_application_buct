use crate::tensor::Tensor;

impl Tensor {
    pub fn relu(&self) -> Tensor {
        self.map(|_, x| x.max(0.0))
    }

    pub fn leaky_relu(&self, negative_slope: f32) -> Tensor {
        let slope = negative_slope;
        self.map(move |_, x| if x > 0.0 { x } else { slope * x })
    }

    pub fn sigmoid(&self) -> Tensor {
        self.map(|_, x| 1.0 / (1.0 + (-x).exp()))
    }

    pub fn tanh_activation(&self) -> Tensor {
        self.map(|_, x| x.tanh())
    }

    pub fn softmax(&self, dim: usize) -> Tensor {
        let shape = self.shape();
        let ndim = shape.len();

        if dim >= ndim {
            panic!("Softmax dimension {} out of range", dim);
        }

        let dim_size = shape[dim];
        let inner_size: usize = if dim + 1 < ndim {
            shape[dim + 1..].iter().product()
        } else {
            1
        };
        let outer_size: usize = shape[..dim].iter().product();

        let input_data = self.data();
        let mut result = Vec::with_capacity(input_data.len());

        for outer_idx in 0..outer_size {
            for inner_idx in 0..inner_size {
                let mut max_val = f32::NEG_INFINITY;

                for d in 0..dim_size {
                    let idx = outer_idx * dim_size * inner_size + d * inner_size + inner_idx;
                    max_val = max_val.max(input_data[idx]);
                }

                let mut exp_sum = 0.0f32;
                for d in 0..dim_size {
                    let idx = outer_idx * dim_size * inner_size + d * inner_size + inner_idx;
                    exp_sum += (input_data[idx] - max_val).exp();
                }

                for d in 0..dim_size {
                    let idx = outer_idx * dim_size * inner_size + d * inner_size + inner_idx;
                    let exp_val = (input_data[idx] - max_val).exp() / exp_sum;
                    result.push(exp_val);
                }
            }
        }

        Tensor::build(result, shape.to_vec()).unwrap()
    }

    pub fn gelu(&self) -> Tensor {
        let sqrt_2_over_pi = 0.7978845608028654f32;
        let coeff = 0.044715f32;
        self.map(move |_, x| {
            let x3 = x * x * x;
            let inner = sqrt_2_over_pi * (x + coeff * x3);
            0.5 * x * (1.0 + inner.tanh())
        })
    }

    pub fn silu(&self) -> Tensor {
        self.map(|_, x| {
            let sigmoid = 1.0 / (1.0 + (-x).exp());
            x * sigmoid
        })
    }

    pub fn mish(&self) -> Tensor {
        self.map(|_, x| {
            let softplus = if x > 20.0 {
                x
            } else {
                (1.0 + x.exp()).ln()
            };
            let tanh_softplus = softplus.tanh();
            x * tanh_softplus
        })
    }

    pub fn elu(&self, alpha: f32) -> Tensor {
        let a = alpha;
        self.map(move |_, x| {
            if x > 0.0 {
                x
            } else {
                a * (x.exp() - 1.0)
            }
        })
    }

    pub fn hardswish(&self) -> Tensor {
        self.map(|_, x| {
            if x <= -3.0 {
                0.0
            } else if x >= 3.0 {
                x
            } else {
                x * (x + 3.0) / 6.0
            }
        })
    }

    pub fn hardtanh(&self, min_val: f32, max_val: f32) -> Tensor {
        let min_v = min_val;
        let max_v = max_val;
        self.map(move |_, x| {
            if x < min_v {
                min_v
            } else if x > max_v {
                max_v
            } else {
                x
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Tensor;

    #[test]
    fn test_relu() {
        let input = Tensor::build(vec![-1.0, 0.0, 1.0, -2.0], vec![4]).unwrap();
        let output = input.relu();
        assert_eq!(output.data(), &vec![0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_leaky_relu() {
        let input = Tensor::build(vec![-1.0, 0.5, 1.0], vec![3]).unwrap();
        let output = input.leaky_relu(0.1);
        let expected = vec![-0.1, 0.5, 1.0];
        for (a, &b) in output.data().iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-5);
        }
    }

    #[test]
    fn test_sigmoid() {
        let input = Tensor::build(vec![0.0, 0.0], vec![2]).unwrap();
        let output = input.sigmoid();
        for &val in output.data() {
            assert!((val - 0.5).abs() < 1e-5);
        }
    }

    #[test]
    fn test_tanh_activation() {
        let input = Tensor::build(vec![0.0, 0.0], vec![2]).unwrap();
        let output = input.tanh_activation();
        for &val in output.data() {
            assert!(val.abs() < 1e-5);
        }
    }

    #[test]
    fn test_softmax_1d() {
        let input = Tensor::build(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
        let output = input.softmax(0);
        let sum: f32 = output.data().iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_gelu() {
        let input = Tensor::build(vec![0.0, 1.0, 2.0], vec![3]).unwrap();
        let output = input.gelu();
        assert!(output[[0]].abs() < 1e-3);
        assert!(output[[1]] > 0.7);
        assert!(output[[2]] > 1.8);
    }

    #[test]
    fn test_silu() {
        let input = Tensor::build(vec![0.0, 1.0, 2.0], vec![3]).unwrap();
        let output = input.silu();
        assert!(output[[0]].abs() < 1e-3);
        assert!(output[[1]] > 0.5);
        assert!(output[[2]] > 1.5);
    }

    #[test]
    fn test_mish() {
        let input = Tensor::build(vec![0.0, 1.0], vec![2]).unwrap();
        let output = input.mish();
        assert!(output[[0]].abs() < 1e-3);
        assert!(output[[1]] > 0.5);
    }

    #[test]
    fn test_elu() {
        let input = Tensor::build(vec![-1.0, 0.5, 1.0], vec![3]).unwrap();
        let output = input.elu(1.0);
        assert!(output[[0]] < 0.0);
        assert_eq!(output[[1]], 0.5);
        assert_eq!(output[[2]], 1.0);
    }

    #[test]
    fn test_hardswish() {
        let input = Tensor::build(vec![-5.0, 0.0, 5.0], vec![3]).unwrap();
        let output = input.hardswish();
        assert_eq!(output[[0]], 0.0);
        assert_eq!(output[[1]], 0.0);
        assert_eq!(output[[2]], 5.0);
    }

    #[test]
    fn test_hardtanh() {
        let input = Tensor::build(vec![-5.0, 0.0, 5.0], vec![3]).unwrap();
        let output = input.hardtanh(-1.0, 1.0);
        assert_eq!(output[[0]], -1.0);
        assert_eq!(output[[1]], 0.0);
        assert_eq!(output[[2]], 1.0);
    }
}
