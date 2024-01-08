use std::ptr::NonNull;

use crate::{
    allocator::Allocator,
    dimension::Dimensions,
    ops::Ops,
    storage::ViewStorage,
    tensor::TensorBase,
};

pub type TensorView<'a, T, A, D, O> = TensorBase<ViewStorage<&'a T, A>, D, O>;

pub type TensorViewMut<'a, T, A, D, O> = TensorBase<ViewStorage<&'a mut T, A>, D, O>;

impl<'a, T, A, D, O> TensorView<'a, T, A, D, O>
where
    A: Allocator,
    D: Dimensions,
    O: Ops,
{
    #[inline(always)]
    pub fn new(ptr: NonNull<T>, allocator: A, dims: D, strides: D, ops: O) -> Self {
        TensorView {
            storage: ViewStorage::new(allocator),
            ptr,
            dims,
            strides,
            ops,
        }
    }
}

impl<'a, T, A, D, O> TensorViewMut<'a, T, A, D, O>
where
A: Allocator,
D: Dimensions,
    O: Ops,
{
    pub fn new(ptr: NonNull<T>, allocator: A, dims: D, strides: D, ops: O) -> Self {
        TensorViewMut {
            storage: ViewStorage::new(allocator),
            ptr,
            dims,
            strides,
            ops,
        }
    }
}
