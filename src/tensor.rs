use std::ptr::NonNull;

use crate::{
    allocator::Allocator,
    dimension::Dimensions,
    ops::Ops,
    storage::{
        traits::{RawStorage, RawStorageClone, StorageOwned},
        OwnedStorage,
        OwnedArcStorage,
    },
};

pub struct TensorBase<S, D, O>
where
    S: RawStorage,
{
    pub(crate) storage: S,
    pub(crate) ptr: NonNull<S::Elem>,
    pub(crate) dims: D,
    pub(crate) strides: D,
    pub(crate) ops: O,
}

type ArcTensor<T, A, D, O> = TensorBase<OwnedArcStorage<T, A>, D, O>;

impl<T, A, S, D, O> TensorBase<S, D, O>
where
    A: Allocator,
    S: RawStorage<Elem = T, Allocator = A>,
    D: Dimensions,
    O: Ops,
{
    pub fn to_owned(&self) -> TensorBase<OwnedStorage<T, A>, D, O>
    where
        S: RawStorageClone,
    {
        let mut storage = unsafe {
            self.storage.to_owned_with_ptr(self.ptr)
        };
        let ptr = storage.as_nonnull_mut();

        TensorBase {
            storage,
            ptr,
            dims: self.dims.clone(),
            strides: self.strides.clone(),
            ops: self.ops.clone(),
        }
    }

    pub fn into_shared(self) -> ArcTensor<T, A, D, O>
    where
        S: StorageOwned
    {
        let storage = self.storage.into_shared();

        TensorBase {
            storage,
            ptr: self.ptr,
            dims: self.dims,
            strides: self.strides,
            ops: self.ops,
        }
    }
}

impl<S, D, O> Clone for TensorBase<S, D, O>
where
    S: RawStorageClone,
    D: Dimensions,
    O: Ops,
{
    fn clone(&self) -> Self {
        let (storage, ptr) = unsafe {
            self.storage.clone_with_ptr(self.ptr)
        };
        TensorBase {
            storage,
            ptr,
            dims: self.dims.clone(),
            strides: self.strides.clone(),
            ops: self.ops.clone(),
        }
    }

    /// `Tensor` implements `.clone_from()` to reuse an tensor's existing
    /// allocation. Semantically equivalent to `*self = other.clone()`, but
    /// potentially more efficient.
    fn clone_from(&mut self, other: &Self) {
        self.ptr = unsafe {
            self.storage.clone_from_with_ptr(&other.storage, self.ptr)
        };
        self.dims.clone_from(&other.dims);
        self.strides.clone_from(&other.strides);
        self.ops.clone_from(&other.ops);
    }
}
