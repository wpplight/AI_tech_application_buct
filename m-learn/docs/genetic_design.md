# Genetic Algorithm Crate 设计文档

## 1. 概述

这是一个轻量级、可配置的遗传算法库，专为实数优化问题设计。

### 设计目标
- 简单易用的 API
- 支持自定义适应度函数、交叉算法
- 支持逐步迭代和状态查询
- **用户只需定义单个染色体的操作，框架自动处理并行调度**
- **内部使用 Vec<f32>，避免数据拷贝，可选集成 optim crate 的并行能力**

---

## 2. 核心概念

```
┌─────────────────────────────────────────────────────────────────┐
│                      GeneticAlgorithm                            │
│                                                                 │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐            │
│  │  Population │ → │   Fitness   │ → │  Selection  │            │
│  │  Vec<f32>   │   │    Fn       │   │    Fn       │            │
│  │  [M*N]      │   │             │   │             │            │
│  └─────────────┘   └─────────────┘   └─────────────┘            │
│         ↑                                    ↓                    │
│         │              ┌─────────────┐                          │
│         └────────────── │  Crossover  │ ← 函数式接口            │
│                        │    Fn        │                          │
│                        └─────────────┘                          │
│                              ↑                                   │
│                        可选: optim::GlobalPool                  │
└─────────────────────────────────────────────────────────────────┘

数据布局: Vec<f32> 扁平化存储 [M * N]
         M = 种群大小, N = 基因长度
         索引公式: population[i * N + j] = 第i个染色体的第j个基因
```

---

## 3. 模块结构

```
crates/genetic/
├── src/
│   ├── lib.rs              # 模块导出
│   ├── genetic.rs          # 主结构体 GeneticAlgorithm
│   ├── crossover.rs        # 预置交叉函数
│   ├── selection.rs        # 预置选择函数
│   └── utils.rs            # 辅助函数
└── Cargo.toml
```

依赖：
- `rand` - 随机数生成
- `optim` - 全局并行池（基于Rayon）

---

## 4. 函数式接口设计

### 4.1 设计原则

**用户只需要定义单个小操作，框架自动并行调度。**

| 操作 | 用户定义 | 框架自动处理 |
|------|---------|-------------|
| 适应度评估 | `Fn(&[f32]) -> f32` | 并行计算所有染色体 |
| 交叉操作 | `Fn(&[f32], &[f32]) -> (Vec<f32>, Vec<f32>)` | 并行执行所有对 |
| 选择操作 | `Fn(&[f32], usize) -> Vec<usize>` | 执行选择逻辑 |

### 4.2 适应度函数类型

```rust
/// 适应度评估函数类型
/// 
/// 输入: 单个染色体的基因数据（只读切片，零拷贝）
/// 输出: 适应度值（f32，越大越好）
pub type FitnessFn = Box<dyn Fn(&[f32]) -> f32 + Send + Sync>;

/// 示例：最大化所有基因之和
let fitness_fn: FitnessFn = Box::new(|genes| {
    genes.iter().sum::<f32>()
});

/// 示例：Rosenbrock 函数
let rosenbrock_fn: FitnessFn = Box::new(|genes| {
    let mut sum = 0.0;
    for i in 0..genes.len() - 1 {
        sum += 100.0 * (genes[i+1] - genes[i].powi(2)).powi(2) 
               + (genes[i] - 1.0).powi(2);
    }
    1.0 / (1.0 + sum)  // 越小越好，所以取倒数
});
```

### 4.3 交叉函数类型

