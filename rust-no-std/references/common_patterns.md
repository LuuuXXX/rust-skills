# 常见 std 替代模式

## 概述

在 no-std 环境中，许多标准库功能需要替换为 no-std 兼容的替代方案。本指南提供了常见的替代模式。

## 1. 集合类型替代

### Vec 替代

```rust
use alloc::vec::Vec;

// 使用方法相同，但需要分配器
let mut v: Vec<i32> = Vec::new();
v.push(42);
v.pop();
```

### HashMap 替代

```rust
use alloc::collections::BTreeMap;

let mut map: BTreeMap<&str, i32> = BTreeMap::new();
map.insert("key", 42);
map.get("key");
```

### String 替代

```rust
use alloc::string::String;
use alloc::vec::Vec;

let s: String = String::from("Hello");
let bytes: Vec<u8> = s.into_bytes();
```

## 2. I/O 替代

### 嵌入式 I/O

```rust
use core::fmt::Write;

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // 实现串口输出
        Ok(())
    }
}

static mut SERIAL: Serial = Serial;

fn print(s: &str) {
    let _ = unsafe { SERIAL.write_str(s) };
}
```

### 文件系统替代

```rust
use core::fmt::Write;

struct FileSystem;

impl Write for FileSystem {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // 实现文件写入
        Ok(())
    }
}
```

## 3. 线程替代

### 无线程环境

```rust
// 在 no-std 环境中，通常没有线程支持
// 使用事件循环或状态机代替
```

### 轻量级并发

```rust
use spin::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

fn increment() {
    let mut counter = COUNTER.lock();
    *counter += 1;
}
```

## 4. 同步原语替代

### Mutex 替代

```rust
use spin::Mutex;

static DATA: Mutex<Vec<u8>> = Mutex::new(Vec::new());

fn add_data(data: &[u8]) {
    let mut guard = DATA.lock();
    guard.extend_from_slice(data);
}
```

### RwLock 替代

```rust
use spin::RwLock;

static CONFIG: RwLock<Config> = RwLock::new(Config::default());

fn read_config() -> Config {
    CONFIG.read().clone()
}

fn write_config(config: Config) {
    *CONFIG.write() = config;
}
```

## 5. 环境变量替代

```rust
// 在 no-std 环境中，通常没有环境变量
// 使用配置文件或硬编码值代替
```

## 6. 进程替代

```rust
// 在 no-std 环境中，通常没有进程概念
// 使用任务或协程代替
```

## 7. 时间和日期替代

### 时间戳

```rust
use core::time::Duration;

struct Timer;

impl Timer {
    fn now() -> Duration {
        // 实现获取当前时间
        Duration::new(0, 0)
    }
}
```

### 延时

```rust
fn delay(duration: Duration) {
    // 实现延时功能
}
```

## 8. 随机数生成替代

```rust
struct Random;

impl Random {
    fn next_u32() -> u32 {
        // 实现随机数生成
        0
    }
}
```

## 9. 序列化和反序列化替代

### Postcard 序列化

```rust
use postcard;

fn serialize<T>(value: &T) -> Result<Vec<u8>, postcard::Error>
where
    T: serde::Serialize,
{
    postcard::to_allocvec(value)
}

fn deserialize<T>(data: &[u8]) -> Result<T, postcard::Error>
where
    T: serde::Deserialize<'static>,
{
    postcard::from_bytes(data)
}
```

### JSON 序列化

```rust
use serde_json;

fn serialize_json<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: serde::Serialize,
{
    serde_json::to_string(value)
}

fn deserialize_json<T>(data: &str) -> Result<T, serde_json::Error>
where
    T: serde::Deserialize<'static>,
{
    serde_json::from_str(data)
}
```

## 10. 日志替代

### Defmt 日志

```rust
#[macro_use]
extern crate defmt;

defmt::info!("This is an info message");
defmt::warn!("This is a warning");
defmt::error!("This is an error");
```

### 自定义日志

```rust
use core::fmt::Write;

struct Logger;

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // 实现日志输出
        Ok(())
    }
}

static mut LOGGER: Logger = Logger;

fn log(message: &str) {
    let _ = unsafe { LOGGER.write_str(message) };
}
```

## 11. 网络替代

### 嵌入式网络

```rust
use embedded_hal::blocking::spi::Transfer;

struct NetworkInterface;

impl Transfer<u8> for NetworkInterface {
    type Error = NetworkError;

    fn transfer(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        // 实现网络传输
        Ok(())
    }
}
```

## 12. 文件系统替代

### 嵌入式文件系统

```rust
use embedded_fs::{Directory, File, FileSystem};

struct MyFileSystem;

impl FileSystem for MyFileSystem {
    type Error = FsError;

    fn open(&self, path: &str) -> Result<File<Self>, Self::Error> {
        // 实现文件打开
    }

    fn create_dir(&self, path: &str) -> Result<Directory<Self>, Self::Error> {
        // 实现目录创建
    }
}
```

## 13. 加密替代

### AES 加密

```rust
use aes::Aes128;
use block_modes::{BlockMode, Cbc};

type Aes128Cbc = Cbc<Aes128, pkcs7::Pkcs7>;

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes128Cbc::new_from_slices(key, iv)?;
    let mut buffer = data.to_vec();
    cipher.encrypt(&mut buffer)?;
    Ok(buffer)
}
```

## 14. 压缩替代

### 无损压缩

```rust
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

fn compress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    Ok(compress_prepend_size(data))
}

fn decompress(data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
    decompress_size_prepended(data).map_err(|_| DecompressionError)
}
```

## 15. 数学运算替代

### 浮点运算

```rust
use core::f32;

fn calculate_sqrt(x: f32) -> f32 {
    x.sqrt()
}
```

### 复数运算

```rust
use num_complex::Complex;

fn complex_add(a: Complex<f32>, b: Complex<f32>) -> Complex<f32> {
    a + b
}
```

## 最佳实践

1. **最小化依赖**：只使用必要的 no-std 兼容库
2. **代码复用**：创建可复用的替代实现
3. **性能考虑**：选择高效的替代方案
4. **错误处理**：实现适当的错误处理
5. **测试**：确保替代方案的正确性
6. **文档**：记录替代方案的使用方法

## 常见陷阱

### 1. 忘记分配器

```rust
// 错误：未设置全局分配器
#![no_std]

fn main() {
    let v: Vec<i32> = Vec::new();  // 编译错误
}
```

### 2. 使用标准库类型

```rust
// 错误：使用 std 类型
use std::vec::Vec;  // 编译错误
```

### 3. 忽略 panic 处理

```rust
// 错误：未实现 panic 处理程序
#![no_std]

fn main() {
    // 可能panic，但没有处理程序
}
```

## 推荐库

- `alloc` - 基本分配支持
- `embedded-hal` - 嵌入式硬件抽象层
- `spin` - 线程安全原语
- `defmt` - 轻量级日志
- `postcard` - 序列化
- `lz4_flex` - 压缩
- `num-complex` - 复数运算
- `aes` - 加密

## 性能考虑

- 选择高效的替代方案
- 避免不必要的分配
- 考虑内存使用
- 优化算法复杂度
- 测试性能影响