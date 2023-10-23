use ark_ff::MontConfig;
#[cfg(not(feature = "zkllvm"))]
use ark_ff::fields::{Fp256, MontBackend};

#[derive(MontConfig)]
#[modulus = "28948022309329048855892746252171976963363056481941560715954676764349967630337"]
#[generator = "5"]
pub struct FqConfig;

#[cfg(not(feature = "zkllvm"))]
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

#[cfg(feature = "zkllvm")]
pub use super::zkllvm::fq::Fq;
