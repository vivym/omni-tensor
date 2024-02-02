use std::{
    marker::PhantomData,
    ptr::NonNull,
};

use crate::{tensor::TensorBase, dimension::Dimensions, backend::Backend};
use super::{
    OwnedStorage,
    traits::{
        RawStorage,
        RawStorageMut,
        RawStorageClone,
        Storage,
        StorageMut,
    },
};

#[derive(Copy, Clone)]
pub struct RawViewStorage<T, B> {
    ptr: PhantomData<T>,
    backend: B,
}

#[derive(Copy, Clone)]
pub struct ViewStorage<T, B> {
    ptr: PhantomData<T>,
    backend: B,
}

// impl<T, A> RawViewStorage<T, A> {
//     pub(crate) fn new() -> Self {
//         Self {
//             ptr: PhantomData,
//             allocator,
//         }
//     }
// }

impl<T, B> ViewStorage<T, B> {
    pub(crate) fn new(backend: B) -> Self {
        Self {
            ptr: PhantomData,
            backend,
        }
    }
}

unsafe impl<T, B> RawStorage for RawViewStorage<*const T, B> where B: Backend {
    type Elem = T;
    type Backend = B;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }

    fn backend(&self) -> Self::Backend {
        self.backend
    }
}

unsafe impl<T, B> RawStorageClone for RawViewStorage<*const T, B> where B: Backend {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (*self, ptr)
    }
}

unsafe impl<T, B> RawStorage for RawViewStorage<*mut T, B> where B: Backend {
    type Elem = T;
    type Backend = B;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }

    fn backend(&self) -> Self::Backend {
        self.backend
    }
}

unsafe impl<T, B> RawStorageClone for RawViewStorage<*mut T, B> where B: Backend {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (*self, ptr)
    }
}

unsafe impl<T, B> RawStorageMut for RawViewStorage<*mut T, B> where B: Backend {
    #[inline]
    fn try_ensure_unique<D>(_: &mut TensorBase<Self, D>)
    where
        D: Dimensions
    {}

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> { None }
}

unsafe impl<'a, T, B> RawStorage for ViewStorage<&'a T, B> where B: Backend {
    type Elem = T;
    type Backend = B;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }

    fn backend(&self) -> Self::Backend {
        self.backend
    }
}

unsafe impl<'a, T, B> RawStorageClone for ViewStorage<&'a T, B> where B: Backend {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (*self, ptr)
    }
}

unsafe impl<'a, T, B> Storage for ViewStorage<&'a T, B> where B: Backend {
    fn into_owned<D>(
        self_: TensorBase<Self, D>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>
    where
        D: Dimensions
    {
        self_.to_owned()
    }

    fn try_into_owned_nocopy<D>(
        self_: TensorBase<Self, D>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>, TensorBase<Self, D>> {
        Err(self_)
    }
}

unsafe impl<'a, T, B> RawStorage for ViewStorage<&'a mut T, B> where B: Backend {
    type Elem = T;
    type Backend = B;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }

    fn backend(&self) -> Self::Backend {
        self.backend
    }
}

unsafe impl<'a, T, B> RawStorageClone for ViewStorage<&'a mut T, B> where B: Backend {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (*self, ptr)
    }
}

unsafe impl<'a, T, B> RawStorageMut for ViewStorage<&'a mut T, B> where B: Backend {
    #[inline]
    fn try_ensure_unique<D>(_: &mut TensorBase<Self, D>)
    where
        D: Dimensions
    {}

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> { Some(true) }
}

unsafe impl<'a, T, B> Storage for ViewStorage<&'a mut T, B> where B: Backend {
    fn into_owned<D>(
        self_: TensorBase<Self, D>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>
    where
        D: Dimensions
    {
        self_.to_owned()
    }

    fn try_into_owned_nocopy<D>(
        self_: TensorBase<Self, D>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>, TensorBase<Self, D>> {
        Err(self_)
    }
}

unsafe impl<'a, T, B> StorageMut for ViewStorage<&'a mut T, B> where B: Backend {}
