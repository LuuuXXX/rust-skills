# 任务调度简化策略

## 概述

在将 Rust 多线程代码转换为单线程代码时，需要简化任务调度逻辑。本指南提供了常见的任务调度简化策略。

## 线程池转换为单任务

### 1. 基本转换

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

### 2. 复杂任务转换

```rust
// 多线程版本
use rayon::prelude::*;

fn multi_thread_complex_example() {
    let data = vec![1, 2, 3, 4, 5];

    let results: Vec<_> = data.into_par_iter()
        .map(|x| complex_computation(x))
        .collect();

    process_results(results);
}

// 单线程版本
fn single_thread_complex_example() {
    let data = vec![1, 2, 3, 4, 5];

    let results: Vec<_> = data
        .into_iter()
        .map(|x| complex_computation(x))
        .collect();

    process_results(results);
}
```

## 异步任务转换为同步

### 1. 基本异步转换

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

### 2. 异步链式调用转换

```rust
// 多线程异步版本
use tokio::runtime::Runtime;

async fn fetch_data() -> String {
    // 模拟异步数据获取
    "data".to_string()
}

async fn process_data(data: String) -> String {
    // 模拟数据处理
    format!("processed: {}", data)
}

fn multi_thread_async_example() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let data = fetch_data().await;
        let result = process_data(data).await;
        println!("{}", result);
    });
}

// 单线程同步版本
fn fetch_data_sync() -> String {
    // 同步数据获取
    "data".to_string()
}

fn process_data_sync(data: String) -> String {
    // 同步数据处理
    format!("processed: {}", data)
}

fn single_thread_example() {
    let data = fetch_data_sync();
    let result = process_data_sync(data);
    println!("{}", result);
}
```

## 事件循环模式

### 1. 简单事件循环

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

struct EventLoop {
    tasks: Vec<Box<dyn FnOnce()>>,
}

impl EventLoop {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Box<dyn FnOnce()>) {
        self.tasks.push(task);
    }

    fn run(&mut self) {
        while let Some(task) = self.tasks.pop() {
            task();
        }
    }
}

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();
    let mut event_loop = EventLoop::new();

    thread::spawn(move || {
        event_loop.add_task(Box::new(|| {
            println!("Task 1");
        }));
        tx.send(()).unwrap();
    });

    rx.recv().unwrap();
    event_loop.run();
}

// 单线程版本
struct SingleThreadEventLoop {
    tasks: Vec<Box<dyn FnOnce()>>,
}

impl SingleThreadEventLoop {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Box<dyn FnOnce()>) {
        self.tasks.push(task);
    }

    fn run(&mut self) {
        while let Some(task) = self.tasks.pop() {
            task();
        }
    }
}

fn single_thread_example() {
    let mut event_loop = SingleThreadEventLoop::new();
    event_loop.add_task(Box::new(|| {
        println!("Task 1");
    }));
    event_loop.run();
}
```

### 2. 带优先级的事件循环

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

struct PriorityEventLoop {
    tasks: Vec<(i32, Box<dyn FnOnce()>)>,
}

impl PriorityEventLoop {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, priority: i32, task: Box<dyn FnOnce()>) {
        self.tasks.push((priority, task));
        self.tasks.sort_by_key(|(p, _)| *p);
    }

    fn run(&mut self) {
        while let Some((_, task)) = self.tasks.pop() {
            task();
        }
    }
}

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();
    let mut event_loop = PriorityEventLoop::new();

    thread::spawn(move || {
        event_loop.add_task(1, Box::new(|| {
            println!("High priority task");
        }));
        tx.send(()).unwrap();
    });

    thread::spawn(move || {
        event_loop.add_task(2, Box::new(|| {
            println!("Low priority task");
        }));
        tx.send(()).unwrap();
    });

    rx.recv().unwrap();
    rx.recv().unwrap();
    event_loop.run();
}

// 单线程版本
struct SingleThreadPriorityEventLoop {
    tasks: Vec<(i32, Box<dyn FnOnce()>)>,
}

impl SingleThreadPriorityEventLoop {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, priority: i32, task: Box<dyn FnOnce()>) {
        self.tasks.push((priority, task));
        self.tasks.sort_by_key(|(p, _)| *p);
    }

    fn run(&mut self) {
        while let Some((_, task)) = self.tasks.pop() {
            task();
        }
    }
}

fn single_thread_example() {
    let mut event_loop = SingleThreadPriorityEventLoop::new();
    event_loop.add_task(1, Box::new(|| {
        println!("High priority task");
    }));
    event_loop.add_task(2, Box::new(|| {
        println!("Low priority task");
    }));
    event_loop.run();
}
```

