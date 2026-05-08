#[cfg(test)]
mod tests {
    use super::super::*;
    use tokio::sync::mpsc;

    /// execute_sync 内部使用 block_on，不能在 #[tokio::test] 的 runtime 内调用
    #[test]
    fn test_execute_sync() {
        let result = GlobalAsyncPool::execute_sync(|| 42);
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_spawn_await_result() {
        let handle = GlobalAsyncPool::spawn(async { 42 });
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_spawn_blocking() {
        let handle = GlobalAsyncPool::spawn_blocking(|| 24);
        let result = handle.await.unwrap();
        assert_eq!(result, 24);
    }

    #[tokio::test]
    async fn test_pool_info() {
        let info = GlobalAsyncPool::pool_info();
        assert!(info.worker_threads > 0);
        assert_eq!(info.max_blocking_threads, 512);
    }

    #[tokio::test]
    async fn test_join_all() {
        let handles: Vec<_> = (0..5)
            .map(|i| GlobalAsyncPool::spawn(async move { i * 2 }))
            .collect();

        let results = GlobalAsyncPool::join_all(handles).await;
        assert_eq!(results, vec![0, 2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn test_nested_spawn_no_deadlock() {
        // 创建一个任务依赖链，确保不会死锁
        let (tx1, mut rx1) = mpsc::channel(10);
        let (tx2, mut rx2) = mpsc::channel(10);
        let (tx3, mut rx3) = mpsc::channel(10);
        
        // 第一个任务：直接发送结果
        let h1 = GlobalAsyncPool::spawn(async move {
            tx1.send(0).await.unwrap();
        });
        
        // 第二个任务：等待第一个任务，然后发送结果
        let h2 = GlobalAsyncPool::spawn(async move {
            rx1.recv().await.unwrap();
            tx2.send(2).await.unwrap();
        });
        
        // 第三个任务：等待第二个任务，然后发送结果
        let h3 = GlobalAsyncPool::spawn(async move {
            rx2.recv().await.unwrap();
            tx3.send(4).await.unwrap();
        });
        
        // 等待所有任务完成
        let results = GlobalAsyncPool::join_all(vec![h1, h2, h3]).await;
        
        // 收集最终结果
        let mut values = Vec::new();
        if let Some(v) = rx3.recv().await {
            values.push(v);
        }
        
        // 验证没有死锁且任务按顺序执行
        assert_eq!(results.len(), 3);
        assert_eq!(values, vec![4]);
    }
}
