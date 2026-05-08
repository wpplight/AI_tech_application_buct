use crate::tensor::Tensor;
use crate::tensor::TensorError;
use rayon::prelude::*;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use std::sync::Arc;

fn check_shape_compatible(lhs: &Tensor, rhs: &Tensor) -> Result<(), TensorError> {
    if lhs.shape() == rhs.shape() {
        Ok(())
    } else {
        Err(TensorError::IncompatibleShapes)
    }
}

const PARALLEL_THRESHOLD: usize = 100_000;

const MATMUL_PARALLEL_THRESHOLD: usize = 1_000_000;

const COLUMN_CACHE_THRESHOLD: usize = 64;

const TILE_SIZE: usize = 64;

const MICRO_TILE: usize = 8;

fn transpose_to_column_cache(b: &[f32], k: usize, n: usize, num_chunks: usize) -> Vec<f32> {
    let chunk_size = (n + num_chunks - 1) / num_chunks;
    let b = Arc::new(b.to_vec());
    
    let chunks: Vec<Vec<f32>> = (0..num_chunks).into_par_iter()
        .map(|chunk_idx| {
            let start = chunk_idx * chunk_size;
            let end = (start + chunk_size).min(n);
            let local_n = end - start;
            
            let mut cache = Vec::with_capacity(k * local_n);
            for k_idx in 0..k {
                for j in start..end {
                    cache.push(b[k_idx * n + j]);
                }
            }
            cache
        })
        .collect();
    
    chunks.into_iter().flatten().collect()
}

fn matmul_with_column_cache(a: &[f32], b_cache: &[f32], m: usize, k: usize, n: usize, chunk_size: usize) -> Vec<f32> {
    let num_chunks = (n + chunk_size - 1) / chunk_size;
    let a = Arc::new(a.to_vec());
    let b_cache = Arc::new(b_cache.to_vec());
    
    (0..num_chunks).into_par_iter()
        .map(|chunk_idx| {
            let start = chunk_idx * chunk_size;
            let end = (start + chunk_size).min(n);
            let local_n = end - start;
            let b_offset = chunk_idx * k * local_n;
            
            let mut chunk_result = Vec::with_capacity(m * local_n);
            for i in 0..m {
                for j in 0..local_n {
                    let mut sum = 0.0f32;
                    let b_col_offset = b_offset + j * k;
                    for k_idx in 0..k {
                        sum += a[i * k + k_idx] * b_cache[b_col_offset + k_idx];
                    }
                    chunk_result.push(sum);
                }
            }
            chunk_result
        })
        .flatten_iter()
        .collect()
}

fn check_matmul_shape(lhs: &Tensor, rhs: &Tensor) -> Result<(Vec<usize>, usize, usize, usize), TensorError> {
    let lhs_shape = lhs.shape();
    let rhs_shape = rhs.shape();

    if lhs_shape.is_empty() || rhs_shape.is_empty() {
        return Err(TensorError::InvalidShape);
    }

    let lhs_ndim = lhs_shape.len();
    let rhs_ndim = rhs_shape.len();

    if lhs_ndim == 1 && rhs_ndim == 1 {
        if lhs_shape[0] != rhs_shape[0] {
            return Err(TensorError::IncompatibleShapes);
        }
        let k = lhs_shape[0];
        return Ok((vec![], 0, k, 1));
    }

    if lhs_ndim == 1 && rhs_ndim == 2 {
        let k = lhs_shape[0];
        let n = rhs_shape[1];
        if k != rhs_shape[0] {
            return Err(TensorError::IncompatibleShapes);
        }
        let result_shape = vec![n];
        return Ok((result_shape, 1, k, n));
    }

    if lhs_ndim == 1 && rhs_ndim > 2 {
        let k = lhs_shape[0];
        let n = rhs_shape[rhs_ndim - 1];
        if k != rhs_shape[rhs_ndim - 2] {
            return Err(TensorError::IncompatibleShapes);
        }
        let batch_dims = rhs_shape[..rhs_ndim - 2].to_vec();
        let mut result_shape = batch_dims;
        result_shape.push(n);
        return Ok((result_shape, 1, k, n));
    }

    if lhs_ndim >= 2 && rhs_ndim == 1 {
        let m = lhs_shape[lhs_ndim - 2];
        let k = lhs_shape[lhs_ndim - 1];
        if k != rhs_shape[0] {
            return Err(TensorError::IncompatibleShapes);
        }
        let result_shape = lhs_shape[..lhs_ndim - 1].to_vec();
        return Ok((result_shape, m, k, 1));
    }

    if lhs_ndim >= 2 && rhs_ndim >= 2 {
        let k_lhs = lhs_shape[lhs_ndim - 1];
        let k_rhs = rhs_shape[rhs_ndim - 2];

        if k_lhs != k_rhs {
            return Err(TensorError::IncompatibleShapes);
        }

        let m = lhs_shape[lhs_ndim - 2];
        let n = rhs_shape[rhs_ndim - 1];

        let batch_lhs: Vec<usize> = lhs_shape[..lhs_ndim - 2].to_vec();
        let batch_rhs: Vec<usize> = rhs_shape[..rhs_ndim - 2].to_vec();

        let batch_shape = if batch_lhs.is_empty() {
            batch_rhs.clone()
        } else if batch_rhs.is_empty() {
            batch_lhs.clone()
        } else if batch_lhs == batch_rhs {
            batch_lhs.clone()
        } else {
            return Err(TensorError::IncompatibleShapes);
        };

        let mut result_shape = batch_shape;
        result_shape.push(m);
        result_shape.push(n);
        return Ok((result_shape, m, k_lhs, n));
    }

    Err(TensorError::IncompatibleShapes)
}

