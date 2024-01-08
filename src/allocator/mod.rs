use std::alloc::Layout;

pub mod host;

pub trait Allocator: Default + Clone {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
    unsafe fn copy(&self, src: *const u8, dst: *mut u8, size: usize);
}

pub use host::HostAllocator;
