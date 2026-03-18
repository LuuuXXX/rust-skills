---
name: rust-single-thread
description: 将现有的 Rust 多线程代码转换为单线程代码的专业技能。当需要在单线程环境中运行 Rust 代码，或需要简化并发逻辑时，应使用此技能。适用于需要将多线程依赖替换为单线程替代方案的场景。
license: MIT
---

# Rust 单线程转换指南

## 概述

此技能提供将 Rust 多线程代码转换为单线程代码的完整指南，包含业界最佳实践和可复用的代码模板。单线程环境适用于嵌入式系统、简单应用程序或需要简化并发逻辑的场景，在这些环境中多线程可能带来不必要的复杂性或资源开销。


## 核心能力

此技能提供以下核心能力：

### 1. 并发模型转换
- std::thread → 单线程事件循环
- async/await → 同步调用链
- 并发数据结构 → 顺序数据结构

### 2. 同步原语替换
- Mutex、RwLock → 无锁或单线程安全
- Atomic → 普通变量
- Channel → 直接函数调用

### 3. 任务调度简化
- 线程池 → 单任务队列
- 并发任务 → 顺序执行
- 异步任务 → 同步等待

### 4. 错误处理优化
- 线程局部错误 → 全局错误处理
- 并发错误传播 → 顺序错误处理
- 资源清理简化

## 工作流程决策树

```
开始转换 → [项目是否有多线程代码？]
           ├─ 是 → 分析多线程依赖 → [依赖是否可替换？]
           │        ├─ 是 → 选择单线程替代方案 → 实施转换
           │        └─ 否 → 考虑重新设计架构
           └─ 否 → 跳过转换步骤
```

## 第一步：分析现有代码

在开始转换之前，必须分析现有代码的多线程依赖情况。

### 识别多线程依赖

使用以下方法识别多线程依赖：

1. **代码审查**：查找 `use std::thread::*;`、`use std::sync::*;` 语句
2. **构建分析**：运行 `cargo check` 查看并发相关错误
3. **性能分析**：识别真正的并发需求 vs. 不必要的并发

### 常见多线程依赖类别

| 多线程模块 | 单线程替代方案 | 适用场景 |
|-----------|---------------|---------|
| std::thread | 单线程事件循环 | 简单并发需求 |
| std::sync::Mutex | 无锁或单线程安全 | 线程安全访问 |
| std::sync::Arc | 普通引用 | 共享所有权 |
| std::sync::mpsc | 直接函数调用 | 通信需求 |
| std::sync::Barrier | 状态机 | 同步点 |
| std::sync::Once | 初始化检查 | 单次初始化 |
| async/await | 同步调用链 | 异步操作 |

## 第二步：选择合适的替代方案

根据项目需求选择合适的单线程替代方案。

### 并发模型转换

#### 1. 线程转换为事件循环

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("{}", msg);
}

// 单线程版本
fn single_thread_example() {
    let msg = "hello"; // 直接创建数据
    println!("{}", msg);
}
```

#### 2. Mutex 转换为单线程安全

```rust
// 多线程版本
use std::sync::Mutex;

struct SharedData {
    data: i32,
}

fn multi_thread_example() {
    let shared = Mutex::new(SharedData { data: 0 });

    // 在单线程环境中，可以直接访问
    let mut data = shared.lock().unwrap();
    data.data += 1;
}

// 单线程版本
struct SharedData {
    data: i32,
}

fn single_thread_example() {
    let mut data = SharedData { data: 0 };
    data.data += 1; // 直接访问，无需锁
}
```

### 同步原语替换

#### 1. Channel 转换为直接调用

```rust
// 多线程版本
use std::sync::mpsc;

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(calculate_result()).unwrap();
    });

    let result = rx.recv().unwrap();
    process_result(result);
}

// 单线程版本
fn single_thread_example() {
    let result = calculate_result(); // 直接调用
    process_result(result); // 直接处理
}
```

#### 2. Atomic 转换为普通变量

```rust
// 多线程版本
use std::sync::atomic::{AtomicU32, Ordering};

fn multi_thread_example() {
    let counter = AtomicU32::new(0);
    counter.fetch_add(1, Ordering::SeqCst);
}

// 单线程版本
fn single_thread_example() {
    let mut counter = 0;
    counter += 1; // 普通变量操作
}
```

### 任务调度简化

#### 1. 线程池转换为单任务

```rust
// 多线程版本
use rayon::prelude::*;

