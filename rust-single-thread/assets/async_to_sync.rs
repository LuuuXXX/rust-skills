//! 异步到同步转换实现
//!
//! 在单线程环境中，异步代码可以被转换为同步代码，以简化代码并提高性能。

use std::future::Future;
use tokio::runtime::Runtime;

/// 单线程版本的异步运行时
pub struct SingleThreadRuntime;

impl SingleThreadRuntime {
    /// 创建新的单线程运行时
    pub fn new() -> Self {
        Self
    }

    /// 运行异步任务（同步版本）
    pub fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T>,
    {
        // 在单线程环境中，直接运行 future
        future
    }
}

/// 将异步函数转换为同步函数
pub fn convert_async_to_sync<F, T>(async_func: F) -> impl Fn() -> T
where
    F: Fn() -> Box<dyn Future<Output = T>>,
{
    move || {
        let runtime = SingleThreadRuntime::new();
        runtime.block_on(async_func())
    }
}

/// 将异步操作转换为同步操作
pub async fn async_operation() -> String {
    // 模拟异步操作
    "async result".to_string()
}

/// 同步版本的异步操作
pub fn sync_operation() -> String {
    // 直接返回结果
    "sync result".to_string()
}

/// 示例：使用单线程版本的异步运行时
pub fn example_usage() {
    // 多线程异步版本
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result = async_operation().await;
        println!("Async result: {}", result);
    });

    // 单线程同步版本
    let result = sync_operation();
    println!("Sync result: {}", result);

    // 转换助手
    let async_func = || Box::new(async_operation()) as Box<dyn Future<Output = String>>;
    let sync_func = convert_async_to_sync(async_func);

    let result = sync_func();
    println!("Converted result: {}", result);
}

/// 异步链式调用转换为同步链式调用
pub async fn async_chain() -> String {
    let data = async_fetch_data().await;
    let processed = async_process_data(data).await;
    processed
}

pub fn sync_chain() -> String {
    let data = sync_fetch_data();
    let processed = sync_process_data(data);
    processed
}

/// 异步任务调度转换为同步任务调度
pub async fn async_task_scheduler() -> Vec<String> {
    let tasks = vec![
        async_task1(),
        async_task2(),
        async_task3(),
    ];

    let results = futures::future::join_all(tasks).await;
    results
}

pub fn sync_task_scheduler() -> Vec<String> {
    let tasks = vec![
        sync_task1(),
        sync_task2(),
        sync_task3(),
    ];

    tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_thread_runtime() {
        let runtime = SingleThreadRuntime::new();
        let result = runtime.block_on(async { "test".to_string() });
        assert_eq!(result, "test");
    }

    #[test]
    fn test_async_to_sync_conversion() {
        let async_func = || Box::new(async_operation()) as Box<dyn Future<Output = String>>;
        let sync_func = convert_async_to_sync(async_func);

        let result = sync_func();
        assert_eq!(result, "async result");
    }

    #[test]
    fn test_sync_operation() {
        let result = sync_operation();
        assert_eq!(result, "sync result");
    }
}