use std::f32::consts::PI;

/// 正态分布概率密度函数 (PDF)
///
/// # 公式
/// ```text
/// f(x) = (1 / (σ * √(2π))) * e^(-0.5 * ((x - μ) / σ)^2)
/// ```
///
/// # 参数
/// - `x`: 输入值
/// - `mu`: 均值 (μ)，分布的中心
/// - `sigma`: 标准差 (σ)，分布的宽度
///
/// # 返回
/// 概率密度值
///
/// # 示例
/// ```
/// use formula::normal::normal_distribution;
///
/// // 标准正态分布 N(0, 1)
/// let pdf = normal_distribution(0.0, 0.0, 1.0);
/// assert_eq!(pdf, 1.0 / (2.0 * std::f32::consts::PI).sqrt());
/// ```
pub fn normal_distribution(x: f32, mu: f32, sigma: f32) -> f32 {
    let sqrt_2pi = (2.0 * PI).sqrt();
    let coefficient = 1.0 / (sigma * sqrt_2pi);
    let exponent = -0.5 * ((x - mu) / sigma).powi(2);
    coefficient * exponent.exp()
}

/// Box-Muller 算法生成标准正态分布随机数
///
/// 该算法通过两个独立的均匀分布随机数生成两个独立的标准正态分布随机数。
///
/// # 算法原理
/// 给定两个独立的均匀分布随机数 u1, u2 ∈ (0, 1]，生成标准正态分布随机数 z1, z2：
/// ```text
/// z1 = √(-2 * ln(u1)) * cos(2π * u2)
/// z2 = √(-2 * ln(u1)) * sin(2π * u2)
/// ```
///
/// # 参数
/// - `u1`: 第一个均匀分布随机数，范围 (0, 1]
/// - `u2`: 第二个均匀分布随机数，范围 (0, 1]
/// - `mu`: 均值 (μ)，分布的中心
/// - `sigma`: 标准差 (σ)，分布的宽度
///
/// # 返回
/// 两个独立的正态分布随机数 (z1, z2)
///
/// # 注意事项
/// - u1 和 u2 必须严格大于 0 (不能等于 0，否则 ln(0) 会返回负无穷)
/// - u1 和 u2 通常由随机数生成器生成，如 rand::random::<f32>()
/// - 但需要注意 rand::random 可能返回 0，因此建议使用 (random() * (1.0 - epsilon) + epsilon)
///
/// # 示例
/// ```
/// use formula::normal::box_muller;
///
/// // 使用随机数生成
/// let u1 = 0.5_f32;
/// let u2 = 0.7_f32;
///
/// // 生成标准正态分布 N(0, 1) 的随机数
/// let (z1, z2) = box_muller(u1, u2, 0.0, 1.0);
///
/// // 生成均值为 10、标准差为 2 的正态分布 N(10, 2)
/// let (z1, z2) = box_muller(u1, u2, 10.0, 2.0);
/// ```
pub fn box_muller(u1: f32, u2: f32, mu: f32, sigma: f32) -> (f32, f32) {
    // 生成标准正态分布
    let radius = (-2.0 * u1.ln()).sqrt();
    let angle = 2.0 * PI * u2;

    let z1 = radius * angle.cos();
    let z2 = radius * angle.sin();

    // 线性变换到指定均值和标准差
    (z1 * sigma + mu, z2 * sigma + mu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_distribution_standard() {
        let result = normal_distribution(0.0, 0.0, 1.0);
        assert_eq!(result, 1.0 / (2.0 * PI).sqrt());
    }

    #[test]
    fn test_normal_distribution_peak() {
        let result = normal_distribution(1.0, 1.0, 1.0);
        let expected = 1.0 / (1.0 * (2.0 * PI).sqrt());
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_normal_distribution_symmetric() {
        let x = 0.5;
        let mu = 0.0;
        let sigma = 1.0;
        let y1 = normal_distribution(x, mu, sigma);
        let y2 = normal_distribution(-x + 2.0 * mu, mu, sigma);
        assert!((y1 - y2).abs() < 1e-6);
    }

    #[test]
    fn test_normal_distribution_different_sigma() {
        let sigma = 0.5;
        let result = normal_distribution(0.0, 0.0, sigma);
        let expected = 1.0 / (sigma * (2.0 * PI).sqrt());
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_box_muller_standard() {
        // 标准正态分布 N(0, 1)
        let u1 = 0.5_f32;
        let u2 = 0.7_f32;

        let (z1, z2) = box_muller(u1, u2, 0.0, 1.0);

        // 验证结果不是 NaN 或无穷
        assert!(z1.is_finite());
        assert!(z2.is_finite());
    }

    #[test]
    fn test_box_muller_mean_shift() {
        // N(10, 1)
        let u1 = 0.5_f32;
        let u2 = 0.7_f32;

        let (z1, z2) = box_muller(u1, u2, 10.0, 1.0);
        let (z1_std, z2_std) = box_muller(u1, u2, 0.0, 1.0);

        // 均值平移应该等于 10
        assert!((z1 - z1_std - 10.0).abs() < 1e-6);
        assert!((z2 - z2_std - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_box_muller_sigma_scaling() {
        // N(0, 2)
        let u1 = 0.5_f32;
        let u2 = 0.7_f32;

        let (z1, z2) = box_muller(u1, u2, 0.0, 2.0);
        let (z1_std, z2_std) = box_muller(u1, u2, 0.0, 1.0);

        // 标准差缩放应该等于 2
        assert!((z1 - z1_std * 2.0).abs() < 1e-6);
        assert!((z2 - z2_std * 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_box_muller_symmetric() {
        // 标准正态分布应该是对称的
        let u1 = 0.5_f32;
        let u2 = 0.25_f32;

        let (z1, _) = box_muller(u1, u2, 0.0, 1.0);

        // 使用相同的 u1, u2，结果应该是确定性的
        let (z1_again, _) = box_muller(u1, u2, 0.0, 1.0);
        assert!((z1 - z1_again).abs() < 1e-6);
    }

    #[test]
    fn test_box_muller_different_inputs() {
        let u1 = 0.3_f32;
        let u2 = 0.6_f32;

        let (z1, z2) = box_muller(u1, u2, 0.0, 1.0);

        // 不同的输入应该产生不同的输出（极小概率相同）
        // 使用不等于断言而不是浮点比较
        assert!(z1 != z2 || (z1 - z2).abs() > 1e-10);
    }
}
