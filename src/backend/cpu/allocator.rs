use crate::backend::{Allocator, CpuBackend};

impl Allocator for CpuBackend {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        std::alloc::dealloc(ptr, layout);
    }
}

// impl<T, B> OwnedStorage<T, B> where B: BackendCpu {
//     pub(crate) fn from_vec(v: Vec<T>) -> Self {
//         let mut v = ManuallyDrop::new(v);
//         let len = v.len();
//         let capacity = v.capacity();
//         let ptr = unsafe { NonNull::new_unchecked(v.as_mut_ptr()) };
//         Self {
//             ptr,
//             size: len,
//             capacity,
//             backend: PhantomData,
//         }
//     }

//     pub(crate) fn take_as_vec(&mut self) -> Vec<T> {
//         let capacity = self.capacity;
//         let len = self.size;
//         self.size = 0;
//         self.capacity = 0;
//         unsafe {
//             Vec::from_raw_parts(self.ptr.as_ptr(), len, capacity)
//         }
//     }
// }
