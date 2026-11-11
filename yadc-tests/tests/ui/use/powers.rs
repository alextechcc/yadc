#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use yadc_tests::*;
use yadc::num_traits::Inv;
use yadc::{PowiAs, RootAs};

fn main() {
    let altitude = SEA_LEVEL + 100.0 * METRE;
    let _ = altitude.powi::<2>(); // this power has no dimension: a point has no powers
    let _ = altitude.root::<2>(); // a point or a logarithm has no roots
    let _ = DECIBEL.root::<2>(); // a logarithm has no roots either (its first is itself)
    let _ = METRE.root::<2>(); // this root has no dimension
    let _ = altitude.inv(); // a point has no reciprocal
}