```rust
/// 交叉函数类型
/// 
/// 输入: 两个父代染色体的基因数据（只读切片）
/// 输出: 两个子代染色体的基因数据
/// 
/// 用户只需定义"单对染色体的交叉逻辑"
/// 框架自动并行调度所有配对
pub type CrossoverFn = Box<dyn Fn(&[f32], &[f32]) -> (Vec<f32>, Vec<f32>) + Send + Sync>;

/// 示例：单点交叉
let single_point: CrossoverFn = Box::new(|p1, p2| {
    let mid = p1.len() / 2;
    let mut c1 = p1[..mid].to_vec();
    c1.extend_from_slice(&p2[mid..]);
    let mut c2 = p2[..mid].to_vec();
    c2.extend_from_slice(&p1[mid..]);
    (c1, c2)
});

/// 示例：SBX 交叉（需要传入 eta）
pub fn sbx_crossover(eta: f64) -> CrossoverFn {
    Box::new(move |p1, p2| {
        let mut c1 = Vec::with_capacity(p1.len());
        let mut c2 = Vec::with_capacity(p2.len());
        
        for i in 0..p1.len() {
            let x1 = p1[i] as f64;
            let x2 = p2[i] as f64;
            
            let (y1, y2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let u = rand::random::<f64>();
            
            let beta = if u <= 0.5 {
                (2.0 * u).powf(1.0 / (eta + 1.0))
            } else {
                (1.0 / (2.0 * (1.0 - u))).powf(1.0 / (eta + 1.0))
            };
            
            c1.push(0.5 * ((y1 + y2) - beta * (y2 - y1)) as f32);
            c2.push(0.5 * ((y1 + y2) + beta * (y2 - y1)) as f32);
        }
        
        (c1, c2)
    })
}
```

### 4.4 选择函数类型

```rust
/// 选择函数类型
/// 
/// 输入: 
///   - fitness: 所有染色体的适应度数组（只读）
///   - num_to_select: 需要选择的数量
/// 
/// 输出: 被选中的染色体索引
/// 
/// 用户只需定义"选择逻辑"
/// 框架负责调用和结果处理
pub type SelectionFn = Box<dyn Fn(&[f32], usize) -> Vec<usize> + Send + Sync>;

/// 示例：锦标赛选择
pub fn tournament_selection(k: usize) -> SelectionFn {
    Box::new(move |fitness, num_to_select| {
        let pop_size = fitness.len();
        let mut selected = Vec::with_capacity(num_to_select);
        
        for _ in 0..num_to_select {
            // 随机选择 k 个参赛者索引
            let contestants: Vec<usize> = (0..pop_size)
                .collect::<Vec<_>>()
                .choose_multiple(&mut rand::rng(), k);
            
            // 返回适应度最高的索引
            let winner = contestants.iter()
                .max_by(|&&a, &&b| fitness[a].partial_cmp(&fitness[b]).unwrap())
                .unwrap();
            selected.push(*winner);
        }
        
        selected
    })
}
```

### 4.5 变异函数类型

```rust
/// 变异函数类型
///
/// 输入: 单个染色体的基因数据（只读）
/// 输出: 变异后的基因数据
///
/// 变异在交叉阶段触发，以 mutation_rate 概率作用于每个子代
pub type MutationFn = Box<dyn Fn(&[f32]) -> Vec<f32> + Send + Sync>;

/// 示例：均匀变异 - 随机选择一个基因，用新值替换
pub fn uniform_mutation(min: f32, max: f32) -> MutationFn {
    Box::new(move |genes: &[f32]| {
        let mut mutated = genes.to_vec();
        let idx = rand::thread_rng().gen_range(0..genes.len());
        mutated[idx] = rand::thread_rng().gen_range(min..max);
        mutated
    })
}

/// 示例：高斯变异 - 随机选择一个基因，添加高斯噪声
pub fn gaussian_mutation(sigma: f32) -> MutationFn {
    Box::new(move |genes: &[f32]| {
        let mut mutated = genes.to_vec();
        let idx = rand::thread_rng().gen_range(0..genes.len());
        let noise = rand::thread_rng().gen_range(-sigma..sigma);
        mutated[idx] += noise;
        mutated
    })
}

/// 示例：多项式变异 - 用于实数优化，效果更平滑
pub fn polynomial_mutation(eta: f64, min: f32, max: f32) -> MutationFn {
    Box::new(move |genes: &[f32]| {
        let mut mutated = genes.to_vec();
        let idx = rand::thread_rng().gen_range(0..genes.len());
        let x: f64 = genes[idx] as f64;
        
        let u: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let delta: f64;
        
        if u < 0.5 {
            let delta_l = (2.0 * u).powf(1.0 / (eta + 1.0)) - 1.0;
            delta = delta_l * (x - min as f64) / (x - min as f64 + 1e-10);
        } else {
            let delta_r = 1.0 - (2.0 * (1.0 - u)).powf(1.0 / (eta + 1.0));
            delta = delta_r * (max as f64 - x) / (max as f64 - x + 1e-10);
        }
        
        mutated[idx] = (x + delta).clamp(min as f64, max as f64) as f32;
        mutated
    })
}
```

---

## 5. 主结构体：GeneticAlgorithm

### 5.1 数据结构

