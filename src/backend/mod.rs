use std::alloc::Layout;

pub mod cpu;

pub enum BackendKind {
    Cpu,
    Cuda,
}

pub trait Backend: Allocator + MemOps + Copy + Clone + Default {
    const KIND: BackendKind;
}

#[derive(Default, Copy, Clone)]
pub struct CpuBackend;

impl Backend for CpuBackend {
    const KIND: BackendKind = BackendKind::Cpu;
}

// #[derive(Default, Copy, Clone)]
// pub struct CudaBackend;

// impl Backend for CudaBackend {
//     const KIND: BackendKind = BackendKind::Cuda;
// }

pub trait Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
}

pub trait MemOps {
    unsafe fn copy<T>(&self, src: *const T, dst: *mut T, count: usize);

    unsafe fn fill<T>(&self, ptr: *mut T, value: T, count: usize);
}
