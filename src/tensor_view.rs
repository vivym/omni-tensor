use std::ptr::NonNull;

use crate::{
    backend::Backend,
    dimension::Dimensions,
    storage::ViewStorage,
    tensor::TensorBase,
};

pub type TensorView<'a, T, B, D> = TensorBase<ViewStorage<&'a T, B>, D>;

pub type TensorViewMut<'a, T, B, D> = TensorBase<ViewStorage<&'a mut T, B>, D>;

impl<'a, T, B, D> TensorView<'a, T, B, D>
where
    B: Backend,
    D: Dimensions,
{
    #[inline(always)]
    pub fn new(ptr: NonNull<T>, dims: D, strides: D, backend: B) -> Self {
        Self {
            storage: ViewStorage::new(backend),
            ptr,
            dims,
            strides,
        }
    }
}

impl<'a, T, B, D> TensorViewMut<'a, T, B, D>
where
    B: Backend,
    D: Dimensions,
{
    pub fn new(ptr: NonNull<T>, dims: D, strides: D, backend: B) -> Self {
        TensorViewMut {
            storage: ViewStorage::new(backend),
            ptr,
            dims,
            strides,
        }
    }
}
