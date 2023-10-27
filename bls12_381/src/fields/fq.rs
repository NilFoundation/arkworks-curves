#[cfg(not(feature = "zkllvm"))]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[cfg(not(feature = "zkllvm"))]
#[derive(MontConfig)]
#[modulus = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787"]
#[generator = "2"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
pub struct FqConfig;

#[cfg(not(feature = "zkllvm"))]
pub type Fq = Fp384<MontBackend<FqConfig, 6>>;

#[cfg(feature = "zkllvm")]
pub use super::zkllvm::fq::Fq;

pub const FQ_ONE: Fq = ark_ff::MontFp!("1");
pub const FQ_ZERO: Fq = ark_ff::MontFp!("0");
