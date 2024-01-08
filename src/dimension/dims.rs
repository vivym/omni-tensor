use crate::index::Ix;

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
