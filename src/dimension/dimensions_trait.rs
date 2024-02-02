use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::index::Ix;

pub trait Dimensions:
    Clone
    + Debug
    + Send
    + Sync
    + Default
    + PartialEq
    + Eq
    + Index<usize, Output = Ix>
    + IndexMut<usize, Output = Ix>
{
    const NDIM: Option<usize>;

    type Pattern: IntoDimension<Dims = Self> + Clone + Debug + PartialEq + Eq + Default;

    type Smaller: Dimensions;

    type Larger: Dimensions;

    fn ndim(&self) -> usize;

    fn into_pattern(self) -> Self::Pattern;

    fn as_slice(&self) -> &[Ix];

    fn as_slice_mut(&mut self) -> &mut [Ix];

    fn size(&self) -> usize {
        self.as_slice().iter().product()
    }

    fn default_strides(&self) -> Self {
        let mut strides = Self::default();

        if self.as_slice().iter().all(|&i| i != 0) {
            let mut it = strides.as_slice_mut().iter_mut().rev();

            if let Some(stride) = it.next() {
                *stride = 1;
            }
            let mut cum_prod = 1;
            for (stride, dim) in it.zip(self.as_slice().iter().rev()) {
                cum_prod *= *dim;
                *stride = cum_prod;
            }
        }
        strides
    }

    fn fortran_strides(&self) -> Self {
        // Shape (a, b, c) -> strides (1, a, a * b)
        let mut strides = Self::zeros(self.ndim());
        // For empty arrays, use all zero strides.
        if self.as_slice().iter().all(|&d| d != 0) {
            let mut it = strides.as_slice_mut().iter_mut();
            if let Some(rs) = it.next() {
                *rs = 1;
            }
            let mut cum_prod = 1;
            for (rs, &dim) in it.zip(self.as_slice()) {
                cum_prod *= dim;
                *rs = cum_prod;
            }
        }
        strides
    }

    fn zeros(ndim: usize) -> Self;

    fn equal(&self, rhs: &Self) -> bool {
        self.as_slice() == rhs.as_slice()
    }

    fn is_contiguous(dims: &Self, strides: &Self) -> bool {
        let defaults = dims.default_strides();
        if strides.equal(&defaults) {
            return true;
        }
        if dims.ndim() == 1 {
            return strides[0] as isize == -1;
        }
        let order = strides._fastest_varying_stride_order();
        let strides = strides.as_slice();
        let dims = dims.as_slice();
        let mut cstride = 1;
        for &i in order.as_slice() {
            // a dimension of length 1 can have unequal strides
            if dims[i] != 1 && (strides[i] as isize).unsigned_abs() != cstride {
                return false;
            }
            cstride *= dims[i];
        }
        true
    }

    fn _fastest_varying_stride_order(&self) -> Self {
        let mut indices = self.clone();
        for (i, elem) in indices.as_slice_mut().iter_mut().enumerate() {
            *elem = i;
        }
        let strides = self.as_slice();
        indices
            .as_slice_mut()
            .sort_by_key(|&i| (strides[i] as isize).abs());
        indices
    }
}

pub trait IntoDimension {
    type Dims: Dimensions;

    fn into_dimension(self) -> Self::Dims;
}

impl<D: Dimensions> IntoDimension for D {
    type Dims = D;

    fn into_dimension(self) -> Self::Dims {
        self
    }
}
