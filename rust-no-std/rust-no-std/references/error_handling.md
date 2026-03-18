# 错误处理模式

## 概述

在 no-std 环境中，错误处理需要特别注意，因为不能使用标准库的 `std::error::Error` trait。

## 自定义错误 trait

```rust
use core::fmt;

/// No-std 兼容的错误 trait
pub trait NoStdError: fmt::Debug + fmt::Display {
    fn source(&self) -> Option<&(dyn NoStdError + 'static)> {
        None
    }
}

impl<E> NoStdError for E
where
    E: fmt::Debug + fmt::Display + 'static,
{
    fn source(&self) -> Option<&(dyn NoStdError + 'static)> {
        None
    }
}
```

## 错误类型定义

### 1. 简单错误类型

```rust
#[derive(Debug)]
pub enum MyError {
    IoError,
    ParseError,
    ConfigurationError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError => write!(f, "I/O error"),
            MyError::ParseError => write!(f, "Parse error"),
            MyError::ConfigurationError => write!(f, "Configuration error"),
        }
    }
}

impl NoStdError for MyError {}
```

### 2. 带有源错误的错误类型

```rust
#[derive(Debug)]
pub struct ChainError {
    kind: ErrorKind,
    source: Option<Box<dyn NoStdError>>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Io,
    Parse,
    Config,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "I/O error"),
            ErrorKind::Parse => write!(f, "Parse error"),
            ErrorKind::Config => write!(f, "Configuration error"),
        }
    }
}

impl NoStdError for ChainError {
    fn source(&self) -> Option<&(dyn NoStdError + 'static)> {
        self.source.as_deref()
    }
}
```

## Result 类型

```rust
pub type Result<T> = core::result::Result<T, Box<dyn NoStdError>>;
```

## 错误传播

### 1. 简单传播

```rust
fn read_file(path: &str) -> Result<Vec<u8>> {
    let data = match read_raw_file(path) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(MyError::IoError)),
    };
    Ok(data)
}
```

### 2. 链式错误

```rust
fn parse_config(data: &[u8]) -> Result<Config> {
    let config = match parse_raw_config(data) {
        Ok(config) => config,
        Err(e) => return Err(Box::new(ChainError {
            kind: ErrorKind::Parse,
            source: Some(e),
        })),
    };
    Ok(config)
}
```

## 错误处理策略

### 1. 恢复策略

```rust
fn handle_error(result: Result<()>) {
    match result {
        Ok(()) => println!("操作成功"),
        Err(e) => {
            println!("发生错误: {}", e);
            // 尝试恢复或优雅降级
            recover_from_error(e);
        }
    }
}
```

### 2. 不可恢复错误

```rust
fn critical_operation() -> Result<()> {
    let result = perform_critical_task();
    if result.is_err() {
        // 在 no-std 环境中，可能需要重启或panic
        loop {} // 无限循环表示严重错误
    }
    result
}
```

## Panic 处理

在 no-std 环境中，panic 处理需要特别注意。

```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // 记录panic信息
    if let Some(location) = info.location() {
        println!("panic occurred in file '{}' at line {}",
                 location.file(), location.line());
    }

    // 在嵌入式系统中，可能需要重启
    loop {}
}
```

## 最佳实践

1. **明确的错误类型**：定义清晰的错误类型层次结构
2. **丰富的错误信息**：提供足够的上下文信息
3. **错误链**：支持错误源链接
4. **一致的错误处理**：在整个项目中使用一致的错误处理模式
5. **资源清理**：确保在错误发生时正确清理资源
6. **测试**：全面测试错误处理路径

## 常见错误场景

### 1. I/O 错误

```rust
pub enum IoError {
    ReadError,
    WriteError,
    DeviceNotFound,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::ReadError => write!(f, "读取错误"),
            IoError::WriteError => write!(f, "写入错误"),
            IoError::DeviceNotFound => write!(f, "设备未找到"),
        }
    }
}

impl NoStdError for IoError {}
```

### 2. 解析错误

```rust
pub enum ParseError {
    InvalidFormat,
    MissingField,
    OutOfRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "无效格式"),
            ParseError::MissingField => write!(f, "缺少字段"),
            ParseError::OutOfRange => write!(f, "超出范围"),
        }
    }
}

impl NoStdError for ParseError {}
```

## 推荐库

- `thiserror` - 错误定义宏（需要适配 no-std）
- `anyhow` - 上下文丰富的错误（需要适配 no-std）
- `failure` - 错误处理库（需要适配 no-std）

## 性能考虑

- 错误创建和传播应尽可能高效
- 避免在错误路径中进行不必要的分配
- 考虑错误类型的内存占用
- 在嵌入式环境中，错误处理应可预测