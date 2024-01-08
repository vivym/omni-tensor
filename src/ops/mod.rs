use std::ops::{Add, Div, Mul, Sub};

use crate::elem::Elem;

pub mod host;

pub trait Ops: Default + Clone {
    fn _fill<T: Elem>(&self, value: T, out: &mut [T]);

    fn _add<L, R, O>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Add<R, Output = O>,
        R: Elem,
        O: Elem;

    fn _sub<L, R, O>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Sub<R, Output = O>,
        R: Elem,
        O: Elem;

    fn _mul<L: Elem, R: Elem, O: Elem>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Mul<R, Output = O>,
        R: Elem,
        O: Elem;

    fn _div<L: Elem, R: Elem, O: Elem>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Div<R, Output = O>,
        R: Elem,
        O: Elem;
}

pub use host::HostOps;
