use crate::dimension::{Dimensions, IntoDimension};

#[derive(Copy, Clone, Debug)]
pub struct Shape<D> {
    pub(crate) dims: D,
    pub(crate) strides: Strides<Contiguous>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Contiguous {}

#[derive(Copy, Clone, Debug)]
pub struct StridedShape<D> {
    pub(crate) dims: D,
    pub(crate) strides: Strides<D>,
}

/// Strides description
#[derive(Copy, Clone, Debug)]
pub(crate) enum Strides<D> {
    /// Row-major strides ("C" order)
    C,
    /// Column-major strides ("F" order)
    F,
    /// Custom strides
    Custom(D),
}

impl<D> Shape<D> where D: Dimensions {
    pub fn raw_dims(&self) -> &D {
        &self.dims
    }

    pub fn size(&self) -> usize {
        self.dims.size()
    }
}

impl<D> Strides<D> {
    pub(crate) fn strides_for_dims(self, dims: &D) -> D
    where
        D: Dimensions,
    {
        match self {
            Strides::C => dims.default_strides(),
            Strides::F => dims.fortran_strides(),
            Strides::Custom(strides) => {
                debug_assert_eq!(
                    strides.ndim(),
                    dims.ndim(),
                    "Custom strides given with {} dimensions, expected {}",
                    strides.ndim(),
                    dims.ndim()
                );
                strides
            },
        }
    }
}

pub trait ShapeBuilder {
    type Dims: Dimensions;
    type Strides;

    fn into_shape(self) -> Shape<Self::Dims>;

    fn f(self) -> Shape<Self::Dims>;

    fn set_f(self, is_f: bool) -> Shape<Self::Dims>;

    fn strides(self, strides: Self::Strides) -> StridedShape<Self::Dims>;
}

impl<T> ShapeBuilder for T where T: IntoDimension {
    type Dims = T::Dims;
    type Strides = T;

    fn into_shape(self) -> Shape<Self::Dims> {
        Shape {
            dims: self.into_dimension(),
            strides: Strides::C,
        }
    }

    fn f(self) -> Shape<Self::Dims> {
        self.set_f(true)
    }

    fn set_f(self, is_f: bool) -> Shape<Self::Dims> {
        self.into_shape().set_f(is_f)
    }

    fn strides(self, strides: Self::Strides) -> StridedShape<Self::Dims> {
        self.into_shape().strides(strides.into_dimension())
    }
}

impl<D> ShapeBuilder for Shape<D> where D: Dimensions {
    type Dims = D;
    type Strides = D;

    fn into_shape(self) -> Shape<Self::Dims> {
        self
    }

    fn f(self) -> Shape<Self::Dims> {
        self.set_f(true)
    }

    fn set_f(mut self, is_f: bool) -> Shape<Self::Dims> {
        self.strides = if is_f { Strides::F } else { Strides::C };
        self
    }

    fn strides(self, strides: Self::Strides) -> StridedShape<Self::Dims> {
        StridedShape {
            dims: self.dims,
            strides: Strides::Custom(strides),
        }
    }
}
