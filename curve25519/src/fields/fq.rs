#[cfg(not(feature = "zkllvm"))]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[cfg(not(feature = "zkllvm"))]
#[derive(MontConfig)]
#[modulus = "57896044618658097711785492504343953926634992332820282019728792003956564819949"]
#[generator = "2"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "1"]
pub struct FqConfig;

#[cfg(not(feature = "zkllvm"))]
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

#[cfg(feature = "zkllvm")]
pub use super::zkllvm::fq::Fq;
