use std::fmt::Debug;

use crate::index::Ix;
use super::{dims::Dims, dyn_dims::DynDims};

pub trait Dimensions: Clone + Debug + Send + Sync + Default + PartialEq + Eq {
    const NDIM: Option<usize>;

    type Pattern: IntoDimension<Dim = Self> + Clone + Debug + PartialEq + Eq + Default;

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
}

impl Dimensions for Dims<[Ix; 0]> {
    const NDIM: Option<usize> = Some(0);

    type Pattern = ();

    type Smaller = Self;

    type Larger = Dims<[Ix; 1]>;

    fn ndim(&self) -> usize {
        0
    }

    fn into_pattern(self) -> Self::Pattern {}

    fn as_slice(&self) -> &[Ix] {
        &[]
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut []
    }
}

impl Dimensions for Dims<[Ix; 1]> {
    const NDIM: Option<usize> = Some(1);

    type Pattern = Ix;

    type Smaller = Dims<[Ix; 0]>;

    type Larger = Dims<[Ix; 2]>;

    fn ndim(&self) -> usize {
        1
    }

    fn into_pattern(self) -> Self::Pattern {
        self.0[0]
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 2]> {
    const NDIM: Option<usize> = Some(2);

    type Pattern = (Ix, Ix);

    type Smaller = Dims<[Ix; 1]>;

    type Larger = Dims<[Ix; 3]>;

    fn ndim(&self) -> usize {
        2
    }

    fn into_pattern(self) -> Self::Pattern {
        (self.0[0], self.0[1])
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 3]> {
    const NDIM: Option<usize> = Some(3);

    type Pattern = (Ix, Ix, Ix);

    type Smaller = Dims<[Ix; 2]>;

    type Larger = Dims<[Ix; 4]>;

    fn ndim(&self) -> usize {
        3
    }

    fn into_pattern(self) -> Self::Pattern {
        (self.0[0], self.0[1], self.0[2])
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 4]> {
    const NDIM: Option<usize> = Some(4);

    type Pattern = (Ix, Ix, Ix, Ix);

    type Smaller = Dims<[Ix; 3]>;

    type Larger = Dims<[Ix; 5]>;

    fn ndim(&self) -> usize {
        4
    }

    fn into_pattern(self) -> Self::Pattern {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 5]> {
    const NDIM: Option<usize> = Some(5);

    type Pattern = (Ix, Ix, Ix, Ix, Ix);

    type Smaller = Dims<[Ix; 4]>;

    type Larger = Dims<[Ix; 6]>;

    fn ndim(&self) -> usize {
        5
    }

    fn into_pattern(self) -> Self::Pattern {
        (self.0[0], self.0[1], self.0[2], self.0[3], self.0[4])
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 6]> {
    const NDIM: Option<usize> = Some(6);

    type Pattern = (Ix, Ix, Ix, Ix, Ix, Ix);

    type Smaller = Dims<[Ix; 5]>;

    type Larger = Dims<[Ix; 7]>;

    fn ndim(&self) -> usize {
        6
    }

    fn into_pattern(self) -> Self::Pattern {
        (self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 7]> {
    const NDIM: Option<usize> = Some(7);

    type Pattern = (Ix, Ix, Ix, Ix, Ix, Ix, Ix);

    type Smaller = Dims<[Ix; 6]>;

    type Larger = Dims<[Ix; 8]>;

    fn ndim(&self) -> usize {
        7
    }

    fn into_pattern(self) -> Self::Pattern {
        (
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
        )
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for Dims<[Ix; 8]> {
    const NDIM: Option<usize> = Some(8);

    type Pattern = (Ix, Ix, Ix, Ix, Ix, Ix, Ix, Ix);

    type Smaller = Dims<[Ix; 7]>;

    type Larger = DynDims;

    fn ndim(&self) -> usize {
        8
    }

    fn into_pattern(self) -> Self::Pattern {
        (
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
        )
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

impl Dimensions for DynDims {
    const NDIM: Option<usize> = None;

    type Pattern = Self;

    type Smaller = Dims<[Ix; 8]>;

    type Larger = Self;

    fn ndim(&self) -> usize {
        self.0.len()
    }

    fn into_pattern(self) -> Self::Pattern {
        self
    }

    fn as_slice(&self) -> &[Ix] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Ix] {
        &mut self.0
    }
}

pub trait IntoDimension {
    type Dim: Dimensions;

    fn into_dimension(self) -> Self::Dim;
}

impl<D: Dimensions> IntoDimension for D {
    type Dim = D;

    fn into_dimension(self) -> Self::Dim {
        self
    }
}

impl IntoDimension for () {
    type Dim = Dims<[Ix; 0]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([])
    }
}

impl IntoDimension for Ix {
    type Dim = Dims<[Ix; 1]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self])
    }
}

impl IntoDimension for (Ix, Ix) {
    type Dim = Dims<[Ix; 2]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self.0, self.1])
    }
}

impl IntoDimension for (Ix, Ix, Ix) {
    type Dim = Dims<[Ix; 3]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self.0, self.1, self.2])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix) {
    type Dim = Dims<[Ix; 4]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self.0, self.1, self.2, self.3])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix) {
    type Dim = Dims<[Ix; 5]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self.0, self.1, self.2, self.3, self.4])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix, Ix) {
    type Dim = Dims<[Ix; 6]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self.0, self.1, self.2, self.3, self.4, self.5])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix, Ix, Ix) {
    type Dim = Dims<[Ix; 7]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([self.0, self.1, self.2, self.3, self.4, self.5, self.6])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix, Ix, Ix, Ix) {
    type Dim = Dims<[Ix; 8]>;

    fn into_dimension(self) -> Self::Dim {
        Dims::new([
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        ])
    }
}
