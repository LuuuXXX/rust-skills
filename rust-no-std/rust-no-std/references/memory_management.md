# 内存管理指南

## 概述

在 no-std 环境中，内存管理是关键挑战。Rust 的 no-std 环境不提供标准库的内存分配器，因此需要手动实现或使用外部分配器。

## 分配器类型

### 1. Global Allocator

全局分配器是 no-std 环境中最常用的分配器类型。

```rust
#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::{GlobalAlloc, Layout};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, Size4KiB};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 实现分配逻辑
        // 返回分配的内存指针
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // 实现释放逻辑
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
```

### 2. 嵌入式分配器

适用于资源受限的环境。

```rust
use core::alloc::{AllocError, Allocator};
use core::ptr::NonNull;

struct EmbeddedAllocator;

unsafe impl Allocator for EmbeddedAllocator {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        // 实现分配逻辑
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: core::alloc::Layout) {
        // 实现释放逻辑
    }
}
```

## 常用分配器实现

### 1. 堆分配器

```rust
use linked_list_allocator::Heap;

static mut HEAP: Heap = Heap::empty();

pub fn init_heap(start: usize, size: usize) {
    unsafe {
        HEAP.init(start as *mut u8, size);
    }
}

pub fn alloc(size: usize) -> *mut u8 {
    unsafe {
        HEAP.alloc(size)
    }
}

pub fn dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        HEAP.dealloc(ptr, size);
    }
}
```

### 2. 虚拟内存分配器

```rust
use x86_64::structures::paging::Page;

fn allocate_pages(
    mapper: &mut Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    pages: usize,
) -> Result<Page, MapToError<Size4KiB>> {
    let page_range = {
        let end_page = Page::containing_address(
            VirtualAddress(start + pages * PAGE_SIZE - 1u64)
        );
        Page::range_inclusive(
            Page::containing_address(VirtualAddress(start)),
            end_page,
        )
    };

    for page in page_range {
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            mapper.map_to(page, frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, frame_allocator)?;
        }
    }

    Ok(Page::containing_address(VirtualAddress(start)))
}
```

## 分配策略

### 1. 静态分配

适用于已知大小的数据。

```rust
static mut BUFFER: [u8; 1024] = [0; 1024];

fn use_buffer() {
    unsafe {
        let ptr = &mut BUFFER as *mut [u8];
        // 使用缓冲区
    }
}
```

### 2. 动态分配

使用 alloc crate。

```rust
use alloc::vec::Vec;
use alloc::string::String;

fn create_dynamic_data() -> (Vec<u8>, String) {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let s = String::from("Hello, no-std!");

    (vec, s)
}
```

## 最佳实践

1. **最小化分配**：在 no-std 环境中，分配操作成本很高
2. **预分配内存**：如果可能，预先分配所需内存
3. **重用内存**：实现对象池来重用内存
4. **错误处理**：始终处理分配失败的情况
5. **线程安全**：确保分配器是线程安全的（如果需要）

## 常见问题

### 分配失败

```rust
fn safe_alloc(layout: Layout) -> Option<*mut u8> {
    let ptr = unsafe { GLOBAL.alloc(layout) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}
```

### 内存泄漏

```rust
fn ensure_deallocation(ptr: *mut u8, layout: Layout) {
    unsafe {
        GLOBAL.dealloc(ptr, layout);
    }
}
```

## 推荐库

- `linked_list_allocator` - 简单的堆分配器
- `spin` - 线程安全的分配器
- `heapless` - 无堆分配器，适用于固定大小的容器
- `alloc-init` - 初始化分配器

## 性能考虑

- 分配器实现应尽可能高效
- 考虑内存碎片问题
- 在嵌入式环境中，分配时间应可预测
- 避免频繁的分配和释放操作