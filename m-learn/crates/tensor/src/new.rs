use crate::tensor::Tensor;
use crate::tensor::TensorError;
use rand::Rng;

impl Tensor {
    pub fn new(shape: Vec<usize>, fill: f32) -> Result<Self, TensorError> {
        if shape.is_empty() {
            Err(TensorError::InvalidShape)
        } else {
            let size = shape.iter().product();
            let data = vec![fill; size];
            Ok(Self::new_internal(data, shape))
        }
    }

    pub fn ones(shape: Vec<usize>) -> Result<Self, TensorError> {
        if shape.is_empty() {
            Err(TensorError::InvalidShape)
        } else {
            let size = shape.iter().product();
            let data = vec![1.0; size];
            Ok(Self::new_internal(data, shape))
        }
    }

    pub fn zeros(shape: Vec<usize>) -> Result<Self, TensorError> {
        if shape.is_empty() {
            Err(TensorError::InvalidShape)
        } else {
            let size = shape.iter().product();
            let data = vec![0.0; size];
            Ok(Self::new_internal(data, shape))
        }
    }

    pub fn arange(len: usize) -> Result<Self, TensorError> {
        if len == 0 {
            Err(TensorError::InvalidShape)
        } else {
            let data = (0..len as usize).map(|i| i as f32).collect();
            Ok(Self::new_internal(data, vec![len]))
        }
    }

    /// 生成 [0, 1) 范围的均匀分布随机张量
    pub fn rand(shape: Vec<usize>) -> Result<Self, TensorError> {
        let mut rng = rand::rng();
        if shape.is_empty() {
            Err(TensorError::InvalidShape)
        } else {
            let size = shape.iter().product();
            let data = (0..size).map(|_| rng.random::<f32>()).collect();
            Ok(Self::new_internal(data, shape))
        }
    }

    //生成在范围内固定步长的向量
    pub fn range(begin: f32, end: f32, step: f32) -> Result<Self, TensorError> {
        if step == 0.0 {
            Err(TensorError::InvalidShape)
        } else {
            let len = ((end - begin) / step).ceil() as usize;
            let data = (0..len).map(|i| begin + i as f32 * step).collect();
            Ok(Self::new_internal(data, vec![len]))
        }
    }

    /// 生成标准正态分布（均值0，标准差1）的随机张量
    /// 使用 Box-Muller 变换生成
    pub fn randn(shape: Vec<usize>) -> Result<Self, TensorError> {
        Self::normal(shape, 0.0, 1.0)
    }

    /// 生成指定均值和标准差的正态分布随机张量
    ///
    /// 使用 Box-Muller 算法生成正态分布随机数
    ///
    /// # 参数
    /// - `shape`: 张量的形状
    /// - `mu`: 均值 (μ)，分布的中心
    /// - `sigma`: 标准差 (σ)，分布的宽度
    ///
    /// # 返回
    /// 包含正态分布随机数的张量
    ///
    /// # 示例
    /// ```text,no_run
    /// use tensor::Tensor;
    ///
    /// // 生成标准正态分布 N(0, 1) 的 2x3 张量
    /// let t = Tensor::normal(vec![2, 3], 0.0, 1.0)?;
    ///
    /// // 生成 N(10, 2) 的张量
    /// let t = Tensor::normal(vec![2, 2], 10.0, 2.0)?;
    /// # Ok::<(), _>(())
    /// ```
    pub fn normal(shape: Vec<usize>, mu: f32, sigma: f32) -> Result<Self, TensorError> {
        if shape.is_empty() {
            Err(TensorError::InvalidShape)
        } else {
            let size = shape.iter().product();
            let mut rng = rand::rng();
            let epsilon = 1e-6_f32;

            let data: Vec<f32> = (0..size)
                .map(|_| {
                    // 生成 (0, 1) 的均匀分布随机数，避免 0
                    let u1 = rng.random::<f32>() * (1.0 - epsilon) + epsilon;
                    let u2 = rng.random::<f32>() * (1.0 - epsilon) + epsilon;

                    let (z1, _z2) = formula::box_muller(u1, u2, mu, sigma);

                    // 每次迭代只使用一个随机数，第二个用于下一次迭代
                    z1
                })
                .collect();
            Ok(Self::new_internal(data, shape))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_standard() {
        // N(0, 1) 标准正态分布
        let t = Tensor::normal(vec![10000], 0.0, 1.0).unwrap();

        // 验证形状
        assert_eq!(*t.shape(), vec![10000]);

        // 验证数据范围（标准正态分布 99.7% 在 [-3, 3] 之间）
        let data = t.data();
        let out_of_range = data.iter().filter(|&&x| x < -3.0 || x > 3.0).count();
        let percentage = out_of_range as f32 / data.len() as f32;
        assert!(percentage < 0.01, "超过 1% 的数据在 [-3, 3] 之外");
    }

    #[test]
    fn test_normal_shifted() {
        // N(10, 1) 均值平移到 10
        let t = Tensor::normal(vec![1000], 10.0, 1.0).unwrap();

        // 计算均值（应该接近 10）
        let data = t.data();
        let mean: f32 = data.iter().sum::<f32>() / data.len() as f32;

        // 均值应该在 10 ± 0.5 范围内（有随机性）
        assert!((mean - 10.0).abs() < 0.5);
    }

    #[test]
    fn test_normal_scaled() {
        // N(0, 2) 标准差为 2
        let t = Tensor::normal(vec![1000], 0.0, 2.0).unwrap();

        let data = t.data();

        // 计算标准差（应该接近 2）
        let mean: f32 = data.iter().sum::<f32>() / data.len() as f32;
        let variance: f32 =
            data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / data.len() as f32;
        let std_dev = variance.sqrt();

        // 标准差应该在 2 ± 0.3 范围内（有随机性）
        assert!((std_dev - 2.0).abs() < 0.3);
    }

    #[test]
    fn test_normal_2d() {
        // 2D 张量 N(0, 1)
        let t = Tensor::normal(vec![10, 20], 0.0, 1.0).unwrap();

        // 验证形状
        assert_eq!(*t.shape(), vec![10, 20]);
        assert_eq!(t.data().len(), 200);
    }

    #[test]
    fn test_normal_3d() {
        // 3D 张量 N(5, 2)
        let t = Tensor::normal(vec![5, 3, 4], 5.0, 2.0).unwrap();

        // 验证形状
        assert_eq!(*t.shape(), vec![5, 3, 4]);
        assert_eq!(t.data().len(), 60);

        // 验证均值接近 5
        let data = t.data();
        let mean: f32 = data.iter().sum::<f32>() / data.len() as f32;
        assert!((mean - 5.0).abs() < 1.0);
    }

    #[test]
    fn test_normal_empty_shape() {
        // 空形状应该返回错误
        let result = Tensor::normal(vec![], 0.0, 1.0);
        assert!(result.is_err());
    }
}
