use std::{ptr::NonNull, sync::Arc};

use rawpointer::PointerExt;

use crate::{
    backend::Backend,
    tensor::TensorBase,
    dimension::Dimensions,
};
use super::traits::{
    RawStorage,
    RawStorageMut,
    RawStorageClone,
    Storage,
    StorageMut,
    StorageOwned,
    StorageShared,
};

pub struct OwnedStorage<T, B> where B: Backend {
    pub(crate) ptr: NonNull<T>,
    pub(crate) size: usize,
    pub(crate) capacity: usize,
    pub(crate) backend: B,
}

impl<T, B> OwnedStorage<T, B> where B: Backend {
    pub(crate) fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.size)
        }
    }

    pub(crate) fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub(crate) fn as_nonnull_mut(&mut self) -> NonNull<T> {
        self.ptr
    }

    pub(crate) fn release_memory(&mut self) {
        if self.capacity == 0 {
            return;
        }
        unsafe {
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            self.backend.dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
        self.size = 0;
        self.capacity = 0;
    }
}

impl<T, B> Clone for OwnedStorage<T, B> where B: Backend {
    fn clone(&self) -> Self {
        let ptr = unsafe {
            let layout = std::alloc::Layout::array::<T>(self.size).unwrap();
            let ptr = self.backend.alloc(layout) as *mut T;
            self.backend.copy(self.as_ptr() as _, ptr as _, layout.size());
            NonNull::new_unchecked(ptr)
        };
        Self {
            ptr,
            size: self.size,
            capacity: self.size,
            backend: self.backend,
        }
    }

    fn clone_from(&mut self, other: &Self) {
        if self.capacity >= other.capacity {
            unsafe {
                self.backend.copy(
                    other.as_ptr() as _, self.as_ptr() as _, self.size
                );
            }
        } else {
            self.release_memory();

            let ptr = unsafe {
                let layout = std::alloc::Layout::array::<T>(other.size).unwrap();
                let ptr = self.backend.alloc(layout) as *mut T;
                self.backend.copy(other.as_ptr() as _, ptr as _, layout.size());
                NonNull::new_unchecked(ptr)
            };
            self.ptr = ptr;
            self.capacity = other.size;
        }
        self.size = other.size;
        self.backend = other.backend;
    }
}

impl<T, B> Drop for OwnedStorage<T, B> where B: Backend {
    fn drop(&mut self) {
        self.release_memory();
    }
}

unsafe impl<T, B> RawStorage for OwnedStorage<T, B> where B: Backend {
    type Elem = T;
    type Backend = B;

    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool {
        let slice = self.as_slice();
        let self_ptr_start = slice.as_ptr();
        let self_ptr_end = unsafe { self_ptr_start.add(slice.len()) };
        self_ptr_start <= ptr && ptr <= self_ptr_end
    }

    fn backend(&self) -> Self::Backend {
        self.backend
    }
}

unsafe impl<T, B> RawStorageClone for OwnedStorage<T, B> where B: Backend {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        let mut storage = self.clone();
        let mut new_ptr = storage.as_nonnull_mut();
        let elem_size = std::mem::size_of::<T>();
        if elem_size != 0 {
            let offset =
                (ptr.as_ptr() as isize - self.as_ptr() as isize) / elem_size as isize;
            new_ptr = new_ptr.offset(offset);
        }
        (storage, new_ptr)
    }

    unsafe fn clone_from_with_ptr(
        &mut self, other: &Self, ptr: NonNull<Self::Elem>
    ) -> NonNull<Self::Elem> {
        let elem_size = std::mem::size_of::<T>();
        let offset = if elem_size != 0 {
            (ptr.as_ptr() as isize - other.as_ptr() as isize) / elem_size as isize
        } else {
            0
        };
        self.clone_from(other);
        self.as_nonnull_mut().offset(offset)
    }
}

unsafe impl<T, B> RawStorageMut for OwnedStorage<T, B> where B: Backend {
    #[inline]
    fn try_ensure_unique<D>(_: &mut TensorBase<Self, D>)
    where
        D: Dimensions
    {}

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> {
        Some(true)
    }
}