```rust
pub struct GeneticAlgorithm {
    population: Vec<f32>,           // 种群数据，扁平化存储 [M * N]
    fitness_cache: Vec<f32>,        // 适应度缓存 [M]
    gene_size: usize,              // 染色体长度（基因数量）
    pop_size: usize,               // 种群大小
    generation: usize,             // 当前迭代次数
    
    // 函数式接口
    fitness_fn: FitnessFn,
    crossover_fn: CrossoverFn,
    selection_fn: SelectionFn,
    
    // 参数
    mutation_rate: f32,             // 变异概率
    elite_count: usize,             // 精英保留数量
}

impl GeneticAlgorithm {
    /// 创建遗传算法实例
    ///
    /// # 参数
    /// - `gene_size`: 染色体长度（基因数量）
    /// - `fitness_fn`: 适应度评估函数
    pub fn new(
        gene_size: usize,
        fitness_fn: impl Fn(&[f32]) -> f32 + 'static + Send + Sync,
    ) -> Self {
        Self {
            population: vec![0.0; DEFAULT_POP_SIZE * gene_size],
            fitness_cache: vec![0.0; DEFAULT_POP_SIZE],
            gene_size,
            pop_size: DEFAULT_POP_SIZE,
            generation: 0,
            fitness_fn: Box::new(fitness_fn),
            crossover_fn: Box::new(single_point_crossover),
            selection_fn: tournament_selection(DEFAULT_TOURNAMENT_SIZE),
            mutation_rate: 0.1,
            elite_count: 2,
        }
    }
    
    /// 随机初始化种群
    pub fn randomize(&mut self) {
        for i in 0..self.population.len() {
            self.population[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
    }
    
    /// 设置种群大小（可选，默认100）
    pub fn population_size(mut self, size: usize) -> Self {
        self.pop_size = size;
        self.population = vec![0.0; size * self.gene_size];
        self.fitness_cache = vec![0.0; size];
        self
    }
    
    /// 设置交叉函数
    pub fn crossover<F>(mut self, crossover_fn: F) -> Self 
    where
        F: Fn(&[f32], &[f32]) -> (Vec<f32>, Vec<f32>) + 'static + Send + Sync,
    {
        self.crossover_fn = Box::new(crossover_fn);
        self
    }
    
    /// 设置选择函数
    pub fn selection<F>(mut self, selection_fn: F) -> Self 
    where
        F: Fn(&[f32], usize) -> Vec<usize> + 'static + Send + Sync,
    {
        self.selection_fn = Box::new(selection_fn);
        self
    }
    
    /// 设置锦标赛大小
    pub fn tournament_size(mut self, k: usize) -> Self {
        self.selection_fn = tournament_selection(k);
        self
    }
    
    /// 设置变异概率
    pub fn mutation_rate(mut self, rate: f32) -> Self {
        self.mutation_rate = rate;
        self
    }
    
    /// 设置精英保留数量
    pub fn elite_count(mut self, count: usize) -> Self {
        self.elite_count = count;
        self
    }
}
```

---

## 6. 并行调度设计

### 6.1 整体流程

