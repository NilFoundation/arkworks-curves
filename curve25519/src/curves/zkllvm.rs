use ark_std::fmt;
use ark_std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Deref, DerefMut,
};

use zkllvm_macros::*;

use super::{Fq, Fr};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct EdwardsAffine(pub __zkllvm_curve_curve25519);

deref_impl!(EdwardsAffine, __zkllvm_curve_curve25519);

fmt_impl!(EdwardsAffine);

from_builtin_impl!(EdwardsAffine, __zkllvm_curve_curve25519);

curve_arith_impl!(EdwardsAffine, Fr);

curve_init_impl!(EdwardsAffine, __zkllvm_curve_curve25519, Fq);
