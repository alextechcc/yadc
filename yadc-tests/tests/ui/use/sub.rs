#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use yadc_tests::*;

fn main() {
    let altitude = SEA_LEVEL + 100.0 * METRE;
    let _ = METRE - SECOND; // cannot subtract: the dimensions differ
    let _ = METRE - altitude; // cannot subtract a point from an amount
    let _ = KELVIN - ABSOLUTE_ZERO; // cannot subtract a point from an amount
    let _ = SEA_LEVEL - UNIX_EPOCH; // cannot subtract: the dimensions differ
    let _ = ONE - FIRST; // cannot subtract: the labels have no difference
    let _ = WATT - WATT_LEVEL; // only a logarithm subtracts from a logarithm
}