```
┌─────────────────────────────────────────────────────────────────┐
│                         step() 迭代                              │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  1. 适应度评估                                            │   │
│  │     - 获取线程数 → 划分种群 → 并行调用 fitness_fn        │   │
│  │     - 用户代码: |genes| { sum(genes) }                   │   │
│  │     - 框架自动并行，用户无感知                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              ↓                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  2. 锦标赛选择                                            │   │
│  │     - 调用 selection_fn(fitness, num_to_select)        │   │
│  │     - 返回索引列表 Vec<usize>                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              ↓                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  3. 交叉操作                                              │   │
│  │     - 成对调用 crossover_fn(parent1, parent2)            │   │
│  │     - 框架自动并行所有配对                                │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              ↓                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  4. 变异操作 + 精英保留                                   │   │
│  │     - 变异: 每个基因独立判断                              │   │
│  │     - 精英: 替换最差个体                                  │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 适应度评估（并行实现）

```rust
impl GeneticAlgorithm {
    /// 并行评估所有染色体的适应度
    ///
    /// # 流程
    /// 1. 获取线程数（通过 optim::GlobalPool）
    /// 2. 划分种群到各线程
    /// 3. 并行调用 fitness_fn（零拷贝切片访问）
    /// 4. 拼接结果
    fn evaluate_all_parallel(&self) -> Vec<f32> {
        let pop_size = self.pop_size;
        let gene_size = self.gene_size;
        let fitness_fn = &self.fitness_fn;
        
        // 获取线程数和分块数
        let num_threads = optim::GlobalPool::num_threads();
        let chunk_size = optim::GlobalPool::parallel_chunk_size(pop_size);
        let num_chunks = optim::GlobalPool::optimal_num_chunks(pop_size);
        
        // 并行计算
        let results: Vec<Vec<f32>> = (0..num_chunks)
            .map(|chunk_id| {
                let start = chunk_id * chunk_size;
                let end = std::cmp::min(start + chunk_size, pop_size);
                
                let mut chunk_fitness = Vec::with_capacity(end - start);
                
                for i in start..end {
                    // 零拷贝：直接引用原始数据的切片
                    let genes = &self.population[i * gene_size..(i + 1) * gene_size];
                    
                    // 调用用户定义的适应度函数
                    let fitness = fitness_fn(genes);
                    chunk_fitness.push(fitness);
                }
                
                chunk_fitness
            })
            .collect();
        
        // 拼接所有分块结果
        let mut result = Vec::with_capacity(pop_size);
        for chunk in results {
            result.extend(chunk);
        }
        
        result
    }
}
```

### 6.3 选择操作

```rust
impl GeneticAlgorithm {
    /// 锦标赛选择
    ///
    /// # 流程
    /// 1. 调用用户定义的 selection_fn
    /// 2. 返回被选中的染色体索引（不是复制数据）
    fn select_parents(&self, fitness: &[f32]) -> Vec<usize> {
        let num_to_select = self.pop_size / 2;
        
        // 调用用户定义的选择函数
        (self.selection_fn)(fitness, num_to_select)
    }
}
```

### 6.4 交叉操作（并行实现）

```rust
impl GeneticAlgorithm {
    /// 并行交叉操作
    ///
    /// # 流程
    /// 1. 根据索引提取父代数据
    /// 2. 成对调用 crossover_fn
    /// 3. 拼接所有子代到新种群
    fn crossover_parallel(&self, parent_indices: &[usize]) -> Vec<f32> {
        let num_pairs = parent_indices.len() / 2;
        let gene_size = self.gene_size;
        let crossover_fn = &self.crossover_fn;
        
        // 使用 optim::GlobalPool 获取分块信息
        let num_chunks = optim::GlobalPool::optimal_num_chunks(num_pairs);
        let chunk_size = optim::GlobalPool::parallel_chunk_size(num_pairs);
        
        // 并行交叉
        let results: Vec<Vec<f32>> = (0..num_chunks)
            .map(|chunk_id| {
                let start = chunk_id * chunk_size;
                let end = std::cmp::min(start + chunk_size, num_pairs);
                
                let mut chunk_offspring = Vec::with_capacity((end - start) * 2 * gene_size);
                
                for pair_idx in start..end {
                    // 提取父代数据（零拷贝切片）
                    let p1 = &self.population[parent_indices[pair_idx * 2] * gene_size..];
                    let p2 = &self.population[parent_indices[pair_idx * 2 + 1] * gene_size..];
                    let p1 = &p1[..gene_size];
                    let p2 = &p2[..gene_size];
                    
                    // 调用用户定义的交叉函数
                    let (child1, child2) = crossover_fn(p1, p2);
                    
                    chunk_offspring.extend_from_slice(&child1);
                    chunk_offspring.extend_from_slice(&child2);
                }
                
                chunk_offspring
            })
            .collect();
        
        // 拼接所有分块结果
        let total_size = num_pairs * 2 * gene_size;
        let mut result = Vec::with_capacity(total_size);
        for chunk in results {
            result.extend(chunk);
        }
        
        result
    }
}
```

### 6.5 变异操作

```rust
impl GeneticAlgorithm {
    /// 变异操作
    fn mutate(&mut self, offspring: Vec<f32>) -> Vec<f32> {
        let mut result = offspring;
        
        for i in 0..result.len() {
            if rand::random::<f32>() < self.mutation_rate {
                // 添加扰动: [-0.1, 0.1]
                let noise = rand::random::<f32>() * 0.2 - 0.1;
                result[i] += noise;
            }
        }
        
        result
    }
}
```

### 6.6 step 方法（核心迭代）

```rust
impl GeneticAlgorithm {
    /// 执行一轮遗传算法迭代
    pub fn step(&mut self) {
        // 1. 评估适应度（并行）
        self.fitness_cache = self.evaluate_all_parallel();
        
        // 2. 选择父代（返回索引）
        let parent_indices = self.select_parents(&self.fitness_cache);
        
        // 3. 交叉操作（并行）
        let offspring = self.crossover_parallel(&parent_indices);
        
        // 4. 变异操作
        let mutated = self.mutate(offspring);
        
        // 5. 精英保留：保留上一代最优个体
        if self.elite_count > 0 {
            self.keep_elite(mutated);
        } else {
            self.population = mutated;
        }
        
        self.generation += 1;
    }
    