fn multi_thread_example() {
    let results: Vec<_> = (0..100).into_par_iter()
        .map(|x| x * x)
        .collect();
}

// 单线程版本
fn single_thread_example() {
    let results: Vec<_> = (0..100)
        .map(|x| x * x)
        .collect();
}
```

#### 2. 异步转换为同步

```rust
// 多线程异步版本
use tokio::runtime::Runtime;

fn multi_thread_async_example() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result = async_operation().await;
        process_result(result);
    });
}

// 单线程同步版本
fn single_thread_example() {
    let result = sync_operation(); // 同步版本
    process_result(result);
}
```

## 第三步：实施转换

### Cargo.toml 配置

```toml
[package]
name = "my-single-thread-project"
version = "0.1.0"
edition = "2021"

[dependencies]
# 移除多线程依赖
# rayon = "1.5.1"
# tokio = "1.21.2"

# 保留必要的单线程依赖
log = "0.4.17"

[features]
default = []
std = []

[profile.release]
codegen-units = 1
debug = true
lto = true
opt-level = "s"
```

### 代码转换示例

#### 1. 并发数据结构转换

```rust
// 多线程版本
use std::sync::Mutex;
use std::collections::HashMap;

struct ConcurrentData {
    data: Mutex<HashMap<String, i32>>,
}

// 单线程版本
use std::collections::HashMap;

struct SingleThreadData {
    data: HashMap<String, i32>,
}
```

#### 2. 错误处理转换

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

#### 3. 资源管理转换

```rust
// 多线程版本
use std::thread;
use std::sync::{Arc, Mutex};

struct SharedResource {
    data: Mutex<i32>,
}

fn multi_thread_example() {
    let resource = Arc::new(SharedResource { data: Mutex::new(0) });

    let handles: Vec<_> = (0..10).map(|_| {
        let resource = Arc::clone(&resource);
        thread::spawn(move || {
            let mut data = resource.data.lock().unwrap();
            *data += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

// 单线程版本
struct SingleThreadResource {
    data: i32,
}

fn single_thread_example() {
    let mut resource = SingleThreadResource { data: 0 };

    for _ in 0..10 {
        resource.data += 1; // 直接访问，无需锁
    }
}
```

## 第四步：测试和验证

### 构建测试
```bash
cargo build
cargo test
```

### 常见问题解决

#### 1. 死锁问题
```
error: potential deadlock detected
```
解决方案：移除不必要的锁，确保单线程访问。

#### 2. 并发数据竞争
```
error: data race detected
```
解决方案：使用单线程安全的数据结构。

#### 3. 性能下降
```
warning: single-threaded version may be slower
```
解决方案：优化算法，减少不必要的同步。

#### 4. 功能缺失
```
error: feature not available in single-threaded mode
```
解决方案：实现自定义替代方案或重新设计。

## 资源

此技能包含生产环境中使用的代码模板和参考资料：

### scripts/（脚本）
可直接运行的 Rust 脚本，用于自动化转换任务。

**示例：**
- `analyze_concurrency.py` - 分析多线程依赖并生成转换建议
- `validate_single_thread_build.py` - 验证单线程构建配置

**适用于：** Rust 脚本、构建自动化脚本、代码分析工具。

**注意：** 脚本可以在不加载到上下文的情况下执行，但 Claude 仍可以读取它们以进行修补或环境调整。

### references/（参考资料）
用于指导转换过程的详细文档和最佳实践。

**示例：**
- `concurrency_analysis.md` - 并发代码分析指南
- `synchronization_patterns.md` - 同步原语替换模式
- `task_scheduling.md` - 任务调度简化策略
- `error_handling.md` - 单线程错误处理模式

**适用于：** 深入的技术文档、代码示例、构建配置指南。

### assets/（素材）
不用于加载到上下文，而是在 Claude 生成的输出中使用的文件。

**示例：**
- `Cargo.toml_templates/` - 各种单线程目标的 Cargo.toml 模板
- `mutex_replacement.rs` - Mutex 替换实现
- `channel_replacement.rs` - Channel 替换实现
- `async_to_sync.rs` - 异步到同步转换

**适用于：** 代码模板、配置文件、示例实现。

---

**不需要的目录可以删除。** 并非每个技能都需要所有三种类型的资源。