unsafe impl<T, B> Storage for OwnedStorage<T, B>
where
    B: Backend
{
    #[inline]
    fn into_owned<D>(
        self_: TensorBase<Self, D>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D> {
        self_
    }

    #[inline]
    fn try_into_owned_nocopy<D>(
        self_: TensorBase<Self, D>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>, TensorBase<Self, D>> {
        Ok(self_)
    }
}

unsafe impl<T, B> StorageMut for OwnedStorage<T, B> where B: Backend {}

unsafe impl<T, B> StorageOwned for OwnedStorage<T, B>
where
    B: Backend
{
    fn empty(size: usize, backend: Self::Backend) -> (Self, NonNull<T>) {
        let capacity = size;
        let ptr = unsafe {
            let layout = std::alloc::Layout::array::<T>(size).unwrap();
            NonNull::new_unchecked(backend.alloc(layout) as *mut T)
        };
        (Self {
            ptr,
            size,
            capacity,
            backend,
        }, ptr)
    }

    unsafe fn from_raw_ptr(
        ptr: NonNull<T>, size: usize, backend: Self::Backend
    ) -> Self {
        let capacity = size;
        Self {
            ptr,
            size,
            capacity,
            backend,
        }
    }

    fn fill(&mut self, value: Self::Elem) {
        unsafe {
            self.backend.fill(self.as_mut_ptr(), value, self.size);
        }
    }

    fn into_shared(self) -> OwnedArcStorage<T, B> {
        OwnedArcStorage(Arc::new(self))
    }
}

pub struct OwnedArcStorage<T, B>(Arc<OwnedStorage<T, B>>) where B: Backend;

impl<T, B> Clone for OwnedArcStorage<T, B> where B: Backend {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

unsafe impl<T, B> RawStorage for OwnedArcStorage<T, B> where B: Backend {
    type Elem = T;
    type Backend = B;

    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool {
        self.0._is_pointer_inbounds(ptr)
    }

    fn backend(&self) -> Self::Backend {
        self.0.backend
    }
}

unsafe impl<T, B> RawStorageClone for OwnedArcStorage<T, B>  where B: Backend {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (self.clone(), ptr)
    }
}

unsafe impl<T, B> RawStorageMut for OwnedArcStorage<T, B>  where B: Backend {
    #[inline]
    fn try_ensure_unique<D>(self_: &mut TensorBase<Self, D>)
    where
        D: Dimensions
    {
        if Arc::get_mut(&mut self_.storage.0).is_some() {
            return;
        }
        if self_.dims.size() <= self_.storage.0.size / 2 {
            // Clone only the visible elements if the current view is less than
            // half of backing data.
            *self_ = self_.to_owned().into_shared();
        } else {
            let elem_size = std::mem::size_of::<T>() as isize;
            let offset = if elem_size != 0 {
                (self_.ptr.as_ptr() as isize - self_.storage.0.as_ptr() as isize) / elem_size
            } else {
                0
            };

            let rcvec = &mut self_.storage.0;
            let rvec = Arc::make_mut(rcvec);
            unsafe {
                self_.ptr = rvec.as_nonnull_mut().offset(offset);
            }
        }
    }

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> {
        Some(Arc::get_mut(&mut self.0).is_some())
    }
}

unsafe impl<T, B> Storage for OwnedArcStorage<T, B>
where
    B: Backend
{
    #[inline]
    fn into_owned<D>(
        mut self_: TensorBase<Self, D>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>
    where
        D: Dimensions
    {
        Self::ensure_unique(&mut self_);
        let storage = Arc::try_unwrap(self_.storage.0).ok().unwrap();
        TensorBase {
            storage,
            ptr: self_.ptr,
            dims: self_.dims,
            strides: self_.strides,
        }
    }

    #[inline]
    fn try_into_owned_nocopy<D>(
        self_: TensorBase<Self, D>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>, TensorBase<Self, D>> {
        match Arc::try_unwrap(self_.storage.0) {
            Ok(storage) => Ok(TensorBase {
                storage,
                ptr: self_.ptr,
                dims: self_.dims,
                strides: self_.strides,
            }),
            Err(storage) => Err(TensorBase {
                storage: OwnedArcStorage(storage),
                ptr: self_.ptr,
                dims: self_.dims,
                strides: self_.strides,
            }),
        }
    }

    #[inline]
    fn to_shared<D>(
        self_: &TensorBase<Self, D>
    ) -> TensorBase<OwnedArcStorage<Self::Elem, Self::Backend>, D>
    where
        D: Dimensions
    {
        // to shared using clone of OwnedArcStorage without clone of raw data.
        self_.clone()
    }
}

unsafe impl<T, B> StorageMut for OwnedArcStorage<T, B>
where
    B: Backend
{}

unsafe impl<T, B> StorageShared for OwnedArcStorage<T, B>
where
    B: Backend
{}