## 状态机模式

### 1. 基本状态机

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

enum State {
    Initial,
    Processing,
    Finished,
}

struct StateMachine {
    state: State,
    tx: Sender<State>,
}

impl StateMachine {
    fn new(tx: Sender<State>) -> Self {
        Self { state: State::Initial, tx }
    }

    fn process(&mut self) {
        match self.state {
            State::Initial => {
                self.state = State::Processing;
                self.tx.send(State::Processing).unwrap();
            }
            State::Processing => {
                self.state = State::Finished;
                self.tx.send(State::Finished).unwrap();
            }
            State::Finished => {}
        }
    }
}

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();
    let mut state_machine = StateMachine::new(tx);

    thread::spawn(move || {
        state_machine.process();
    });

    for state in rx {
        match state {
            State::Processing => {
                println!("Processing...");
            }
            State::Finished => {
                println!("Finished!");
                break;
            }
            _ => {}
        }
    }
}

// 单线程版本
enum State {
    Initial,
    Processing,
    Finished,
}

struct SingleThreadStateMachine {
    state: State,
}

impl SingleThreadStateMachine {
    fn new() -> Self {
        Self { state: State::Initial }
    }

    fn process(&mut self) {
        match self.state {
            State::Initial => {
                self.state = State::Processing;
                println!("Processing...");
            }
            State::Processing => {
                self.state = State::Finished;
                println!("Finished!");
            }
            State::Finished => {}
        }
    }
}

fn single_thread_example() {
    let mut state_machine = SingleThreadStateMachine::new();
    state_machine.process();
    state_machine.process();
}
```

### 2. 带数据的复杂状态机

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

enum State {
    Initial(i32),
    Processing(i32),
    Finished(i32),
}

struct StateMachine {
    state: State,
    tx: Sender<State>,
}

impl StateMachine {
    fn new(tx: Sender<State>) -> Self {
        Self { state: State::Initial(0), tx }
    }

    fn process(&mut self) {
        match self.state {
            State::Initial(value) => {
                self.state = State::Processing(value + 1);
                self.tx.send(State::Processing(value + 1)).unwrap();
            }
            State::Processing(value) => {
                self.state = State::Finished(value * 2);
                self.tx.send(State::Finished(value * 2)).unwrap();
            }
            State::Finished(_) => {}
        }
    }
}

fn multi_thread_example() {
    let (tx, rx) = mpsc::channel();
    let mut state_machine = StateMachine::new(tx);

    thread::spawn(move || {
        state_machine.process();
    });

    for state in rx {
        match state {
            State::Processing(value) => {
                println!("Processing with value: {}", value);
            }
            State::Finished(value) => {
                println!("Finished with value: {}", value);
                break;
            }
            _ => {}
        }
    }
}

// 单线程版本
enum State {
    Initial(i32),
    Processing(i32),
    Finished(i32),
}

struct SingleThreadStateMachine {
    state: State,
}

impl SingleThreadStateMachine {
    fn new() -> Self {
        Self { state: State::Initial(0) }
    }

    fn process(&mut self) {
        match self.state {
            State::Initial(value) => {
                self.state = State::Processing(value + 1);
                println!("Processing with value: {}", value + 1);
            }
            State::Processing(value) => {
                self.state = State::Finished(value * 2);
                println!("Finished with value: {}", value * 2);
            }
            State::Finished(_) => {}
        }
    }
}

fn single_thread_example() {
    let mut state_machine = SingleThreadStateMachine::new();
    state_machine.process();
    state_machine.process();
}
```

## 批处理模式

### 1. 简单批处理

```rust
// 多线程版本
use rayon::prelude::*;

fn multi_thread_batch_example() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let results: Vec<_> = data.into_par_iter()
        .map(|x| batch_process(x))
        .collect();

    process_results(results);
}

// 单线程版本
fn single_thread_batch_example() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let results: Vec<_> = data
        .into_iter()
        .map(|x| batch_process(x))
        .collect();

    process_results(results);
}
```

