#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

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
///
const MIN_HEAP_SIZE: usize = 0x8000; // 32 K
pub struct EarlyAllocator;

impl EarlyAllocator {
}

impl BaseAllocator for EarlyAllocator {
    fn init(&mut self, start: usize, end: usize) {
        assert!(end > MIN_HEAP_SIZE);
          
    }
    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        
    }
}

impl ByteAllocator for EarlyAllocator {
}

impl PageAllocator for EarlyAllocator {
}
