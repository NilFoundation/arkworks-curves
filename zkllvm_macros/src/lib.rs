//! Macros which helps to implement traits on zkLLVM types.
//!
//! These are common implementations for all zkLLVM types.
//! Some of them just panics, because we can't have them right now or they do not make sense at all.

mod arithmetics;

/// Implements `Hash`, assuming that `Self.0` is `Hash`.
#[macro_export]
macro_rules! hash_impl {
    ($($t:ty)*) => ($(
        impl Hash for $t {
            fn hash<H>(&self, _: &mut H)
            where
                H: Hasher
            {
                todo!("default hash implementation for zkLLVM types is not done yet");
            }
        }
    )*)
}

/// Implements `Deref` and `DerefMut` on `T`.
#[macro_export]
macro_rules! deref_impl {
    ($($t:ty, $builtin:ident)*) => ($(
        impl Deref for $t {
            type Target = $builtin;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    )*)
}

/// Implements `fmt::Debug` and `fmt::Display`,
/// assuming that `T.0` is `fmt::Debug` and `fmt::Display`.
#[macro_export]
macro_rules! fmt_impl {
    ($($t:ty)*) => ($(
        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    )*)
}

/// Implements `One` and `Zero`.
#[macro_export]
macro_rules! zero_one_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                Self(0g)
            }

            #[inline]
            fn is_zero(&self) -> bool {
                self == &Self::zero()
            }
        }

        impl One for $t {
            fn one() -> Self {
                Self(1g)
            }
        }
    )*)
}

/// Implements `From<T>`, assuming that `Self.0` is convertable using `as`.
#[macro_export]
macro_rules! from_impls {
    ($t:ty, $($integer:ident)*) => ($(
        impl From<$integer> for $t {
            fn from(_value: $integer) -> Self {
                // Self(value as $builtin)
                todo!("zkLLVM field types casts are not implemented yet");
            }
        }
    )*)
}

/// Implements `From<T>`, assuming that `Self.0` is convertable using `as` where `T` is
/// `bool`, `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`.
#[macro_export]
macro_rules! from_integer_impls {
    ($t:ty) => (
        from_impls! { $t, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }
    )
}

/// Implements `From<T> for BI` and `From<BI> for T` where `T` is a wrapper and `BI` is a built-in.
#[macro_export]
macro_rules! from_builtin_impl {
    ($($t:ty, $builtin:ident)*) => ($(
        impl const From<$builtin> for $t {
            #[inline(always)]
            fn from(value: $builtin) -> Self {
                Self(value)
            }
        }

        impl const From<$t> for $builtin {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.0
            }
        }
    )*)
}

/// Implements `Sum` and `Product`.
#[macro_export]
macro_rules! sum_product_impl {
    ($($t:ty)*) => ($(
        impl Sum for $t {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    Self(0g),
                    |a, b| a + b,
                )
            }
        }

        impl Product for $t {
            fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    Self(1g),
                    |a, b| a * b,
                )
            }
        }

        impl<'a> Sum<&'a $t> for $t {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    Self(0g),
                    |a, b| a + b,
                )
            }
        }

        impl<'a> Product<&'a $t> for $t {
            fn product<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    Self(1g),
                    |a, b| a * b,
                )
            }
        }
    )*)
}

/// Implements `Zeroize`.
#[macro_export]
macro_rules! zeroize_impl {
    ($($t:ty)*) => ($(
        impl Zeroize for $t {
            fn zeroize(&mut self) {
                self.0 = 0g;
            }
        }
    )*)
}

/// Implements `FromStr`.
#[macro_export]
macro_rules! from_str_impl {
    ($($t:ty)*) => ($(
        impl FromStr for $t {
            type Err = ();

            fn from_str(_s: &str) -> Result<Self, Self::Err> {
                todo!("creating zkLLVM field from string is not yet implemented");
            }
        }
    )*)
}

/// Implements a number of serialization traits.
/// We actually do not need them, so they left blank.
#[macro_export]
macro_rules! serialize_impl {
    ($($t:ty)*) => ($(
        impl CanonicalSerialize for $t {
            fn serialize_with_mode<W: Write>(
                &self,
                _: W,
                _: Compress,
            ) -> Result<(), SerializationError> {
                unimplemented!("serialization is impossible");
            }

            fn serialized_size(&self, _: Compress) -> usize {
                unimplemented!("serialization is impossible");
            }
        }

        impl CanonicalSerializeWithFlags for $t {
            fn serialize_with_flags<W: Write, F: Flags>(
                &self,
                _: W,
                _: F,
            ) -> Result<(), SerializationError> {
                unimplemented!("serialization is impossible");
            }

            fn serialized_size_with_flags<F: Flags>(&self) -> usize {
                unimplemented!("serialization is impossible");
            }
        }

        impl Valid for $t {
            fn check(&self) -> Result<(), SerializationError> {
                unimplemented!("serialization is impossible");
            }
        }

        impl CanonicalDeserialize for $t {
            fn deserialize_with_mode<R: Read>(
                _: R,
                _: Compress,
                _: Validate,
            ) -> Result<Self, SerializationError> {
                unimplemented!("serialization is impossible");
            }
        }

        impl CanonicalDeserializeWithFlags for $t {
            fn deserialize_with_flags<R: Read, F: Flags>(
                _: R,
            ) -> Result<(Self, F), SerializationError> {
                unimplemented!("serialization is impossible");
            }
        }
    )*)
}
