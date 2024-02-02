use crate::backend::{CpuBackend, MemOps};

impl MemOps for CpuBackend {
    unsafe fn copy<T>(&self, src: *const T, dst: *mut T, count: usize) {
        std::ptr::copy_nonoverlapping(src, dst, count);
    }

    // unsafe fn copy_discontinuous_unchecked<T>(
    //     &self,
    //     dims: &[usize],
    //     src: *const T,
    //     src_strides: &[usize],
    //     dst: *mut T,
    //     dst_stride: &[usize],
    // ) {
    //     // for i in 0..count {
    //     //     let src_ptr = src.add(i * src_stride);
    //     //     let dst_ptr = dst.add(i * dst_stride);
    //     //     std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, 1);
    //     // }
    // }

    unsafe fn fill<T>(&self, ptr: *mut T, value: T, count: usize) {
        let slice = std::slice::from_raw_parts_mut(ptr, count);
        slice.iter_mut().for_each(|x| *x = value);
    }
}
