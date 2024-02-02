pub mod conversion;
pub mod dims_max;
pub mod dimensions_trait;
pub mod dims;
pub mod dyn_dims;

pub use dims_max::{
    DimsMax,
    DimsMaxOf,
};
pub use dimensions_trait::{Dimensions, IntoDimension};
pub use dims::{
    Dims,
    Dims0,
    Dims1,
    Dims2,
    Dims3,
    Dims4,
    Dims5,
    Dims6,
    Dims7,
    Dims8,
};

pub fn offset_from_low_addr_ptr_to_logical_ptr<D: Dimensions>(dims: &D, strides: &D) -> usize {
    let zip_iter = dims.as_slice().iter().zip(strides.as_slice().iter());
    let offset = zip_iter.fold(0, |offset, (&dim, &stride)| {
        let stride = stride as isize;
        if stride < 0 && dim > 1 {
            offset - stride * (dim as isize - 1)
        } else {
            offset
        }
    });
    debug_assert!(offset >= 0, "Negative offset");
    offset as usize
}
