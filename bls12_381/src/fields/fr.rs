#[cfg(not(feature = "zkllvm"))]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[cfg(not(feature = "zkllvm"))]
#[derive(MontConfig)]
#[modulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[generator = "7"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "1"]
pub struct FrConfig;

#[cfg(not(feature = "zkllvm"))]
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

#[cfg(feature = "zkllvm")]
pub use super::zkllvm::fr::Fr;
