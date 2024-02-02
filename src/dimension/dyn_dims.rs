use std::ops::{Deref, DerefMut};

use crate::index::Ix;
use super::{
    dimensions_trait::Dimensions,
    dims::Dims,
};

const CAP: usize = 4;

#[derive(Clone, Debug, Eq)]
enum DynDimsRepr<T> {
    Inline(u32, [T; CAP]),
    Alloc(Box<[T]>),
}

impl<T> Deref for DynDimsRepr<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Inline(len, ref x) => {
                debug_assert!(len as usize <= x.len());
                unsafe { x.get_unchecked(..len as usize) }
            },
            Self::Alloc(ref x) => x,
        }
    }
}

impl<T> DerefMut for DynDimsRepr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match *self {
            Self::Inline(len, ref mut x) => {
                debug_assert!(len as usize <= x.len());
                unsafe { x.get_unchecked_mut(..len as usize) }
            },
            Self::Alloc(ref mut x) => x,
        }
    }
}

impl<T: PartialEq> PartialEq for DynDimsRepr<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Inline(len1, ref x1), &Self::Inline(len2, ref x2)) => {
                len1 == len2 && (0..CAP as usize)
                    .filter(|&i| i < len1 as usize)
                    .all(|i| x1[i] == x2[i])
            },
            _ => self[..] == other[..],
        }
    }
}

impl Default for DynDimsRepr<Ix> {
    fn default() -> Self {
        DynDimsRepr::Inline(0, [Default::default(); CAP])
    }
}

impl<T: Copy> DynDimsRepr<T> {
    fn from_vec(v: Vec<T>) -> Self {
        DynDimsRepr::Alloc(v.into_boxed_slice())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DynDimsImpl(DynDimsRepr<Ix>);

impl Deref for DynDimsImpl {
    type Target = [Ix];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DynDimsImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DynDimsImpl {
    pub(crate) fn unsqueeze(&self, axis: usize) -> Self {
        let len = self.len();
        debug_assert!(axis < len);
        let repr = if axis < CAP {
            let mut out = [1; CAP];
            out[0..axis].copy_from_slice(&self[0..axis]);
            out[axis + 1..=len].copy_from_slice(&self[axis..len]);
            DynDimsRepr::Inline((len + 1) as u32, out)
        } else {
            let mut out = Vec::with_capacity(len + 1);
            out.extend_from_slice(&self[0..axis]);
            out.push(1);
            out.extend_from_slice(&self[axis..len]);
            DynDimsRepr::from_vec(out)
        };
        DynDimsImpl(repr)
    }

    pub(crate) fn squeeze(&self, axis: usize) -> Self {
        debug_assert!(axis < self.len());
        let repr = match self.0 {
            DynDimsRepr::Inline(0, _) => DynDimsRepr::Inline(0, [0; CAP]),
            DynDimsRepr::Inline(1, _) => DynDimsRepr::Inline(0, [0; CAP]),
            DynDimsRepr::Inline(2, ref x) => {
                let mut out = [0; CAP];
                out[0] = x[1 - axis];
                DynDimsRepr::Inline(1, out)
            },
            DynDimsRepr::Inline(3, ref x) => {
                let mut out = [0; CAP];
                out[0] = x[1 - axis];
                out[1] = x[2 - axis];
                DynDimsRepr::Inline(2, out)
            },
            DynDimsRepr::Inline(4, ref x) => {
                let mut out = [0; CAP];
                out[0] = x[1 - axis];
                out[1] = x[2 - axis];
                out[2] = x[3 - axis];
                DynDimsRepr::Inline(3, out)
            },
            ref repr => {
                let mut out = Vec::with_capacity(repr.len() - 1);
                out.extend_from_slice(&repr[0..axis]);
                out.extend_from_slice(&repr[axis + 1..repr.len()]);
                DynDimsRepr::from_vec(out)
            },
        };
        DynDimsImpl(repr)
    }
}

pub type DynDims = Dims<DynDimsImpl>;

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

    fn zeros(ndim: usize) -> Self {
        DynDims::zeros(ndim)
    }
}
