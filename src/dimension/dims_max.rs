use super::{
    dimensions_trait::Dimensions,
    dims::{
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
    dyn_dims::DynDims,
};

pub trait DimsMax<Rhs: Dimensions> {
    /// The resulting dimension type after broadcasting.
    type Output: Dimensions;
}

impl<D: Dimensions> DimsMax<D> for D {
    type Output = D;
}

macro_rules! impl_broadcast {
    ($smaller:ty, $larger:ty) => {
        impl DimsMax<$larger> for $smaller {
            type Output = $larger;
        }

        impl DimsMax<$smaller> for $larger {
            type Output = $larger;
        }
    };
}

impl_broadcast!(Dims0, Dims1);
impl_broadcast!(Dims0, Dims2);
impl_broadcast!(Dims0, Dims3);
impl_broadcast!(Dims0, Dims4);
impl_broadcast!(Dims0, Dims5);
impl_broadcast!(Dims0, Dims6);
impl_broadcast!(Dims0, Dims7);
impl_broadcast!(Dims0, Dims8);
impl_broadcast!(Dims0, DynDims);
impl_broadcast!(Dims1, Dims2);
impl_broadcast!(Dims1, Dims3);
impl_broadcast!(Dims1, Dims4);
impl_broadcast!(Dims1, Dims5);
impl_broadcast!(Dims1, Dims6);
impl_broadcast!(Dims1, Dims7);
impl_broadcast!(Dims1, Dims8);
impl_broadcast!(Dims1, DynDims);
impl_broadcast!(Dims2, Dims3);
impl_broadcast!(Dims2, Dims4);
impl_broadcast!(Dims2, Dims5);
impl_broadcast!(Dims2, Dims6);
impl_broadcast!(Dims2, Dims7);
impl_broadcast!(Dims2, Dims8);
impl_broadcast!(Dims2, DynDims);
impl_broadcast!(Dims3, Dims4);
impl_broadcast!(Dims3, Dims5);
impl_broadcast!(Dims3, Dims6);
impl_broadcast!(Dims3, Dims7);
impl_broadcast!(Dims3, Dims8);
impl_broadcast!(Dims3, DynDims);
impl_broadcast!(Dims4, Dims5);
impl_broadcast!(Dims4, Dims6);
impl_broadcast!(Dims4, Dims7);
impl_broadcast!(Dims4, Dims8);
impl_broadcast!(Dims4, DynDims);
impl_broadcast!(Dims5, Dims6);
impl_broadcast!(Dims5, Dims7);
impl_broadcast!(Dims5, Dims8);
impl_broadcast!(Dims5, DynDims);
impl_broadcast!(Dims6, Dims7);
impl_broadcast!(Dims6, Dims8);
impl_broadcast!(Dims6, DynDims);
impl_broadcast!(Dims7, Dims8);
impl_broadcast!(Dims7, DynDims);
impl_broadcast!(Dims8, DynDims);

pub type DimsMaxOf<D1, D2> = <D1 as DimsMax<D2>>::Output;
