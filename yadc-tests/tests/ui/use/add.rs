#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use yadc_tests::*;


fn main() {
    let altitude = SEA_LEVEL + 100.0 * METRE;
    let _ = METRE + SECOND; // cannot add: the dimensions differ
    let _ = altitude + altitude; // cannot add a point to a point
    let _ = METRE + altitude; // cannot add these in this order
    let _ = KELVIN + ABSOLUTE_ZERO; // a temperature is a point: write it first
    let _ = FIRST + FIRST; // cannot add: the labels have no sum
    let _ = altitude + SECOND; // a point moves only by an amount of its dimension
    let level: PowerLevel = WATT_LEVEL + 3.0 * DECIBEL;
    let _ = level + level; // a sum of two levels is the logarithm of watts squared, not a level
    let _ = DECIBEL + level; // a sum keeps its left operand: write the level first
    let _ = level + 3.0 * METRE; // only a logarithm adds to a logarithm
    let _ = 2.0 * level; // doubling a level squares the power: not a level
}
