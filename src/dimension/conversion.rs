use std::ops::{Index, IndexMut};

use crate::index::Ix;
use super::{
    dims::Dims,
    dimensions_trait::{Dimensions, IntoDimension},
    dyn_dims::DynDims,
};

impl Index<usize> for Dims<[Ix; 0]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 0]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 1]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 1]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 2]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 2]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 3]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 3]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 4]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 4]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 5]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 5]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 6]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 6]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 7]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 7]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for Dims<[Ix; 8]> {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Dims<[Ix; 8]> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<usize> for DynDims {
    type Output = Ix;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for DynDims {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl IntoDimension for [Ix; 0] {
    type Dims = Dims<[Ix; 0]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 1] {
    type Dims = Dims<[Ix; 1]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 2] {
    type Dims = Dims<[Ix; 2]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 3] {
    type Dims = Dims<[Ix; 3]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 4] {
    type Dims = Dims<[Ix; 4]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 5] {
    type Dims = Dims<[Ix; 5]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 6] {
    type Dims = Dims<[Ix; 6]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 7] {
    type Dims = Dims<[Ix; 7]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for [Ix; 8] {
    type Dims = Dims<[Ix; 8]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new(self)
    }
}

impl IntoDimension for () {
    type Dims = Dims<[Ix; 0]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([])
    }
}

impl IntoDimension for Ix {
    type Dims = Dims<[Ix; 1]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self])
    }
}

impl IntoDimension for (Ix, Ix) {
    type Dims = Dims<[Ix; 2]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self.0, self.1])
    }
}

impl IntoDimension for (Ix, Ix, Ix) {
    type Dims = Dims<[Ix; 3]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self.0, self.1, self.2])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix) {
    type Dims = Dims<[Ix; 4]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self.0, self.1, self.2, self.3])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix) {
    type Dims = Dims<[Ix; 5]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self.0, self.1, self.2, self.3, self.4])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix, Ix) {
    type Dims = Dims<[Ix; 6]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self.0, self.1, self.2, self.3, self.4, self.5])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix, Ix, Ix) {
    type Dims = Dims<[Ix; 7]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([self.0, self.1, self.2, self.3, self.4, self.5, self.6])
    }
}

impl IntoDimension for (Ix, Ix, Ix, Ix, Ix, Ix, Ix, Ix) {
    type Dims = Dims<[Ix; 8]>;

    fn into_dimension(self) -> Self::Dims {
        Dims::new([
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        ])
    }
}
