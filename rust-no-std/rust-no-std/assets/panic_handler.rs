#![no_std]
#![feature(panic_info_message)]

use core::fmt;

/// 自定义 panic 处理程序
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // 记录 panic 信息
    if let Some(location) = info.location() {
        let _ = writeln!(Serial, "panic occurred in file '{}' at line {}",
                       location.file(), location.line());
    }

    if let Some(message) = info.message() {
        let _ = writeln!(Serial, "message: {}", message);
    }

    // 在嵌入式系统中，可能需要重启
    loop {}
}

/// 串口输出实现
struct Serial;

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 实现串口输出
        // 这里假设有一个全局的串口实例
        unsafe {
            SERIAL.write_str(s);
        }
        Ok(())
    }
}

static mut SERIAL: Serial = Serial;

/// 初始化 panic 处理程序
pub fn init_panic_handler() {
    // 初始化串口或其他输出设备
    // 例如：初始化 UART
}

/// 安全的 panic 处理
pub fn safe_panic(message: &str) -> ! {
    panic!(message);
}

/// 调试 panic 处理
#[cfg(debug_assertions)]
pub fn debug_panic(message: &str) -> ! {
    panic!(message);
}

/// 发布 panic 处理
#[cfg(not(debug_assertions))]
pub fn debug_panic(message: &str) -> ! {
    loop {} // 在发布版本中静默失败
}

/// 自定义 panic 信息
pub struct PanicInfo {
    message: &'static str,
    location: Option<(&'static str, u32)>,
}

impl PanicInfo {
    /// 创建新的 panic 信息
    pub fn new(message: &'static str, file: &'static str, line: u32) -> Self {
        Self {
            message,
            location: Some((file, line)),
        }
    }

    /// 获取消息
    pub fn message(&self) -> &str {
        self.message
    }

    /// 获取位置信息
    pub fn location(&self) -> Option<(&str, u32)> {
        self.location.map(|(file, line)| (file, line))
    }
}

/// 自定义 panic
#[macro_export]
macro_rules! custom_panic {
    ($message:expr) => {
        {
            let info = PanicInfo::new($message, file!(), line!());
            panic::panic_info_hook(&info);
            loop {}
        }
    };
}

/// panic 钩子
pub fn panic_info_hook(info: &PanicInfo) {
    // 实现自定义 panic 钩子
    let _ = writeln!(Serial, "Custom panic: {}", info.message());
    if let Some((file, line)) = info.location() {
        let _ = writeln!(Serial, "at {}:{}:", file, line);
    }
}

/// 错误恢复
pub fn recover_from_panic() {
    // 实现错误恢复逻辑
    // 例如：重启系统或重置状态
}