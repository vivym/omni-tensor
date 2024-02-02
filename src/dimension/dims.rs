use crate::index::Ix;
use super::{
    dimensions_trait::Dimensions,
    dyn_dims::DynDims,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Dims<I>(pub(crate) I);   // TODO: remove pub(crate)

impl<I> Dims<I> {
    pub(crate) fn new(dims: I) -> Self {
        Self(dims)
    }
}

pub type Dims0 = Dims<[Ix; 0]>;

pub type Dims1 = Dims<[Ix; 1]>;

pub type Dims2 = Dims<[Ix; 2]>;

pub type Dims3 = Dims<[Ix; 3]>;

pub type Dims4 = Dims<[Ix; 4]>;

pub type Dims5 = Dims<[Ix; 5]>;

pub type Dims6 = Dims<[Ix; 6]>;

pub type Dims7 = Dims<[Ix; 7]>;

pub type Dims8 = Dims<[Ix; 8]>;

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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 0);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 1);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 2);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 3);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 4);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 5);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 6);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 7);
        Self::default()
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

    fn zeros(ndim: usize) -> Self {
        debug_assert_eq!(ndim, 8);
        Self::default()
    }
}
