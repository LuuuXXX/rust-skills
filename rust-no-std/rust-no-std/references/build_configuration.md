# 构建配置指南

## 概述

正确的构建配置是成功将 Rust 项目转换为 no-std 环境的关键。本指南提供了 Cargo.toml 配置的最佳实践。

## 基本配置

### 1. 基本包配置

```toml
[package]
name = "my-no-std-project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your@email.com>"]
description = "A no-std Rust project"
license = "MIT OR Apache-2.0"
readme = "README.md"
keywords = ["no-std", "embedded", "rust"]
categories = ["embedded", "no-std"]

[dependencies]
# 基本依赖
alloc = { version = "0.0.0", features = ["rustc-std-alloc"] }
```

### 2. 特性标志

```toml
[features]
default = []
std = []  # 启用标准库支持
debug = []  # 调试功能
release = []  # 发布功能

[dependencies]
# 条件依赖
embedded-hal = { version = "0.2", optional = true }
log = { version = "0.4", optional = true }
```

## 目标配置

### 1. 嵌入式目标

```toml
[package.metadata]
# 嵌入式目标配置
target = "thumbv7em-none-eabihf"

[profile.dev]
panic = "abort"
lto = true
codegen-units = 1
debug = true
opt-level = "s"

[profile.release]
panic = "abort"
lto = true
codegen-units = 1
debug = true
opt-level = "s"
```

### 2. WebAssembly 目标

```toml
[target.wasm32-unknown-unknown]
# WASM 特定配置
rustflags = [
    "-C", "link-arg=--export-table",
    "-C", "link-arg=--export-dynamic",
]

[profile.release]
lto = true
opt-level = "z"
codegen-units = 1
```

## 依赖管理

### 1. no-std 兼容依赖

```toml
[dependencies]
# 内存分配
linked-list-allocator = "0.9.0"

# 嵌入式支持
embedded-hal = "0.2.7"
nb = "1.0.0"

# 日志
log = { version = "0.4.17", optional = true }
defmt = { version = "0.3.2", optional = true }

# 并发
spin = "0.9.8"

# 序列化
postcard = { version = "1.0.4", features = ["alloc"] }

# 加密
aes = "0.8.3"
crypto-common = "0.1.6"
```

### 2. 条件依赖

```toml
[dependencies]
# 根据特性标志选择依赖
embedded-hal = { version = "0.2.7", optional = true }
std = { package = "std", version = "0.0.0", optional = true }
```

## 构建脚本

### 1. 构建前脚本

```toml
[build-dependencies]
# 构建时依赖
cc = "1.0.79"
```

### 2. 自定义构建逻辑

```toml
[package.metadata]
# 自定义元数据
board = "stm32f4"
chip = "stm32f407"
```

## 优化配置

### 1. 开发配置

```toml
[profile.dev]
# 开发优化
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
lto = false
codegen-units = 16
```

### 2. 发布配置

```toml
[profile.release]
# 发布优化
opt-level = "s"  # 或 "z" 用于极致优化
debug = false
debug-assertions = false
overflow-checks = false
lto = true
codegen-units = 1
panic = "abort"
```

## 测试配置

### 1. 单元测试

```toml
[[test]]
name = "unit_tests"
harness = false  # 禁用测试支架，适用于 no-std
```

### 2. 集成测试

```toml
[[test]]
name = "integration_tests"
harness = true
```

## 工具链配置

### 1. Rustup 工具链

```toml
[toolchain]
channel = "stable"
components = ["rustc", "cargo", "rust-src", "rust-std", "rust-docs"]
```

### 2. 自定义工具链

```toml
[toolchain.my-toolchain]
channel = "nightly"
components = ["rustc", "cargo", "rust-src", "rust-std", "rust-docs", "clippy", "rustfmt"]
```

## 预处理器定义

```toml
[env]
# 预处理器定义
DEFMT_LOG = "trace"
```

## 链接器脚本

```toml
[build]
# 自定义链接器脚本
rustflags = [
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=-nostartfiles",
]

[package.metadata]
# 链接器脚本路径
linker-script = "link.x"
```

## 最佳实践

1. **最小化依赖**：只包含必要的依赖
2. **条件编译**：使用特性标志管理不同配置
3. **优化配置**：为不同环境配置适当的优化级别
4. **测试策略**：确保测试覆盖 no-std 环境
5. **文档**：提供清晰的文档说明构建要求
6. **版本管理**：使用语义化版本管理依赖

## 常见问题解决

### 1. 分配器未设置

```
error: requires `alloc` language item
```

解决方案：确保分配器已正确实现并设置为全局分配器。

### 2. 缺少 panic 处理程序

```
error: requires `panic_impl` language item
```

解决方案：在 `Cargo.toml` 中添加 panic 配置。

### 3. 链接器错误

```
error: linking with `link.exe` failed
```

解决方案：配置正确的链接器脚本和标志。

### 4. 目标不支持

```
error: the target `thumbv7em-none-eabihf` is not installed
```

解决方案：使用 `rustup target add thumbv7em-none-eabihf` 安装目标。

## 推荐工具

- `cargo-config` - Cargo 配置管理
- `cargo-make` - 高级构建工具
- `cargo-watch` - 自动重新构建
- `cargo-expand` - 查看宏展开结果

## 示例项目结构

```
my-no-std-project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
├── memory/
│   └── allocator.rs
├── hal/
│   └── gpio.rs
├── tests/
│   └── integration.rs
└── link.x
```

## 性能考虑

- 优化构建时间：使用适当的优化级别
- 减小编译输出大小：使用适当的优化标志
- 内存使用：考虑目标设备的内存限制
- 启动时间：优化初始化代码