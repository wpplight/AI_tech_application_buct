pub mod pool;
pub mod task;

pub use task::{TaskError, TaskResult};

pub use pool::{global_config, PoolConfig, PoolInfo};

pub use pool::GlobalPool;
