use ark_ff::{
    AdditiveGroup, BigInt, FftField, Field, LegendreSymbol, PrimeField, SqrtPrecomputation,
};
use ark_serialize::{
    CanonicalSerialize, CanonicalSerializeWithFlags, CanonicalDeserialize,
    CanonicalDeserializeWithFlags, Compress, Flags, SerializationError, Valid, Validate,
};
use ark_std::{One, Zero};
use ark_std::fmt;
use ark_std::hash::{Hash, Hasher};
use ark_std::io::{Write, Read};
use ark_std::iter::{self, Iterator, Product, Sum};
use ark_std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign, Deref, DerefMut};
use ark_std::rand::Rng;
use ark_std::rand::distributions::{Distribution, Standard};
use ark_std::str::FromStr;
use num_bigint::BigUint;
use zeroize::Zeroize;

use zkllvm_macros::*;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fq(pub __zkllvm_field_pallas_base);

arith_impl!(Fq);

deref_impl!(Fq, __zkllvm_field_pallas_base);

hash_impl!(Fq);

fmt_impl!(Fq);

zero_one_impl!(Fq);

from_integer_impls!(Fq);

from_builtin_impl!(Fq, __zkllvm_field_pallas_base);

sum_product_impl!(Fq);

zeroize_impl!(Fq);

from_str_impl!(Fq);

serialize_impl!(Fq);

impl From<BigUint> for Fq {
    #[inline]
    fn from(_val: BigUint) -> Fq {
        todo!("conversion from `BigUint` is not implemented for zkLLVM fields yet");
    }
}

impl From<Fq> for BigUint {
    #[inline(always)]
    fn from(_other: Fq) -> Self {
        todo!("conversion into `BigUint` is not implemented for zkLLVM fields yet");
    }
}

impl From<BigInt<256>> for Fq {
    #[inline(always)]
    fn from(_int: BigInt<256>) -> Self {
        todo!("conversion from `BigInt<256>` is not implemented for zkLLVM fields yet");
    }
}

impl From<Fq> for BigInt<256> {
    #[inline(always)]
    fn from(_fp: Fq) -> Self {
        todo!("conversion into `BigInt<256>` is not implemented for zkLLVM fields yet");
    }
}

impl Distribution<Fq> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Fq {
        todo!("sampling zkLLVM fields is not implemented yet");
    }
}

impl FftField for Fq {
    const GENERATOR: Self = Fq::ONE;

    const TWO_ADICITY: u32 = todo!();

    const TWO_ADIC_ROOT_OF_UNITY: Self = todo!();
}

impl PrimeField for Fq {
    type BigInt = BigInt<256>;

    const MODULUS: Self::BigInt = todo!();

    const MODULUS_MINUS_ONE_DIV_TWO: Self::BigInt = todo!();

    const MODULUS_BIT_SIZE: u32 = todo!();

    const TRACE: Self::BigInt = todo!();

    const TRACE_MINUS_ONE_DIV_TWO: Self::BigInt = todo!();

    fn from_bigint(_repr: Self::BigInt) -> Option<Self> {
        todo!("conversion from `BigInt<256>` is not implemented for zkLLVM fields yet");
    }

    fn into_bigint(self) -> Self::BigInt {
        todo!("conversion into `BigInt<256>` is not implemented for zkLLVM fields yet");
    }
}

impl Field for Fq {
    type BasePrimeField = Fq;

    type BasePrimeFieldIter = iter::Once<Self::BasePrimeField>;

    const SQRT_PRECOMP: Option<SqrtPrecomputation<Self>> = None;

    const ONE: Self = Fq(1g);

    fn extension_degree() -> u64 {
        1
    }

    fn to_base_prime_field_elements(&self) -> Self::BasePrimeFieldIter {
        iter::once(*self)
    }

    fn from_base_prime_field_elems(
        elems: impl IntoIterator<Item = Self::BasePrimeField>,
    ) -> Option<Self> {
        let mut elems = elems.into_iter();
        let elem = elems.next()?;
        if elems.next().is_some() {
            return None;
        }
        Some(elem)
    }

    fn from_base_prime_field(elem: Self::BasePrimeField) -> Self {
        elem
    }

    fn from_random_bytes_with_flags<F: Flags>(_bytes: &[u8]) -> Option<(Self, F)> {
        todo!()
    }

    fn legendre(&self) -> LegendreSymbol {
        todo!()
    }

    fn square(&self) -> Self {
        todo!()
    }

    fn square_in_place(&mut self) -> &mut Self {
        todo!()
    }

    fn inverse(&self) -> Option<Self> {
        todo!()
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        todo!()
    }

    fn frobenius_map_in_place(&mut self, _power: usize) {
        todo!()
    }
}

impl AdditiveGroup for Fq {
    type Scalar = Fq;

    const ZERO: Self = Fq(0g);
}
