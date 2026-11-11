#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use yadc_tests::*;
use yadc::Unit;

fn is_quantity<Q: yadc::Quantity>() {}

fn main() {
    let altitude = SEA_LEVEL + 100.0 * METRE;
    let _ = altitude * 2.0; // a point cannot be scaled
    let _ = 2.0 * altitude;
    let _ = altitude / 2.0;
    let _ = FIRST * 2.0; // cannot scale: this label cannot be added to itself
    let two = 2.0 * ONE; // a plain number is a scaling too
    let _ = FIRST * two;
    let _ = MILLIWATT_LEVEL * two;
    let _ = SEA_LEVEL.value(altitude); // a point is not a unit of itself
    is_quantity::<Altitude>(); // not a `Quantity`
    is_quantity::<Rank>();
}
