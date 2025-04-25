#![no_std]

use core::{alloc::Layout, ptr::NonNull};

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///思路：首先初始化位分配器大小为最小堆大小，页分配器大小为0
/// 然后，当位分配器的大小不足以满足分配请求时，将位分配器的大小增加。但不能大于页分配器的内存起始。
/// 当页分配器大小不满足时，就重新init指定范围，init是insert
#[inline]
const fn align_down(pos: usize, align: usize) -> usize {
    pos & !(align - 1)
}
#[inline]
const fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}
pub struct EarlyAllocator<const PAGE_SIZE:usize>{
    b_count:usize,
    p_pos:usize,
    b_pos:usize,
    start:usize,
    end:usize,
    total_pages: usize,
    used_pages: usize,
}

impl<const PAGE_SIZE:usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self{
            b_count:0,
            p_pos:0,
            b_pos:0,
            end:0,
            start:0,
            total_pages: 0,
            used_pages: 0,
        }
    }
}

impl<const PAGE_SIZE:usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start_vaddr: usize, size: usize) {
        self.start = start_vaddr;
        self.end = start_vaddr + size;
        self.b_pos = self.start;
        self.p_pos = self.end;
    }
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}

impl<const PAGE_SIZE:usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let start = align_up(self.b_pos, layout.align());
        let end = start  + layout.size();
        if end > self.p_pos {
            return Err(AllocError::NoMemory);
        }else{
            self.b_count += 1;
            self.b_pos = end;
            NonNull::new(start as *mut u8).ok_or(AllocError::NoMemory)
        }
    }
    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        self.b_count -= 1;
        if self.b_count == 0 {
            self.b_pos = self.start;
        }
    }
    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }
    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
    fn total_bytes(&self) -> usize {
        self.p_pos - self.start
    }
}

impl<const PAGE_SIZE:usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let next = align_down(self.p_pos - num_pages * PAGE_SIZE, align_pow2);
        if next < self.b_pos {
            return Err(AllocError::NoMemory);
        }
        self.p_pos = next;
        self.used_pages += num_pages;
        self.total_pages = self.used_pages;
        Ok(next)
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / PAGE_SIZE
    }
    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        //不释放
    }
    fn total_pages(&self) -> usize {
        self.total_pages
    }
    fn used_pages(&self) -> usize {
        self.used_pages
    }
}
