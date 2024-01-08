use std::{
    marker::PhantomData,
    ptr::NonNull,
};

use crate::{allocator::Allocator, tensor::TensorBase, dimension::Dimensions, ops::Ops};
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
pub struct RawViewStorage<T, A> {
    ptr: PhantomData<T>,
    allocator: A,
}

#[derive(Copy, Clone)]
pub struct ViewStorage<T, A> {
    ptr: PhantomData<T>,
    allocator: A,
}

// impl<T, A> RawViewStorage<T, A> {
//     pub(crate) fn new(allocator: A) -> Self {
//         Self {
//             ptr: PhantomData,
//             allocator,
//         }
//     }
// }

impl<T, A> ViewStorage<T, A> {
    pub(crate) fn new(allocator: A) -> Self {
        Self {
            ptr: PhantomData,
            allocator,
        }
    }
}

unsafe impl<T, A> RawStorage for RawViewStorage<*const T, A>
where
    A: Allocator
{
    type Elem = T;
    type Allocator = A;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }
}

unsafe impl<T, A> RawStorageClone for RawViewStorage<*const T, A>
where
    A: Allocator
{
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (Self { ptr: self.ptr, allocator: self.allocator.clone() }, ptr)
    }

    unsafe fn to_owned_with_ptr(
        &self, _ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator> {
        // TODO: implement this
        unimplemented!("RawViewStorage::to_owned_with_ptr")
    }
}

unsafe impl<T, A> RawStorage for RawViewStorage<*mut T, A>
where
    A: Allocator
{
    type Elem = T;
    type Allocator = A;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }
}

unsafe impl<T, A> RawStorageClone for RawViewStorage<*mut T, A>
where
    A: Allocator
{
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (Self { ptr: self.ptr, allocator: self.allocator.clone() }, ptr)
    }

    unsafe fn to_owned_with_ptr(
        &self, _ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator> {
        // TODO: implement this
        unimplemented!("RawViewStorage::to_owned_with_ptr")
    }
}

unsafe impl<T, A> RawStorageMut for RawViewStorage<*mut T, A>
where
    A: Allocator
{
    #[inline]
    fn try_ensure_unique<D, O>(_: &mut TensorBase<Self, D, O>)
    where
        D: Dimensions,
        O: Ops,
    {}

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> { None }
}

unsafe impl<'a, T, A> RawStorage for ViewStorage<&'a T, A>
where
    A: Allocator
{
    type Elem = T;
    type Allocator = A;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }
}

unsafe impl<'a, T, A> RawStorageClone for ViewStorage<&'a T, A>
where
    A: Allocator
{
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (Self { ptr: self.ptr, allocator: self.allocator.clone() }, ptr)
    }

    unsafe fn to_owned_with_ptr(
        &self, _ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator> {
        // TODO: implement this
        unimplemented!("ViewStorage::to_owned_with_ptr")
    }
}

unsafe impl<'a, T, A> Storage for ViewStorage<&'a T, A>
where
    A: Allocator
{
    fn into_owned<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>
    where
        D: Dimensions,
        O: Ops
    {
        self_.to_owned()
    }

    fn try_into_owned_nocopy<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>, TensorBase<Self, D, O>>
    {
        Err(self_)
    }
}

unsafe impl<'a, T, A> RawStorage for ViewStorage<&'a mut T, A>
where
    A: Allocator
{
    type Elem = T;
    type Allocator = A;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool { true }
}

unsafe impl<'a, T, A> RawStorageClone for ViewStorage<&'a mut T, A>
where
    A: Allocator
{
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (Self { ptr: self.ptr, allocator: self.allocator.clone() }, ptr)
    }

    unsafe fn to_owned_with_ptr(
        &self, _ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator> {
        // TODO: implement this
        unimplemented!("ViewStorage::to_owned_with_ptr")
    }
}

unsafe impl<'a, T, A> RawStorageMut for ViewStorage<&'a mut T, A>
where
    A: Allocator
{
    #[inline]
    fn try_ensure_unique<D, O>(_: &mut TensorBase<Self, D, O>)
    where
        D: Dimensions,
        O: Ops,
    {}

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> { Some(true) }
}

unsafe impl<'a, T, A> Storage for ViewStorage<&'a mut T, A>
where
    A: Allocator
{
    fn into_owned<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>
    where
        D: Dimensions,
        O: Ops
    {
        self_.to_owned()
    }

    fn try_into_owned_nocopy<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>, TensorBase<Self, D, O>>
    {
        Err(self_)
    }
}

unsafe impl<'a, T, A> StorageMut for ViewStorage<&'a mut T, A>
where
    A: Allocator
{}
