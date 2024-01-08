use std::alloc::Layout;

use super::Allocator;

#[derive(Default, Copy, Clone)]
pub struct HostAllocator;

impl Allocator for HostAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::dealloc(ptr, layout);
    }

    unsafe fn copy(&self, src: *const u8, dst: *mut u8, size: usize) {
        std::ptr::copy_nonoverlapping(src, dst, size);
    }
}
