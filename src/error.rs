use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ShapeError {
    #[error("Incompatible shape")]
    IncompatibleShape,
    #[error("Incompatible layout")]
    IncompatibleLayout,
}


#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum OmniError {
    #[error("Shape error: {0}")]
    ShapeError(#[from] ShapeError),
}

pub type OmniResult<T> = Result<T, OmniError>;
