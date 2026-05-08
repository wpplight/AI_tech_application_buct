use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub const PARALLEL_THRESHOLD: usize = 100_000;

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub num_threads: Option<usize>,
    pub thread_name_prefix: Option<String>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            num_threads: None,
            thread_name_prefix: None,
        }
    }
}

impl PoolConfig {
    pub fn with_num_threads(mut self, n: usize) -> Self {
        self.num_threads = Some(n);
        self
    }

    pub fn with_thread_name_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.thread_name_prefix = Some(prefix.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PoolInfo {
    pub num_threads: usize,
}

pub struct GlobalPool;

static GLOBAL_POOL: once_cell::sync::Lazy<ThreadPool> =
    once_cell::sync::Lazy::new(|| {
        ThreadPoolBuilder::new()
            .num_threads(global_config().num_threads.unwrap_or_else(num_cpus::get))
            .thread_name(|i| {
                global_config()
                    .thread_name_prefix
                    .clone()
                    .map(|p| format!("{}-{}", p, i))
                    .unwrap_or_else(|| format!("rayon-{}", i))
            })
            .build()
            .unwrap()
    });

static CONFIG: once_cell::sync::Lazy<PoolConfig> =
    once_cell::sync::Lazy::new(|| PoolConfig::default());

pub fn global_config() -> &'static PoolConfig {
    &CONFIG
}

impl GlobalPool {
    pub fn install<OP, R>(op: OP) -> R
    where
        OP: FnOnce() -> R + Send,
        R: Send,
    {
        GLOBAL_POOL.install(op)
    }

    pub fn pool_info() -> PoolInfo {
        PoolInfo {
            num_threads: GLOBAL_POOL.current_num_threads(),
        }
    }

    pub fn current_thread_index() -> Option<usize> {
        GLOBAL_POOL.current_thread_index()
    }

    pub fn num_threads() -> usize {
        GLOBAL_POOL.current_num_threads()
    }

    pub fn available_parallelism() -> usize {
        rayon::current_num_threads()
    }

    pub fn should_parallelize(total_items: usize) -> bool {
        total_items >= PARALLEL_THRESHOLD
    }

    pub fn parallel_chunk_size(total_items: usize) -> usize {
        let num_threads = Self::num_threads();
        if total_items <= num_threads {
            return 1;
        }
        (total_items + num_threads - 1) / num_threads
    }

    pub fn optimal_num_chunks(total_items: usize) -> usize {
        let num_threads = Self::num_threads();
        num_threads.min(total_items)
    }
}

pub fn par_map<T, R, F>(data: &[T], f: F) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(&T) -> R + Send + Sync + Clone + 'static,
{
    data.par_iter().map(f).collect()
}

pub fn par_sum<T>(data: &[T]) -> T
where
    T: Send + Sync + std::iter::Sum + Clone + 'static,
{
    data.par_iter().cloned().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_pool_info() {
        let info = GlobalPool::pool_info();
        assert!(info.num_threads > 0);
    }

    #[test]
    fn test_install() {
        let result = GlobalPool::install(|| 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_par_map() {
        let data = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = par_map(&data, |&x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_par_sum() {
        let data = vec![1, 2, 3, 4, 5];
        let result: i32 = par_sum(&data);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_parallel_chunk_size() {
        let threads = GlobalPool::num_threads();
        assert!(GlobalPool::parallel_chunk_size(threads - 1) >= 1);
        assert!(GlobalPool::parallel_chunk_size(threads * 2) >= 2);
    }

    #[test]
    fn test_optimal_num_chunks() {
        let threads = GlobalPool::num_threads();
        assert_eq!(GlobalPool::optimal_num_chunks(2), 2);
        assert_eq!(GlobalPool::optimal_num_chunks(threads), threads);
        assert!(GlobalPool::optimal_num_chunks(threads + 10) <= threads);
    }
}
