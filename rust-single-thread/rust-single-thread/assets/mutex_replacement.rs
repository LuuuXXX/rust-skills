//! Mutex 替换实现
//!
//! 在单线程环境中，Mutex 可以被替换为普通变量，以简化代码并提高性能。

use std::sync::Mutex;

/// 单线程版本的共享数据结构
pub struct SingleThreadData {
    data: i32,
}

impl SingleThreadData {
    /// 创建新的单线程数据结构
    pub fn new() -> Self {
        Self { data: 0 }
    }

    /// 获取数据值
    pub fn get(&self) -> i32 {
        self.data
    }

    /// 设置数据值
    pub fn set(&mut self, value: i32) {
        self.data = value;
    }

    /// 增加数据值
    pub fn increment(&mut self) {
        self.data += 1;
    }

    /// 减少数据值
    pub fn decrement(&mut self) {
        self.data -= 1;
    }
}

/// 多线程到单线程的转换助手
pub struct MutexConverter<T> {
    inner: T,
}

impl<T> MutexConverter<T> {
    /// 创建新的转换器
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// 获取内部值（在单线程环境中安全）
    pub fn get(&self) -> &T {
        &self.inner
    }

    /// 获取可变引用（在单线程环境中安全）
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

/// 将多线程 Mutex 转换为单线程版本
pub fn convert_mutex_to_single_thread<T>(mutex: &Mutex<T>) -> MutexConverter<T>
where
    T: Clone,
{
    MutexConverter::new(mutex.lock().unwrap().clone())
}

/// 将多线程 Arc<Mutex<T>> 转换为单线程版本
pub fn convert_arc_mutex_to_single_thread<T>(arc_mutex: &std::sync::Arc<Mutex<T>>) -> MutexConverter<T>
where
    T: Clone,
{
    MutexConverter::new(arc_mutex.lock().unwrap().clone())
}

/// 示例：使用单线程版本的共享数据
pub fn example_usage() {
    // 多线程版本
    let shared = std::sync::Mutex::new(0);
    {
        let mut data = shared.lock().unwrap();
        *data += 1;
    }

    // 单线程版本
    let mut single_thread_data = SingleThreadData::new();
    single_thread_data.increment();

    // 转换助手
    let multi_thread_data = std::sync::Mutex::new(42);
    let converter = convert_mutex_to_single_thread(&multi_thread_data);
    let value = converter.get();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_thread_data() {
        let mut data = SingleThreadData::new();
        assert_eq!(data.get(), 0);

        data.set(42);
        assert_eq!(data.get(), 42);

        data.increment();
        assert_eq!(data.get(), 43);

        data.decrement();
        assert_eq!(data.get(), 42);
    }

    #[test]
    fn test_mutex_converter() {
        let original = std::sync::Mutex::new(100);
        let converter = convert_mutex_to_single_thread(&original);
        let value = converter.get();
        assert_eq!(*value, 100);
    }
}