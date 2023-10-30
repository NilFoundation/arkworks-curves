use ark_std::fmt;
use ark_std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Deref, DerefMut,
};

use zkllvm_macros::*;

use crate::{Fq, Fr};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct G1Affine(pub __zkllvm_curve_bls12381);

deref_impl!(G1Affine, __zkllvm_curve_bls12381);

fmt_impl!(G1Affine);

from_builtin_impl!(G1Affine, __zkllvm_curve_bls12381);

curve_arith_impl!(G1Affine, Fr);

curve_init_impl!(G1Affine, __zkllvm_curve_bls12381, Fq);
