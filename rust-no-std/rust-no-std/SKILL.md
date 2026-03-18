---
name: rust-no-std
description: 将现有的 Rust 代码从标准库环境转换为 no-std 环境的专业技能。当需要在嵌入式系统、操作系统内核、WebAssembly 或其他无标准库环境中运行 Rust 代码时，应使用此技能。适用于需要将 std 依赖替换为 no-std 兼容替代方案的场景。
license: MIT
---

# Rust No Std 转换指南

## 概述

此技能提供将 Rust 代码从标准库环境转换为 no-std 环境的完整指南，包含业界最佳实践和可复用的代码模板。no-std 环境适用于嵌入式系统、操作系统开发、WebAssembly 等资源受限环境，在这些环境中无法使用标准库。


## 核心能力

此技能提供以下核心能力：

### 1. 依赖替换策略
- std::collections → alloc::collections
- std::io → 嵌入式 I/O 实现
- std::thread → 无线程或自定义线程实现
- std::sync → 无同步原语或自定义实现

### 2. 内存管理方案
- Box、Vec、String → 基于分配器的实现
- Option、Result → 无变化（标准库类型）
- 自定义内存分配器集成

### 3. 错误处理
- std::error::Error → 自定义错误 trait
- Result 类型处理
- 错误传播策略

### 4. 构建配置
- Cargo.toml 配置
- 特性标志管理
- 条件编译

## 工作流程决策树

```
开始转换 → [项目是否已有 std 依赖？]
           ├─ 是 → 分析 std 依赖 → [依赖是否可替换？]
           │        ├─ 是 → 选择 no-std 替代方案 → 实施转换
           │        └─ 否 → 考虑自定义实现或重新设计
           └─ 否 → 跳过转换步骤
```

## 第一步：分析现有代码

在开始转换之前，必须分析现有代码的 std 依赖情况。

### 识别 std 依赖

使用以下方法识别 std 依赖：

1. **Cargo.toml 分析**：检查 `dependencies` 部分中的 std 相关依赖
2. **代码审查**：查找 `use std::*;` 语句
3. **构建分析**：运行 `cargo check --target <no-std-target>` 查看错误

### 常见 std 依赖类别

| std 模块 | no-std 替代方案 | 适用场景 |
|---------|---------------|---------|
| std::collections | alloc::collections | 需要集合类型 |
| std::io | 嵌入式 I/O | 文件/网络操作 |
| std::thread | 无线程/自定义 | 并发需求 |
| std::sync | 无同步/自定义 | 线程安全 |
| std::fs | 嵌入式文件系统 | 文件操作 |
| std::net | 嵌入式网络 | 网络通信 |
| std::process | 无进程 | 进程管理 |
| std::env | 无环境变量 | 环境访问 |

## 资源

此技能包含示例资源目录，演示如何组织不同类型的捆绑资源：

### scripts/（脚本）
可直接运行的 Rust 脚本，用于自动化转换任务。

**示例：**
- `convert_std_to_no_std.py` - 分析 std 依赖并生成转换建议
- `validate_no_std_build.py` - 验证 no-std 构建配置

**适用于：** Rust 脚本、构建自动化脚本、代码分析工具。

**注意：** 脚本可以在不加载到上下文的情况下执行，但 Claude 仍可以读取它们以进行修补或环境调整。

### references/（参考资料）
用于指导转换过程的详细文档和最佳实践。

**示例：**
- `memory_management.md` - 内存分配器实现指南
- `error_handling.md` - no-std 错误处理模式
- `build_configuration.md` - Cargo.toml 配置最佳实践
- `common_patterns.md` - 常见 std 替代模式

**适用于：** 深入的技术文档、代码示例、构建配置指南。

### assets/（素材）
不用于加载到上下文，而是在 Claude 生成的输出中使用的文件。

**示例：**
- `Cargo.toml_templates/` - 各种 no-std 目标的 Cargo.toml 模板
- `error_trait.rs` - 自定义错误 trait 实现
- `panic_handler.rs` - 不同平台的 panic 处理程序
- `allocator.rs` - 常见分配器实现

**适用于：** 代码模板、配置文件、示例实现。

---

**不需要的目录可以删除。** 并非每个技能都需要所有三种类型的资源。建议保留 references/ 目录中的技术文档，这些对实际转换工作最有价值。
