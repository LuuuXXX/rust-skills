# 同步原语替换模式

## 概述

在将 Rust 多线程代码转换为单线程代码时，需要替换各种同步原语。本指南提供了常见的同步原语替换模式。

## Mutex 替换模式

### 1. 基本替换

```rust
// 多线程版本
use std::sync::Mutex;

struct SharedData {
    data: Mutex<i32>,
}

impl SharedData {
    fn new() -> Self {
        Self { data: Mutex::new(0) }
    }

    fn increment(&self) {
        let mut data = self.data.lock().unwrap();
        *data += 1;
    }

    fn get(&self) -> i32 {
        let data = self.data.lock().unwrap();
        *data
    }
}

// 单线程版本
struct SingleThreadData {
    data: i32,
}

impl SingleThreadData {
    fn new() -> Self {
        Self { data: 0 }
    }

    fn increment(&mut self) {
        self.data += 1;
    }

    fn get(&self) -> i32 {
        self.data
    }
}
```

### 2. 读写锁替换

```rust
// 多线程版本
use std::sync::RwLock;

struct SharedData {
    data: RwLock<i32>,
}

impl SharedData {
    fn new() -> Self {
        Self { data: RwLock::new(0) }
    }

    fn read(&self) -> i32 {
        let data = self.data.read().unwrap();
        *data
    }

    fn write(&self, value: i32) {
        let mut data = self.data.write().unwrap();
        *data = value;
    }
}

// 单线程版本
struct SingleThreadData {
    data: i32,
}

impl SingleThreadData {
    fn new() -> Self {
        Self { data: 0 }
    }

    fn read(&self) -> i32 {
        self.data
    }

    fn write(&mut self, value: i32) {
        self.data = value;
    }
}
```

### 3. 原子操作替换

```rust
// 多线程版本
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter {
    value: AtomicUsize,
}

impl Counter {
    fn new() -> Self {
        Self { value: AtomicUsize::new(0) }
    }

    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

// 单线程版本
struct SingleThreadCounter {
    value: usize,
}

impl SingleThreadCounter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get(&self) -> usize {
        self.value
    }
}
```

## Channel 替换模式

### 1. 简单 channel 替换

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

### 2. 双向 channel 替换

```rust
// 多线程版本
use std::sync::mpsc;

fn multi_thread_example() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let result = do_work();
        tx1.send(result).unwrap();
    });

    thread::spawn(move || {
        let result = rx1.recv().unwrap();
        tx2.send(process_result(result)).unwrap();
    });

    let final_result = rx2.recv().unwrap();
}

// 单线程版本
fn single_thread_example() {
    let result = do_work();
    let processed = process_result(result);
    // 直接使用结果
}
```

## Arc 替换模式

### 1. 共享所有权替换

```rust
// 多线程版本
use std::sync::Arc;

struct SharedResource {
    data: i32,
}

fn multi_thread_example() {
    let resource = Arc::new(SharedResource { data: 0 });

    let handles: Vec<_> = (0..10).map(|_| {
        let resource = Arc::clone(&resource);
        thread::spawn(move || {
            // 使用 resource
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
        // 直接使用 resource
    }
}
```

### 2. 引用计数替换

```rust
// 多线程版本
use std::sync::Arc;

struct Data {
    value: i32,
}

fn multi_thread_example() {
    let data = Arc::new(Data { value: 42 });

    let handles: Vec<_> = (0..5).map(|_| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("Value: {}", data.value);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

// 单线程版本
struct Data {
    value: i32,
}

fn single_thread_example() {
    let data = Data { value: 42 };

    for _ in 0..5 {
        println!("Value: {}", data.value);
    }
}
```

## Barrier 替换模式

### 1. 同步点替换

```rust
// 多线程版本
use std::sync::Barrier;

fn multi_thread_example() {
    let barrier = Barrier::new(3);

    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            println!("Thread {} before barrier", i);
            barrier.wait();
            println!("Thread {} after barrier", i);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

// 单线程版本
fn single_thread_example() {
    for i in 0..3 {
        println!("Thread {} before barrier", i);
        // 模拟 barrier
        println!("Thread {} after barrier", i);
    }
}
```

## Once 替换模式

### 1. 单次初始化替换

```rust
// 多线程版本
use std::sync::Once;

static mut GLOBAL_DATA: Option<i32> = None;
static INIT: Once = Once::new();

fn multi_thread_example() {
    INIT.call_once(|| {
        unsafe {
            GLOBAL_DATA = Some(42);
        }
    });

    unsafe {
        println!("Global data: {}", GLOBAL_DATA.unwrap());
    }
}

// 单线程版本
static mut GLOBAL_DATA: Option<i32> = None;

fn single_thread_example() {
    if unsafe { GLOBAL_DATA.is_none() } {
        unsafe {
            GLOBAL_DATA = Some(42);
        }
    }

    unsafe {
        println!("Global data: {}", GLOBAL_DATA.unwrap());
    }
}
```

## Condvar 替换模式

### 1. 条件变量替换

```rust
// 多线程版本
use std::sync::{Mutex, Condvar};

struct SharedData {
    data: Mutex<i32>,
    cond: Condvar,
}

fn multi_thread_example() {
    let shared = SharedData {
        data: Mutex::new(0),
        cond: Condvar::new(),
    };

    thread::spawn(move || {
        let mut data = shared.data.lock().unwrap();
        *data = 42;
        shared.cond.notify_one();
    });

    let mut data = shared.data.lock().unwrap();
    while *data != 42 {
        data = shared.cond.wait(data).unwrap();
    }
}

// 单线程版本
struct SingleThreadData {
    data: i32,
}

fn single_thread_example() {
    let mut data = SingleThreadData { data: 0 };

    // 直接设置数据
    data.data = 42;

    // 不需要等待条件
}
```

## 任务调度替换模式

### 1. 线程池替换

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

### 2. 异步任务替换

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

## 错误处理替换模式

### 1. 并发错误传播替换

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

## 最佳实践

1. **逐步替换**：一次替换一种同步原语
2. **测试验证**：确保每次替换后功能正确
3. **性能测试**：比较转换前后的性能
4. **代码审查**：确保没有遗漏的同步原语
5. **文档记录**：记录替换决策和理由

## 常见陷阱

### 1. 忘记移除同步原语
```rust
// 错误：仍然使用 Mutex 但在单线程环境中
fn single_thread_with_mutex() {
    let data = Mutex::new(0);
    let mut value = data.lock().unwrap();
    *value += 1;
}
```

### 2. 数据竞争
```rust
// 错误：可能导致数据竞争的代码
fn data_race() {
    let counter = AtomicUsize::new(0);

    thread::spawn(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    });

    thread::spawn(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    });
}
```

### 3. 死锁风险
```rust
// 错误：可能导致死锁的代码
fn deadlock_risk() {
    let lock1 = Mutex::new(0);
    let lock2 = Mutex::new(0);

    thread::spawn(move || {
        let _ = lock1.lock().unwrap();
        let _ = lock2.lock().unwrap();
    });

    thread::spawn(move || {
        let _ = lock2.lock().unwrap();
        let _ = lock1.lock().unwrap();
    });
}
```

## 推荐库

- `once_cell` - 单次初始化
- `parking_lot` - 高性能同步原语
- `crossbeam` - 并发数据结构
- `rayon` - 数据并行库

## 性能考虑

- 单线程代码通常比多线程代码更快（减少同步开销）
- 单线程代码更容易优化
- 需要根据具体场景进行性能测试
- 避免不必要的同步原语