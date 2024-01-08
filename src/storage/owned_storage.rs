use std::{ptr::NonNull, sync::Arc};

use rawpointer::PointerExt;

use crate::{allocator::Allocator, tensor::TensorBase, dimension::Dimensions, ops::Ops};
use super::traits::{
    RawStorage,
    RawStorageMut,
    RawStorageClone,
    Storage,
    StorageMut,
    StorageOwned,
    StorageShared,
};

pub struct OwnedStorage<T, A: Allocator> {
    ptr: NonNull<T>,
    size: usize,
    capacity: usize,
    allocator: A,
}

impl<T, A> OwnedStorage<T, A> where A: Allocator {
    pub(crate) fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.size)
        }
    }

    pub(crate) fn as_ptr(&self) -> *const T {
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
            self.allocator.dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
        self.size = 0;
        self.capacity = 0;
    }
}

impl<T, A> Clone for OwnedStorage<T, A> where T: Clone, A: Allocator {
    fn clone(&self) -> Self {
        let ptr = unsafe {
            let layout = std::alloc::Layout::array::<T>(self.size).unwrap();
            let ptr = self.allocator.alloc(layout) as *mut T;
            self.allocator.copy(self.as_ptr() as _, ptr as _, layout.size());
            NonNull::new_unchecked(ptr)
        };
        Self {
            ptr,
            size: self.size,
            capacity: self.size,
            allocator: self.allocator.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        if self.capacity >= other.capacity {
            unsafe {
                self.allocator.copy(
                    other.as_ptr() as _, self.as_ptr() as _, self.size
                );
            }
        } else {
            self.release_memory();

            let ptr = unsafe {
                let layout = std::alloc::Layout::array::<T>(other.size).unwrap();
                let ptr = self.allocator.alloc(layout) as *mut T;
                self.allocator.copy(other.as_ptr() as _, ptr as _, layout.size());
                NonNull::new_unchecked(ptr)
            };
            self.ptr = ptr;
            self.capacity = other.size;
        }
        self.size = other.size;
        self.allocator = other.allocator.clone();
    }
}

impl<T, A> Drop for OwnedStorage<T, A> where A: Allocator {
    fn drop(&mut self) {
        self.release_memory();
    }
}


unsafe impl<T, A> RawStorage for OwnedStorage<T, A> where A: Allocator {
    type Elem = T;
    type Allocator = A;

    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool {
        let slice = self.as_slice();
        let self_ptr_start = slice.as_ptr();
        let self_ptr_end = unsafe { self_ptr_start.add(slice.len()) };
        self_ptr_start <= ptr && ptr <= self_ptr_end
    }
}

unsafe impl<T, A> RawStorageClone for OwnedStorage<T, A> where T: Clone, A: Allocator {
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

    unsafe fn to_owned_with_ptr(
        &self, ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator>
    {
        let allocator = self.allocator.clone();
        let elem_size = std::mem::size_of::<T>();
        if elem_size > 0 {
            let ptr_end = self.as_ptr().add(self.size);
            let num_elems = (ptr_end as usize - ptr.as_ptr() as usize) / elem_size;
            let storage = OwnedStorage::empty(num_elems, allocator);
            self.allocator.copy(ptr.as_ptr() as _, storage.ptr.as_ptr() as _, num_elems);

            storage
        } else {
            OwnedStorage::empty(0, allocator)
        }
    }
}

unsafe impl<T, A> RawStorageMut for OwnedStorage<T, A> where T: Clone, A: Allocator {
    #[inline]
    fn try_ensure_unique<D, O>(_: &mut TensorBase<Self, D, O>)
    where
        D: Dimensions
    {}

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> {
        Some(true)
    }
}

unsafe impl<T, A> Storage for OwnedStorage<T, A>
where
    T: Clone,
    A: Allocator,
{
    #[inline]
    fn into_owned<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>
    {
        self_
    }

    #[inline]
    fn try_into_owned_nocopy<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>, TensorBase<Self, D, O>> {
        Ok(self_)
    }
}

unsafe impl<T, A> StorageMut for OwnedStorage<T, A> where T: Clone, A: Allocator {}

unsafe impl<T, A> StorageOwned for OwnedStorage<T, A>
where
    T: Clone,
    A: Allocator,
{
    fn empty(size: usize, allocator: Self::Allocator) -> Self {
        let capacity = size;
        let ptr = unsafe {
            let layout = std::alloc::Layout::array::<T>(size).unwrap();
            NonNull::new_unchecked(allocator.alloc(layout) as *mut T)
        };
        Self {
            ptr,
            size,
            capacity,
            allocator,
        }
    }

    fn into_shared(self) -> OwnedArcStorage<T, A> {
        OwnedArcStorage(Arc::new(self))
    }
}

pub struct OwnedArcStorage<T, A: Allocator>(Arc<OwnedStorage<T, A>>);

impl<T, A> Clone for OwnedArcStorage<T, A> where A: Allocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

unsafe impl<T, A> RawStorage for OwnedArcStorage<T, A> where A: Allocator {
    type Elem = T;
    type Allocator = A;

    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool {
        self.0._is_pointer_inbounds(ptr)
    }
}

unsafe impl<T, A> RawStorageClone for OwnedArcStorage<T, A>  where T: Clone, A: Allocator {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (self.clone(), ptr)
    }

    unsafe fn to_owned_with_ptr(
        &self, ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator>
    {
        self.0.to_owned_with_ptr(ptr)
    }
}

unsafe impl<T, A> RawStorageMut for OwnedArcStorage<T, A>  where T: Clone, A: Allocator {
    #[inline]
    fn try_ensure_unique<D, O>(self_: &mut TensorBase<Self, D, O>)
    where
        D: Dimensions,
        O: Ops,
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

unsafe impl<T, A> Storage for OwnedArcStorage<T, A>
where
    T: Clone,
    A: Allocator,
{
    #[inline]
    fn into_owned<D, O>(
        mut self_: TensorBase<Self, D, O>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>
    where
        D: Dimensions,
        O: Ops,
    {
        Self::ensure_unique(&mut self_);
        let data = Arc::try_unwrap(self_.storage.0).ok().unwrap();
        TensorBase {
            storage: data,
            ptr: self_.ptr,
            dims: self_.dims,
            strides: self_.strides,
            ops: self_.ops,
        }
    }

    #[inline]
    fn try_into_owned_nocopy<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>, TensorBase<Self, D, O>> {
        match Arc::try_unwrap(self_.storage.0) {
            Ok(storage) => Ok(TensorBase {
                storage,
                ptr: self_.ptr,
                dims: self_.dims,
                strides: self_.strides,
                ops: self_.ops,
            }),
            Err(storage) => Err(TensorBase {
                storage: OwnedArcStorage(storage),
                ptr: self_.ptr,
                dims: self_.dims,
                strides: self_.strides,
                ops: self_.ops,
            }),
        }
    }

    #[inline]
    fn to_shared<D, O>(
        self_: &TensorBase<Self, D, O>
    ) -> TensorBase<OwnedArcStorage<Self::Elem, Self::Allocator>, D, O>
    where
        D: Dimensions,
        O: Ops
    {
        // to shared using clone of OwnedArcStorage without clone of raw data.
        self_.clone()
    }
}

unsafe impl<T, A> StorageMut for OwnedArcStorage<T, A>
where
    T: Clone,
    A: Allocator,
{}

unsafe impl<T, A> StorageShared for OwnedArcStorage<T, A>
where
    T: Clone,
    A: Allocator,
{}
