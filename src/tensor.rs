use std::ptr::NonNull;

use num_traits::{One, Zero};
use rawpointer::PointerExt;

use crate::{
    backend::Backend,
    dimension::{Dimensions, offset_from_low_addr_ptr_to_logical_ptr},
    shape_builder::ShapeBuilder,
    storage::{
        traits::{RawStorage, RawStorageClone, StorageOwned},
        OwnedStorage,
        OwnedArcStorage,
    },
};

pub struct TensorBase<S, D>
where
    S: RawStorage,
{
    pub(crate) storage: S,
    pub(crate) ptr: NonNull<S::Elem>,
    pub(crate) dims: D,
    pub(crate) strides: D,
}

type ArcTensor<T, B, D> = TensorBase<OwnedArcStorage<T, B>, D>;

impl<T, B, S, D> TensorBase<S, D>
where
    B: Backend,
    S: RawStorage<Elem = T, Backend = B>,
    D: Dimensions,
{
    pub fn is_contiguous(&self) -> bool {
        D::is_contiguous(&self.dims, &self.strides)
    }

    pub fn as_slice_memory_order(&self) -> Option<&[T]> {
        if self.is_contiguous() {
            let offset = offset_from_low_addr_ptr_to_logical_ptr(&self.dims, &self.strides);
            unsafe {
                Some(std::slice::from_raw_parts(
                    self.ptr.sub(offset).as_ptr(), self.dims.size()
                ))
            }
        } else {
            None
        }
    }

    pub fn to_owned(&self) -> TensorBase<OwnedStorage<T, B>, D> {
        if let Some(slice) = self.as_slice_memory_order() {
            let src_ptr = slice.as_ptr();
            let size = slice.len();
            let backend = self.storage.backend();
            let (mut storage, ptr) = OwnedStorage::empty(size, backend);
            unsafe { backend.copy(src_ptr, ptr.as_ptr(), size); }
            TensorBase {
                storage,
                ptr,
                dims: self.dims.clone(),
                strides: self.strides.clone(),
            }
        } else {
            unimplemented!("to_owned for non-contiguous tensors")
        }
    }

    pub fn into_shared(self) -> ArcTensor<T, B, D>
    where
        S: StorageOwned
    {
        let storage = self.storage.into_shared();

        TensorBase {
            storage,
            ptr: self.ptr,
            dims: self.dims,
            strides: self.strides,
        }
    }
}

impl<T, B, S, D> TensorBase<S, D>
where
    B: Backend,
    S: StorageOwned<Elem = T, Backend = B>,
    D: Dimensions,
{
    pub fn zeros<Sh>(shape: Sh) -> Self
    where
        T: Clone + Zero,
        Sh: ShapeBuilder<Dims = D>,
    {
        Self::from_elem(shape, T::zero())
    }

    pub fn ones<Sh>(shape: Sh) -> Self
    where
        T: Clone + One,
        Sh: ShapeBuilder<Dims = D>,
    {
        Self::from_elem(shape, T::one())
    }

    pub fn from_elem<Sh>(shape: Sh, elem: T) -> Self
    where
        T: Clone,
        Sh: ShapeBuilder<Dims = D>,
    {
        let shape = shape.into_shape();
        let dims = shape.dims;
        let strides = dims.default_strides();
        let size = shape.size();
        let (mut storage, ptr) = S::empty(size, S::Backend::default());
        storage.fill(elem);
        Self {
            storage,
            ptr,
            dims,
            strides,
        }
    }
}

impl<S, D> Clone for TensorBase<S, D>
where
    S: RawStorageClone,
    D: Dimensions,
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
    }
}
