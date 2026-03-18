# 单线程错误处理模式

## 概述

在将 Rust 多线程代码转换为单线程代码时，错误处理模式也需要相应调整。本指南提供了常见的单线程错误处理模式。

## 并发错误传播转换为顺序错误处理

### 1. 基本转换

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            tx.send(Err(e)).unwrap();
        } else {
            tx.send(Ok(())).unwrap();
        }
    });

    match rx.recv()? {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    do_work()?; // 直接调用，简化错误处理
    Ok(())
}
```

### 2. 复杂错误传播

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_complex_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = do_first_task();
        if let Err(e) = result {
            tx.send(Err(e)).unwrap();
        } else {
            tx.send(Ok(result.unwrap())).unwrap();
        }
    });

    let intermediate = rx.recv()??;

    let final_result = do_second_task(intermediate);
    if let Err(e) = final_result {
        tx.send(Err(e)).unwrap();
    } else {
        tx.send(Ok(final_result.unwrap())).unwrap();
    }

    match rx.recv()? {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

// 单线程版本
fn single_thread_complex_example() -> Result<(), Box<dyn std::error::Error>> {
    let intermediate = do_first_task()?;
    let final_result = do_second_task(intermediate)?;
    Ok(())
}
```

## 全局错误处理

### 1. 简单全局错误处理

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            tx.send(e).unwrap();
        }
    });

    if let Ok(e) = rx.recv() {
        handle_error(e);
    }
}

// 单线程版本
fn single_thread_example() {
    if let Err(e) = do_work() {
        handle_error(e);
    }
}
```

### 2. 带上下文的全局错误处理

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

struct Context {
    // 上下文数据
}

fn multi_thread_example(context: &Context) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work_with_context(context) {
            tx.send(e).unwrap();
        }
    });

    if let Ok(e) = rx.recv() {
        handle_error_with_context(context, e);
    }
}

// 单线程版本
fn single_thread_example(context: &Context) {
    if let Err(e) = do_work_with_context(context) {
        handle_error_with_context(context, e);
    }
}
```

## 错误恢复策略

### 1. 简单恢复

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = do_work();
        tx.send(result).unwrap();
    });

    match rx.recv()? {
        Ok(()) => Ok(()),
        Err(e) => {
            recover_from_error(e)?;
            Ok(())
        }
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    match do_work() {
        Ok(()) => Ok(()),
        Err(e) => {
            recover_from_error(e)?;
            Ok(())
        }
    }
}
```

### 2. 重试机制

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let max_retries = 3;

    thread::spawn(move || {
        let mut retries = 0;
        loop {
            if retries >= max_retries {
                tx.send(Err("Max retries exceeded".into())).unwrap();
                break;
            }

            match do_work() {
                Ok(()) => {
                    tx.send(Ok(())).unwrap();
                    break;
                }
                Err(e) => {
                    retries += 1;
                    if retries < max_retries {
                        // 等待后重试
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
        }
    });

    match rx.recv()? {
        Ok(()) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let max_retries = 3;
    let mut retries = 0;

    loop {
        match do_work() {
            Ok(()) => return Ok(()),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }
                // 等待后重试
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
```

## 资源清理

### 1. 简单资源清理

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let resource = create_resource()?;

    thread::spawn(move || {
        if let Err(e) = use_resource(&resource) {
            tx.send(Err(e)).unwrap();
        } else {
            tx.send(Ok(())).unwrap();
        }
    });

    match rx.recv()? {
        Ok(()) => {
            cleanup_resource(resource);
            Ok(())
        }
        Err(e) => {
            cleanup_resource(resource);
            Err(e)
        }
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let resource = create_resource()?;
    use_resource(&resource)?;
    cleanup_resource(resource);
    Ok(())
}
```

### 2. 带错误处理的资源清理

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let resource = create_resource()?;

    thread::spawn(move || {
        if let Err(e) = use_resource(&resource) {
            tx.send(Err(e)).unwrap();
        } else {
            tx.send(Ok(())).unwrap();
        }
    });

    match rx.recv()? {
        Ok(()) => {
            if let Err(e) = cleanup_resource(resource) {
                return Err(e);
            }
            Ok(())
        }
        Err(e) => {
            if let Err(cleanup_e) = cleanup_resource(resource) {
                // 记录清理错误，但返回原始错误
                eprintln!("Cleanup error: {}", cleanup_e);
            }
            Err(e)
        }
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let resource = create_resource()?;
    use_resource(&resource)?;
    cleanup_resource(resource)?;
    Ok(())
}
```

## 错误日志记录

