#[cfg(not(feature = "zkllvm"))]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[cfg(not(feature = "zkllvm"))]
#[derive(MontConfig)]
#[modulus = "28948022309329048855892746252171976963363056481941647379679742748393362948097"]
#[generator = "5"]
pub struct FrConfig;

#[cfg(not(feature = "zkllvm"))]
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

#[cfg(feature = "zkllvm")]
pub use super::zkllvm::fr::Fr;
