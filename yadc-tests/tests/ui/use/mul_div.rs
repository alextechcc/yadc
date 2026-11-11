#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use yadc_tests::*;

fn main() {
    let altitude = SEA_LEVEL + 100.0 * METRE;
    let _ = altitude * METRE; // cannot multiply: a point has no products
    let _ = METRE * altitude;
    let _ = altitude / SECOND; // cannot divide: a point has no quotients
    let _ = METRE / altitude;
    let _ = WATT_LEVEL * METRE; // a level has no products
    let _ = DECIBEL * DECIBEL;
}
