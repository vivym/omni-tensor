use std::ops::{Add, Div, Mul, Sub};

use crate::elem::Elem;
use super::Ops;

#[derive(Default, Clone)]
pub struct HostOps;

#[inline(always)]
fn zip_binary_op<L, R, O, F>(lhs: &[L], rhs: &[R], out: &mut [O], f: F)
where
    L: Elem,
    R: Elem,
    O: Elem,
    F: Fn(L, R) -> O,
{
    lhs.iter()
        .zip(rhs.iter())
        .zip(out.iter_mut())
        .for_each(|((x, y), z)| *z = f(*x, *y));
}

impl Ops for HostOps {
    fn _fill<T: Elem>(&self, value: T, out: &mut [T]) {
        out.iter_mut().for_each(|x| *x = value);
    }

    fn _add<L, R, O>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Add<R, Output = O>,
        R: Elem,
        O: Elem,
    {
        zip_binary_op(lhs, rhs, out, |x, y| x + y);
    }

    fn _sub<L, R, O>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Sub<R, Output = O>,
        R: Elem,
        O: Elem,
    {
        zip_binary_op(lhs, rhs, out, |x, y| x - y);
    }

    fn _mul<L, R, O>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Mul<R, Output = O>,
        R: Elem,
        O: Elem,
    {
        zip_binary_op(lhs, rhs, out, |x, y| x * y);
    }

    fn _div<L, R, O>(&self, lhs: &[L], rhs: &[R], out: &mut [O])
    where
        L: Elem + Div<R, Output = O>,
        R: Elem,
        O: Elem,
    {
        zip_binary_op(lhs, rhs, out, |x, y| x / y);
    }
}