    /// 精英保留：用上一代最优个体替换最差个体
    fn keep_elite(&mut self, mut new_population: Vec<f32>) {
        let elite_size = self.elite_count.min(self.pop_size);
        let gene_size = self.gene_size;
        
        // 获取上一代最优个体的索引
        let mut indices: Vec<usize> = (0..self.pop_size).collect();
        indices.sort_by(|&a, &b| {
            self.fitness_cache[b]
                .partial_cmp(&self.fitness_cache[a])
                .unwrap()
        });
        
        // 替换新种群中最差的个体
        for i in 0..elite_size {
            let worst_idx = self.pop_size - 1 - i;
            let worst_start = worst_idx * gene_size;
            let elite_start = indices[i] * gene_size;
            
            for j in 0..gene_size {
                new_population[worst_start + j] = self.population[elite_start + j];
            }
        }
        
        self.population = new_population;
    }
}
```

### 6.7 获取迭代状态

```rust
impl GeneticAlgorithm {
    /// 获取种群大小
    pub fn population_size(&self) -> usize {
        self.pop_size
    }
    
    /// 获取基因长度
    pub fn gene_size(&self) -> usize {
        self.gene_size
    }
    
    /// 获取适应度缓存
    pub fn fitness_cache(&self) -> &[f32] {
        &self.fitness_cache
    }
    
    /// 获取适应度缓存（排序后）
    pub fn get_fitness_sorted(&self) -> Vec<f32> {
        let mut sorted = self.fitness_cache.clone();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        sorted
    }
    
    /// 获取最佳染色体
    pub fn best_chromosome(&self) -> Vec<f32> {
        let best_idx = self.fitness_cache.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        
        let start = best_idx * self.gene_size;
        self.population[start..start + self.gene_size].to_vec()
    }
    
    /// 获取当前迭代次数
    pub fn generation(&self) -> usize {
        self.generation
    }
}
```

---

## 7. 预置辅助函数

### 7.1 交叉函数

```rust
/// 单点交叉
pub fn single_point_crossover(p1: &[f32], p2: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mid = p1.len() / 2;
    let mut c1 = p1[..mid].to_vec();
    c1.extend_from_slice(&p2[mid..]);
    let mut c2 = p2[..mid].to_vec();
    c2.extend_from_slice(&p1[mid..]);
    (c1, c2)
}

/// 双点交叉
pub fn two_point_crossover(p1: &[f32], p2: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let len = p1.len();
    let p1_idx = rand::random::<usize>() % len;
    let p2_idx = rand::random::<usize>() % len;
    let (start, end) = if p1_idx <= p2_idx {
        (p1_idx, p2_idx)
    } else {
        (p2_idx, p1_idx)
    };
    
    let mut c1 = p1[..start].to_vec();
    c1.extend_from_slice(&p2[start..end]);
    c1.extend_from_slice(&p1[end..]);
    
    let mut c2 = p2[..start].to_vec();
    c2.extend_from_slice(&p1[start..end]);
    c2.extend_from_slice(&p2[end..]);
    
    (c1, c2)
}

