use std::{ops::Add, ptr::NonNull};

use crate::{
    allocator::{HostAllocator, Allocator},
    dimension::{
        DimsMax,
        DimsMaxOf,
        Dimensions,
        Dims0,
        Dims1,
        Dims2,
        Dims3,
        Dims4,
        Dims5,
        Dims6,
        Dims7,
        Dims8,
    },
    elem::Elem,
    error::{ShapeError, OmniResult},
    ops::{Ops, HostOps},
    storage::{Storage, StorageOwned, OwnedStorage, StorageMut},
    tensor_view::{TensorView, TensorViewMut},
};

pub struct TensorBase<S: Storage, D: Dimensions, O: Ops> {
    storage: S,
    shape: D,
    strides: D,
    ops: O,
}

impl<T, A, D, O> TensorBase<OwnedStorage<T, A>, D, O>
where
    T: Elem,
    A: Allocator,
    D: Dimensions,
    O: Ops,
{
    pub fn empty(shape: D) -> Self {
        let strides = shape.default_strides();
        let storage = OwnedStorage::empty(shape.size(), A::default());
        let ops = O::default();
        Self {
            storage,
            shape,
            strides,
            ops,
        }
    }

    // pub fn fill(&mut self, value: S::Elem) {
    //     self.ops._fill(
    //         value,
    //         unsafe { self.storage.as_mut_slice() },
    //     );
    // }

    pub fn shape(&self) -> &D {
        &self.shape
    }

    pub fn strides(&self) -> &D {
        &self.strides
    }

    pub fn ndim(&self) -> usize {
        self.shape.ndim()
    }

    pub unsafe fn as_ptr(&self) -> *const T {
        self.storage.as_ptr()
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        self.storage.as_mut_ptr()
    }

    // pub fn into_dimensionality<D2>(self) -> OmniResult<TensorBase<S, D2, O>>
    // where
    //     D2: Dimensions
    // {
    //     if D::NDIM == D2::NDIM {
    //         // safe because D == D2
    //         let shape = unsafe { unlimited_transmute::<D, D2>(self.shape) };
    //         let strides = unsafe { unlimited_transmute::<D, D2>(self.strides) };
    //     } else if D::NDIM == None || D2::NDIM == None {
    //         // safe because D::NDIM == None || D2::NDIM == None
    //         let shape = unsafe { unlimited_transmute::<D, D2>(self.shape) };
    //         let strides = unsafe { unlimited_transmute::<D, D2>(self.strides) };
    //     }
    //     return Err(ShapeError::IncompatibleShape.into())
    // }
}

impl<T, S, D, O> TensorBase<S, D, O>
where
    T: Elem,
    S: Storage<Elem = T>,
    D: Dimensions,
    O: Ops,
{
    pub(crate) fn from_storage(storage: S, shape: D, strides: D, ops: O) -> Self {
        Self {
            storage,
            shape,
            strides,
            ops,
        }
    }

    pub fn view(&self) -> TensorView<'_, T, D, O> {
        let ptr = unsafe { self.storage.get_ptr() };
        TensorView::new(
            ptr,
            self.shape.clone(),
            self.strides.clone(),
            self.ops.clone(),
        )
    }
}

impl<T, S, D, O> TensorBase<S, D, O>
where
    T: Elem,
    S: StorageMut<Elem = T>,
    D: Dimensions,
    O: Ops,
{
    pub fn view_mut(&mut self) -> TensorViewMut<'_, T, D, O> {
        let ptr = unsafe {
            self.storage.ensure_unique();
            self.storage.get_ptr()
        };
        TensorViewMut::new(
            ptr,
            self.shape.clone(),
            self.strides.clone(),
            self.ops.clone(),
        )
    }
}

impl<A, B, S, S2, D, D2, O> Add<TensorBase<S2, D2, O>> for TensorBase<S, D, O>
where
    A: Elem + Add<B, Output = A>,
    B: Elem,
    S: StorageOwned<Elem = A> + StorageMut,
    S2: Storage<Elem = B>,
    D: Dimensions + DimsMax<D2>,
    D2: Dimensions,
    O: Ops,
{
    type Output = TensorBase<S, DimsMaxOf<D, D2>, O>;

    fn add(self, rhs: TensorBase<S2, D2, O>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<A, B, S, S2, D, D2, O> Add<&TensorBase<S2, D2, O>> for TensorBase<S, D, O>
where
    A: Elem + Add<B, Output = A>,
    B: Elem,
    S: StorageOwned<Elem = A> + StorageMut,
    S2: Storage<Elem = B>,
    D: Dimensions + DimsMax<D2>,
    D2: Dimensions,
    O: Ops,
{
    type Output = TensorBase<S, DimsMaxOf<D, D2>, O>;

    fn add(self, _rhs: &TensorBase<S2, D2, O>) -> Self::Output {
        // let out = self.into_dimensionality().unwrap();
        // TODO:
        // out
        unimplemented!()
    }
}

impl<A, B, S, S2, D, D2, O> Add<TensorBase<S2, D2, O>> for &TensorBase<S, D, O>
where
    A: Elem + Add<B, Output = A>,
    B: Elem,
    S: Storage<Elem = A>,
    S2: StorageOwned<Elem = B> + StorageMut,
    D: Dimensions,
    D2: Dimensions + DimsMax<D>,
    O: Ops,
{
    type Output = TensorBase<S, DimsMaxOf<D2, D>, O>;

    fn add(self, _rhs: TensorBase<S2, D2, O>) -> Self::Output {
        // let out = self.into_dimensionality().unwrap();
        // TODO:
        // out
        unimplemented!()
    }
}

impl<A, B, S, S2, D, D2, O> Add<&TensorBase<S2, D2, O>> for &TensorBase<S, D, O>
where
    A: Elem + Add<B, Output = A>,
    B: Elem,
    S: Storage<Elem = A>,
    S2: Storage<Elem = B>,
    D: Dimensions + DimsMax<D2>,
    D2: Dimensions,
    O: Ops,
{
    type Output = TensorBase<S, DimsMaxOf<D, D2>, O>;

    fn add(self, _rhs: &TensorBase<S2, D2, O>) -> Self::Output {
        // let out = self.into_dimensionality().unwrap();
        unimplemented!()
    }
}

pub type HostTensor<T, D> = TensorBase<OwnedStorage<T, HostAllocator>, D, HostOps>;

pub type HostTensor0D<T> = HostTensor<T, Dims0>;

pub type HostTensor1D<T> = HostTensor<T, Dims1>;

pub type HostTensor2D<T> = HostTensor<T, Dims2>;

pub type HostTensor3D<T> = HostTensor<T, Dims3>;

pub type HostTensor4D<T> = HostTensor<T, Dims4>;

pub type HostTensor5D<T> = HostTensor<T, Dims5>;

pub type HostTensor6D<T> = HostTensor<T, Dims6>;

pub type HostTensor7D<T> = HostTensor<T, Dims7>;

pub type HostTensor8D<T> = HostTensor<T, Dims8>;

/// Transmute from A to B.
///
/// Like transmute, but does not have the compile-time size check which blocks
/// using regular transmute in some cases.
///
/// **Panics** if the size of A and B are different.
#[inline]
unsafe fn unlimited_transmute<A, B>(data: A) -> B {
    // safe when sizes are equal and caller guarantees that representations are equal
    assert_eq!(std::mem::size_of::<A>(), std::mem::size_of::<B>());
    let old_data = std::mem::ManuallyDrop::new(data);
    (&*old_data as *const A as *const B).read()
}
