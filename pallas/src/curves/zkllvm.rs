use ark_std::fmt;
use ark_std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Deref, DerefMut,
};

use zkllvm_macros::*;

use super::{Fq, Fr};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Affine(pub __zkllvm_curve_pallas);

deref_impl!(Affine, __zkllvm_curve_pallas);

fmt_impl!(Affine);

from_builtin_impl!(Affine, __zkllvm_curve_pallas);

curve_arith_impl!(Affine, Fr);

curve_init_impl!(Affine, __zkllvm_curve_pallas, Fq);
