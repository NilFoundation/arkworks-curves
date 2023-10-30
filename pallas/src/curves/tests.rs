use crate::Projective;
use ark_algebra_test_templates::*;

test_group!(g1; Projective; sw);
#[cfg(not(feature = "zkllvm"))]
test_group!(g1_glv; Projective; glv);
