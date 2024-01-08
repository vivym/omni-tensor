use std::ptr::NonNull;

use crate::{dimension::Dimensions, tensor::TensorBase, allocator::Allocator, ops::Ops};
use super::{OwnedStorage, OwnedArcStorage};

pub unsafe trait RawStorage: Sized {
    type Elem;
    type Allocator: Allocator;

    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool;
}

pub unsafe trait RawStorageClone: RawStorage {
    /// Safety: `ptr` must point inside the current storage.
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>);

    unsafe fn clone_from_with_ptr(
        &mut self, other: &Self, ptr: NonNull<Self::Elem>
    ) -> NonNull<Self::Elem> {
        let (data, ptr) = other.clone_with_ptr(ptr);
        *self = data;
        ptr
    }

    unsafe fn to_owned_with_ptr(
        &self, ptr: NonNull<Self::Elem>
    )-> OwnedStorage<Self::Elem, Self::Allocator>;
}

pub unsafe trait RawStorageMut: RawStorageClone {
    fn try_ensure_unique<D, O>(self_: &mut TensorBase<Self, D, O>)
    where
        D: Dimensions,
        O: Ops;

    fn try_is_unique(&mut self) -> Option<bool>;
}

pub unsafe trait Storage: RawStorageClone {
    fn into_owned<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>
    where
        D: Dimensions,
        O: Ops;

    fn try_into_owned_nocopy<D, O>(
        self_: TensorBase<Self, D, O>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Allocator>, D, O>, TensorBase<Self, D, O>>;

    fn to_shared<D, O>(
        self_: &TensorBase<Self, D, O>
    ) -> TensorBase<OwnedArcStorage<Self::Elem, Self::Allocator>, D, O>
    where
        Self::Elem: Clone,
        D: Dimensions,
        O: Ops
    {
        self_.to_owned().into_shared()
    }
}

pub unsafe trait StorageMut: Storage + RawStorageMut {
    // ensure_unique
    fn ensure_unique<D, O>(self_: &mut TensorBase<Self, D, O>)
    where
        D: Dimensions,
        O: Ops,
    {
        Self::try_ensure_unique(self_)
    }

    fn is_unique(&mut self) -> bool {
        self.try_is_unique().unwrap()
    }
}

pub unsafe trait StorageOwned: Storage {
    fn empty(size: usize, allocator: Self::Allocator) -> Self;

    fn into_shared(self) -> OwnedArcStorage<Self::Elem, Self::Allocator>;
}

pub unsafe trait StorageShared: Storage + RawStorageClone + Clone {}