### 2. 分页批处理

```rust
// 多线程版本
use rayon::prelude::*;

fn multi_thread_paged_example() {
    let data = (0..1000).collect::<Vec<_>>();

    let results: Vec<_> = data.into_par_iter()
        .chunks(100)
        .map(|chunk| process_chunk(chunk))
        .flatten()
        .collect();

    process_results(results);
}

// 单线程版本
fn single_thread_paged_example() {
    let data = (0..1000).collect::<Vec<_>>();

    let results: Vec<_> = data
        .chunks(100)
        .map(|chunk| process_chunk(chunk))
        .flatten()
        .collect();

    process_results(results);
}
```

## 并发队列转换为顺序队列

### 1. 无界队列转换

```rust
// 多线程版本
use std::sync::mpsc;

fn multi_thread_queue_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    for item in rx {
        println!("Received: {}", item);
    }
}

// 单线程版本
fn single_thread_queue_example() {
    let items = (0..10).collect::<Vec<_>>();

    for item in items {
        println!("Processed: {}", item);
    }
}
```

### 2. 有界队列转换

```rust
// 多线程版本
use std::sync::mpsc;

fn multi_thread_bounded_queue_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    for item in rx {
        println!("Received: {}", item);
    }
}

// 单线程版本
fn single_thread_bounded_queue_example() {
    let items = (0..10).collect::<Vec<_>>();

    for item in items {
        println!("Processed: {}", item);
    }
}
```

## 任务依赖管理

### 1. 简单依赖

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_dependency_example() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let result = do_first_task();
        tx1.send(result).unwrap();
    });

    thread::spawn(move || {
        let result1 = rx1.recv().unwrap();
        let result2 = do_second_task(result1);
        tx2.send(result2).unwrap();
    });

    let final_result = rx2.recv().unwrap();
    process_final_result(final_result);
}

// 单线程版本
fn single_thread_dependency_example() {
    let result1 = do_first_task();
    let result2 = do_second_task(result1);
    let final_result = process_final_result(result2);
}
```

### 2. 复杂依赖图

```rust
// 多线程版本
use std::thread;
use std::sync::mpsc;

fn multi_thread_complex_dependency_example() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    thread::spawn(move || {
        let result = do_task_a();
        tx1.send(result).unwrap();
    });

    thread::spawn(move || {
        let result_a = rx1.recv().unwrap();
        let result_b = do_task_b(result_a);
        tx2.send(result_b).unwrap();
    });

    thread::spawn(move || {
        let result_b = rx2.recv().unwrap();
        let result_c = do_task_c(result_b);
        tx3.send(result_c).unwrap();
    });

    let final_result = rx3.recv().unwrap();
    process_final_result(final_result);
}

// 单线程版本
fn single_thread_complex_dependency_example() {
    let result_a = do_task_a();
    let result_b = do_task_b(result_a);
    let result_c = do_task_c(result_b);
    let final_result = process_final_result(result_c);
}
```

## 最佳实践

1. **顺序执行**：将并发任务转换为顺序执行
2. **事件驱动**：使用事件循环替代线程池
3. **状态管理**：使用状态机替代并发状态
4. **批处理**：将小任务合并为批处理
5. **同步调用**：将异步调用转换为同步调用

## 常见陷阱

### 1. 过度简化
```rust
// 错误：过度简化导致功能丢失
fn over_simplified() {
    // 移除了必要的并发逻辑
}
```

### 2. 性能下降
```rust
// 错误：单线程版本性能下降
fn performance_issue() {
    // 复杂任务在单线程中执行太慢
}
```

### 3. 死锁风险
```rust
// 错误：仍然存在并发问题
fn deadlock_risk() {
    // 未完全移除同步原语
}
```

## 推荐库

- `tokio` - 异步运行时（用于异步到同步转换）
- `rayon` - 数据并行库（用于线程池转换）
- `crossbeam` - 并发数据结构
- `event_loop` - 事件循环实现

## 性能考虑

- 单线程代码通常比多线程代码更快（减少同步开销）
- 需要根据具体场景进行性能测试
- 避免不必要的任务创建和调度
- 优化算法复杂度