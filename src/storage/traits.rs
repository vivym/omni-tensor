use std::ptr::NonNull;

use crate::{dimension::Dimensions, tensor::TensorBase, backend::Backend};
use super::{OwnedStorage, OwnedArcStorage};

pub unsafe trait RawStorage: Sized {
    type Elem;
    type Backend: Backend;

    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool;

    fn backend(&self) -> Self::Backend;
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

    // unsafe fn to_owned_with_ptr(
    //     &self, ptr: NonNull<Self::Elem>
    // )-> OwnedStorage<Self::Elem, Self::Backend>;
}

pub unsafe trait RawStorageMut: RawStorageClone {
    fn try_ensure_unique<D>(self_: &mut TensorBase<Self, D>)
    where
        D: Dimensions;

    fn try_is_unique(&mut self) -> Option<bool>;
}

pub unsafe trait Storage: RawStorageClone {
    fn into_owned<D>(
        self_: TensorBase<Self, D>
    ) -> TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>
    where
        D: Dimensions;

    fn try_into_owned_nocopy<D>(
        self_: TensorBase<Self, D>
    ) -> Result<TensorBase<OwnedStorage<Self::Elem, Self::Backend>, D>, TensorBase<Self, D>>;

    fn to_shared<D>(
        self_: &TensorBase<Self, D>
    ) -> TensorBase<OwnedArcStorage<Self::Elem, Self::Backend>, D>
    where
        Self::Elem: Clone,
        D: Dimensions,
    {
        self_.to_owned().into_shared()
    }
}

pub unsafe trait StorageMut: Storage + RawStorageMut {
    // ensure_unique
    fn ensure_unique<D>(self_: &mut TensorBase<Self, D>)
    where
        D: Dimensions,
    {
        Self::try_ensure_unique(self_)
    }

    fn is_unique(&mut self) -> bool {
        self.try_is_unique().unwrap()
    }
}

pub unsafe trait StorageOwned: Storage {
    fn empty(size: usize, backend: Self::Backend) -> (Self, NonNull<Self::Elem>);

    unsafe fn from_raw_ptr(ptr: NonNull<Self::Elem>, size: usize, backend: Self::Backend) -> Self;

    fn fill(&mut self, value: Self::Elem);

    fn into_shared(self) -> OwnedArcStorage<Self::Elem, Self::Backend>;
}

pub unsafe trait StorageShared: Storage + RawStorageClone + Clone {}