/// SBX 交叉（模拟二进制交叉）- 实数优化推荐
pub fn sbx_crossover(eta: f64) -> CrossoverFn {
    Box::new(move |p1, p2| {
        let mut c1 = Vec::with_capacity(p1.len());
        let mut c2 = Vec::with_capacity(p2.len());
        
        for i in 0..p1.len() {
            let x1 = p1[i] as f64;
            let x2 = p2[i] as f64;
            
            let (y1, y2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let u = rand::random::<f64>();
            
            let beta = if u <= 0.5 {
                (2.0 * u).powf(1.0 / (eta + 1.0))
            } else {
                (1.0 / (2.0 * (1.0 - u))).powf(1.0 / (eta + 1.0))
            };
            
            c1.push(0.5 * ((y1 + y2) - beta * (y2 - y1)) as f32);
            c2.push(0.5 * ((y1 + y2) + beta * (y2 - y1)) as f32);
        }
        
        (c1, c2)
    })
}

/// 算术交叉（加权平均）
pub fn arithmetic_crossover(alpha: f32) -> CrossoverFn {
    Box::new(move |p1, p2| {
        let c1: Vec<f32> = p1.iter()
            .zip(p2.iter())
            .map(|(a, b)| alpha * a + (1.0 - alpha) * b)
            .collect();
        let c2: Vec<f32> = p1.iter()
            .zip(p2.iter())
            .map(|(a, b)| (1.0 - alpha) * a + alpha * b)
            .collect();
        (c1, c2)
    })
}
```

### 7.2 选择函数

```rust
/// 锦标赛选择
pub fn tournament_selection(k: usize) -> SelectionFn {
    Box::new(move |fitness, num_to_select| {
        let pop_size = fitness.len();
        let mut selected = Vec::with_capacity(num_to_select);
        
        for _ in 0..num_to_select {
            let contestants: Vec<usize> = (0..pop_size)
                .collect::<Vec<_>>()
                .choose_multiple(&mut rand::rng(), k);
            
            let winner = contestants.iter()
                .max_by(|&&a, &&b| fitness[a].partial_cmp(&fitness[b]).unwrap())
                .unwrap();
            selected.push(*winner);
        }
        
        selected
    })
}

/// 轮盘赌选择（按适应度比例选择）
pub fn roulette_selection() -> SelectionFn {
    Box::new(|fitness, num_to_select| {
        let total: f32 = fitness.iter().sum();
        let mut selected = Vec::with_capacity(num_to_select);
        
        for _ in 0..num_to_select {
            let mut r = rand::random::<f32>() * total;
            for (i, &f) in fitness.iter().enumerate() {
                r -= f;
                if r <= 0.0 {
                    selected.push(i);
                    break;
                }
            }
            if selected.len() == num_to_select - 1 {
                // 最后一个随便选
                let last_idx = (0..fitness.len())
                    .find(|i| !selected.contains(i))
                    .unwrap();
                selected.push(last_idx);
            }
        }
        
        selected
    })
}
```

---

## 8. 使用示例

### 8.1 基本使用

```rust
use genetic::{GeneticAlgorithm, sbx_crossover};

// 定义适应度函数：最大化所有基因之和
let fitness_fn = |genes: &[f32]| {
    genes.iter().sum::<f32>()
};

// 创建遗传算法
let mut ga = GeneticAlgorithm::new(10, fitness_fn)
    .population_size(100)
    .tournament_size(5)
    .mutation_rate(0.05)
    .randomize();

// 迭代1000轮
for _ in 0..1000 {
    ga.step();
}

// 获取最佳结果
let best = ga.best_chromosome();
println!("最佳解: {:?}", best);
```

### 8.2 使用 SBX 交叉（实数优化推荐）

```rust
let mut ga = GeneticAlgorithm::new(10, fitness_fn)
    .crossover(sbx_crossover(15.0));  // eta = 15
```

### 8.3 自定义交叉算法

```rust
let mut ga = GeneticAlgorithm::new(10, fitness_fn)
    .crossover(|p1, p2| {
        // 用户自定义交叉：取平均
        let child1: Vec<f32> = p1.iter().zip(p2.iter())
            .map(|(a, b)| (a + b) / 2.0)
            .collect();
        let child2 = child1.clone();
        (child1, child2)
    });
```

### 8.4 优化神经网络权重

```rust
// 假设网络权重展平为 100 个参数
let net_weights = vec![0.0; 100];

// 适应度函数：负的loss（越大越好）
let fitness_fn = |genes: &[f32]| {
    let loss = compute_network_loss(genes);  // 用户自定义
    -loss
};

let mut ga = GeneticAlgorithm::new(100, fitness_fn)
    .population_size(50)
    .mutation_rate(0.1)
    .randomize();

for _ in 0..500 {
    ga.step();
}

let optimal_weights = ga.best_chromosome();
```

### 8.5 监控迭代过程

```rust
let mut ga = GeneticAlgorithm::new(10, fitness_fn);
let mut history = Vec::new();

for gen in 0..1000 {
    ga.step();
    
    if gen % 100 == 0 {
        let fitness = ga.get_fitness_sorted();
        println!("Generation {}: best fitness = {}", gen, fitness[0]);
        history.push(fitness[0]);
    }
}
```

---

## 9. 完整 API 列表

### 类型别名

| 类型 | 说明 |
|------|------|
| `FitnessFn` | `Fn(&[f32]) -> f32` - 适应度评估函数 |
| `CrossoverFn` | `Fn(&[f32], &[f32]) -> (Vec<f32>, Vec<f32>)` - 交叉函数 |
| `SelectionFn` | `Fn(&[f32], usize) -> Vec<usize>` - 选择函数 |

### 预置函数

| 函数 | 说明 |
|------|------|
| `single_point_crossover` | 单点交叉 |
| `two_point_crossover` | 双点交叉 |
| `sbx_crossover(eta)` | SBX 交叉 |
| `arithmetic_crossover(alpha)` | 算术交叉 |
| `tournament_selection(k)` | 锦标赛选择 |
| `roulette_selection()` | 轮盘赌选择 |

### GeneticAlgorithm 方法

| 方法 | 说明 |
|------|------|
| `new(gene_size, fitness_fn)` | 创建实例 |
| `randomize()` | 随机初始化种群 |
| `population_size(size)` | 设置种群大小 |
| `crossover(fn)` | 设置交叉函数 |
| `selection(fn)` | 设置选择函数 |
| `tournament_size(k)` | 设置锦标赛大小 |
| `mutation_rate(rate)` | 设置变异概率 |
| `elite_count(count)` | 设置精英保留数量 |
| `step()` | 执行一轮迭代 |
| `population_size()` | 获取种群大小 |
| `gene_size()` | 获取基因长度 |
| `fitness_cache()` | 获取适应度缓存 |
| `get_fitness_sorted()` | 获取排序后的适应度 |
| `best_chromosome()` | 获取最佳染色体 |
| `generation()` | 获取当前迭代次数 |

---

## 10. 配置默认值

| 参数 | 默认值 | 说明 |
|------|--------|------|
| 种群大小 | 100 | 染色体数量 |
| 变异概率 | 0.1 | 每个基因变异的概率 |
| 锦标赛大小 | 5 | 选择时的候选数量 |
| 精英数量 | 2 | 保留的最优个体数量 |
| 交叉算法 | SinglePoint | 默认使用单点交叉 |

---

## 11. 设计决策

### 11.1 为什么内部使用 Vec<f32> 而不是 Tensor？

- **避免数据拷贝**：直接操作扁平化数据，切片引用零拷贝
- **不依赖 tensor 库**：可独立使用，轻量化设计
- **并行友好**：通过 optim::GlobalPool 进行并行调度

### 11.2 为什么用函数式接口？

用户只需要定义"单个小操作"，框架自动处理并行调度。用户不需要了解并行框架的具体实现。

### 11.3 并行对用户透明

```
用户代码: 
    |genes| { sum(genes) }
    
框架执行:
    线程1: evaluate genes[0..25]  
    线程2: evaluate genes[25..50]
    ...
    拼接结果
```

### 11.4 适应度缓存的作用

- 存储最近一次迭代的适应度结果
- 方便导出用于绘图
- 避免重复计算

### 11.5 精英保留策略

保留上一代的最优个体，直接替换新种群中的最差个体，避免优秀基因丢失。

---

## 12. 与 optim crate 集成

使用全局并行池进行任务调度：

```rust
// 在 Cargo.toml 中添加依赖
[dependencies]
optim = { path = "../optim" }

// 使用 GlobalPool 进行并行调度
fn evaluate_all_parallel(&self) -> Vec<f32> {
    // 自动获取最优线程数和分块数
    let num_chunks = optim::GlobalPool::optimal_num_chunks(self.pop_size);
    let chunk_size = optim::GlobalPool::parallel_chunk_size(self.pop_size);
    
    // 使用 install 在全局线程池中执行任务
    optim::GlobalPool::install(|| {
        // 并行计算逻辑
    })
}
```

---

## 13. 待扩展功能

- [ ] 自适应变异率（根据迭代进度调整）
- [ ] 早停机制（当适应度不再提升时）
- [ ] 并行变异操作
- [ ] 约束处理（边界约束、惩罚函数）