fn matmul_2d_tiled(a: &[f32], m: usize, k_dim: usize, b: &[f32], n: usize) -> Vec<f32> {
    let mut c = vec![0.0f32; m * n];
    let mut packed_a = vec![0.0f32; TILE_SIZE * TILE_SIZE];
    let mut packed_b = vec![0.0f32; TILE_SIZE * TILE_SIZE];

    for i_block in (0..m).step_by(TILE_SIZE) {
        for j_block in (0..n).step_by(TILE_SIZE) {
            for k_block in (0..k_dim).step_by(TILE_SIZE) {
                let i_end = (i_block + TILE_SIZE).min(m);
                let j_end = (j_block + TILE_SIZE).min(n);
                let k_end = (k_block + TILE_SIZE).min(k_dim);
                let tile_i = i_end - i_block;
                let tile_j = j_end - j_block;
                let tile_k = k_end - k_block;

                for local_i in 0..tile_i {
                    for local_k in 0..tile_k {
                        packed_a[local_i * tile_k + local_k] =
                            a[(i_block + local_i) * k_dim + (k_block + local_k)];
                    }
                }
                for local_k in 0..tile_k {
                    for local_j in 0..tile_j {
                        packed_b[local_k * tile_j + local_j] =
                            b[(k_block + local_k) * n + (j_block + local_j)];
                    }
                }

                for i_micro in (0..tile_i).step_by(MICRO_TILE) {
                    for j_micro in (0..tile_j).step_by(MICRO_TILE) {
                        for k_micro in (0..tile_k).step_by(MICRO_TILE) {
                            let di = MICRO_TILE.min(tile_i - i_micro);
                            let dj = MICRO_TILE.min(tile_j - j_micro);
                            let dk = MICRO_TILE.min(tile_k - k_micro);
                            for ii in 0..di {
                                for jj in 0..dj {
                                    let mut sum = 0.0f32;
                                    for kk in 0..dk {
                                        sum += packed_a[(i_micro + ii) * tile_k + (k_micro + kk)]
                                            * packed_b[(k_micro + kk) * tile_j + (j_micro + jj)];
                                    }
                                    let ci = i_block + i_micro + ii;
                                    let cj = j_block + j_micro + jj;
                                    c[ci * n + cj] += sum;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    c
}

fn matvec_parallel(a: &[f32], k_dim: usize, b: &[f32], n: usize) -> Vec<f32> {
    let a = Arc::new(a.to_vec());
    let b = Arc::new(b.to_vec());
    
    let num_chunks = optim::GlobalPool::optimal_num_chunks(n).min(256);
    
    if num_chunks <= 1 || n < 1000 {
        let mut result = vec![0.0f32; n];
        for j in 0..n {
            let mut sum = 0.0f32;
            for k in 0..k_dim {
                sum += a[k] * b[j * k_dim + k];
            }
            result[j] = sum;
        }
        result
    } else {
        let chunk_size = (n + num_chunks - 1) / num_chunks;
        
        (0..num_chunks).into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = (start + chunk_size).min(n);
                let mut chunk_result = Vec::with_capacity(end - start);
                for j in start..end {
                    let mut sum = 0.0f32;
                    for k in 0..k_dim {
                        sum += a[k] * b[j * k_dim + k];
                    }
                    chunk_result.push(sum);
                }
                chunk_result
            })
            .flatten_iter()
            .collect()
    }
}

fn matmul_2d_row_parallel(a: &[f32], m: usize, k_dim: usize, b: &[f32], n: usize) -> Vec<f32> {
    let a = Arc::new(a.to_vec());
    let b = Arc::new(b.to_vec());
    
    let num_chunks = optim::GlobalPool::optimal_num_chunks(m).min(256);
    
    if num_chunks <= 1 || m * n < 10000 {
        let mut c = vec![0.0f32; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0f32;
                for k in 0..k_dim {
                    sum += a[i * k_dim + k] * b[j * k_dim + k];
                }
                c[i * n + j] = sum;
            }
        }
        c
    } else {
        let chunk_size = (m + num_chunks - 1) / num_chunks;
        
        (0..num_chunks).into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = (start + chunk_size).min(m);
                let mut chunk_result = Vec::with_capacity((end - start) * n);
                for i in start..end {
                    for j in 0..n {
                        let mut sum = 0.0f32;
                        for k in 0..k_dim {
                            sum += a[i * k_dim + k] * b[j * k_dim + k];
                        }
                        chunk_result.push(sum);
                    }
                }
                chunk_result
            })
            .flatten_iter()
            .collect()
    }
}

fn batch_matmul_parallel(
    a_data: &[f32],
    _a_shape: &[usize],
    b_data: &[f32],
    _b_shape: &[usize],
    result_shape: &[usize],
    m: usize,
    k: usize,
    n: usize,
) -> Vec<f32> {
    let a_data = Arc::new(a_data.to_vec());
    let b_data = Arc::new(b_data.to_vec());
    
    let result_ndim = result_shape.len();
    let num_threads = rayon::current_num_threads();
    
    if result_ndim == 1 && m == 1 {
        if n > COLUMN_CACHE_THRESHOLD && n >= num_threads * 4 {
            let num_chunks = num_threads.min(n / 4);
            let b_cache = transpose_to_column_cache(&b_data, k, n, num_chunks);
            let chunk_size = (n + num_chunks - 1) / num_chunks;
            matmul_with_column_cache(&a_data, &b_cache, m, k, n, chunk_size)
        } else {
            matvec_parallel(&a_data, k, &b_data, n)
        }
    } else if result_ndim <= 2 {
        if n > COLUMN_CACHE_THRESHOLD && n >= num_threads * 4 {
            let num_chunks = num_threads.min(n / 4);
            let b_cache = transpose_to_column_cache(&b_data, k, n, num_chunks);
            let chunk_size = (n + num_chunks - 1) / num_chunks;
            matmul_with_column_cache(&a_data, &b_cache, m, k, n, chunk_size)
        } else {
            matmul_2d_row_parallel(&a_data, m, k, &b_data, n)
        }
    } else {
        let batch_size: usize = result_shape[..result_ndim - 2].iter().product();
        
        (0..batch_size).into_par_iter()
            .map(|batch_idx| {
                let a_batch_offset = batch_idx * m * k;
                let b_batch_offset = if result_shape[result_ndim - 2] == 1 {
                    0
                } else {
                    batch_idx * k * n
                };
                
                let a_slice = &a_data[a_batch_offset..a_batch_offset + m * k];
                let b_slice = &b_data[b_batch_offset..b_batch_offset + k * n];
                
                if n > COLUMN_CACHE_THRESHOLD && n >= num_threads * 4 {
                    let num_chunks = num_threads.min(n / 4);
                    let b_cache = transpose_to_column_cache(b_slice, k, n, num_chunks);
                    let chunk_size = (n + num_chunks - 1) / num_chunks;
                    matmul_with_column_cache(a_slice, &b_cache, m, k, n, chunk_size)
                } else {
                    matmul_2d_row_parallel(a_slice, m, k, b_slice, n)
                }
            })
            .flatten_iter()
            .collect()
    }
}

impl Tensor {
    pub fn matmul(&self, other: &Tensor) -> Result<Self, TensorError> {
        let (result_shape, m, k, n) = check_matmul_shape(self, other)?;

        if m == 0 && n == 1 {
            let scalar = self
                .data()
                .iter()
                .zip(other.data().iter())
                .map(|(a, b)| a * b)
                .sum::<f32>();
            return Tensor::build(vec![scalar], result_shape);
        }

        let total_elements = self.data().len() * other.data().len();
        let batch_size: usize = if result_shape.len() <= 2 {
            if m == 1 && self.shape().len() == 1 && other.shape().len() > 2 {
                result_shape.get(0).copied().unwrap_or(1)
            } else {
                1
            }
        } else {
            result_shape[..result_shape.len() - 2].iter().product()
        };

        let use_serial = total_elements < MATMUL_PARALLEL_THRESHOLD 
            && !(result_shape.len() == 1 && m == 1 && n >= 1000);

        if use_serial {
            let result_data = if batch_size <= 1 {
                matmul_2d_tiled(self.data(), m, k, other.data(), n)
            } else {
                let is_lhs_1d = self.shape().len() == 1;
                let is_rhs_1d = other.shape().len() == 1;
                let mut out = Vec::with_capacity(batch_size * m * n);
                for batch_idx in 0..batch_size {
                    let a_slice = if is_lhs_1d {
                        self.data()
                    } else {
                        let a_off = batch_idx * m * k;
                        &self.data()[a_off..a_off + m * k]
                    };
                    let b_slice = if is_rhs_1d {
                        other.data()
                    } else {
                        let b_off = batch_idx * k * n;
                        &other.data()[b_off..b_off + k * n]
                    };
                    let c_block = matmul_2d_tiled(a_slice, m, k, b_slice, n);
                    out.extend(c_block);
                }
                out
            };
            Tensor::build(result_data, result_shape)
        } else {
            let result_data = batch_matmul_parallel(
                self.data(),
                self.shape(),
                other.data(),
                other.shape(),
                &result_shape,
                m,
                k,
                n,
            );
            Tensor::build(result_data, result_shape)
        }
    }
}

#[allow(dead_code)]
fn chunk_ranges(len: usize, n: usize) -> Vec<(usize, usize)> {
    if len == 0 || n == 0 {
        return vec![];
    }
    let n = n.min(len);
    let chunk_size = (len + n - 1) / n;
    (0..n)
        .map(|i| {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(len);
            (start, end)
        })
        .filter(|&(s, e)| s < e)
        .collect()
}

fn parallel_map_op<F>(data: &[f32], shape: &[usize], op: F) -> Vec<f32>
where
    F: Fn(&[usize], f32) -> f32 + Sync + Send + 'static,
{
    let len = data.len();
    let ndim = shape.len();

    if len < PARALLEL_THRESHOLD {
        let mut result = Vec::with_capacity(len);
        let mut indices = vec![0usize; ndim];
        let mut strides = vec![1usize; ndim];

        for i in (0..ndim).rev() {
            if i == ndim - 1 {
                strides[i] = 1;
            } else {
                strides[i] = shape[i + 1..].iter().product::<usize>();
            }
        }

        for linear_idx in 0..len {
            for i in 0..ndim {
                indices[i] = (linear_idx / strides[i]) % shape[i];
            }
            let value = data[linear_idx];
            result.push(op(&indices, value));
        }
        result
    } else {
        let strides_vec: Vec<usize> = (0..ndim)
            .rev()
            .map(|i| {
                if i == ndim - 1 {
                    1
                } else {
                    shape[i + 1..].iter().product::<usize>()
                }
            })
            .collect();

        let data = Arc::new(data.to_vec());
        let shape = Arc::new(shape.to_vec());
        let op = Arc::new(op);

        (0..len).into_par_iter()
            .map(|linear_idx| {
                let mut indices = vec![0usize; ndim];
                for i in 0..ndim {
                    indices[i] = (linear_idx / strides_vec[i]) % shape[i];
                }
                let value = data[linear_idx];
                op(&indices, value)
            })
            .collect()
    }
}

fn parallel_map_op_with<C, F>(data: &[f32], shape: &[usize], ctx: C, f: F) -> Vec<f32>
where
    C: Copy + Send + Sync + 'static,
    F: Fn(&[usize], f32, C) -> f32 + Send + Sync + 'static,
{
    let len = data.len();
    let ndim = shape.len();

    if len < PARALLEL_THRESHOLD {
        let mut result = Vec::with_capacity(len);
        let mut indices = vec![0usize; ndim];
        let mut strides = vec![1usize; ndim];

        for i in (0..ndim).rev() {
            if i == ndim - 1 {
                strides[i] = 1;
            } else {
                strides[i] = shape[i + 1..].iter().product::<usize>();
            }
        }

        for linear_idx in 0..len {
            for i in 0..ndim {
                indices[i] = (linear_idx / strides[i]) % shape[i];
            }
            let value = data[linear_idx];
            result.push(f(&indices, value, ctx));
        }
        result
    } else {
        let strides_vec: Vec<usize> = (0..ndim)
            .rev()
            .map(|i| {
                if i == ndim - 1 {
                    1
                } else {
                    shape[i + 1..].iter().product::<usize>()
                }
            })
            .collect();

        let data = Arc::new(data.to_vec());
        let shape = Arc::new(shape.to_vec());
        let f = Arc::new(f);
        let ctx = ctx;

        (0..len).into_par_iter()
            .map(|linear_idx| {
                let mut indices = vec![0usize; ndim];
                for i in 0..ndim {
                    indices[i] = (linear_idx / strides_vec[i]) % shape[i];
                }
                let value = data[linear_idx];
                f(&indices, value, ctx)
            })
            .collect()
    }
}

fn parallel_binary_op<F>(lhs_data: &[f32], rhs_data: &[f32], op: F) -> Vec<f32>
where
    F: Fn(f32, f32) -> f32 + Sync + Send + 'static,
{
    let len = lhs_data.len();

    if len < PARALLEL_THRESHOLD {
        lhs_data
            .iter()
            .zip(rhs_data.iter())
            .map(|(&a, &b)| op(a, b))
            .collect()
    } else {
        let op = Arc::new(op);
        let lhs = Arc::new(lhs_data.to_vec());
        let rhs = Arc::new(rhs_data.to_vec());
        
        (0..len).into_par_iter()
            .map(|i| op(lhs[i], rhs[i]))
            .collect()
    }
}

fn parallel_scalar_op<F>(data: &[f32], scalar: f32, op: F) -> Vec<f32>
where
    F: Fn(f32, f32) -> f32 + Sync + Send + 'static,
{
    let len = data.len();

    if len < PARALLEL_THRESHOLD {
        data.iter().map(|&a| op(a, scalar)).collect()
    } else {
        let op = Arc::new(op);
        let data = Arc::new(data.to_vec());
        let scalar = scalar;
        
        (0..len).into_par_iter()
            .map(|i| op(data[i], scalar))
            .collect()
    }
}

impl Add for Tensor {
    type Output = Result<Tensor, TensorError>;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &Tensor {
    type Output = Result<Tensor, TensorError>;

    fn add(self, rhs: Self) -> Self::Output {
        check_shape_compatible(self, rhs)?;
        let data = parallel_binary_op(self.data(), rhs.data(), |a, b| a + b);
        Ok(Tensor::build(data, self.shape().clone())?)
    }
}

impl Sub for Tensor {
    type Output = Result<Tensor, TensorError>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub for &Tensor {
    type Output = Result<Tensor, TensorError>;

    fn sub(self, rhs: Self) -> Self::Output {
        check_shape_compatible(self, rhs)?;
        let data = parallel_binary_op(self.data(), rhs.data(), |a, b| a - b);
        Ok(Tensor::build(data, self.shape().clone())?)
    }
}

impl Mul for Tensor {
    type Output = Result<Tensor, TensorError>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul for &Tensor {
    type Output = Result<Tensor, TensorError>;

    fn mul(self, rhs: Self) -> Self::Output {
        check_shape_compatible(self, rhs)?;
        let data = parallel_binary_op(self.data(), rhs.data(), |a, b| a * b);
        Ok(Tensor::build(data, self.shape().clone())?)
    }
}

impl Div for Tensor {
    type Output = Result<Tensor, TensorError>;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl Div for &Tensor {
    type Output = Result<Tensor, TensorError>;

    fn div(self, rhs: Self) -> Self::Output {
        check_shape_compatible(self, rhs)?;
        let data = parallel_binary_op(self.data(), rhs.data(), |a, b| a / b);
        Ok(Tensor::build(data, self.shape().clone())?)
    }
}

impl Add<f32> for Tensor {
    type Output = Tensor;

    fn add(self, rhs: f32) -> Self::Output {
        &self + rhs
    }
}

impl Add<f32> for &Tensor {
    type Output = Tensor;

    fn add(self, rhs: f32) -> Self::Output {
        let data = parallel_scalar_op(self.data(), rhs, |a, b| a + b);
        Tensor::build(data, self.shape().clone()).unwrap()
    }
}

impl Sub<f32> for Tensor {
    type Output = Tensor;

    fn sub(self, rhs: f32) -> Self::Output {
        &self - rhs
    }
}

impl Sub<f32> for &Tensor {
    type Output = Tensor;

    fn sub(self, rhs: f32) -> Self::Output {
        let data = parallel_scalar_op(self.data(), rhs, |a, b| a - b);
        Tensor::build(data, self.shape().clone()).unwrap()
    }
}

impl Mul<f32> for Tensor {
    type Output = Tensor;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl Mul<f32> for &Tensor {
    type Output = Tensor;

    fn mul(self, rhs: f32) -> Self::Output {
        let data = parallel_scalar_op(self.data(), rhs, |a, b| a * b);
        Tensor::build(data, self.shape().clone()).unwrap()
    }
}

impl Div<f32> for Tensor {
    type Output = Tensor;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}

impl Div<f32> for &Tensor {
    type Output = Tensor;

    fn div(self, rhs: f32) -> Self::Output {
        let data = parallel_scalar_op(self.data(), rhs, |a, b| a / b);
        Tensor::build(data, self.shape().clone()).unwrap()
    }
}

impl Tensor {
    pub fn pow(&self, exp: f32) -> Self {
        let data: Vec<f32> = self.data().iter().map(|a| a.powf(exp)).collect();
        Tensor::build(data, self.shape().clone()).unwrap()
    }

    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(&[usize], f32) -> f32 + Sync + Send + 'static,
    {
        let data = parallel_map_op(self.data(), self.shape(), f);
        Tensor::build(data, self.shape().clone()).unwrap()
    }

    pub fn map_with<C, F>(&self, ctx: C, f: F) -> Self
    where
        C: Copy + Send + Sync + 'static,
        F: Fn(&[usize], f32, C) -> f32 + Send + Sync + 'static,
    {
        let data = parallel_map_op_with(self.data(), self.shape(), ctx, f);
        Tensor::build(data, self.shape().clone()).unwrap()
    }

    pub fn flatten_index(&self, indices: &[usize]) -> usize {
        let shape = self.shape();
        let ndim = shape.len();

        let mut strides = vec![1usize; ndim];
        for i in (0..ndim - 1).rev() {
            strides[i] = shape[i + 1..].iter().product::<usize>();
        }

        indices
            .iter()
            .enumerate()
            .map(|(i, &idx)| idx * strides[i])
            .sum()
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> Result<Tensor, TensorError> {
        let shape = self.shape();
        if shape.is_empty() {
            return Err(TensorError::InvalidShape);
        }

        let first_dim = shape[0];
        if range.start >= first_dim || range.end > first_dim {
            return Err(TensorError::IndexError {
                dim: 0,
                max: first_dim - 1,
            });
        }

        let new_first_dim = range.end - range.start;
        let row_size = shape[1..].iter().product::<usize>();
        let start = range.start * row_size;
        let end = range.end * row_size;
        let sub_data: Vec<f32> = self.data()[start..end].to_vec();
        let mut new_shape = shape.clone();
        new_shape[0] = new_first_dim;

        Tensor::build(sub_data, new_shape)
    }

    pub fn slice_multi(&self, slice_str: &str) -> Result<Tensor, TensorError> {
        let trimmed = slice_str.trim();
        if trimmed.is_empty() {
            return Err(TensorError::SliceError("Slice string is empty".to_string()));
        }

        let parts: Vec<&str> = trimmed.split(',').collect();
        let shape = self.shape();
        let ndim = shape.len();

        if parts.len() > ndim {
            return Err(TensorError::SliceError(
                format!("Too many slice dimensions (got {}, tensor has {})", parts.len(), ndim)
            ));
        }

        let mut new_shape = shape.clone();
        let mut start_indices = vec![0usize; ndim];
        let mut end_indices = shape.clone();

        for (dim_idx, part) in parts.iter().enumerate() {
            let range_parts: Vec<&str> = part.trim().split(':').collect();
            let (start, end) = match range_parts.as_slice() {
                [] => (0, shape[dim_idx]),
                [""] => (0, shape[dim_idx]),
                ["", ""] => (0, shape[dim_idx]),
                ["", end] if !end.is_empty() => {
                    let e = end.parse::<usize>()
                        .map_err(|_| TensorError::SliceError(format!("Invalid end index: {}", end)))?;
                    (0, e)
                }
                [start, ""] if !start.is_empty() => {
                    let s = start.parse::<usize>()
                        .map_err(|_| TensorError::SliceError(format!("Invalid start index: {}", start)))?;
                    (s, shape[dim_idx])
                }
                ["", _] => (0, shape[dim_idx]),
                [_, ""] => (0, shape[dim_idx]),
                [start, end] => {
                    let s = start.parse::<usize>()
                        .map_err(|_| TensorError::SliceError(format!("Invalid start index: {}", start)))?;
                    let e = end.parse::<usize>()
                        .map_err(|_| TensorError::SliceError(format!("Invalid end index: {}", end)))?;
                    if s >= e {
                        return Err(TensorError::SliceError(
                            format!("Start index {} must be less than end index {}", s, e)
                        ));
                    }
                    (s, e)
                }
                _ => {
                    return Err(TensorError::SliceError(
                        format!("Invalid slice format: {}", part)
                    ));
                }
            };

            if start >= shape[dim_idx] {
                return Err(TensorError::SliceError(
                    format!("Start index {} exceeds dimension size {}", start, shape[dim_idx])
                ));
            } else if end > shape[dim_idx] {
                return Err(TensorError::SliceError(
                    format!("End index {} exceeds dimension size {}", end, shape[dim_idx])
                ));
            }

            new_shape[dim_idx] = end - start;
            start_indices[dim_idx] = start;
            end_indices[dim_idx] = end;
        }

        let mut result_data = Vec::new();
        let total_size = new_shape.iter().product::<usize>();
        result_data.reserve(total_size);

        let strides: Vec<usize> = (0..ndim)
            .map(|i| {
                if i == ndim - 1 {
                    1
                } else {
                    shape[i + 1..].iter().product()
                }
            })
            .collect();

        let mut current_indices = start_indices.clone();
        for _ in 0..total_size {
            let mut data_idx = 0usize;
            for dim_idx in 0..ndim {
                data_idx += current_indices[dim_idx] * strides[dim_idx];
            }
            result_data.push(self.data()[data_idx]);

            for dim_idx in (0..ndim).rev() {
                current_indices[dim_idx] += 1;
                if current_indices[dim_idx] < end_indices[dim_idx] {
                    break;
                }
                current_indices[dim_idx] = start_indices[dim_idx];
            }
        }

        Tensor::build(result_data, new_shape)
    }

    pub fn transpose(&self) -> Result<Tensor, TensorError> {
        let shape = self.shape();
        let ndim = shape.len();
        
        if ndim < 2 {
            return Err(TensorError::InvalidShape);
        }
        
        let rows = shape[ndim - 2];
        let cols = shape[ndim - 1];
        let outer_size = if ndim == 2 { 1 } else { shape[..ndim - 2].iter().product() };
        
        let data = self.data();
        let mut result = Vec::with_capacity(data.len());
        
        for outer in 0..outer_size {
            for col in 0..cols {
                for row in 0..rows {
                    let old_idx = outer * rows * cols + row * cols + col;
                    let new_idx = outer * rows * cols + col * rows + row;
                    if new_idx < data.len() {
                        result.push(data[old_idx]);
                    }
                }
            }
        }
        
        let mut new_shape = shape.clone();
        new_shape[ndim - 2] = cols;
        new_shape[ndim - 1] = rows;
        
        Tensor::build(result, new_shape)
    }
}

impl Index<usize> for Tensor {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data()[index]
    }
}

impl Index<[usize; 1]> for Tensor {
    type Output = f32;

    fn index(&self, indices: [usize; 1]) -> &Self::Output {
        &self.data()[indices[0]]
    }
}

impl Index<[usize; 2]> for Tensor {
    type Output = f32;

    fn index(&self, indices: [usize; 2]) -> &Self::Output {
        let flat = self.flatten_index(&indices);
        &self.data()[flat]
    }
}

impl Index<[usize; 3]> for Tensor {
    type Output = f32;

    fn index(&self, indices: [usize; 3]) -> &Self::Output {
        let flat = self.flatten_index(&indices);
        &self.data()[flat]
    }
}

impl Index<[usize; 4]> for Tensor {
    type Output = f32;

    fn index(&self, indices: [usize; 4]) -> &Self::Output {
        let flat = self.flatten_index(&indices);
        &self.data()[flat]
    }
}

impl IndexMut<usize> for Tensor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.get_data_mut()[index]
    }
}

impl IndexMut<[usize; 1]> for Tensor {
    fn index_mut(&mut self, indices: [usize; 1]) -> &mut Self::Output {
        &mut self.get_data_mut()[indices[0]]
    }
}

impl IndexMut<[usize; 2]> for Tensor {
    fn index_mut(&mut self, indices: [usize; 2]) -> &mut Self::Output {
        let flat = self.flatten_index(&indices);
        &mut self.get_data_mut()[flat]
    }
}

impl IndexMut<[usize; 3]> for Tensor {
    fn index_mut(&mut self, indices: [usize; 3]) -> &mut Self::Output {
        let flat = self.flatten_index(&indices);
        &mut self.get_data_mut()[flat]
    }
}

impl IndexMut<[usize; 4]> for Tensor {
    fn index_mut(&mut self, indices: [usize; 4]) -> &mut Self::Output {
        let flat = self.flatten_index(&indices);
        &mut self.get_data_mut()[flat]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tensor(data: Vec<f32>, shape: Vec<usize>) -> Tensor {
        Tensor::build(data, shape).unwrap()
    }

    #[test]
    fn test_add_tensors() {
        let x = make_tensor(vec![1.0, 2.0, 4.0, 8.0], vec![4]);
        let y = make_tensor(vec![2.0, 2.0, 2.0, 2.0], vec![4]);
        let result = (x + y).unwrap();
        assert_eq!(result.data(), &vec![3.0, 4.0, 6.0, 10.0]);
    }

    #[test]
    fn test_sub_tensors() {
        let x = make_tensor(vec![1.0, 2.0, 4.0, 8.0], vec![4]);
        let y = make_tensor(vec![2.0, 2.0, 2.0, 2.0], vec![4]);
        let result = (x - y).unwrap();
        assert_eq!(result.data(), &vec![-1.0, 0.0, 2.0, 6.0]);
    }

    #[test]
    fn test_mul_tensors() {
        let x = make_tensor(vec![1.0, 2.0, 4.0, 8.0], vec![4]);
        let y = make_tensor(vec![2.0, 2.0, 2.0, 2.0], vec![4]);
        let result = (x * y).unwrap();
        assert_eq!(result.data(), &vec![2.0, 4.0, 8.0, 16.0]);
    }

    #[test]
    fn test_div_tensors() {
        let x = make_tensor(vec![1.0, 2.0, 4.0, 8.0], vec![4]);
        let y = make_tensor(vec![2.0, 2.0, 2.0, 2.0], vec![4]);
        let result = (x / y).unwrap();
        assert_eq!(result.get_data(), &vec![0.5, 1.0, 2.0, 4.0]);
    }

    #[test]
    fn test_pow_tensors() {
        let x = make_tensor(vec![1.0, 2.0, 4.0, 8.0], vec![4]);
        let result = x.pow(2.0);
        assert_eq!(result.get_data(), &vec![1.0, 4.0, 16.0, 64.0]);
    }

    #[test]
    fn test_add_scalar() {
        let x = make_tensor(vec![1.0, 2.0, 4.0, 8.0], vec![4]);
        let result = x + 2.0;
        assert_eq!(result.get_data(), &vec![3.0, 4.0, 6.0, 10.0]);
    }

    #[test]
    fn test_incompatible_shapes() {
        let x = make_tensor(vec![1.0, 2.0], vec![2]);
        let y = make_tensor(vec![1.0, 2.0, 3.0], vec![3]);
        let result = x + y;
        assert!(result.is_err());
    }

    #[test]
    fn test_1d_index() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![4]);
        assert_eq!(t[0], 1.0);
        assert_eq!(t[1], 2.0);
        assert_eq!(t[2], 3.0);
        assert_eq!(t[3], 4.0);
    }

    #[test]
    fn test_2d_index() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(t[[0, 0]], 1.0);
        assert_eq!(t[[0, 1]], 2.0);
        assert_eq!(t[[1, 0]], 3.0);
        assert_eq!(t[[1, 1]], 4.0);
    }

    #[test]
    fn test_3d_index() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], vec![2, 2, 2]);
        assert_eq!(t[[0, 0, 0]], 1.0);
        assert_eq!(t[[0, 0, 1]], 2.0);
        assert_eq!(t[[0, 1, 0]], 3.0);
        assert_eq!(t[[0, 1, 1]], 4.0);
        assert_eq!(t[[1, 0, 0]], 5.0);
        assert_eq!(t[[1, 0, 1]], 6.0);
        assert_eq!(t[[1, 1, 0]], 7.0);
        assert_eq!(t[[1, 1, 1]], 8.0);
    }

    #[test]
    fn test_4d_index() {
        let data: Vec<f32> = (1..=16).map(|i| i as f32).collect();
        let t = make_tensor(data, vec![2, 2, 2, 2]);
        assert_eq!(t[[0, 0, 0, 0]], 1.0);
        assert_eq!(t[[0, 0, 0, 1]], 2.0);
        assert_eq!(t[[0, 0, 1, 0]], 3.0);
        assert_eq!(t[[0, 0, 1, 1]], 4.0);
        assert_eq!(t[[1, 1, 1, 0]], 15.0);
        assert_eq!(t[[1, 1, 1, 1]], 16.0);
    }

    #[test]
    fn test_1d_mutable_index() {
        let mut t = make_tensor(vec![1.0, 2.0, 3.0], vec![3]);
        t[1] = 10.0;
        assert_eq!(t[1], 10.0);
    }

    #[test]
    fn test_2d_mutable_index() {
        let mut t = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        t[[1, 0]] = 10.0;
        assert_eq!(t[[1, 0]], 10.0);
    }

    #[test]
    fn test_slice_1d() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![5]);
        let sliced = t.slice(1..4).unwrap();
        assert_eq!(sliced.shape(), &vec![3]);
        assert_eq!(sliced.get_data(), &vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_slice_2d() {
        let data: Vec<f32> = (1..=12).map(|i| i as f32).collect();
        let t = make_tensor(data, vec![3, 4]);
        let sliced = t.slice(0..2).unwrap();
        assert_eq!(sliced.shape(), &vec![2, 4]);
        assert_eq!(
            sliced.get_data(),
            &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        );
    }

    #[test]
    fn test_slice_out_of_bounds() {
        let t = make_tensor(vec![1.0, 2.0, 3.0], vec![3]);
        let result = t.slice(0..10);
        assert!(result.is_err());
    }

    #[test]
    fn test_map_1d() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![4]);
        let result = t.map(|indices, value| {
            assert_eq!(indices.len(), 1);
            value * 2.0
        });
        assert_eq!(result.get_data(), &vec![2.0, 4.0, 6.0, 8.0]);
        assert_eq!(result.shape(), &vec![4]);
    }

    #[test]
    fn test_map_2d() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let result = t.map(|indices, value| {
            assert_eq!(indices.len(), 2);
            value * 3.0
        });
        assert_eq!(result.get_data(), &vec![3.0, 6.0, 9.0, 12.0]);
        assert_eq!(result.shape(), &vec![2, 2]);
    }

    #[test]
    fn test_map_with_index() {
        let t = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let result = t.map(|indices, value| {
            let sum: usize = indices.iter().sum();
            value + sum as f32
        });
        assert_eq!(result.get_data(), &vec![1.0, 3.0, 4.0, 6.0]);
        assert_eq!(result.shape(), &vec![2, 2]);
    }

    #[test]
    fn test_map_3d() {
        let data: Vec<f32> = (0..8).map(|i| i as f32).collect();
        let t = make_tensor(data, vec![2, 2, 2]);
        let result = t.map(|indices, value| {
            let flat = indices[0] * 4 + indices[1] * 2 + indices[2];
            assert!(flat < 8);
            value * 2.0
        });
        assert_eq!(
            result.data(),
            &vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0]
        );
    }

    #[test]
    fn test_map_parallel_large() {
        let data: Vec<f32> = (0..200_000).map(|i| i as f32).collect();
        let t = make_tensor(data, vec![200_000]);
        let result = t.map(|_indices, value| value * 2.0);
        assert_eq!(result.data().len(), 200_000);
        assert_eq!(result.data()[0], 0.0);
        assert_eq!(result.data()[199_999], 399_998.0);
    }

    #[test]
    fn test_matmul_2d_1d() {
        let a = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let b = make_tensor(vec![5.0, 6.0], vec![2]);
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &vec![2]);
        assert_eq!(c.data(), &vec![17.0, 39.0]);
    }

    #[test]
    fn test_matmul_2d_small() {
        let a = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let b = make_tensor(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &vec![2, 2]);
        assert_eq!(c.data(), &vec![19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn test_matmul_2d_identity() {
        let a = make_tensor(vec![1.0, 0.0, 0.0, 1.0], vec![2, 2]);
        let b = make_tensor(vec![1.0, 0.0, 0.0, 1.0], vec![2, 2]);
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &vec![2, 2]);
        assert_eq!(c.data(), &vec![1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_matmul_3d_batch() {
        let a = make_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], vec![2, 2, 2]);
        let b = make_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], vec![2, 2, 2]);
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &vec![2, 2, 2]);
        assert_eq!(
            c.data(),
            &vec![7.0, 10.0, 15.0, 22.0, 67.0, 78.0, 91.0, 106.0]
        );
    }

    #[test]
    fn test_matmul_incompatible_shapes() {
        let a = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let b = make_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![3, 2]);
        let result = a.matmul(&b);
        assert!(result.is_err());
    }

    #[test]
    fn test_matmul_empty_shape() {
        let a = make_tensor(vec![1.0, 2.0], vec![2]);
        let b = make_tensor(vec![1.0], vec![]);
        let result = a.matmul(&b);
        assert!(result.is_err());
    }

    #[test]
    fn test_matmul_2d_large_parallel() {
        let a_data: Vec<f32> = (0..100_000).map(|i| (i as f32) * 0.01).collect();
        let b_data: Vec<f32> = (0..100_000).map(|i| (i as f32) * 0.01).collect();
        let a = make_tensor(a_data, vec![1000, 100]);
        let b = make_tensor(b_data, vec![100, 1000]);
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &vec![1000, 1000]);
        assert_eq!(c.data().len(), 1_000_000);
    }

    #[test]
    fn test_matmul_4d_batch_parallel() {
        let a_data: Vec<f32> = (0..24000).map(|i| (i as f32) * 0.01).collect();
        let b_data: Vec<f32> = (0..24000).map(|i| (i as f32) * 0.01).collect();
        let a = make_tensor(a_data, vec![2, 3, 4000]);
        let b = make_tensor(b_data, vec![2, 4000, 3]);
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &vec![2, 3, 3]);
        assert_eq!(c.data().len(), 18);
    }

    #[test]
    fn test_matmul_1d_2d() {
        let v = make_tensor(vec![1.0, 2.0], vec![2]);
        let m = make_tensor(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let c = v.matmul(&m).unwrap();
        assert_eq!(c.shape(), &vec![2]);
        assert_eq!(c.data(), &vec![7.0, 10.0]);
    }

    #[test]
    fn test_matmul_1d_3d() {
        let v = make_tensor(vec![1.0, 2.0], vec![2]);
        let m = make_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], vec![2, 2, 2]);
        let c = v.matmul(&m).unwrap();
        assert_eq!(c.shape(), &vec![2, 2]);
        assert_eq!(c.data(), &vec![7.0, 10.0, 19.0, 22.0]);
    }

    #[test]
    fn test_slice_multi_basic() {
        let t = make_tensor(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
            vec![3, 4],
        );
        let sliced = t.slice_multi("1:3, 0:2").unwrap();
        assert_eq!(sliced.shape(), &vec![2, 2]);
        assert_eq!(sliced.data(), &vec![5.0, 6.0, 9.0, 10.0]);
    }

    #[test]
    fn test_slice_multi_full() {
        let t = make_tensor(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![2, 3],
        );
        let sliced = t.slice_multi(":, 1:2").unwrap();
        assert_eq!(sliced.shape(), &vec![2, 1]);
        assert_eq!(sliced.data(), &vec![2.0, 5.0]);
    }

    #[test]
    fn test_slice_multi_to_end() {
        let t = make_tensor(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![2, 3],
        );
        let sliced = t.slice_multi("1:, :").unwrap();
        assert_eq!(sliced.shape(), &vec![1, 3]);
        assert_eq!(sliced.data(), &vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_slice_single_dim() {
        let t = make_tensor(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![2, 3],
        );
        let sliced = t.slice_multi("1:2, :").unwrap();
        assert_eq!(sliced.shape(), &vec![1, 3]);
        assert_eq!(sliced.data(), &vec![4.0, 5.0, 6.0]);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SliceRange {
    Full,
    FromStart(usize),
    ToEnd(usize),
    Range(usize, usize),
}

impl SliceRange {
    pub fn parse(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed == ":" {
            return Ok(SliceRange::Full);
        }

        let parts: Vec<&str> = trimmed.split(':').collect();
        match parts.as_slice() {
            [] => Ok(SliceRange::Full),
            [""] => Ok(SliceRange::Full),
            [start, ""] => {
                start.parse::<usize>()
                    .map(SliceRange::ToEnd)
                    .map_err(|_| format!("Invalid start index: {}", start))
            }
            ["", end] => {
                end.parse::<usize>()
                    .map(SliceRange::FromStart)
                    .map_err(|_| format!("Invalid end index: {}", end))
            }
            [start, end] => {
                let start_idx = start.parse::<usize>()
                    .map_err(|_| format!("Invalid start index: {}", start))?;
                let end_idx = end.parse::<usize>()
                    .map_err(|_| format!("Invalid end index: {}", end))?;
                if start_idx >= end_idx {
                    Err(format!("Start index {} must be less than end index {}", start_idx, end_idx))
                } else {
                    Ok(SliceRange::Range(start_idx, end_idx))
                }
            }
            _ => Err(format!("Invalid slice format: {}", trimmed).to_string()),
        }
    }

    pub fn apply(&self, dim_size: usize) -> Result<(usize, usize), String> {
        match self {
            SliceRange::Full => Ok((0, dim_size)),
            SliceRange::FromStart(end) => {
                if *end > dim_size {
                    Err(format!("End index {} exceeds dimension size {}", end, dim_size))
                } else {
                    Ok((0, *end))
                }
            }
            SliceRange::ToEnd(start) => {
                if *start >= dim_size {
                    Err(format!("Start index {} exceeds dimension size {}", start, dim_size))
                } else {
                    Ok((*start, dim_size))
                }
            }
            SliceRange::Range(start, end) => {
                if *start >= dim_size {
                    Err(format!("Start index {} exceeds dimension size {}", start, dim_size))
                } else if *end > dim_size {
                    Err(format!("End index {} exceeds dimension size {}", end, dim_size))
                } else if *start >= *end {
                    Err(format!("Start index {} must be less than end index {}", start, end))
                } else {
                    Ok((*start, *end))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiSlice {
    #[allow(dead_code)]
    ranges: Vec<SliceRange>,
}

impl MultiSlice {
    pub fn from_str(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("Slice string is empty".to_string());
        }

        let ranges: Result<Vec<SliceRange>, String> = trimmed
            .split(',')
            .map(|part| SliceRange::parse(part))
            .collect();

        ranges.map(|ranges| MultiSlice { ranges })
}
}
