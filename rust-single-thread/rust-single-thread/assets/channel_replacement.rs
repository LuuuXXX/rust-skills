//! Channel 替换实现
//!
//! 在单线程环境中，Channel 可以被替换为直接函数调用，以简化代码并提高性能。

use std::sync::mpsc;

/// 单线程版本的通信通道
pub struct SingleThreadChannel<T> {
    data: Option<T>,
}

impl<T> SingleThreadChannel<T> {
    /// 创建新的单线程通道
    pub fn new() -> Self {
        Self { data: None }
    }

    /// 发送数据（直接设置）
    pub fn send(&mut self, data: T) {
        self.data = Some(data);
    }

    /// 接收数据（直接获取）
    pub fn recv(&mut self) -> Option<T> {
        self.data.take()
    }

    /// 检查是否有数据
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

/// 多线程到单线程的转换助手
pub struct ChannelConverter<T> {
    inner: SingleThreadChannel<T>,
}

impl<T> ChannelConverter<T> {
    /// 创建新的转换器
    pub fn new() -> Self {
        Self {
            inner: SingleThreadChannel::new(),
        }
    }

    /// 发送数据
    pub fn send(&mut self, data: T) {
        self.inner.send(data);
    }

    /// 接收数据
    pub fn recv(&mut self) -> Option<T> {
        self.inner.recv()
    }

    /// 检查是否有数据
    pub fn has_data(&self) -> bool {
        self.inner.has_data()
    }
}

/// 将多线程 channel 转换为单线程版本
pub fn convert_channel_to_single_thread<T>(tx: mpsc::Sender<T>, rx: mpsc::Receiver<T>) -> ChannelConverter<T> {
    let mut converter = ChannelConverter::new();

    // 发送端直接调用
    converter.send(rx.recv().unwrap());

    converter
}

/// 将多线程 Arc<Channel> 转换为单线程版本
pub fn convert_arc_channel_to_single_thread<T>(
    tx: std::sync::Arc<mpsc::Sender<T>>,
    rx: std::sync::Arc<mpsc::Receiver<T>>,
) -> ChannelConverter<T> {
    let mut converter = ChannelConverter::new();

    // 发送端直接调用
    converter.send(rx.recv().unwrap());

    converter
}

/// 示例：使用单线程版本的通道
pub fn example_usage() {
    // 多线程版本
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Received: {}", msg);

    // 单线程版本
    let mut channel = SingleThreadChannel::new();
    channel.send("hello");
    let msg = channel.recv().unwrap();
    println!("Received: {}", msg);

    // 转换助手
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    let converter = convert_channel_to_single_thread(tx, rx);
    let value = converter.recv().unwrap();
    println!("Converted value: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_thread_channel() {
        let mut channel = SingleThreadChannel::new();
        assert!(!channel.has_data());

        channel.send(42);
        assert!(channel.has_data());
        assert_eq!(channel.recv(), Some(42));
        assert!(!channel.has_data());
    }

    #[test]
    fn test_channel_converter() {
        let mut converter = ChannelConverter::new();
        assert!(!converter.has_data());

        converter.send("test");
        assert!(converter.has_data());
        assert_eq!(converter.recv(), Some("test"));
        assert!(!converter.has_data());
    }
}