### 1. 基本日志记录

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            tx.send(e).unwrap();
        }
    });

    if let Ok(e) = rx.recv() {
        log_error(e);
    }
}

// 单线程版本
fn single_thread_example() {
    if let Err(e) = do_work() {
        log_error(e);
    }
}
```

### 2. 结构化日志记录

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

#[derive(Debug)]
struct ErrorInfo {
    error: Box<dyn std::error::Error>,
    context: String,
    timestamp: std::time::SystemTime,
}

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            let error_info = ErrorInfo {
                error: e,
                context: "multi_thread_example".to_string(),
                timestamp: std::time::SystemTime::now(),
            };
            tx.send(error_info).unwrap();
        }
    });

    if let Ok(error_info) = rx.recv() {
        log_structured_error(&error_info);
    }
}

// 单线程版本
fn single_thread_example() {
    if let Err(e) = do_work() {
        let error_info = ErrorInfo {
            error: e,
            context: "single_thread_example".to_string(),
            timestamp: std::time::SystemTime::now(),
        };
        log_structured_error(&error_info);
    }
}
```

## 错误类型转换

### 1. 自定义错误类型

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Network(reqwest::Error),
    Database(sqlx::Error),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::Network(error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::Database(error)
    }
}

fn multi_thread_example() -> Result<(), AppError> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            tx.send(AppError::from(e)).unwrap();
        }
    });

    match rx.recv()? {
        AppError::Io(e) => Err(AppError::Io(e)),
        AppError::Network(e) => Err(AppError::Network(e)),
        AppError::Database(e) => Err(AppError::Database(e)),
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), AppError> {
    match do_work() {
        Ok(()) => Ok(()),
        Err(e) => Err(AppError::from(e)),
    }
}
```

### 2. 错误链

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

#[derive(Debug)]
struct ChainError {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error>>,
}

#[derive(Debug)]
enum ErrorKind {
    Io,
    Network,
    Database,
}

impl std::error::Error for ChainError {}

impl std::fmt::Display for ChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "I/O error"),
            ErrorKind::Network => write!(f, "Network error"),
            ErrorKind::Database => write!(f, "Database error"),
        }
    }
}

fn multi_thread_example() -> Result<(), ChainError> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            let chain_error = ChainError {
                kind: ErrorKind::Io,
                source: Some(e),
            };
            tx.send(chain_error).unwrap();
        }
    });

    match rx.recv()? {
        chain_error => Err(chain_error),
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), ChainError> {
    match do_work() {
        Ok(()) => Ok(()),
        Err(e) => {
            let chain_error = ChainError {
                kind: ErrorKind::Io,
                source: Some(e),
            };
            Err(chain_error)
        }
    }
}
```

## 错误边界

### 1. 局部错误处理

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            tx.send(e).unwrap();
        }
    });

    match rx.recv()? {
        e => Err(e),
    }
}

// 单线程版本
fn single_thread_example() -> Result<(), Box<dyn std::error::Error>> {
    match do_work() {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}
```

### 2. 全局错误处理

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = do_work() {
            tx.send(e).unwrap();
        }
    });

    if let Ok(e) = rx.recv() {
        handle_global_error(e);
    }
}

// 单线程版本
fn single_thread_example() {
    if let Err(e) = do_work() {
        handle_global_error(e);
    }
}
```

## 最佳实践

1. **直接返回**：将并发错误传播转换为直接返回
2. **全局处理**：使用全局错误处理替代并发错误传播
3. **资源清理**：确保资源在错误发生时正确清理
4. **日志记录**：记录错误信息以便调试
5. **类型转换**：统一错误类型以便处理

## 常见陷阱

### 1. 忘记错误处理
```rust
// 错误：忽略错误处理
fn ignore_errors() {
    do_work().unwrap(); // 可能panic
}
```

### 2. 资源泄漏
```rust
// 错误：资源未正确清理
fn resource_leak() {
    let resource = create_resource();
    use_resource(&resource);
    // 忘记 cleanup_resource
}
```

### 3. 错误信息丢失
```rust
// 错误：错误信息不完整
fn lost_error_info() {
    if let Err(_) = do_work() {
        // 丢失错误信息
    }
}
```

## 推荐库

- `anyhow` - 灵活的错误处理
- `thiserror` - 自定义错误类型
- `log` - 错误日志记录
- `tracing` - 结构化日志和跟踪

## 性能考虑

- 单线程错误处理通常比多线程错误处理更简单
- 减少错误传播的开销
- 便于调试和错误追踪
- 需要确保错误处理不会成为性能瓶颈