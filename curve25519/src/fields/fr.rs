#[cfg(not(feature = "zkllvm"))]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[cfg(not(feature = "zkllvm"))]
#[derive(MontConfig)]
#[modulus = "7237005577332262213973186563042994240857116359379907606001950938285454250989"]
#[generator = "2"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "1"]
pub struct FrConfig;

#[cfg(not(feature = "zkllvm"))]
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

#[cfg(feature = "zkllvm")]
pub use super::zkllvm::fr::Fr;
