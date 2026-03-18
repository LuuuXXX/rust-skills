#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use linked_list_allocator::Heap;

/// 全局分配器实现
pub struct GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 使用堆分配器
        HEAP.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // 使用堆分配器释放
        HEAP.dealloc(ptr, layout)
    }
}

/// 全局分配器实例
#[global_allocator]
static mut GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;

/// 堆分配器
static mut HEAP: Heap = Heap::empty();

/// 初始化分配器
pub fn init_allocator(start: usize, size: usize) {
    unsafe {
        HEAP.init(start as *mut u8, size);
    }
}

/// 分配内存
pub fn alloc(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    unsafe {
        let ptr = GLOBAL_ALLOCATOR.alloc(layout);
        if ptr.is_null() {
            Err(AllocError)
        } else {
            Ok(NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(ptr, layout.size())))
        }
    }
}

/// 释放内存
pub fn dealloc(ptr: NonNull<u8>, layout: Layout) {
    unsafe {
        GLOBAL_ALLOCATOR.dealloc(ptr.as_ptr(), layout)
    }
}

/// 分配错误
#[derive(Debug)]
pub struct AllocError;

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "memory allocation failed")
    }
}

impl NoStdError for AllocError {}

/// 安全分配
pub fn safe_alloc(layout: Layout) -> Option<NonNull<[u8]>> {
    match alloc(layout) {
        Ok(ptr) => Some(ptr),
        Err(_) => None,
    }
}

/// 安全释放
pub fn safe_dealloc(ptr: NonNull<u8>, layout: Layout) {
    dealloc(ptr, layout)
}

/// 对象池分配器
pub struct ObjectPoolAllocator {
    pool: Vec<NonNull<[u8]>>,
    free_list: Vec<NonNull<[u8]>>,
}

impl ObjectPoolAllocator {
    /// 创建新的对象池分配器
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            free_list: Vec::new(),
        }
    }

    /// 预分配内存
    pub fn preallocate(&mut self, layout: Layout, count: usize) {
        for _ in 0..count {
            if let Some(ptr) = safe_alloc(layout) {
                self.pool.push(ptr);
                self.free_list.push(ptr);
            }
        }
    }

    /// 从池中分配
    pub fn alloc_from_pool(&mut self, layout: Layout) -> Option<NonNull<[u8]>> {
        self.free_list.pop()
    }

    /// 释放到池中
    pub fn dealloc_to_pool(&mut self, ptr: NonNull<[u8]>) {
        self.free_list.push(ptr);
    }
}

/// 虚拟内存分配器
pub struct VirtualMemoryAllocator {
    start: usize,
    end: usize,
    used: usize,
}

impl VirtualMemoryAllocator {
    /// 创建新的虚拟内存分配器
    pub fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            end: start + size,
            used: 0,
        }
    }

    /// 分配虚拟内存
    pub fn alloc(&mut self, layout: Layout) -> Option<NonNull<[u8]>> {
        let size = layout.size();
        let align = layout.align();

        // 查找合适的内存块
        let addr = self.find_free_block(size, align)?;

        // 标记为已使用
        self.used += size;

        unsafe {
            Some(NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(addr as *mut u8, size)))
        }
    }

    /// 释放虚拟内存
    pub fn dealloc(&mut self, ptr: NonNull<[u8]>) {
        let size = ptr.as_ptr() as usize - self.start;
        self.used -= size;
    }

    /// 查找空闲块
    fn find_free_block(&self, size: usize, align: usize) -> Option<usize> {
        // 简单的首次适应算法
        let mut current = self.start;
        while current + size <= self.end {
            // 检查对齐
            if current % align == 0 {
                return Some(current);
            }
            current += 1;
        }
        None
    }
}

/// 分配统计
pub struct AllocationStats {
    total_allocated: usize,
    total_freed: usize,
    peak_usage: usize,
    current_usage: usize,
}

impl AllocationStats {
    /// 创建新的统计信息
    pub fn new() -> Self {
        Self {
            total_allocated: 0,
            total_freed: 0,
            peak_usage: 0,
            current_usage: 0,
        }
    }

    /// 记录分配
    pub fn record_alloc(&mut self, size: usize) {
        self.total_allocated += size;
        self.current_usage += size;
        if self.current_usage > self.peak_usage {
            self.peak_usage = self.current_usage;
        }
    }

    /// 记录释放
    pub fn record_free(&mut self, size: usize) {
        self.total_freed += size;
        self.current_usage -= size;
    }

    /// 获取统计信息
    pub fn stats(&self) -> (usize, usize, usize, usize) {
        (self.total_allocated, self.total_freed, self.peak_usage, self.current_usage)
    }
}

/// 统计分配器
pub struct StatsAllocator {
    inner: GlobalAllocator,
    stats: AllocationStats,
}

unsafe impl GlobalAlloc for StatsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.inner.alloc(layout);
        if !ptr.is_null() {
            self.stats.record_alloc(layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
        self.stats.record_free(layout.size());
    }
}

/// 获取分配统计
pub fn get_allocation_stats() -> AllocationStats {
    // 实现获取统计信息的逻辑
    AllocationStats::new()
}

/// 内存检查
pub fn check_memory() -> Result<(), MemoryCheckError> {
    // 实现内存检查逻辑
    Ok(())
}

/// 内存检查错误
#[derive(Debug)]
pub enum MemoryCheckError {
    Corruption,
    Leak,
    OutOfBounds,
}

impl fmt::Display for MemoryCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryCheckError::Corruption => write!(f, "memory corruption detected"),
            MemoryCheckError::Leak => write!(f, "memory leak detected"),
            MemoryCheckError::OutOfBounds => write!(f, "out of bounds access"),
        }
    }
}

impl NoStdError for MemoryCheckError {}